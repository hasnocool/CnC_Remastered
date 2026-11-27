//! Source-derived placed Overlay and Smudge records.
//!
//! These are mutable scenario instances, deliberately separate from catalog
//! definitions. Rendering assets and platform color types stay outside this crate.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EditorColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl EditorColor {
    pub const WHITE: Self = Self { red: 255, green: 255, blue: 255, alpha: 255 };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum OverlayTypeFlag {
    #[default]
    None = 0,
    TiberiumOrGold = 1,
    Gems = 2,
    Wall = 4,
    Crate = 8,
    Flag = 16,
}

impl OverlayTypeFlag {
    pub const fn bits(self) -> u8 { self as u8 }
    pub const fn contains(self, other: Self) -> bool {
        self.bits() & other.bits() == other.bits()
    }
    /// Source rule: only flags outside the Crate bit make an overlay non-placeable.
    pub const fn is_placeable_bits(bits: u8) -> bool { bits & !Self::Crate.bits() == 0 }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlayTypeDefinition {
    pub id: i8,
    pub name: String,
    pub flags: u8,
}

impl OverlayTypeDefinition {
    pub const fn is_wall(&self) -> bool { self.flags & OverlayTypeFlag::Wall.bits() != 0 }
    pub const fn is_resource(&self) -> bool {
        self.flags & (OverlayTypeFlag::TiberiumOrGold.bits() | OverlayTypeFlag::Gems.bits()) != 0
    }
    pub const fn is_gem(&self) -> bool { self.flags & OverlayTypeFlag::Gems.bits() != 0 }
    pub const fn is_placeable(&self) -> bool { OverlayTypeFlag::is_placeable_bits(self.flags) }
    /// OverlayType's source occupancy mask is exactly one cell.
    pub const fn occupy_mask(&self) -> [(u16, u16); 1] { [(0, 0)] }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedOverlay {
    pub type_id: i8,
    pub icon: i32,
    pub tint: EditorColor,
}

impl PlacedOverlay {
    pub const fn new(type_id: i8) -> Self {
        Self { type_id, icon: 0, tint: EditorColor::WHITE }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum SmudgeTypeFlag {
    #[default]
    None = 0,
    Bib = 1,
    Bib1 = 3,
    Bib2 = 5,
    Bib3 = 9,
}

impl SmudgeTypeFlag {
    pub const fn bits(self) -> u8 { self as u8 }
    pub const fn is_bib_bits(bits: u8) -> bool { bits & Self::Bib.bits() != 0 }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmudgeTypeDefinition {
    pub id: i8,
    pub name: String,
    pub flags: u8,
    pub width: u16,
    pub height: u16,
}

impl SmudgeTypeDefinition {
    pub const fn is_bib(&self) -> bool { SmudgeTypeFlag::is_bib_bits(self.flags) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedSmudge {
    pub type_id: i8,
    pub icon: i32,
    pub data: i32,
    pub tint: EditorColor,
}

impl PlacedSmudge {
    pub const fn new(type_id: i8) -> Self {
        Self { type_id, icon: 0, data: 0, tint: EditorColor::WHITE }
    }
}

/// Source-derived immutable BuildingType catalog entry. Masks are row-major
/// offsets from the placement anchor; the base mask excludes the bib extension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildingTypeDefinition {
    pub id: i8,
    pub name: String,
    pub display_name: String,
    pub tile_name: String,
    pub width: u16,
    pub height: u16,
    pub has_bib: bool,
    pub owner_house: String,
    pub theaters: Option<Vec<String>>,
    pub is_fake: bool,
    pub has_turret: bool,
    pub factory_overlay: Option<String>,
    pub base_occupy_mask: Vec<(u16, u16)>,
    pub occupy_mask: Vec<(u16, u16)>,
}

impl BuildingTypeDefinition {
    /// Builds the full mask using the editor's exact bib rule: retain all
    /// base rows except the final row, then append two completely occupied rows.
    pub fn from_base_mask(
        id: i8, name: String, display_name: String, tile_name: String,
        base_occupy_mask: Vec<(u16, u16)>, width: u16, height: u16,
        has_bib: bool, owner_house: String, is_fake: bool,
        has_turret: bool, factory_overlay: Option<String>,
    ) -> Self {
        let occupy_mask = if has_bib && height > 0 {
            let mut full = base_occupy_mask.iter()
                .copied().filter(|(_, y)| *y < height - 1).collect::<Vec<_>>();
            for y in height..height.saturating_add(2) {
                for x in 0..width { full.push((x, y)); }
            }
            full
        } else {
            base_occupy_mask.clone()
        };
        Self {
            id, name: if is_fake && !name.is_empty() {
                let mut characters = name.chars().collect::<Vec<_>>();
                characters.pop();
                characters.push('f');
                characters.into_iter().collect()
            } else { name.clone() },
            display_name, tile_name: name, width, height, has_bib,
            owner_house, theaters: None, is_fake, has_turret, factory_overlay,
            base_occupy_mask, occupy_mask,
        }
    }
}

/// Mutable editor Building instance. BibCells are owned by this building,
/// matching the original model's private-set HashSet<int>.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedBuilding {
    pub type_id: i8,
    pub house: String,
    pub strength: i32,
    pub direction: i8,
    pub trigger: String,
    pub base_priority: i32,
    pub is_prebuilt: bool,
    pub sellable: bool,
    pub rebuild: bool,
    pub tint: EditorColor,
    pub bib_cells: Vec<rts_core::CellCoord>,
}


impl PlacedBuilding {
    /// Mirrors Building.Clone: copy editable properties but omit derived BibCells.
    pub fn clone_without_bib_cells(&self) -> Self {
        Self {
            type_id: self.type_id,
            house: self.house.clone(),
            strength: self.strength,
            direction: self.direction,
            trigger: self.trigger.clone(),
            base_priority: self.base_priority,
            is_prebuilt: self.is_prebuilt,
            sellable: self.sellable,
            rebuild: self.rebuild,
            tint: self.tint,
            bib_cells: Vec::new(),
        }
    }

    pub fn new(type_id: i8, house: String, direction: i8) -> Self {
        Self {
            type_id, house, strength: 256, direction, trigger: "None".into(),
            base_priority: -1, is_prebuilt: true, sellable: false,
            rebuild: false, tint: EditorColor::WHITE, bib_cells: Vec::new(),
        }
    }

    /// The source getter applies 75% alpha to non-prebuilt buildings.
    pub const fn effective_tint(&self) -> EditorColor {
        if self.is_prebuilt {
            self.tint
        } else {
            EditorColor { alpha: ((self.tint.alpha as u16 * 75) / 100) as u8, ..self.tint }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_flags_keep_source_numeric_values_and_placeability_rule() {
        assert_eq!(OverlayTypeFlag::TiberiumOrGold.bits(), 1);
        assert_eq!(OverlayTypeFlag::Gems.bits(), 2);
        assert_eq!(OverlayTypeFlag::Wall.bits(), 4);
        assert_eq!(OverlayTypeFlag::Crate.bits(), 8);
        assert_eq!(OverlayTypeFlag::Flag.bits(), 16);
        assert!(OverlayTypeFlag::is_placeable_bits(0));
        assert!(OverlayTypeFlag::is_placeable_bits(8));
        assert!(!OverlayTypeFlag::is_placeable_bits(1));
        assert!(!OverlayTypeFlag::is_placeable_bits(4));
    }

    #[test]
    fn placed_overlay_and_smudge_keep_independent_fields_and_white_tint() {
        let overlay = PlacedOverlay::new(4);
        assert_eq!(overlay.icon, 0);
        assert_eq!(overlay.tint, EditorColor::WHITE);
        let smudge = PlacedSmudge::new(7);
        assert_eq!((smudge.icon, smudge.data), (0, 0));
        assert_eq!(smudge.tint, EditorColor::WHITE);
    }

    #[test]
    fn building_type_expands_bib_mask_by_replacing_last_base_row() {
        let kind = BuildingTypeDefinition::from_base_mask(
            1, "FACTORY".into(), "Factory".into(), "FACTORY".into(),
            vec![(0,0),(1,0),(0,1)], 2, 2, true, "GDI".into(), false, true, None,
        );
        assert_eq!(kind.base_occupy_mask, vec![(0,0),(1,0),(0,1)]);
        assert_eq!(kind.occupy_mask, vec![(0,0),(0,2),(1,2),(0,3),(1,3)]);
        assert_eq!(kind.width, 2);
        assert_eq!(kind.height, 2);
    }

    #[test]
    fn building_clone_copies_properties_but_not_derived_bib_ownership() {
        let mut building = PlacedBuilding::new(1, "GDI".into(), 2);
        building.bib_cells.push(rts_core::CellCoord::new(4, 5));
        building.strength = 99;
        let clone = building.clone_without_bib_cells();
        assert_eq!(clone.strength, 99);
        assert_eq!(clone.house, "GDI");
        assert!(clone.bib_cells.is_empty());
    }

    #[test]
    fn non_prebuilt_building_tint_uses_seventy_five_percent_alpha() {
        let mut building = PlacedBuilding::new(1, "GDI".into(), 0);
        building.is_prebuilt = false;
        building.tint = EditorColor { red: 1, green: 2, blue: 3, alpha: 200 };
        assert_eq!(building.effective_tint().alpha, 150);
        assert_eq!(building.tint.alpha, 200);
    }

    #[test]
    fn bib_flags_share_the_low_bit() {
        assert_eq!(SmudgeTypeFlag::Bib1.bits(), 3);
        assert_eq!(SmudgeTypeFlag::Bib2.bits(), 5);
        assert_eq!(SmudgeTypeFlag::Bib3.bits(), 9);
        assert!(SmudgeTypeFlag::is_bib_bits(SmudgeTypeFlag::Bib3.bits()));
        assert!(!SmudgeTypeFlag::is_bib_bits(0));
    }
}
