//! AbstractClass and AbstractTypeClass behavior from both ABSTRACT.CPP/H
//! translation units.

use crate::coordinate::Coordinate;

pub const RED_ALERT_INI_NAME_BYTES: usize = 24;
pub const TIBERIAN_DAWN_INI_NAME_BYTES: usize = 9;
pub const RED_ALERT_HOUSEF_ALL: u32 = 0x000f_ffff;
pub const TIBERIAN_DAWN_HOUSEF_ALL: u16 = 0xffff;
pub const RTTI_ABSTRACTTYPE_TIBERIAN_DAWN: u8 = 10;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AbstractVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BuildingTargetGeometry {
    pub is_active: bool,
    pub width: i32,
    pub height: i32,
}

pub trait AbstractTargetHost {
    fn target_coordinate(&self, target: u32) -> Option<Coordinate>;
    fn building_geometry(&self, _target: u32) -> Option<BuildingTargetGeometry> { None }
}

pub trait AbstractDeleteHost {
    type VTableToken;
    fn capture_vtable(&mut self, object_id: u64) -> Self::VTableToken;
    fn destruct_without_deallocating(&mut self, object_id: u64);
    fn restore_vtable(&mut self, object_id: u64, token: Self::VTableToken);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AbstractClass {
    pub variant: AbstractVariant,
    pub rtti: u8,
    pub id: i32,
    pub coord: Coordinate,
    pub height: i32,
    pub is_active: bool,
    pub is_recently_created: bool,
    pub save_load_padding: [u8; 8],
}

impl AbstractClass {
    /// Red Alert initializes Coord to 0xffffffff and Height to zero.
    pub const fn red_alert(rtti: u8, id: i32) -> Self {
        Self {
            variant: AbstractVariant::RedAlert,
            rtti,
            id,
            coord: Coordinate::from_raw(0xffff_ffff),
            height: 0,
            is_active: false,
            is_recently_created: false,
            save_load_padding: [0; 8],
        }
    }

    /// Tiberian Dawn initializes Coord to zero; its class also includes eight
    /// save/load compatibility bytes.
    pub const fn tiberian_dawn() -> Self {
        Self {
            variant: AbstractVariant::TiberianDawn,
            rtti: 0,
            id: 0,
            coord: Coordinate::ZERO,
            height: 0,
            is_active: false,
            is_recently_created: false,
            save_load_padding: [0; 8],
        }
    }

    pub const fn center_coord(&self) -> Coordinate { self.coord }
    pub const fn target_coord(&self) -> Coordinate { self.coord }
    pub fn set_center_coord(&mut self, coord: Coordinate) { self.coord = coord; }
    pub const fn owner(&self) -> i32 { -1 }

    /// Both implementations set both the active and recently-created flags.
    pub fn set_active(&mut self) {
        self.is_active = true;
        self.is_recently_created = true;
    }

    pub fn clear_active(&mut self) { self.is_active = false; }

    pub fn distance_to_coordinate(&self, coord: Coordinate) -> i32 {
        self.coord.distance(coord).min(i32::MAX as u32) as i32
    }

    /// Port of AbstractClass::Distance(TARGET): subtract an active building's
    /// average footprint radius and clamp the result to zero.
    pub fn distance_to_target(&self, target: u32, host: &impl AbstractTargetHost) -> Option<i32> {
        let target_coord = host.target_coordinate(target)?;
        let mut distance = self.distance_to_coordinate(target_coord);
        if let Some(building) = host.building_geometry(target) {
            if building.is_active {
                let radius = building.width
                    .wrapping_add(building.height)
                    .wrapping_mul(0x100 / 4);
                distance = distance.saturating_sub(radius).max(0);
            }
        }
        Some(distance)
    }

    /// Red Alert encodes target RTTI in the high byte and ID in the low 24 bits.
    /// Tiberian Dawn's AbstractClass does not expose this member.
    pub fn as_target(&self) -> Option<u32> {
        (self.variant == AbstractVariant::RedAlert)
            .then(|| ((self.rtti as u32) << 24) | (self.id as u32 & 0x00ff_ffff))
    }

    /// Preserve the original vtable across the virtual destructor chain.
    /// The host models the game's custom operator delete, which does not free
    /// the object's backing storage.
    pub fn delete_this(&self, object_id: u64, host: &mut impl AbstractDeleteHost) {
        let token = host.capture_vtable(object_id);
        host.destruct_without_deallocating(object_id);
        host.restore_vtable(object_id, token);
    }

