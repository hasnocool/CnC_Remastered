//! Tiberian Dawn application shell.
//! This executable exercises the shared renderer; full gameplay and asset integration remain in progress.

use map_editor_model::{CellLayers, MapDocument};
use rts_core::{CellCoord, GameVariant, MapDimensions};
use rts_render::MapCanvas;

struct TiberianDawnApp {
    map: MapDocument,
    canvas: MapCanvas,
}

impl TiberianDawnApp {
    fn new() -> Self {
        let dimensions = MapDimensions::new(32, 32).expect("valid demo dimensions");
        let mut map = MapDocument::new(GameVariant::TiberianDawn, dimensions).expect("valid demo map");
        for y in 8..16 {
            for x in 8..20 {
                let _ = map.set_cell(CellCoord::new(x, y), CellLayers {
                    terrain: Some(1), ..Default::default()
                });
            }
        }
        Self { map, canvas: MapCanvas::default() }
    }
}

impl eframe::App for TiberianDawnApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Tiberian Dawn");
            ui.label("Application shell — original gameplay and asset runtime integration remains in progress.");
            egui::ScrollArea::both().show(ui, |ui| { let _ = self.canvas.show(ui, &self.map); });
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "Tiberian Dawn",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(TiberianDawnApp::new()))),
    )
}
