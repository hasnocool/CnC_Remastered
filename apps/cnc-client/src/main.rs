//! Lightweight bootstrap executable that starts the user-facing rts-launcher.

use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

fn launcher_path() -> Result<PathBuf, String> {
    if let Some(explicit) = env::var_os("RTS_LAUNCHER") {
        return Ok(PathBuf::from(explicit));
    }
    let current = env::current_exe().map_err(|error| error.to_string())?;
    let directory = current.parent().unwrap_or(Path::new("."));
    let name = if cfg!(windows) { "rts-launcher.exe" } else { "rts-launcher" };
    Ok(directory.join(name))
}

fn main() -> ExitCode {
    let executable = match launcher_path() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Cannot locate launcher: {error}");
            return ExitCode::FAILURE;
        }
    };
    match Command::new(&executable).args(env::args_os().skip(1)).spawn() {
        Ok(mut child) => match child.wait() {
            Ok(status) => if status.success() { ExitCode::SUCCESS } else { ExitCode::FAILURE },
            Err(error) => {
                eprintln!("Launcher process monitor failed: {error}");
                ExitCode::FAILURE
            }
        },
        Err(error) => {
            eprintln!("Unable to start {}: {error}", executable.display());
            ExitCode::FAILURE
        }
    }
}
