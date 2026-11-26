//! rts-launcher — standalone game and editor launcher.

use std::{env, path::PathBuf};

fn main() -> eframe::Result {
    let mut args = env::args().skip(1);
    let mut config = PathBuf::from("configs/launcher.toml");
    while let Some(argument) = args.next() {
        if argument == "--config" {
            if let Some(value) = args.next() {
                config = PathBuf::from(value);
            }
        }
    }

    let app = rts_launcher::LauncherApp::new(config)
        .unwrap_or_else(|error| panic!("Unable to initialize launcher: {error}"));
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1120.0, 760.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CnC Remastered — Command Center",
        options,
        Box::new(move |_creation_context| Ok(Box::new(app))),
    )
}
