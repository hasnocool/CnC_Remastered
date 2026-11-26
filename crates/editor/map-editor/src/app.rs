use crate::format::{MapFile, MAX_MAP_FILE_BYTES};
use egui::RichText;
use map_editor_model::{CellLayers, MapDocument};
use rts_core::{CellCoord, GameVariant, MapDimensions};
use rts_render::MapCanvas;
use std::{
    fs,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
};

enum IoCommand {
    Save(PathBuf, MapFile),
    Load(PathBuf),
}
enum IoEvent {
    Saved(Result<(), String>),
    Loaded(Result<MapDocument, String>),
}

pub struct EditorApp {
    map: MapDocument,
    canvas: MapCanvas,
    selected: Option<CellCoord>,
    brush: u16,
    file_path: String,
    status: String,
    io_tx: Sender<IoCommand>,
    io_rx: Receiver<IoEvent>,
}

impl EditorApp {
    pub fn new() -> Result<Self, String> {
        let dimensions = MapDimensions::new(32, 32).map_err(|error| format!("{error:?}"))?;
        let map = MapDocument::new(GameVariant::TiberianDawn, dimensions)
            .map_err(|error| format!("{error:?}"))?;

        let (event_tx, io_rx) = mpsc::channel::<IoEvent>();
        let (io_tx, command_rx) = mpsc::channel::<IoCommand>();
        std::thread::Builder::new().name("map-editor-file-io".into()).spawn(move || {
            while let Ok(command) = command_rx.recv() {
                match command {
                    IoCommand::Save(path, file) => {
                        let result = serde_json::to_vec_pretty(&file)
                            .map_err(|error| error.to_string())
                            .and_then(|bytes| {
                                if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
                                    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
                                }
                                fs::write(path, bytes).map_err(|error| error.to_string())
                            });
                        let _ = event_tx.send(IoEvent::Saved(result));
                    }
                    IoCommand::Load(path) => {
                        let result = fs::metadata(&path)
                            .map_err(|error| error.to_string())
                            .and_then(|metadata| {
                                if metadata.len() > MAX_MAP_FILE_BYTES {
                                    return Err(format!("Map file exceeds the {} byte safety limit", MAX_MAP_FILE_BYTES));
                                }
                                fs::read(path).map_err(|error| error.to_string())
                            })
                            .and_then(|bytes| serde_json::from_slice::<MapFile>(&bytes).map_err(|error| error.to_string()))
                            .and_then(MapFile::into_document);
                        let _ = event_tx.send(IoEvent::Loaded(result));
                    }
                }
            }
        }).map_err(|error| error.to_string())?;

        Ok(Self {
            map,
            canvas: MapCanvas::default(),
            selected: None,
            brush: 1,
            file_path: "./maps/new-map.rtsmap.json".into(),
            status: "New Tiberian Dawn map".into(),
            io_tx,
            io_rx,
        })
    }

    fn new_map(&mut self, variant: GameVariant) {
        let maximum = variant.global_dimensions();
        let dimensions = MapDimensions::new(maximum.width(), maximum.height())
            .expect("non-zero variant map dimensions");
        match MapDocument::new(variant, dimensions) {
            Ok(map) => {
                self.map = map;
                self.selected = None;
                self.canvas.selected = None;
                self.status = "New map created".into();
            }
            Err(error) => self.status = format!("Could not create map: {error:?}"),
        }
    }

    fn save(&mut self) {
        let path = PathBuf::from(self.file_path.trim());
        if path.as_os_str().is_empty() {
            self.status = "Enter a map file path first".into();
            return;
        }
        if self.io_tx.send(IoCommand::Save(path, MapFile::from_document(&self.map))).is_ok() {
            self.status = "Saving map…".into();
        } else {
            self.status = "Map file worker stopped".into();
        }
    }

    fn load(&mut self) {
        let path = PathBuf::from(self.file_path.trim());
        if path.as_os_str().is_empty() {
            self.status = "Enter a map file path first".into();
            return;
        }
        if self.io_tx.send(IoCommand::Load(path)).is_ok() {
            self.status = "Loading map…".into();
        } else {
            self.status = "Map file worker stopped".into();
        }
    }
}

impl eframe::App for EditorApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(event) = self.io_rx.try_recv() {
            match event {
                IoEvent::Saved(Ok(())) => self.status = "Map saved".into(),
                IoEvent::Saved(Err(error)) => self.status = format!("Save failed: {error}"),
                IoEvent::Loaded(Ok(map)) => {
                    self.map = map;
                    self.selected = None;
                    self.canvas.selected = None;
                    self.status = "Map loaded".into();
                }
                IoEvent::Loaded(Err(error)) => self.status = format!("Load failed: {error}"),
            }
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(200));
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(RichText::new("MAP EDITOR").strong().size(26.0));
            ui.horizontal_wrapped(|ui| {
                if ui.button("New Tiberian Dawn").clicked() { self.new_map(GameVariant::TiberianDawn); }
                if ui.button("New Red Alert").clicked() { self.new_map(GameVariant::RedAlert); }
                if ui.button("Save").clicked() { self.save(); }
                if ui.button("Load").clicked() { self.load(); }
                ui.checkbox(&mut self.canvas.show_grid, "Grid");
                ui.add(egui::Slider::new(&mut self.canvas.cell_size, 8.0..=40.0).text("Zoom"));
            });
            ui.horizontal(|ui| {
                ui.label("File");
                ui.text_edit_singleline(&mut self.file_path);
                ui.label("Terrain ID");
                ui.add(egui::DragValue::new(&mut self.brush).range(0..=u16::MAX));
                ui.label("Selected");
                ui.label(self.selected.map(|cell| format!("{}, {}", cell.x, cell.y)).unwrap_or_else(|| "none".into()));
            });
            ui.separator();
            egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                let (_response, clicked) = self.canvas.show(ui, &self.map);
                if let Some(cell) = clicked {
                    self.selected = Some(cell);
                    self.canvas.selected = Some(cell);
                    let mut layers = self.map.cell(cell).copied().unwrap_or_default();
                    layers.terrain = Some(self.brush);
                    if let Err(error) = self.map.set_cell(cell, layers) {
                        self.status = format!("Paint failed: {error:?}");
                    }
                }
            });
            ui.separator();
            ui.label(format!(
                "{:?} — {} × {} — {} waypoints",
                self.map.variant(),
                self.map.dimensions().width(),
                self.map.dimensions().height(),
                self.map.waypoints().len()
            ));
            ui.label(&self.status);
        });
    }
}
