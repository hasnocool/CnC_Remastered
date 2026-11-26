//! Red Alert application shell.
//! This executable exercises the shared renderer; full gameplay and asset integration remain in progress.

use map_editor_model::{CellLayers, MapDocument};
use rts_core::{CellCoord, GameVariant, MapDimensions};
use rts_render::MapCanvas;

struct RedAlertApp {
    map: MapDocument,
    canvas: MapCanvas,
}

impl RedAlertApp {
    fn new() -> Self {
        let dimensions = MapDimensions::new(48, 48).expect("valid demo dimensions");
        let mut map = MapDocument::new(GameVariant::RedAlert, dimensions).expect("valid demo map");
        for y in 10..18 {
            for x in 10..28 {
                let _ = map.set_cell(CellCoord::new(x, y), CellLayers {
                    template: Some(1), ..Default::default()
                });
            }
        }
        Self { map, canvas: MapCanvas::default() }
    }
}

impl eframe::App for RedAlertApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Red Alert");
            ui.label("Application shell — original gameplay and asset runtime integration remains in progress.");
            egui::ScrollArea::both().show(ui, |ui| { let _ = self.canvas.show(ui, &self.map); });
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Red Alert",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(RedAlertApp::new()))),
    )
}
