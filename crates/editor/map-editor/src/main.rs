//! Standalone map editor executable.

mod app;
mod format;

fn main() -> eframe::Result {
    let app = app::EditorApp::new()
        .unwrap_or_else(|error| panic!("Unable to initialize map editor: {error}"));
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 860.0])
            .with_min_inner_size([900.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CnC Remastered — Map Editor",
        options,
        Box::new(move |_creation_context| Ok(Box::new(app))),
    )
}