    pub const fn can_enter_cell(&self) -> AbstractMoveResult { AbstractMoveResult::MoveOk }
    pub fn ai(&mut self) {}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AbstractMoveResult { MoveOk }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AbstractTypeClass {
    RedAlert {
        rtti: u8,
        id: i32,
        full_name: i32,
        ini_name: [u8; RED_ALERT_INI_NAME_BYTES],
    },
    TiberianDawn {
        name: i32,
        ini_name: [u8; TIBERIAN_DAWN_INI_NAME_BYTES],
    },
}

impl AbstractTypeClass {
    pub fn red_alert(rtti: u8, id: i32, full_name: i32, ini_name: &[u8]) -> Self {
        let mut stored = [0u8; RED_ALERT_INI_NAME_BYTES];
        copy_truncated_name(&mut stored, ini_name);
        Self::RedAlert { rtti, id, full_name, ini_name: stored }
    }

    pub fn tiberian_dawn(name: i32, ini_name: &[u8]) -> Self {
        let mut stored = [0u8; TIBERIAN_DAWN_INI_NAME_BYTES];
        copy_truncated_name(&mut stored, ini_name);
        Self::TiberianDawn { name, ini_name: stored }
    }

    pub const fn variant(&self) -> AbstractVariant {
        match self {
            Self::RedAlert { .. } => AbstractVariant::RedAlert,
            Self::TiberianDawn { .. } => AbstractVariant::TiberianDawn,
        }
    }

    pub const fn rtti(&self) -> Option<u8> {
        match self {
            Self::RedAlert { rtti, .. } => Some(*rtti),
            Self::TiberianDawn { .. } => None,
        }
    }

    pub const fn id(&self) -> Option<i32> {
        match self {
            Self::RedAlert { id, .. } => Some(*id),
            Self::TiberianDawn { .. } => None,
        }
    }

    pub const fn what_am_i(&self) -> u8 {
        match self {
            Self::RedAlert { rtti, .. } => *rtti,
            Self::TiberianDawn { .. } => RTTI_ABSTRACTTYPE_TIBERIAN_DAWN,
        }
    }

    pub const fn full_name(&self) -> i32 {
        match self {
            Self::RedAlert { full_name, .. } => *full_name,
            Self::TiberianDawn { name, .. } => *name,
        }
    }

    /// The pinned Red Alert project does not define FIXIT_NAME_OVERRIDE.
    /// The optional branch reproduces its one-based negative override index.
    pub fn full_name_with_override(&self, override_index: Option<usize>, enabled: bool) -> i32 {
        if enabled && self.variant() == AbstractVariant::RedAlert {
            if let Some(index) = override_index {
                return -((index as i32).wrapping_add(1));
            }
        }
        self.full_name()
    }

    pub fn ini_name_bytes(&self) -> &[u8] {
        match self {
            Self::RedAlert { ini_name, .. } => ini_name,
            Self::TiberianDawn { ini_name, .. } => ini_name,
        }
    }

    pub fn ini_name(&self) -> &str {
        let bytes = self.ini_name_bytes();
        let end = bytes.iter().position(|byte| *byte == 0).unwrap_or(bytes.len());
        std::str::from_utf8(&bytes[..end]).unwrap_or("")
    }

    pub fn set_name(&mut self, name: &[u8]) {
        match self {
            Self::RedAlert { ini_name, .. } => copy_truncated_name(ini_name, name),
            Self::TiberianDawn { ini_name, .. } => copy_truncated_name(ini_name, name),
        }
    }

    pub const fn coord_fixup(&self, coord: Coordinate) -> Coordinate { coord }

    pub const fn get_ownable(&self) -> u32 {
        match self {
            Self::RedAlert { .. } => RED_ALERT_HOUSEF_ALL,
            Self::TiberianDawn { .. } => TIBERIAN_DAWN_HOUSEF_ALL as u32,
        }
    }

    pub const fn as_target(&self) -> Option<u32> {
        match self {
            Self::RedAlert { rtti, id, .. } =>
                Some(((*rtti as u32) << 24) | (*id as u32 & 0x00ff_ffff)),
            Self::TiberianDawn { .. } => None,
        }
    }

