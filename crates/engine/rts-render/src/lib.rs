//! Shared 2D rendering primitives for both game variants and the map editor.
//!
//! The first backend uses egui's painter, which is GPU-backed by eframe/wgpu in
//! native applications. Simulation state is deliberately not owned here.

use egui::{Color32, CornerRadius, Painter, Rect, Response, Sense, Stroke, StrokeKind, Vec2};
use map_editor_model::MapDocument;
use rts_core::CellCoord;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera2d {
    pub origin: Vec2,
    pub zoom: f32,
}

impl Default for Camera2d {
    fn default() -> Self {
        Self { origin: Vec2::ZERO, zoom: 1.0 }
    }
}

impl Camera2d {
    pub fn world_to_screen(self, world: Vec2) -> Vec2 {
        self.origin + world * self.zoom
    }

    pub fn screen_to_world(self, screen: Vec2) -> Vec2 {
        (screen - self.origin) / self.zoom.max(0.001)
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.25, 4.0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderPalette {
    pub clear: Color32,
    pub ground: Color32,
    pub terrain: Color32,
    pub overlay: Color32,
    pub template: Color32,
    pub smudge: Color32,
    pub grid: Color32,
    pub selection: Color32,
}

impl Default for RenderPalette {
    fn default() -> Self {
        Self {
            clear: Color32::from_rgb(17, 25, 22),
            ground: Color32::from_rgb(67, 91, 57),
            terrain: Color32::from_rgb(94, 111, 63),
            overlay: Color32::from_rgb(137, 117, 76),
            template: Color32::from_rgb(119, 123, 116),
            smudge: Color32::from_rgb(75, 68, 56),
            grid: Color32::from_rgba_unmultiplied(12, 18, 14, 100),
            selection: Color32::YELLOW,
        }
    }
}

pub struct MapCanvas {
    pub cell_size: f32,
    pub selected: Option<CellCoord>,
    pub show_grid: bool,
    pub palette: RenderPalette,
}

impl Default for MapCanvas {
    fn default() -> Self {
        Self { cell_size: 20.0, selected: None, show_grid: true, palette: RenderPalette::default() }
    }
}

impl MapCanvas {
    /// Allocate a scrollable map canvas, draw all cells, and return the clicked cell.
    pub fn show(&mut self, ui: &mut egui::Ui, map: &MapDocument) -> (Response, Option<CellCoord>) {
        let dimensions = map.dimensions();
        let cell_size = self.cell_size.clamp(4.0, 64.0);
        let desired = Vec2::new(
            dimensions.width() as f32 * cell_size,
            dimensions.height() as f32 * cell_size,
        );
        let (rect, response) = ui.allocate_exact_size(desired, Sense::click_and_drag());
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, CornerRadius::ZERO, self.palette.clear);
        self.paint_cells(&painter, rect, map, cell_size);
        if let Some(selected) = self.selected {
            let selected_rect = cell_rect(rect, selected, cell_size);
            painter.rect_stroke(
                selected_rect.shrink(1.0),
                CornerRadius::ZERO,
                Stroke::new(2.0, self.palette.selection),
                StrokeKind::Inside,
            );
        }

        let clicked = if response.clicked() || response.dragged() {
            response.interact_pointer_pos().and_then(|position| {
                let local = position - rect.min;
                let x = (local.x / cell_size).floor();
                let y = (local.y / cell_size).floor();
                if x >= 0.0 && y >= 0.0 && x < dimensions.width() as f32 && y < dimensions.height() as f32 {
                    Some(CellCoord::new(x as u16, y as u16))
                } else {
                    None
                }
            })
        } else {
            None
        };
        (response, clicked)
    }

    fn paint_cells(&self, painter: &Painter, rect: Rect, map: &MapDocument, cell_size: f32) {
        let dimensions = map.dimensions();
        for y in 0..dimensions.height() {
            for x in 0..dimensions.width() {
                let coordinate = CellCoord::new(x, y);
                let Some(layers) = map.cell(coordinate) else { continue };
                let color = if layers.template.is_some() {
                    self.palette.template
                } else if layers.overlay.is_some() {
                    self.palette.overlay
                } else if layers.smudge.is_some() {
                    self.palette.smudge
                } else if layers.terrain.is_some() {
                    self.palette.terrain
                } else {
                    self.palette.ground
                };
                let cell = cell_rect(rect, coordinate, cell_size);
                painter.rect_filled(cell, CornerRadius::ZERO, color);
                if self.show_grid {
                    painter.rect_stroke(
                        cell,
                        CornerRadius::ZERO,
                        Stroke::new(0.5, self.palette.grid),
                        StrokeKind::Inside,
                    );
                }
            }
        }
    }
}

fn cell_rect(map_rect: Rect, cell: CellCoord, cell_size: f32) -> Rect {
    let min = map_rect.min + Vec2::new(cell.x as f32 * cell_size, cell.y as f32 * cell_size);
    Rect::from_min_size(min, Vec2::splat(cell_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_zoom_is_clamped_and_round_trips() {
        let mut camera = Camera2d::default();
        camera.set_zoom(100.0);
        assert_eq!(camera.zoom, 4.0);
        let point = Vec2::new(12.0, 19.0);
        assert_eq!(camera.screen_to_world(camera.world_to_screen(point)), point);
    }
}
