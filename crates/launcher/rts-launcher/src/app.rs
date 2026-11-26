use crate::{
    config::LauncherConfig,
    supervisor::{AppTarget, LauncherEvent, LauncherService},
};
use egui::RichText;
use std::{path::PathBuf, sync::mpsc::{Receiver, Sender}};

pub struct LauncherApp {
    pub config: LauncherConfig,
    config_path: PathBuf,
    service: LauncherService,
    status: String,
    servers: Vec<(String, rts_net::DiscoveryBeacon)>,
    config_warning: Option<String>,
    config_save_tx: Sender<Result<(), String>>,
    config_save_rx: Receiver<Result<(), String>>,
}
impl LauncherApp {
    pub fn new(config_path: PathBuf) -> Result<Self, String> {
        let (config, config_warning) = LauncherConfig::load_or_default(&config_path);
        let service = LauncherService::new().map_err(|error| error.to_string())?;
        let (config_save_tx, config_save_rx) = std::sync::mpsc::channel();
        Ok(Self {
            config, config_path, service, status: "Ready".into(), servers: Vec::new(),
            config_warning, config_save_tx, config_save_rx,
        })
    }
    fn launch(&mut self, target: AppTarget, executable: String) {
        let path = PathBuf::from(executable);
        self.status = if self.service.launch(target, path.clone(), Vec::new()) {
            format!("Starting {}", path.display())
        } else { "Launcher worker is unavailable".into() };
    }
    fn save_config(&mut self) {
        let config = self.config.clone();
        let path = self.config_path.clone();
        let sender = self.config_save_tx.clone();
        std::thread::spawn(move || {
            let result = config.save_atomic(&path).map_err(|error| error.to_string());
            let _ = sender.send(result);
        });
        self.status = "Settings save queued".into();
    }
}
impl eframe::App for LauncherApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for event in self.service.poll_events() {
            match event {
                LauncherEvent::Started { target, pid } => self.status = format!("{target:?} running (pid {pid:?})"),
                LauncherEvent::Exited { target, code } => self.status = format!("{target:?} exited ({code:?})"),
                LauncherEvent::LaunchFailed { target, message } => self.status = format!("{target:?} launch failed: {message}"),
                LauncherEvent::ServersFound(servers) => {
                    self.servers = servers;
                    self.status = format!("Discovered {} server(s)", self.servers.len());
                }
                LauncherEvent::NetworkFailed(error) => self.status = format!("Network discovery failed: {error}"),
            }
        }
        while let Ok(result) = self.config_save_rx.try_recv() {
            self.status = match result {
                Ok(()) => "Settings saved".into(),
                Err(error) => format!("Settings save failed: {error}"),
            };
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(250));
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let td_path = self.config.paths.tiberian_dawn_executable.clone();
        let ra_path = self.config.paths.red_alert_executable.clone();
        let editor_path = self.config.paths.map_editor_executable.clone();
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("COMMAND CENTER").strong().size(28.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("RUST REMASTER").small().weak());
                });
            });
            ui.label("Tiberian Dawn  •  Red Alert  •  Map Editor");
            ui.separator();
            ui.columns(3, |columns| {
                columns[0].group(|ui| {
                    ui.heading("Tiberian Dawn");
                    ui.label("Standalone application");
                    if ui.button("Launch Tiberian Dawn").clicked() {
                        self.launch(AppTarget::TiberianDawn, td_path.clone());
                    }
                });
                columns[1].group(|ui| {
                    ui.heading("Red Alert");
                    ui.label("Standalone application");
                    if ui.button("Launch Red Alert").clicked() {
                        self.launch(AppTarget::RedAlert, ra_path.clone());
                    }
                });
                columns[2].group(|ui| {
                    ui.heading("Map Editor");
                    ui.label("Create and edit maps");
                    if ui.button("Open Map Editor").clicked() {
                        self.launch(AppTarget::MapEditor, editor_path.clone());
                    }
                });
            });
            ui.separator();
            ui.heading("Multiplayer");
            ui.horizontal(|ui| {
                ui.label("LAN discovery");
                if ui.button("Find servers").clicked() {
                    self.status = if self.service.discover(self.config.network.discovery_port) {
                        "Searching local network for servers…".into()
                    } else { "Network worker is unavailable".into() };
                }
                ui.label(format!("UDP {}", self.config.network.discovery_port));
            });
            if self.servers.is_empty() {
                ui.label("No servers discovered yet. Discovery uses UDP broadcast on the local network.");
            } else {
                for (address, beacon) in &self.servers {
                    ui.label(format!("{} — {:?} — {}/{} — {} ({address})",
                        beacon.name, beacon.game, beacon.players, beacon.max_players, beacon.map_name));
                }
            }
            ui.separator();
            ui.heading("Options");
            egui::Grid::new("launcher-options").num_columns(2).show(ui, |ui| {
                ui.label("Resolution width");
                ui.add(egui::DragValue::new(&mut self.config.graphics.resolution_width).range(640..=7680));
                ui.end_row();
                ui.label("Resolution height");
                ui.add(egui::DragValue::new(&mut self.config.graphics.resolution_height).range(480..=4320));
                ui.end_row();
                ui.label("Frame limit");
                ui.add(egui::Slider::new(&mut self.config.graphics.frame_limit, 30..=360));
                ui.end_row();
                ui.label("Fullscreen");
                ui.checkbox(&mut self.config.graphics.fullscreen, "");
                ui.end_row();
                ui.label("VSync");
                ui.checkbox(&mut self.config.graphics.vsync, "");
                ui.end_row();
                ui.label("Master volume");
                ui.add(egui::Slider::new(&mut self.config.audio.master_volume, 0.0..=1.0));
                ui.end_row();
                ui.label("Region");
                ui.text_edit_singleline(&mut self.config.network.region);
                ui.end_row();
            });
            if ui.button("Save settings").clicked() { self.save_config(); }
            if let Some(warning) = &self.config_warning {
                ui.colored_label(egui::Color32::YELLOW, warning);
            }
            ui.separator();
            ui.label(format!("Status: {}", self.status));
        });
    }
}
