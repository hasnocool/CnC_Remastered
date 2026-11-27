//! Source-derived Tiberian Dawn and Red Alert TerrainType catalogs.
//!
//! Data is transcribed from the pinned upstream source audits:
//! - docs/original-source-reference/MAP_EDITOR/TiberianDawn/TerrainTypes.cs.md
//! - docs/original-source-reference/MAP_EDITOR/RedAlert/TerrainTypes.cs.md
//!
//! Occupancy masks are logical placement footprints. RenderSize and
//! OverlapBounds are deliberately not represented in this catalog.

use crate::{OccupancyMask, OccupancyMaskError, TerrainPlacementDefinition};
use rts_core::{template_catalog::TemplateTheater, GameVariant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainTemplateFlag {
    None,
    Water,
    OreMine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerrainDefinition {
    pub variant: GameVariant,
    /// Original TerrainType sbyte ID, represented as u16 for map cell storage.
    pub id: u16,
    pub name: &'static str,
    pub theaters: &'static [TemplateTheater],
    /// Rows from the source bool[rows, columns] initializer, top to bottom.
    mask_rows: &'static [&'static str],
    pub transformable: bool,
    pub template_flag: TerrainTemplateFlag,
}

impl TerrainDefinition {
    pub fn occupancy(&self) -> Result<OccupancyMask, OccupancyMaskError> {
        let height = self.mask_rows.len();
        let width = self.mask_rows.first().map_or(0, |row| row.len());
        let mut cells = Vec::with_capacity(width * height);
        for row in self.mask_rows {
            if row.len() != width {
                return Err(OccupancyMaskError::IncorrectCellCount {
                    expected: width * height,
                    actual: cells.len() + row.len(),
                });
            }
            for bit in row.bytes() {
                match bit {
                    b'0' => cells.push(false),
                    b'1' => cells.push(true),
                    _ => return Err(OccupancyMaskError::IncorrectCellCount {
                        expected: width * height,
                        actual: cells.len(),
                    }),
                }
            }
        }
        OccupancyMask::new(width as u16, height as u16, cells)
    }

    pub fn supports_theater(&self, theater: TemplateTheater) -> bool {
        self.theaters.contains(&theater)
    }

    /// TerrainTool selects tile 22 for transformable types, otherwise tile 0.
    pub const fn default_icon(&self) -> u8 {
        if self.transformable { 22 } else { 0 }
    }

    pub fn placement_definition(&self) -> Result<TerrainPlacementDefinition, OccupancyMaskError> {
        Ok(TerrainPlacementDefinition {
            id: self.id,
            occupancy: self.occupancy()?,
            transformable: self.transformable,
        })
    }
}

const TD_WT: &[TemplateTheater] = &[TemplateTheater::Winter, TemplateTheater::Temperate];
const TD_D: &[TemplateTheater] = &[TemplateTheater::Desert];
const TD_ALL_TREE8: &[TemplateTheater] = &[TemplateTheater::Winter, TemplateTheater::Temperate, TemplateTheater::Desert];
const TD_TW: &[TemplateTheater] = &[TemplateTheater::Temperate, TemplateTheater::Winter];
const TD_TWD: &[TemplateTheater] = &[TemplateTheater::Temperate, TemplateTheater::Winter, TemplateTheater::Desert];
const RA_TS: &[TemplateTheater] = &[TemplateTheater::Temperate, TemplateTheater::Snow];
const RA_S: &[TemplateTheater] = &[TemplateTheater::Snow];
const RA_I: &[TemplateTheater] = &[TemplateTheater::Interior];

macro_rules! td {
    ($id:expr, $name:literal, $theaters:expr, [$($row:literal),+], $transformable:expr) => {
        TerrainDefinition { variant: GameVariant::TiberianDawn, id: $id, name: $name,
            theaters: $theaters, mask_rows: &[$($row),+], transformable: $transformable,
            template_flag: TerrainTemplateFlag::None }
    };
}
macro_rules! ra {
    ($id:expr, $name:literal, $theaters:expr, [$($row:literal),+], $flag:expr) => {
        TerrainDefinition { variant: GameVariant::RedAlert, id: $id, name: $name,
            theaters: $theaters, mask_rows: &[$($row),+], transformable: false,
            template_flag: $flag }
    };
}

const TD_TERRAINS: &[TerrainDefinition] = &[
    td!(0,"t01",TD_WT,["00","10"],false),
    td!(1,"t02",TD_WT,["00","10"],false),
    td!(2,"t03",TD_WT,["00","10"],false),
    td!(3,"t04",TD_D,["1"],false),
    td!(4,"t05",TD_WT,["00","10"],false),
    td!(5,"t06",TD_WT,["00","10"],false),
    td!(6,"t07",TD_WT,["00","10"],false),
    td!(7,"t08",TD_ALL_TREE8,["10"],false),
    td!(8,"t09",TD_D,["10"],false),
    td!(9,"t10",TD_WT,["00","11"],false),
    td!(10,"t11",TD_WT,["00","11"],false),
    td!(11,"t12",TD_WT,["00","10"],false),
    td!(12,"t13",TD_WT,["00","10"],false),
    td!(13,"t14",TD_WT,["00","11"],false),
    td!(14,"t15",TD_WT,["00","11"],false),
    td!(15,"t16",TD_WT,["00","10"],false),
    td!(16,"t17",TD_WT,["00","10"],false),
    td!(17,"t18",TD_D,["000","010"],false),
    td!(18,"split2",TD_TW,["00","10"],true),
    td!(19,"split3",TD_TWD,["00","10"],true),
    td!(20,"tc01",TD_WT,["000","110"],false),
    td!(21,"tc02",TD_WT,["010","110"],false),
    td!(22,"tc03",TD_WT,["110","110"],false),
    td!(23,"tc04",TD_WT,["0000","1110","1000"],false),
    td!(24,"tc05",TD_WT,["0010","1110","0110"],false),
    td!(25,"rock1",TD_D,["00","11"],false),
    td!(26,"rock2",TD_D,["11","00"],false),
    td!(27,"rock3",TD_D,["000","110"],false),
    td!(28,"rock4",TD_D,["10"],false),
    td!(29,"rock5",TD_D,["10"],false),
    td!(30,"rock6",TD_D,["000","111"],false),
    td!(31,"rock7",TD_D,["11110"],false),
];

const RA_TERRAINS: &[TerrainDefinition] = &[
    ra!(0,"t01",RA_TS,["00","10"],TerrainTemplateFlag::None),
    ra!(1,"t02",RA_TS,["00","10"],TerrainTemplateFlag::None),
    ra!(2,"t03",RA_TS,["00","10"],TerrainTemplateFlag::None),
    ra!(3,"t05",RA_TS,["00","10"],TerrainTemplateFlag::None),
    ra!(4,"t06",RA_TS,["00","10"],TerrainTemplateFlag::None),
    ra!(5,"t07",RA_TS,["00","10"],TerrainTemplateFlag::None),
    ra!(6,"t08",RA_TS,["10"],TerrainTemplateFlag::None),
    ra!(7,"t10",RA_TS,["00","11"],TerrainTemplateFlag::None),
    ra!(8,"t11",RA_TS,["00","11"],TerrainTemplateFlag::None),
    ra!(9,"t12",RA_TS,["00","10"],TerrainTemplateFlag::None),
    ra!(10,"t13",RA_TS,["00","10"],TerrainTemplateFlag::None),
    ra!(11,"t14",RA_TS,["00","11"],TerrainTemplateFlag::None),
    ra!(12,"t15",RA_TS,["00","11"],TerrainTemplateFlag::None),
    ra!(13,"t16",RA_TS,["00","10"],TerrainTemplateFlag::None),
    ra!(14,"t17",RA_TS,["00","10"],TerrainTemplateFlag::None),
    ra!(15,"tc01",RA_TS,["000","110"],TerrainTemplateFlag::None),
    ra!(16,"tc02",RA_TS,["010","110"],TerrainTemplateFlag::None),
    ra!(17,"tc03",RA_TS,["110","110"],TerrainTemplateFlag::None),
    ra!(18,"tc04",RA_TS,["0000","1110","1000"],TerrainTemplateFlag::None),
    ra!(19,"tc05",RA_TS,["0010","1110","0110"],TerrainTemplateFlag::None),
    ra!(20,"ice01",RA_S,["11","11"],TerrainTemplateFlag::Water),
    ra!(21,"ice02",RA_S,["10","10"],TerrainTemplateFlag::Water),
    ra!(22,"ice03",RA_S,["11"],TerrainTemplateFlag::Water),
    ra!(23,"ice04",RA_S,["10"],TerrainTemplateFlag::Water),
    ra!(24,"ice05",RA_S,["10"],TerrainTemplateFlag::Water),
    ra!(25,"boxes01",RA_I,["10"],TerrainTemplateFlag::None),
    ra!(26,"boxes02",RA_I,["10"],TerrainTemplateFlag::None),
    ra!(27,"boxes03",RA_I,["10"],TerrainTemplateFlag::None),
    ra!(28,"boxes04",RA_I,["10"],TerrainTemplateFlag::None),
    ra!(29,"boxes05",RA_I,["10"],TerrainTemplateFlag::None),
    ra!(30,"boxes06",RA_I,["10"],TerrainTemplateFlag::None),
    ra!(31,"boxes07",RA_I,["10"],TerrainTemplateFlag::None),
    ra!(32,"boxes08",RA_I,["10"],TerrainTemplateFlag::None),
    ra!(33,"boxes09",RA_I,["10"],TerrainTemplateFlag::None),
    ra!(34,"mine",RA_TS,["10"],TerrainTemplateFlag::OreMine),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerrainCatalog {
    variant: GameVariant,
    theater: TemplateTheater,
}

impl TerrainCatalog {
    pub const fn new(variant: GameVariant, theater: TemplateTheater) -> Self {
        Self { variant, theater }
    }
    pub const fn variant(&self) -> GameVariant { self.variant }
    pub const fn theater(&self) -> TemplateTheater { self.theater }
    fn entries(&self) -> &'static [TerrainDefinition] {
        match self.variant {
            GameVariant::TiberianDawn => TD_TERRAINS,
            GameVariant::RedAlert => RA_TERRAINS,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &'static TerrainDefinition> + '_ {
        self.entries().iter().filter(move |entry| entry.supports_theater(self.theater))
    }
    pub fn get(&self, id: u16) -> Option<&'static TerrainDefinition> {
        self.iter().find(|entry| entry.id == id)
    }
    pub fn all_entries(variant: GameVariant) -> &'static [TerrainDefinition] {
        match variant {
            GameVariant::TiberianDawn => TD_TERRAINS,
            GameVariant::RedAlert => RA_TERRAINS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_catalogs_have_complete_variant_counts_and_contiguous_ids() {
        assert_eq!(TD_TERRAINS.len(), 32);
        assert_eq!(RA_TERRAINS.len(), 35);
        for (entries, variant) in [(TD_TERRAINS, GameVariant::TiberianDawn), (RA_TERRAINS, GameVariant::RedAlert)] {
            assert!(entries.iter().enumerate().all(|(index, entry)| entry.id == index as u16 && entry.variant == variant));
        }
    }

    #[test]
    fn occupancy_masks_preserve_source_row_and_column_orientation() {
        let td_tree = TerrainCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Temperate).get(0).unwrap().occupancy().unwrap();
        assert_eq!((td_tree.width(), td_tree.height()), (2, 2));
        assert!(td_tree.is_occupied(0, 1));
        assert!(!td_tree.is_occupied(1, 1));
        let ra_ice = TerrainCatalog::new(GameVariant::RedAlert, TemplateTheater::Snow).get(20).unwrap().occupancy().unwrap();
        assert_eq!((ra_ice.width(), ra_ice.height()), (2, 2));
        assert!(ra_ice.is_occupied(0, 0) && ra_ice.is_occupied(1, 0));
    }

    #[test]
    fn theater_filtering_matches_source_registry() {
        let td = TerrainCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Desert);
        assert!(td.get(3).is_some());
        assert!(td.get(0).is_none());
        assert!(td.get(7).is_some());
        let ra = TerrainCatalog::new(GameVariant::RedAlert, TemplateTheater::Interior);
        assert!(ra.get(25).is_some());
        assert!(ra.get(0).is_none());
        let ra_snow = TerrainCatalog::new(GameVariant::RedAlert, TemplateTheater::Snow);
        assert!(ra_snow.get(20).is_some());
        assert!(ra_snow.get(34).is_some());
    }

    #[test]
    fn transformable_and_template_flags_are_source_derived() {
        let td = TerrainCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Temperate);
        assert_eq!(td.get(18).unwrap().default_icon(), 22);
        assert_eq!(td.get(19).unwrap().default_icon(), 22);
        assert_eq!(td.get(0).unwrap().default_icon(), 0);
        let ra = TerrainCatalog::new(GameVariant::RedAlert, TemplateTheater::Snow);
        assert_eq!(ra.get(20).unwrap().template_flag, TerrainTemplateFlag::Water);
        assert_eq!(ra.get(34).unwrap().template_flag, TerrainTemplateFlag::OreMine);
    }

    #[test]
    fn all_masks_are_well_formed_and_nonempty() {
        for definition in TD_TERRAINS.iter().chain(RA_TERRAINS.iter()) {
            let mask = definition.occupancy().unwrap();
            assert!(mask.width() > 0 && mask.height() > 0);
            assert!((0..mask.height()).any(|y| (0..mask.width()).any(|x| mask.is_occupied(x, y))));
        }
    }
}