    pub const fn can_encode_pointers(&self) -> bool { true }
    pub const fn can_decode_pointers(&self) -> bool { true }
}

fn copy_truncated_name<const N: usize>(destination: &mut [u8; N], source: &[u8]) {
    destination.fill(0);
    let copy_length = source.iter().position(|byte| *byte == 0).unwrap_or(source.len())
        .min(N.saturating_sub(1));
    destination[..copy_length].copy_from_slice(&source[..copy_length]);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Targets {
        target: u32,
        coord: Coordinate,
        building: Option<BuildingTargetGeometry>,
    }
    impl AbstractTargetHost for Targets {
        fn target_coordinate(&self, target: u32) -> Option<Coordinate> {
            (target == self.target).then_some(self.coord)
        }
        fn building_geometry(&self, target: u32) -> Option<BuildingTargetGeometry> {
            if target == self.target { self.building } else { None }
        }
    }

    #[derive(Default)]
    struct DeletionLog { captured: usize, destroyed: usize, restored: usize }
    impl AbstractDeleteHost for DeletionLog {
        type VTableToken = u64;
        fn capture_vtable(&mut self, _id: u64) -> u64 { self.captured += 1; 0x1234 }
        fn destruct_without_deallocating(&mut self, _id: u64) { self.destroyed += 1; }
        fn restore_vtable(&mut self, _id: u64, token: u64) {
            assert_eq!(token, 0x1234);
            self.restored += 1;
        }
    }

    #[test]
    fn constructors_preserve_variant_specific_coordinate_defaults() {
        let ra = AbstractClass::red_alert(0x12, 45);
        assert_eq!(ra.coord, Coordinate::from_raw(u32::MAX));
        assert_eq!(ra.height, 0);
        assert_eq!(ra.as_target(), Some(0x1200_002d));

        let td = AbstractClass::tiberian_dawn();
        assert_eq!(td.coord, Coordinate::ZERO);
        assert_eq!(td.as_target(), None);
        assert_eq!(td.save_load_padding, [0; 8]);
    }

    #[test]
    fn set_active_sets_both_flags_and_default_queries() {
        let mut object = AbstractClass::tiberian_dawn();
        object.set_active();
        assert!(object.is_active && object.is_recently_created);
        assert_eq!(object.owner(), -1);
        assert_eq!(object.can_enter_cell(), AbstractMoveResult::MoveOk);
        object.ai();
    }

    #[test]
    fn target_distance_subtracts_active_building_radius() {
        let object = AbstractClass::red_alert(1, 1);
        let target = Targets {
            target: 7,
            coord: Coordinate::from_leptons(100, 0),
            building: Some(BuildingTargetGeometry { is_active: true, width: 1, height: 1 }),
        };
        assert_eq!(object.distance_to_target(7, &target), Some(36));
        let inactive = Targets {
            target: 7,
            coord: Coordinate::from_leptons(100, 0),
            building: Some(BuildingTargetGeometry { is_active: false, width: 1, height: 1 }),
        };
        assert_eq!(object.distance_to_target(7, &inactive), Some(100));
    }

    #[test]
    fn type_names_use_variant_buffer_lengths_and_terminate() {
        let mut ra = AbstractTypeClass::red_alert(4, 2, 99, b"INFANTRY");
        assert_eq!(ra.ini_name(), "INFANTRY");
        ra.set_name(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        assert_eq!(ra.ini_name(), "ABCDEFGHIJKLMNOPQRSTUVW");

        let mut td = AbstractTypeClass::tiberian_dawn(66, b"TANK");
        td.set_name(b"ABCDEFGHIJKLM");
        assert_eq!(td.ini_name(), "ABCDEFGH");
        assert_eq!(td.full_name(), 66);
        assert_eq!(td.what_am_i(), RTTI_ABSTRACTTYPE_TIBERIAN_DAWN);
    }

    #[test]
    fn ownable_masks_and_pointer_hooks_match_variants() {
        let ra = AbstractTypeClass::red_alert(2, 8, 20, b"TANK");
        let td = AbstractTypeClass::tiberian_dawn(20, b"TANK");
        assert_eq!(ra.get_ownable(), RED_ALERT_HOUSEF_ALL);
        assert_eq!(td.get_ownable(), 0xffff);
        assert!(ra.can_encode_pointers() && ra.can_decode_pointers());
    }

    #[test]
    fn optional_name_override_is_one_based_negative() {
        let ra = AbstractTypeClass::red_alert(2, 5, 20, b"TANK");
        assert_eq!(ra.full_name_with_override(Some(2), false), 20);
        assert_eq!(ra.full_name_with_override(Some(2), true), -3);
    }

    #[test]
    fn delete_this_preserves_vtable_through_destructor_chain() {
        let object = AbstractClass::red_alert(1, 1);
        let mut host = DeletionLog::default();
        object.delete_this(99, &mut host);
        assert_eq!((host.captured, host.destroyed, host.restored), (1, 1, 1));
    }
}
