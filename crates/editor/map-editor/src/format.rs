//! Versioned JSON editor interchange format; not a legacy .MAP parity claim.

use map_editor_model::{CellLayers, MapDocument};
use rts_core::{CellCoord, GameVariant, MapDimensions};
use serde::{Deserialize, Serialize};

pub const MAX_MAP_FILE_BYTES: u64 = 4 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapFile {
    pub format_version: u16,
    pub game: String,
    pub width: u16,
    pub height: u16,
    pub cells: Vec<CellLayers>,
    pub waypoints: Vec<WaypointFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointFile {
    pub number: u16,
    pub x: u16,
    pub y: u16,
}

impl MapFile {
    pub fn from_document(document: &MapDocument) -> Self {
        let dimensions = document.dimensions();
        let waypoints = document.waypoints().iter().map(|(number, cell)| WaypointFile {
            number: *number, x: cell.x, y: cell.y,
        }).collect();
        Self {
            format_version: 1,
            game: match document.variant() {
                GameVariant::TiberianDawn => "tiberian_dawn",
                GameVariant::RedAlert => "red_alert",
            }.into(),
            width: dimensions.width(),
            height: dimensions.height(),
            cells: document.cells().to_vec(),
            waypoints,
        }
    }

    pub fn into_document(self) -> Result<MapDocument, String> {
        if self.format_version != 1 {
            return Err(format!("Unsupported editor map version {}", self.format_version));
        }
        let variant = match self.game.as_str() {
            "tiberian_dawn" => GameVariant::TiberianDawn,
            "red_alert" => GameVariant::RedAlert,
            other => return Err(format!("Unknown game variant '{other}'")),
        };
        let dimensions = MapDimensions::new(self.width, self.height)
            .map_err(|error| format!("Invalid map dimensions: {error:?}"))?;
        if self.cells.len() != dimensions.cell_count() {
            return Err(format!("Cell count mismatch: expected {}, found {}", dimensions.cell_count(), self.cells.len()));
        }
        let mut document = MapDocument::new(variant, dimensions)
            .map_err(|error| format!("Map rejected: {error:?}"))?;
        for (index, layers) in self.cells.into_iter().enumerate() {
            let coordinate = dimensions.coord(index).ok_or_else(|| "Invalid cell index".to_string())?;
            document.set_cell(coordinate, layers).map_err(|error| format!("Invalid cell: {error:?}"))?;
        }
        for waypoint in self.waypoints {
            document.add_waypoint(waypoint.number, CellCoord::new(waypoint.x, waypoint.y))
                .map_err(|error| format!("Invalid waypoint: {error:?}"))?;
        }
        Ok(document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_file_round_trips_cells_and_waypoints() {
        let dimensions = MapDimensions::new(8, 6).expect("dimensions");
        let mut document = MapDocument::new(GameVariant::RedAlert, dimensions).expect("document");
        document.set_cell(CellCoord::new(3, 2), CellLayers {
            terrain: Some(17), overlay: Some(4), ..Default::default()
        }).expect("cell");
        document.add_waypoint(2, CellCoord::new(1, 4)).expect("waypoint");
        let encoded = serde_json::to_vec(&MapFile::from_document(&document)).expect("encode");
        let decoded: MapFile = serde_json::from_slice(&encoded).expect("decode");
        let restored = decoded.into_document().expect("restore");
        assert_eq!(restored.cell(CellCoord::new(3, 2)), document.cell(CellCoord::new(3, 2)));
        assert_eq!(restored.waypoints(), document.waypoints());
    }

    #[test]
    fn rejects_cell_count_mismatch() {
        let file = MapFile {
            format_version: 1, game: "red_alert".into(), width: 2, height: 2,
            cells: vec![], waypoints: vec![],
        };
        assert!(file.into_document().is_err());
    }
}
