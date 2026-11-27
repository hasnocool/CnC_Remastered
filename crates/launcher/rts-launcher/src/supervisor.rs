//! Process and server-discovery work runs on a dedicated Tokio worker thread.

use rts_net::{discover_servers, DiscoveryBeacon};
use std::{
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};
use tokio::{process::Command, sync::mpsc as tokio_mpsc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTarget {
    TiberianDawn,
    RedAlert,
    MapEditor,
}

#[derive(Debug)]
enum WorkerCommand {
    Launch { target: AppTarget, executable: PathBuf, args: Vec<String> },
    Discover { port: u16 },
}

#[derive(Debug, Clone)]
pub enum LauncherEvent {
    Started { target: AppTarget, pid: Option<u32> },
    Exited { target: AppTarget, code: Option<i32> },
    LaunchFailed { target: AppTarget, message: String },
    ServersFound(Vec<(String, DiscoveryBeacon)>),
    NetworkFailed(String),
}

pub struct LauncherService {
    commands: tokio_mpsc::UnboundedSender<WorkerCommand>,
    events: Receiver<LauncherEvent>,
}

impl LauncherService {
    pub fn new() -> std::io::Result<Self> {
        let (command_tx, mut command_rx) = tokio_mpsc::unbounded_channel::<WorkerCommand>();
        let (event_tx, event_rx) = mpsc::channel::<LauncherEvent>();
        std::thread::Builder::new()
            .name("rts-launcher-worker".into())
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build();
                let Ok(runtime) = runtime else {
                    let _ = event_tx.send(LauncherEvent::NetworkFailed("Unable to initialize Tokio runtime".into()));
                    return;
                };
                runtime.block_on(async move {
                    while let Some(command) = command_rx.recv().await {
                        match command {
                            WorkerCommand::Launch { target, executable, args } => {
                                let events = event_tx.clone();
                                tokio::spawn(async move {
                                    let mut child = match Command::new(&executable).args(&args).spawn() {
                                        Ok(child) => child,
                                        Err(error) => {
                                            let _ = events.send(LauncherEvent::LaunchFailed {
                                                target,
                                                message: format!("{}: {error}", executable.display()),
                                            });
                                            return;
                                        }
                                    };
                                    let _ = events.send(LauncherEvent::Started { target, pid: child.id() });
                                    match child.wait().await {
                                        Ok(status) => {
                                            let _ = events.send(LauncherEvent::Exited { target, code: status.code() });
                                        }
                                        Err(error) => {
                                            let _ = events.send(LauncherEvent::LaunchFailed {
                                                target,
                                                message: format!("Process monitor failed: {error}"),
                                            });
                                        }
                                    }
                                });
                            }
                            WorkerCommand::Discover { port } => {
                                let events = event_tx.clone();
                                tokio::spawn(async move {
                                    match discover_servers(port, Duration::from_secs(2)).await {
                                        Ok(servers) => {
                                            let servers = servers.into_iter()
                                                .map(|(address, beacon)| (address.to_string(), beacon))
                                                .collect();
                                            let _ = events.send(LauncherEvent::ServersFound(servers));
                                        }
                                        Err(error) => { let _ = events.send(LauncherEvent::NetworkFailed(error.to_string())); }
                                    }
                                });
                            }
                        }
                    }
                });
            })?;
        Ok(Self { commands: command_tx, events: event_rx })
    }

    pub fn launch(&self, target: AppTarget, executable: PathBuf, args: Vec<String>) -> bool {
        self.commands.send(WorkerCommand::Launch { target, executable, args }).is_ok()
    }

    pub fn discover(&self, port: u16) -> bool {
        self.commands.send(WorkerCommand::Discover { port }).is_ok()
    }

    pub fn poll_events(&self) -> impl Iterator<Item = LauncherEvent> + '_ {
        self.events.try_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launch_target_is_copyable_and_distinct() {
        assert_ne!(AppTarget::TiberianDawn, AppTarget::RedAlert);
        assert_ne!(AppTarget::RedAlert, AppTarget::MapEditor);
    }
}
