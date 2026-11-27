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
    fn bib_flags_share_the_low_bit() {
        assert_eq!(SmudgeTypeFlag::Bib1.bits(), 3);
        assert_eq!(SmudgeTypeFlag::Bib2.bits(), 5);
        assert_eq!(SmudgeTypeFlag::Bib3.bits(), 9);
        assert!(SmudgeTypeFlag::is_bib_bits(SmudgeTypeFlag::Bib3.bits()));
        assert!(!SmudgeTypeFlag::is_bib_bits(0));
    }
}
