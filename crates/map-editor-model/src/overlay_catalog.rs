//! Source-derived TD/RA overlay definitions and placement rules.
//!
//! Catalog values are transcribed from the pinned Map Editor source audit.
//! Rendering thumbnails and localized display strings remain external assets;
//! this module stores stable source keys and gameplay/editor metadata only.

use crate::{MapDocument, MapDocumentError, MapEditHistory};
use rts_core::{template_catalog::TemplateTheater, CellCoord, GameVariant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlayTypeFlags(u8);

impl OverlayTypeFlags {
    pub const NONE: Self = Self(0);
    pub const TIBERIUM_OR_GOLD: Self = Self(1);
    pub const GEMS: Self = Self(2);
    pub const WALL: Self = Self(4);
    pub const CRATE: Self = Self(8);
    pub const FLAG: Self = Self(16);

    pub const fn bits(self) -> u8 { self.0 }
    pub const fn contains(self, other: Self) -> bool { self.0 & other.0 == other.0 }
    pub const fn is_resource(self) -> bool {
        self.contains(Self::TIBERIUM_OR_GOLD) || self.contains(Self::GEMS)
    }
    pub const fn is_placeable(self) -> bool {
        self.0 & !Self::CRATE.0 == Self::NONE.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlayDefinition {
    pub variant: GameVariant,
    /// Original OverlayType sbyte ID, represented as u16 for CellLayers parity.
    pub id: u16,
    pub name: &'static str,
    pub display_key: &'static str,
    pub flags: OverlayTypeFlags,
    /// None means the original catalog supplied no theatre restriction.
    pub theaters: Option<&'static [TemplateTheater]>,
}

impl OverlayDefinition {
    pub fn available_in_theater(&self, theater: TemplateTheater) -> bool {
        self.theaters.is_none_or(|theaters| theaters.contains(&theater))
    }
    pub const fn is_placeable(&self) -> bool { self.flags.is_placeable() }
}

const TD_ALL: Option<&[TemplateTheater]> = None;
const TD_TEMPERATE: Option<&[TemplateTheater]> = Some(&[TemplateTheater::Temperate]);
const TD_TEMPERATE_WINTER: Option<&[TemplateTheater]> =
    Some(&[TemplateTheater::Temperate, TemplateTheater::Winter]);
const RA_TEMPERATE_SNOW: Option<&[TemplateTheater]> =
    Some(&[TemplateTheater::Temperate, TemplateTheater::Snow]);

const fn td(id: u16, name: &'static str, display_key: &'static str, flags: OverlayTypeFlags,
            theaters: Option<&'static [TemplateTheater]>) -> OverlayDefinition {
    OverlayDefinition { variant: GameVariant::TiberianDawn, id, name, display_key, flags, theaters }
}
const fn ra(id: u16, name: &'static str, display_key: &'static str, flags: OverlayTypeFlags,
            theaters: Option<&'static [TemplateTheater]>) -> OverlayDefinition {
    OverlayDefinition { variant: GameVariant::RedAlert, id, name, display_key, flags, theaters }
}

const TD_OVERLAYS: &[OverlayDefinition] = &[
    td(1,"sbag","TEXT_STRUCTURE_TITLE_GDI_SANDBAGS",OverlayTypeFlags::WALL,TD_ALL),
    td(2,"cycl","TEXT_STRUCTURE_TITLE_GDI_CHAIN_LINK",OverlayTypeFlags::WALL,TD_ALL),
    td(3,"brik","TEXT_STRUCTURE_TITLE_GDI_CONCRETE",OverlayTypeFlags::WALL,TD_ALL),
    td(4,"barb","TEXT_STRUCTURE_RA_BARB",OverlayTypeFlags::WALL,TD_ALL),
    td(5,"wood","TEXT_STRUCTURE_TD_WOOD",OverlayTypeFlags::WALL,TD_ALL),
    td(6,"ti1","ti1",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(7,"ti2","ti2",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(8,"ti3","ti3",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(9,"ti4","ti4",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(10,"ti5","ti5",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(11,"ti6","ti6",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(12,"ti7","ti7",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(13,"ti8","ti8",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(14,"ti9","ti9",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(15,"ti10","ti10",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(16,"ti11","ti11",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(17,"ti12","ti12",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    td(20,"v12","TEXT_STRUCTURE_TITLE_CIV12",OverlayTypeFlags::NONE,TD_TEMPERATE),
    td(21,"v13","TEXT_STRUCTURE_TITLE_CIV12",OverlayTypeFlags::NONE,TD_TEMPERATE),
    td(22,"v14","TEXT_STRUCTURE_TITLE_CIV13",OverlayTypeFlags::NONE,TD_TEMPERATE_WINTER),
    td(23,"v15","TEXT_STRUCTURE_TITLE_CIV14",OverlayTypeFlags::NONE,TD_TEMPERATE_WINTER),
    td(24,"v16","TEXT_STRUCTURE_TITLE_CIV15",OverlayTypeFlags::NONE,TD_TEMPERATE_WINTER),
    td(25,"v17","TEXT_STRUCTURE_TITLE_CIV16",OverlayTypeFlags::NONE,TD_TEMPERATE_WINTER),
    td(26,"v18","TEXT_STRUCTURE_TITLE_CIV17",OverlayTypeFlags::NONE,TD_TEMPERATE_WINTER),
    td(27,"fpls","fpls",OverlayTypeFlags::FLAG,TD_ALL),
    td(28,"wcrate","wcrate",OverlayTypeFlags::CRATE,TD_ALL),
    td(29,"scrate","scrate",OverlayTypeFlags::CRATE,TD_ALL),
];

const RA_OVERLAYS: &[OverlayDefinition] = &[
    ra(0,"sbag","TEXT_STRUCTURE_RA_SBAG",OverlayTypeFlags::WALL,TD_ALL),
    ra(1,"cycl","TEXT_STRUCTURE_RA_CYCL",OverlayTypeFlags::WALL,TD_ALL),
    ra(2,"brik","TEXT_STRUCTURE_RA_BRIK",OverlayTypeFlags::WALL,TD_ALL),
    ra(3,"barb","TEXT_STRUCTURE_RA_BARB",OverlayTypeFlags::WALL,TD_ALL),
    ra(4,"wood","TEXT_STRUCTURE_TD_WOOD",OverlayTypeFlags::WALL,TD_ALL),
    ra(5,"gold01","gold01",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    ra(6,"gold02","gold02",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    ra(7,"gold03","gold03",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    ra(8,"gold04","gold04",OverlayTypeFlags::TIBERIUM_OR_GOLD,TD_ALL),
    ra(9,"gem01","gem01",OverlayTypeFlags::GEMS,TD_ALL),
    ra(10,"gem02","gem02",OverlayTypeFlags::GEMS,TD_ALL),
    ra(11,"gem03","gem03",OverlayTypeFlags::GEMS,TD_ALL),
    ra(12,"gem04","gem04",OverlayTypeFlags::GEMS,TD_ALL),
    ra(13,"v12","TEXT_STRUCTURE_RA_CIVILIAN",OverlayTypeFlags::NONE,RA_TEMPERATE_SNOW),
    ra(14,"v13","TEXT_STRUCTURE_RA_CIVILIAN",OverlayTypeFlags::NONE,RA_TEMPERATE_SNOW),
    ra(15,"v14","TEXT_STRUCTURE_TITLE_CIV13",OverlayTypeFlags::NONE,RA_TEMPERATE_SNOW),
    ra(16,"v15","TEXT_STRUCTURE_TITLE_CIV14",OverlayTypeFlags::NONE,RA_TEMPERATE_SNOW),
    ra(17,"v16","TEXT_STRUCTURE_TITLE_CIV15",OverlayTypeFlags::NONE,RA_TEMPERATE_SNOW),
    ra(18,"v17","TEXT_STRUCTURE_TITLE_CIV16",OverlayTypeFlags::NONE,RA_TEMPERATE_SNOW),
    ra(19,"v18","TEXT_STRUCTURE_TITLE_CIV17",OverlayTypeFlags::NONE,RA_TEMPERATE_SNOW),
    ra(20,"fpls","fpls",OverlayTypeFlags::FLAG,TD_ALL),
    ra(21,"wcrate","wcrate",OverlayTypeFlags::CRATE,TD_ALL),
    ra(22,"scrate","scrate",OverlayTypeFlags::CRATE,TD_ALL),
    ra(23,"fenc","TEXT_STRUCTURE_RA_FENC",OverlayTypeFlags::WALL,TD_ALL),
    ra(24,"wwcrate","wwcrate",OverlayTypeFlags::CRATE,TD_ALL),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlayCatalog { variant: GameVariant, theater: TemplateTheater }

impl OverlayCatalog {
    pub const fn new(variant: GameVariant, theater: TemplateTheater) -> Self {
        Self { variant, theater }
    }
    pub const fn variant(&self) -> GameVariant { self.variant }
    pub const fn theater(&self) -> TemplateTheater { self.theater }
    pub fn iter(&self) -> impl Iterator<Item = &'static OverlayDefinition> + '_ {
        self.entries().iter().filter(move |entry| entry.available_in_theater(self.theater))
    }
    pub fn placeable(&self) -> impl Iterator<Item = &'static OverlayDefinition> + '_ {
        self.iter().filter(|entry| entry.is_placeable())
    }
    pub fn get(&self, id: u16) -> Option<&'static OverlayDefinition> {
        self.iter().find(|entry| entry.id == id)
    }
    fn entries(&self) -> &'static [OverlayDefinition] {
        match self.variant { GameVariant::TiberianDawn => TD_OVERLAYS, GameVariant::RedAlert => RA_OVERLAYS }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayPlacementError {
    WrongVariant,
    UnavailableInTheater,
    NotPlaceable,
    OutOfBounds(CellCoord),
    ReservedMapRow(CellCoord),
    Occupied(CellCoord),
    MissingOverlay(CellCoord),
    History(MapDocumentError),
}

/// Places one source-defined placeable overlay as a single undoable command.
/// The original OverlaysTool excludes the first and final map rows.
pub fn place_overlay_with_history(
    document: &mut MapDocument,
    history: &mut MapEditHistory,
    coordinate: CellCoord,
    definition: &OverlayDefinition,
    theater: TemplateTheater,
) -> Result<(), OverlayPlacementError> {
    validate_overlay_definition(document, coordinate, definition, theater)?;
    if !definition.is_placeable() { return Err(OverlayPlacementError::NotPlaceable); }
    if coordinate.y == 0 || u32::from(coordinate.y) + 1 >= document.dimensions().height() {
        return Err(OverlayPlacementError::ReservedMapRow(coordinate));
    }
    let mut layers = *document.cell(coordinate).ok_or(OverlayPlacementError::OutOfBounds(coordinate))?;
    if layers.overlay.is_some() { return Err(OverlayPlacementError::Occupied(coordinate)); }
    layers.overlay = Some(definition.id);
    history.apply_cell(document, coordinate, layers).map_err(OverlayPlacementError::History)?;
    Ok(())
}

/// Removes a source-defined placeable overlay as one undoable command.
pub fn remove_overlay_with_history(
    document: &mut MapDocument,
    history: &mut MapEditHistory,
    coordinate: CellCoord,
    definition: &OverlayDefinition,
    theater: TemplateTheater,
) -> Result<(), OverlayPlacementError> {
    validate_overlay_definition(document, coordinate, definition, theater)?;
    if !definition.is_placeable() { return Err(OverlayPlacementError::NotPlaceable); }
    let mut layers = *document.cell(coordinate).ok_or(OverlayPlacementError::OutOfBounds(coordinate))?;
    if layers.overlay != Some(definition.id) { return Err(OverlayPlacementError::MissingOverlay(coordinate)); }
    layers.overlay = None;
    history.apply_cell(document, coordinate, layers).map_err(OverlayPlacementError::History)?;
    Ok(())
}

fn validate_overlay_definition(
    document: &MapDocument,
    coordinate: CellCoord,
    definition: &OverlayDefinition,
    theater: TemplateTheater,
) -> Result<(), OverlayPlacementError> {
    if document.variant() != definition.variant { return Err(OverlayPlacementError::WrongVariant); }
    if !definition.available_in_theater(theater) { return Err(OverlayPlacementError::UnavailableInTheater); }
    if document.cell(coordinate).is_none() { return Err(OverlayPlacementError::OutOfBounds(coordinate)); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rts_core::MapDimensions;

    fn map(variant: GameVariant) -> MapDocument {
        MapDocument::new(variant, MapDimensions::new(8, 6).unwrap()).unwrap()
    }

    #[test]
    fn source_catalog_counts_and_ids_are_variant_specific() {
        assert_eq!(OverlayCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Temperate).iter().count(), 27);
        assert_eq!(OverlayCatalog::new(GameVariant::RedAlert, TemplateTheater::Temperate).iter().count(), 25);
        assert_eq!(OverlayCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Temperate).get(1).unwrap().name, "sbag");
        assert_eq!(OverlayCatalog::new(GameVariant::RedAlert, TemplateTheater::Temperate).get(0).unwrap().name, "sbag");
        assert!(OverlayCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Temperate).get(0).is_none());
    }

    #[test]
    fn source_placeability_rule_allows_none_and_crate_only() {
        let td = OverlayCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Temperate);
        assert!(td.get(20).unwrap().is_placeable());
        assert!(td.get(28).unwrap().is_placeable());
        assert!(!td.get(1).unwrap().is_placeable());
        assert!(!td.get(6).unwrap().is_placeable());
        assert!(!td.get(27).unwrap().is_placeable());
    }

    #[test]
    fn theater_filter_preserves_source_restrictions() {
        let td_temp = OverlayCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Temperate);
        let td_desert = OverlayCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Desert);
        let ra_snow = OverlayCatalog::new(GameVariant::RedAlert, TemplateTheater::Snow);
        assert!(td_temp.get(20).is_some());
        assert!(td_desert.get(20).is_none());
        assert!(td_desert.get(1).is_some());
        assert!(ra_snow.get(13).is_some());
        assert!(ra_snow.get(1).is_some());
    }

    #[test]
    fn placement_and_removal_are_undoable_and_keep_other_layers() {
        let mut document = map(GameVariant::RedAlert);
        let mut history = MapEditHistory::default();
        let at = CellCoord::new(2, 2);
        let definition = *OverlayCatalog::new(GameVariant::RedAlert, TemplateTheater::Temperate).get(21).unwrap();
        let mut layers = *document.cell(at).unwrap();
        layers.smudge = Some(9);
        document.set_cell(at, layers).unwrap();
        place_overlay_with_history(&mut document, &mut history, at, &definition, TemplateTheater::Temperate).unwrap();
        assert_eq!(document.cell(at).unwrap().overlay, Some(21));
        assert_eq!(document.cell(at).unwrap().smudge, Some(9));
        history.undo(&mut document).unwrap();
        assert_eq!(document.cell(at).unwrap().overlay, None);
        history.redo(&mut document).unwrap();
        remove_overlay_with_history(&mut document, &mut history, at, &definition, TemplateTheater::Temperate).unwrap();
        assert_eq!(history.undo_depth(), 2);
        history.undo(&mut document).unwrap();
        assert_eq!(document.cell(at).unwrap().overlay, Some(21));
    }

    #[test]
    fn placement_rejects_special_categories_wrong_theater_and_reserved_rows() {
        let mut document = map(GameVariant::TiberianDawn);
        let mut history = MapEditHistory::default();
        let catalog = OverlayCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Temperate);
        let wall = catalog.get(1).unwrap();
        let crate_type = catalog.get(28).unwrap();
        let temperate_only = catalog.get(20).unwrap();
        let at = CellCoord::new(2, 2);
        assert_eq!(place_overlay_with_history(&mut document, &mut history, at, wall, TemplateTheater::Temperate), Err(OverlayPlacementError::NotPlaceable));
        assert_eq!(place_overlay_with_history(&mut document, &mut history, at, temperate_only, TemplateTheater::Desert), Err(OverlayPlacementError::UnavailableInTheater));
        assert_eq!(place_overlay_with_history(&mut document, &mut history, CellCoord::new(2, 0), crate_type, TemplateTheater::Temperate), Err(OverlayPlacementError::ReservedMapRow(CellCoord::new(2, 0))));
        assert_eq!(history.undo_depth(), 0);
    }

    #[test]
    fn placement_rejects_wrong_variant_and_existing_overlay() {
        let mut document = map(GameVariant::RedAlert);
        let mut history = MapEditHistory::default();
        let td_definition = *OverlayCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Temperate).get(28).unwrap();
        let at = CellCoord::new(3, 3);
        assert_eq!(
            place_overlay_with_history(&mut document, &mut history, at, &td_definition, TemplateTheater::Temperate),
            Err(OverlayPlacementError::WrongVariant)
        );
        let ra_definition = *OverlayCatalog::new(GameVariant::RedAlert, TemplateTheater::Temperate).get(21).unwrap();
        place_overlay_with_history(&mut document, &mut history, at, &ra_definition, TemplateTheater::Temperate).unwrap();
        assert_eq!(
            place_overlay_with_history(&mut document, &mut history, at, &ra_definition, TemplateTheater::Temperate),
            Err(OverlayPlacementError::Occupied(at))
        );
        assert_eq!(history.undo_depth(), 1);
    }

    #[test]
    fn overlay_ids_are_not_shared_between_variants() {
        let td = OverlayCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Temperate);
        let ra = OverlayCatalog::new(GameVariant::RedAlert, TemplateTheater::Temperate);
        assert_eq!(td.get(6).unwrap().name, "ti1");
        assert_eq!(ra.get(6).unwrap().name, "gold02");
    }
}
