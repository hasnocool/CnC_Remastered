use crate::format::{MapFile, MAX_MAP_FILE_BYTES};
use egui::RichText;
use map_editor_model::{CellLayers, MapDocument};
use rts_core::{CellCoord, GameVariant, MapDimensions, template_catalog::TemplateTheater};
use rts_assets::{TemplateAssetProvider, TerrainAssetSet};
use rts_formats::{mix::MixArchive, palette_rgba::{Palette256, PaletteEncoding}};
use rts_render::MapCanvas;
use std::{
    fs,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PaintLayer { Terrain, Template, Overlay, Smudge }

enum IoCommand {
    Save(PathBuf, MapFile),
    Load(PathBuf),
    LoadAssets { mix_path: PathBuf, palette_path: PathBuf, variant: GameVariant, theater: TemplateTheater, palette_encoding: PaletteEncoding, width: u32, height: u32, templates: Vec<Option<(u16, u8)>> },
}
enum IoEvent {
    Saved(Result<(), String>),
    Loaded(Result<MapDocument, String>),
    Assets(Result<TerrainAssetSet, String>),
}

pub struct EditorApp {
    map: MapDocument,
    canvas: MapCanvas,
    selected: Option<CellCoord>,
    brush: u16,
    brush_layer: PaintLayer,
    waypoint_number: u16,
    file_path: String,
    mix_path: String,
    palette_path: String,
    theater: TemplateTheater,
    palette_encoding: PaletteEncoding,
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
                    IoCommand::LoadAssets { mix_path, palette_path, variant, theater, palette_encoding, width, height, templates } => {
                        let result = (|| {
                            const MAX_MIX_BYTES: u64 = 512 * 1024 * 1024;
                            let mix_size = fs::metadata(&mix_path).map_err(|error| error.to_string())?.len();
                            if mix_size > MAX_MIX_BYTES {
                                return Err(format!("MIX archive exceeds the {MAX_MIX_BYTES} byte safety limit"));
                            }
                            let mix_bytes = fs::read(&mix_path).map_err(|error| error.to_string())?;
                            let palette_size = fs::metadata(&palette_path).map_err(|error| error.to_string())?.len();
                            if palette_size != 768 {
                                return Err(format!("Palette must contain exactly 768 RGB bytes, got {palette_size}"));
                            }
                            let palette_bytes = fs::read(&palette_path).map_err(|error| error.to_string())?;
                            let archive = MixArchive::parse(&mix_bytes).map_err(|error| error.to_string())?;
                            let archives = [&archive];
                            let provider = TemplateAssetProvider::new(variant, theater, &archives);
                            let palette = Palette256::parse(&palette_bytes, palette_encoding).map_err(|error| error.to_string())?;
                            provider.build_terrain(width, height, templates, &palette).map_err(|error| error.to_string())
                        })();
                        let _ = event_tx.send(IoEvent::Assets(result));
                    }
                }
            }
        }).map_err(|error| error.to_string())?;

        Ok(Self {
            map,
            canvas: MapCanvas::default(),
            selected: None,
            brush: 1,
            brush_layer: PaintLayer::Terrain,
            waypoint_number: 0,
            file_path: "./maps/new-map.rtsmap.json".into(),
            mix_path: "./assets/conquer.mix".into(),
            palette_path: "./assets/temperat.pal".into(),
            theater: TemplateTheater::Temperate,
            palette_encoding: PaletteEncoding::Rgb8,
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

    fn load_assets(&mut self) {
        let mix_path = PathBuf::from(self.mix_path.trim());
        let palette_path = PathBuf::from(self.palette_path.trim());
        if mix_path.as_os_str().is_empty() || palette_path.as_os_str().is_empty() {
            self.status = "Enter both a MIX archive and palette path".into();
            return;
        }
        let dimensions = self.map.dimensions();
        let command = IoCommand::LoadAssets {
            mix_path,
            palette_path,
            variant: self.map.variant(),
            theater: self.theater,
            palette_encoding: self.palette_encoding,
            width: dimensions.width(),
            height: dimensions.height(),
            templates: self.map.template_cells().collect(),
        };
        if self.io_tx.send(command).is_ok() {
            self.status = "Loading theater assets in background…".into();
        } else {
            self.status = "Asset worker stopped".into();
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
                    self.canvas.clear_terrain_assets();
                    self.status = "Map loaded".into();
                }
                IoEvent::Loaded(Err(error)) => self.status = format!("Load failed: {error}"),
                IoEvent::Assets(Ok(assets)) => {
                    self.canvas.set_terrain_assets(ctx, assets);
                    self.status = "Authentic theater terrain loaded".into();
                }
                IoEvent::Assets(Err(error)) => self.status = format!("Theater asset load failed: {error}"),
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
                if ui.button("Load Theater Assets").clicked() { self.load_assets(); }
                ui.checkbox(&mut self.canvas.show_grid, "Grid");
                ui.add(egui::Slider::new(&mut self.canvas.cell_size, 8.0..=40.0).text("Zoom"));
            });
            ui.horizontal(|ui| {
                ui.label("File");
                ui.text_edit_singleline(&mut self.file_path);
                egui::ComboBox::from_label("Layer")
                    .selected_text(format!("{:?}", self.brush_layer))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.brush_layer, PaintLayer::Terrain, "Terrain");
                        ui.selectable_value(&mut self.brush_layer, PaintLayer::Template, "Template");
                        ui.selectable_value(&mut self.brush_layer, PaintLayer::Overlay, "Overlay");
                        ui.selectable_value(&mut self.brush_layer, PaintLayer::Smudge, "Smudge");
                    });
                ui.label("ID");
                ui.add(egui::DragValue::new(&mut self.brush).range(0..=u16::MAX));
                ui.label("Waypoint");
                ui.add(egui::DragValue::new(&mut self.waypoint_number));
                if ui.button("Add at selected").clicked() {
                    if let Some(cell) = self.selected {
                        self.status = match self.map.add_waypoint(self.waypoint_number, cell) {
                            Ok(()) => format!("Waypoint {} added", self.waypoint_number),
                            Err(error) => format!("Waypoint rejected: {error:?}"),
                        };
                    } else {
                        self.status = "Select a map cell before adding a waypoint".into();
                    }
                }
                ui.label("Selected");
                ui.label(self.selected.map(|cell| format!("{}, {}", cell.x, cell.y)).unwrap_or_else(|| "none".into()));
            });
            ui.horizontal(|ui| {
                ui.label("MIX");
                ui.text_edit_singleline(&mut self.mix_path);
                ui.label("Palette");
                ui.text_edit_singleline(&mut self.palette_path);
            });
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Theater")
                    .selected_text(format!("{:?}", self.theater))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.theater, TemplateTheater::Desert, "Desert");
                        ui.selectable_value(&mut self.theater, TemplateTheater::Jungle, "Jungle");
                        ui.selectable_value(&mut self.theater, TemplateTheater::Temperate, "Temperate");
                        ui.selectable_value(&mut self.theater, TemplateTheater::Winter, "Winter");
                        ui.selectable_value(&mut self.theater, TemplateTheater::Snow, "Snow");
                        ui.selectable_value(&mut self.theater, TemplateTheater::Interior, "Interior");
                    });
                egui::ComboBox::from_label("Palette encoding")
                    .selected_text(match self.palette_encoding { PaletteEncoding::Rgb8 => "RGB8", PaletteEncoding::Rgb6 => "RGB6" })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.palette_encoding, PaletteEncoding::Rgb8, "RGB8");
                        ui.selectable_value(&mut self.palette_encoding, PaletteEncoding::Rgb6, "RGB6");
                    });
            });
            ui.separator();
            egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                let (_response, clicked) = self.canvas.show(ui, &self.map);
                if let Some(cell) = clicked {
                    self.selected = Some(cell);
                    self.canvas.selected = Some(cell);
                    let mut layers = self.map.cell(cell).copied().unwrap_or_default();
                    match self.brush_layer {
                        PaintLayer::Terrain => layers.terrain = Some(self.brush),
                        PaintLayer::Template => layers.template = Some(self.brush),
                        PaintLayer::Overlay => layers.overlay = Some(self.brush),
                        PaintLayer::Smudge => layers.smudge = Some(self.brush),
                    }
                    if let Err(error) = self.map.set_cell(cell, layers) {
                        self.status = format!("Paint failed: {error:?}");
                    } else if matches!(self.brush_layer, PaintLayer::Template) {
                        self.canvas.clear_terrain_assets();
                        self.status = "Template changed; reload theater assets to refresh terrain".into();
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
