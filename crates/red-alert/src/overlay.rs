//! Red Alert overlay object and map-section behavior from REDALERT/OVERLAY.CPP.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection commit
//! f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! Map changes are exposed as ordered actions for the simulation thread. The
//! OverlayPack reader/writer uses the existing framed LCW codec (8 KiB blocks).

use rts_formats::lcw::LcwError;
use rts_formats::lcw_blocks::{compress_blocks, decompress_blocks};

pub const MAP_CELL_WIDTH: usize = 128;
pub const MAP_CELL_HEIGHT: usize = 128;
pub const MAP_CELL_TOTAL: usize = MAP_CELL_WIDTH * MAP_CELL_HEIGHT;
pub const OVERLAY_PACK_BLOCK_SIZE: usize = 8 * 1024;
pub const MZONE_NORMAL_MASK: u8 = 1;
pub const MZONE_CRUSHER_MASK: u8 = 2;

/// The upstream enum has an explicit signed-char representation; -1 means none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct OverlayType(pub i8);

impl OverlayType {
    pub const NONE: Self = Self(-1);
    pub const SANDBAG_WALL: Self = Self(0);
    pub const CYCLONE_WALL: Self = Self(1);
    pub const BRICK_WALL: Self = Self(2);
    pub const BARBWIRE_WALL: Self = Self(3);
    pub const WOOD_WALL: Self = Self(4);
    pub const GOLD1: Self = Self(5);
    pub const GOLD2: Self = Self(6);
    pub const GOLD3: Self = Self(7);
    pub const GOLD4: Self = Self(8);
    pub const GEMS1: Self = Self(9);
    pub const GEMS2: Self = Self(10);
    pub const GEMS3: Self = Self(11);
    pub const GEMS4: Self = Self(12);
    pub const V12: Self = Self(13);
    pub const V13: Self = Self(14);
    pub const V14: Self = Self(15);
    pub const V15: Self = Self(16);
    pub const V16: Self = Self(17);
    pub const V17: Self = Self(18);
    pub const V18: Self = Self(19);
    pub const FLAG_SPOT: Self = Self(20);
    pub const WOOD_CRATE: Self = Self(21);
    pub const STEEL_CRATE: Self = Self(22);
    pub const FENCE: Self = Self(23);
    pub const WATER_CRATE: Self = Self(24);
    pub const COUNT: usize = 25;

    pub const fn from_raw_byte(value: u8) -> Self { Self(value as i8) }
    pub const fn to_raw_byte(self) -> u8 { self.0 as u8 }
    pub const fn is_none(self) -> bool { self.0 == -1 }
    pub const fn is_valid(self) -> bool { self.0 >= 0 && (self.0 as usize) < Self::COUNT }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlayTypeMetadata {
    pub kind: OverlayType,
    pub is_wall: bool,
    pub is_crushable: bool,
    pub is_crate: bool,
    pub land_is_tiberium: bool,
}

impl OverlayTypeMetadata {
    pub const fn new(kind: OverlayType, is_wall: bool, is_crushable: bool, is_crate: bool, land_is_tiberium: bool) -> Self {
        Self { kind, is_wall, is_crushable, is_crate, land_is_tiberium }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayError {
    Lcw(LcwError),
    InvalidMapDimensions,
    InvalidMapLength { expected: usize, actual: usize },
    PackedDataLength { expected: usize, actual: usize },
    InvalidOverlayType(OverlayType),
    MissingOverlayMetadata(OverlayType),
    StagingBufferOverflow { required: usize, capacity: usize },
}

impl From<LcwError> for OverlayError {
    fn from(value: LcwError) -> Self { Self::Lcw(value) }
}

fn metadata_for(catalog: &[OverlayTypeMetadata], kind: OverlayType) -> Result<OverlayTypeMetadata, OverlayError> {
    if !kind.is_valid() { return Err(OverlayError::InvalidOverlayType(kind)); }
    catalog.iter().copied().find(|entry| entry.kind == kind)
        .ok_or(OverlayError::MissingOverlayMetadata(kind))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayMark { Down, Other }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlayMarkContext {
    /// Result of the inherited ObjectClass::Mark call.
    pub base_mark_succeeded: bool,
    pub scenario_init: bool,
    pub existing_overlay: OverlayType,
    pub clear_to_build: bool,
    pub clear_for_float: bool,
    pub clear_for_track: bool,
    pub clear_for_track_ignoring_units: bool,
    pub to_own: Option<u8>,
}

impl Default for OverlayMarkContext {
    fn default() -> Self {
        Self {
            base_mark_succeeded: false,
            scenario_init: false,
            existing_overlay: OverlayType::NONE,
            clear_to_build: false,
            clear_for_float: false,
            clear_for_track: false,
            clear_for_track_ignoring_units: false,
            to_own: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlayAction {
    WriteOverlay(OverlayType),
    SetOverlayData(u8),
    RedrawObjects,
    WallUpdate,
    ResetMovementZones(u8),
    SetCellOwner(u8),
    TiberiumAdjust,
    RecalculateCellAttributes,
    RemoveOverlap,
    SetObjectDown(bool),
    SetObjectInLimbo(bool),
    DeleteObject,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlayMarkResult {
    pub succeeded: bool,
    pub deleted: bool,
    /// Ordered side effects in source statement order.
    pub actions: Vec<OverlayAction>,
}

/// Port of OverlayClass::Mark. Engine-owned ObjectClass and cell predicates are
/// injected in context; this function preserves branch decisions and ordering.
pub fn plan_overlay_mark(
    metadata: OverlayTypeMetadata,
    mark: OverlayMark,
    context: OverlayMarkContext,
) -> OverlayMarkResult {
    if !context.base_mark_succeeded {
        return OverlayMarkResult { succeeded: false, deleted: false, actions: Vec::new() };
    }
    if mark != OverlayMark::Down {
        return OverlayMarkResult { succeeded: true, deleted: false, actions: Vec::new() };
    }

    let mut actions = Vec::new();
    if metadata.is_wall {
        if !context.clear_to_build {
            // The source deletes and returns false before reaching common cleanup.
            actions.push(OverlayAction::DeleteObject);
            return OverlayMarkResult { succeeded: false, deleted: true, actions };
        }
        actions.extend([
            OverlayAction::WriteOverlay(metadata.kind),
            OverlayAction::SetOverlayData(0),
            OverlayAction::RedrawObjects,
            OverlayAction::WallUpdate,
            OverlayAction::ResetMovementZones(if metadata.is_crushable {
                MZONE_NORMAL_MASK
            } else {
                MZONE_NORMAL_MASK | MZONE_CRUSHER_MASK
            }),
        ]);
        if let Some(owner) = context.to_own {
            actions.push(OverlayAction::SetCellOwner(owner));
        }
    } else {
        let clear = if context.scenario_init {
            true
        } else if metadata.kind == OverlayType::WATER_CRATE {
            context.clear_for_float
        } else if metadata.kind == OverlayType::STEEL_CRATE || metadata.kind == OverlayType::WOOD_CRATE {
            context.clear_for_track
        } else {
            context.clear_for_track_ignoring_units
        };

        if (context.scenario_init || context.existing_overlay.is_none()) && clear {
            actions.extend([
                OverlayAction::WriteOverlay(metadata.kind),
                OverlayAction::SetOverlayData(0),
                OverlayAction::RedrawObjects,
            ]);
            if metadata.land_is_tiberium {
                actions.push(OverlayAction::SetOverlayData(1));
                actions.push(OverlayAction::TiberiumAdjust);
            }
        }
    }

    // A failed non-wall cell placement still reaches this successful cleanup.
    actions.extend([
        OverlayAction::RecalculateCellAttributes,
        OverlayAction::RemoveOverlap,
        OverlayAction::SetObjectDown(false),
        OverlayAction::SetObjectInLimbo(true),
        OverlayAction::DeleteObject,
    ]);
    OverlayMarkResult { succeeded: true, deleted: true, actions }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OverlayObjectId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlayObject {
    pub id: OverlayObjectId,
    pub kind: Option<OverlayType>,
    pub cell: Option<i16>,
    pub active: bool,
    pub is_down: bool,
    pub is_in_limbo: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlayPool {
    slots: Vec<Option<OverlayObject>>,
    free: Vec<usize>,
}

impl OverlayPool {
    pub fn new(capacity: usize) -> Self {
        Self { slots: vec![None; capacity], free: (0..capacity).rev().collect() }
    }
    pub fn capacity(&self) -> usize { self.slots.len() }
    pub fn active_count(&self) -> usize { self.slots.iter().filter(|slot| slot.is_some()).count() }

    pub fn allocate(&mut self, kind: OverlayType) -> Option<OverlayObjectId> {
        if !kind.is_valid() { return None; }
        let index = self.free.pop()?;
        self.slots[index] = Some(OverlayObject {
            id: OverlayObjectId(index), kind: Some(kind), cell: None,
            active: true, is_down: false, is_in_limbo: true,
        });
        Some(OverlayObjectId(index))
    }

    pub fn get(&self, id: OverlayObjectId) -> Option<&OverlayObject> {
        self.slots.get(id.0)?.as_ref()
    }
    pub fn get_mut(&mut self, id: OverlayObjectId) -> Option<&mut OverlayObject> {
        self.slots.get_mut(id.0)?.as_mut()
    }

    /// Port of operator delete and destructor class/limbo cleanup.
    pub fn free(&mut self, id: OverlayObjectId, game_active: bool) -> bool {
        let Some(slot) = self.slots.get_mut(id.0) else { return false; };
        let Some(mut object) = slot.take() else { return false; };
        object.active = false;
        if game_active {
            object.is_down = false;
            object.is_in_limbo = true;
        }
        object.kind = None;
        object.cell = None;
        self.free.push(id.0);
        true
    }

    /// Port of OverlayClass::Init / Overlays.Free_All.
    pub fn free_all(&mut self) {
        self.slots.fill(None);
        self.free = (0..self.slots.len()).rev().collect();
    }
}

/// Owns the fixed pool and temporary ToOwn constructor flag. This object is
/// intended to be held by the single simulation-thread owner, not shared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlaySystem {
    pub pool: OverlayPool,
    to_own: Option<u8>,
}

impl OverlaySystem {
    pub fn new(capacity: usize) -> Self { Self { pool: OverlayPool::new(capacity), to_own: None } }
    pub fn to_own(&self) -> Option<u8> { self.to_own }

    pub fn init(&mut self) {
        self.pool.free_all();
        self.to_own = None;
    }

    /// Port of OverlayClass(type, pos, house). Unlimbo is supplied by the
    /// engine and must return true only when the object actually left limbo.
    pub fn construct<F>(
        &mut self,
        kind: OverlayType,
        cell: Option<i16>,
        house: Option<u8>,
        mut unlimbo: F,
    ) -> Option<OverlayObjectId>
    where
        F: FnMut(&mut OverlayObject, i16, Option<u8>) -> bool,
    {
        let id = self.pool.allocate(kind)?;
        if let Some(cell) = cell {
            self.to_own = house;
            if let Some(object) = self.pool.get_mut(id) {
                if unlimbo(object, cell, self.to_own) {
                    object.cell = Some(cell);
                    object.is_in_limbo = false;
                }
            }
            self.to_own = None;
        }
        Some(id)
    }

    pub fn delete(&mut self, id: OverlayObjectId, game_active: bool) -> bool {
        self.pool.free(id, game_active)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildingOwnerDistance {
    pub owner: u8,
    pub distance: i32,
}

/// Strict-less-than selection preserves the first building when distances tie.
pub fn nearest_building_owner(
    candidates: impl IntoIterator<Item = BuildingOwnerDistance>,
) -> Option<u8> {
    let mut best_distance = i32::MAX;
    let mut owner = None;
    for candidate in candidates {
        if candidate.distance < best_distance {
            best_distance = candidate.distance;
            owner = Some(candidate.owner);
        }
    }
    owner
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlayPlacementRequest {
    pub cell: i16,
    pub kind: OverlayType,
    pub wall_owner: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyOverlayEntry {
    pub key: String,
    pub kind: OverlayType,
}

#[derive(Debug, Clone, Copy)]
pub struct OverlayIniInput<'a> {
    pub new_ini_format: i32,
    /// Raw bytes returned by CCINIClass::Get_UUBlock.
    pub packed_data: Option<&'a [u8]>,
    pub game_normal: bool,
    pub legacy_section_present: bool,
    pub legacy_entries: &'a [LegacyOverlayEntry],
    pub map_cell_total: usize,
    pub map_cell_width: usize,
}

impl<'a> OverlayIniInput<'a> {
    pub const fn red_alert(
        new_ini_format: i32,
        packed_data: Option<&'a [u8]>,
        game_normal: bool,
        legacy_section_present: bool,
        legacy_entries: &'a [LegacyOverlayEntry],
    ) -> Self {
        Self {
            new_ini_format,
            packed_data,
            game_normal,
            legacy_section_present,
            legacy_entries,
            map_cell_total: MAP_CELL_TOTAL,
            map_cell_width: MAP_CELL_WIDTH,
        }
    }
}

fn in_overlay_rows(cell: usize, total: usize, width: usize) -> bool {
    cell >= width && cell <= total.saturating_sub(width)
}

fn atoi_legacy(text: &str) -> i32 {
    let bytes = text.as_bytes();
    let mut index = 0;
    while bytes.get(index).is_some_and(u8::is_ascii_whitespace) { index += 1; }
    let negative = match bytes.get(index) {
        Some(b'-') => { index += 1; true }
        Some(b'+') => { index += 1; false }
        _ => false,
    };
    let mut value = 0_i32;
    let mut found = false;
    while let Some(byte) = bytes.get(index).copied() {
        if !byte.is_ascii_digit() { break; }
        found = true;
        value = value.wrapping_mul(10).wrapping_add(i32::from(byte - b'0'));
        index += 1;
    }
    if !found { 0 } else if negative { value.wrapping_neg() } else { value }
}

fn placement_allowed(kind: OverlayType, game_normal: bool, catalog: &[OverlayTypeMetadata]) -> Result<bool, OverlayError> {
    if kind.is_none() { return Ok(false); }
    Ok(game_normal || !metadata_for(catalog, kind)?.is_crate)
}

/// Port of Read_INI. The building callback must return distances in Buildings
/// list order, preserving the first building on equal distance.
pub fn read_overlay_ini<F>(
    input: OverlayIniInput<'_>,
    catalog: &[OverlayTypeMetadata],
    mut building_distances_for_cell: F,
) -> Result<Vec<OverlayPlacementRequest>, OverlayError>
where
    F: FnMut(i16) -> Vec<BuildingOwnerDistance>,
{
    if input.map_cell_width == 0 || input.map_cell_total < input.map_cell_width
        || input.map_cell_total > i16::MAX as usize
    {
        return Err(OverlayError::InvalidMapDimensions);
    }
    let mut placements = Vec::new();

    if input.new_ini_format > 1 {
        if let Some(packed) = input.packed_data.filter(|bytes| !bytes.is_empty()) {
            let raw = decompress_blocks(packed, input.map_cell_total)?;
            if raw.len() != input.map_cell_total {
                return Err(OverlayError::PackedDataLength { expected: input.map_cell_total, actual: raw.len() });
            }
            for (index, byte) in raw.into_iter().enumerate() {
                let kind = OverlayType::from_raw_byte(byte);
                if !placement_allowed(kind, input.game_normal, catalog)? { continue; }
                if !in_overlay_rows(index, input.map_cell_total, input.map_cell_width) { continue; }
                let cell = index as i16;
                let wall_owner = if metadata_for(catalog, kind)?.is_wall {
                    nearest_building_owner(building_distances_for_cell(cell))
                } else { None };
                placements.push(OverlayPlacementRequest { cell, kind, wall_owner });
            }
        }
    }

    if input.new_ini_format < 2 || input.legacy_section_present {
        for entry in input.legacy_entries {
            if !placement_allowed(entry.kind, input.game_normal, catalog)? { continue; }
            let cell = atoi_legacy(&entry.key) as i16;
            if cell < 0 || !in_overlay_rows(cell as usize, input.map_cell_total, input.map_cell_width) {
                continue;
            }
            let wall_owner = if metadata_for(catalog, entry.kind)?.is_wall {
                nearest_building_owner(building_distances_for_cell(cell))
            } else { None };
            placements.push(OverlayPlacementRequest { cell, kind: entry.kind, wall_owner });
        }
    }
    Ok(placements)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlayWritePlan {
    pub clear_legacy_section: bool,
    pub clear_packed_section: bool,
    /// Compressed bytes supplied to Put_UUBlock; INI textual encoding is
    /// performed by the CCINI adapter.
    pub packed_data: Option<Vec<u8>>,
}

/// Port of Write_INI. All map cells are emitted as signed one-byte enum values;
/// the legacy per-cell key loop is disabled in the upstream source.
pub fn write_overlay_ini(
    cell_overlays: &[OverlayType],
    map_cell_total: usize,
    staging_capacity: usize,
) -> Result<OverlayWritePlan, OverlayError> {
    if cell_overlays.len() != map_cell_total {
        return Err(OverlayError::InvalidMapLength { expected: map_cell_total, actual: cell_overlays.len() });
    }
    let raw: Vec<u8> = cell_overlays.iter().map(|kind| kind.to_raw_byte()).collect();
    let packed = compress_blocks(&raw, OVERLAY_PACK_BLOCK_SIZE)?;
    if packed.len() > staging_capacity {
        return Err(OverlayError::StagingBufferOverflow { required: packed.len(), capacity: staging_capacity });
    }
    Ok(OverlayWritePlan {
        clear_legacy_section: true,
        clear_packed_section: true,
        packed_data: (!packed.is_empty()).then_some(packed),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta(kind: OverlayType, wall: bool, crushable: bool, crate_type: bool, tib: bool) -> OverlayTypeMetadata {
        OverlayTypeMetadata::new(kind, wall, crushable, crate_type, tib)
    }

    fn catalog() -> Vec<OverlayTypeMetadata> {
        vec![
            meta(OverlayType::SANDBAG_WALL, true, true, false, false),
            meta(OverlayType::BRICK_WALL, true, false, false, false),
            meta(OverlayType::GOLD1, false, false, false, true),
            meta(OverlayType::WOOD_CRATE, false, false, true, false),
            meta(OverlayType::STEEL_CRATE, false, false, true, false),
            meta(OverlayType::WATER_CRATE, false, false, true, false),
            meta(OverlayType::FENCE, false, false, false, false),
        ]
    }

    #[test]
    fn overlay_enum_preserves_signed_char_ids() {
        assert_eq!(OverlayType::NONE.to_raw_byte(), 0xFF);
        assert_eq!(OverlayType::from_raw_byte(0xFF), OverlayType::NONE);
        assert_eq!(OverlayType::SANDBAG_WALL.0, 0);
        assert_eq!(OverlayType::WATER_CRATE.0, 24);
        assert!(OverlayType::WATER_CRATE.is_valid());
        assert!(!OverlayType(25).is_valid());
    }

    #[test]
    fn pool_allocates_frees_reuses_and_resets() {
        let mut pool = OverlayPool::new(2);
        let first = pool.allocate(OverlayType::SANDBAG_WALL).unwrap();
        let second = pool.allocate(OverlayType::GOLD1).unwrap();
        assert_eq!(pool.active_count(), 2);
        assert!(pool.allocate(OverlayType::FENCE).is_none());
        assert!(pool.free(first, true));
        assert_eq!(pool.allocate(OverlayType::FENCE), Some(first));
        assert!(pool.free(second, false));
        pool.free_all();
        assert_eq!(pool.active_count(), 0);
        assert_eq!(pool.allocate(OverlayType::GOLD1), Some(first));
    }

    #[test]
    fn constructor_scopes_owner_and_skips_unlimbo_for_limbo_position() {
        let mut system = OverlaySystem::new(2);
        let limbo = system.construct(OverlayType::GOLD1, None, Some(3), |_, _, _| {
            panic!("limbo construction must not call Unlimbo")
        }).unwrap();
        assert_eq!(system.to_own(), None);
        let placed = system.construct(OverlayType::SANDBAG_WALL, Some(130), Some(7), |object, cell, owner| {
            assert_eq!(cell, 130);
            assert_eq!(owner, Some(7));
            object.is_down = true;
            true
        }).unwrap();
        assert_eq!(system.to_own(), None);
        assert_eq!(system.pool.get(placed).unwrap().cell, Some(130));
        assert!(system.pool.get(placed).unwrap().is_down);
        assert!(system.pool.get(limbo).unwrap().is_in_limbo);
    }

    #[test]
    fn wall_marking_preserves_zone_masks_owner_and_action_order() {
        let c = catalog();
        let result = plan_overlay_mark(
            c[0], OverlayMark::Down,
            OverlayMarkContext { base_mark_succeeded: true, clear_to_build: true, to_own: Some(4), ..Default::default() },
        );
        assert!(result.succeeded);
        assert_eq!(result.actions, vec![
            OverlayAction::WriteOverlay(OverlayType::SANDBAG_WALL),
            OverlayAction::SetOverlayData(0),
            OverlayAction::RedrawObjects,
            OverlayAction::WallUpdate,
            OverlayAction::ResetMovementZones(MZONE_NORMAL_MASK),
            OverlayAction::SetCellOwner(4),
            OverlayAction::RecalculateCellAttributes,
            OverlayAction::RemoveOverlap,
            OverlayAction::SetObjectDown(false),
            OverlayAction::SetObjectInLimbo(true),
            OverlayAction::DeleteObject,
        ]);
        let solid = plan_overlay_mark(c[1], OverlayMark::Down, OverlayMarkContext {
            base_mark_succeeded: true, clear_to_build: true, ..Default::default()
        });
        assert!(solid.actions.contains(&OverlayAction::ResetMovementZones(3)));
    }

    #[test]
    fn illegal_wall_is_deleted_and_returns_false_before_common_cleanup() {
        let result = plan_overlay_mark(catalog()[0], OverlayMark::Down, OverlayMarkContext {
            base_mark_succeeded: true, clear_to_build: false, ..Default::default()
        });
        assert!(!result.succeeded);
        assert!(result.deleted);
        assert_eq!(result.actions, vec![OverlayAction::DeleteObject]);
    }

    #[test]
    fn non_wall_clearance_uses_float_track_and_ignore_units_rules() {
        let c = catalog();
        let water = plan_overlay_mark(c[5], OverlayMark::Down, OverlayMarkContext {
            base_mark_succeeded: true, clear_for_float: true, ..Default::default()
        });
        assert!(water.actions.contains(&OverlayAction::WriteOverlay(OverlayType::WATER_CRATE)));
        let steel = plan_overlay_mark(c[4], OverlayMark::Down, OverlayMarkContext {
            base_mark_succeeded: true, clear_for_track: true, ..Default::default()
        });
        assert!(steel.actions.contains(&OverlayAction::WriteOverlay(OverlayType::STEEL_CRATE)));
        let ordinary = plan_overlay_mark(c[6], OverlayMark::Down, OverlayMarkContext {
            base_mark_succeeded: true, clear_for_track_ignoring_units: true, ..Default::default()
        });
        assert!(ordinary.actions.contains(&OverlayAction::WriteOverlay(OverlayType::FENCE)));
    }

    #[test]
    fn rejected_non_wall_placement_still_recalculates_cleans_up_and_succeeds() {
        let result = plan_overlay_mark(catalog()[6], OverlayMark::Down, OverlayMarkContext {
            base_mark_succeeded: true,
            existing_overlay: OverlayType::GOLD1,
            clear_for_track_ignoring_units: true,
            ..Default::default()
        });
        assert!(result.succeeded && result.deleted);
        assert!(!result.actions.iter().any(|a| matches!(a, OverlayAction::WriteOverlay(_))));
        assert!(result.actions.contains(&OverlayAction::RecalculateCellAttributes));
        assert!(result.actions.contains(&OverlayAction::RemoveOverlap));
    }

    #[test]
    fn scenario_init_bypasses_existing_overlay_and_passability() {
        let result = plan_overlay_mark(catalog()[6], OverlayMark::Down, OverlayMarkContext {
            base_mark_succeeded: true,
            scenario_init: true,
            existing_overlay: OverlayType::BRICK_WALL,
            ..Default::default()
        });
        assert!(result.actions.contains(&OverlayAction::WriteOverlay(OverlayType::FENCE)));
    }

    #[test]
    fn tiberium_mark_sets_data_one_and_adjusts_after_redraw() {
        let result = plan_overlay_mark(catalog()[2], OverlayMark::Down, OverlayMarkContext {
            base_mark_succeeded: true,
            clear_for_track_ignoring_units: true,
            ..Default::default()
        });
        assert_eq!(result.actions, vec![
            OverlayAction::WriteOverlay(OverlayType::GOLD1),
            OverlayAction::SetOverlayData(0),
            OverlayAction::RedrawObjects,
            OverlayAction::SetOverlayData(1),
            OverlayAction::TiberiumAdjust,
            OverlayAction::RecalculateCellAttributes,
            OverlayAction::RemoveOverlap,
            OverlayAction::SetObjectDown(false),
            OverlayAction::SetObjectInLimbo(true),
            OverlayAction::DeleteObject,
        ]);
    }

    #[test]
    fn base_mark_failure_and_other_marks_do_not_run_overlay_logic() {
        let failed = plan_overlay_mark(catalog()[0], OverlayMark::Down, OverlayMarkContext::default());
        assert!(!failed.succeeded && !failed.deleted && failed.actions.is_empty());
        let other = plan_overlay_mark(catalog()[0], OverlayMark::Other, OverlayMarkContext {
            base_mark_succeeded: true, ..Default::default()
        });
        assert!(other.succeeded && !other.deleted && other.actions.is_empty());
    }

    #[test]
    fn nearest_building_owner_keeps_first_building_on_equal_distance() {
        assert_eq!(nearest_building_owner([
            BuildingOwnerDistance { owner: 2, distance: 10 },
            BuildingOwnerDistance { owner: 4, distance: 10 },
            BuildingOwnerDistance { owner: 5, distance: 12 },
        ]), Some(2));
        assert_eq!(nearest_building_owner(Vec::<BuildingOwnerDistance>::new()), None);
        assert_eq!(nearest_building_owner([BuildingOwnerDistance { owner: 1, distance: i32::MAX }]), None);
    }

    #[test]
    fn packed_read_respects_row_limits_and_multiplayer_crate_filter() {
        let metadata = catalog();
        let mut raw = vec![OverlayType::NONE.to_raw_byte(); 16];
        raw[0] = OverlayType::SANDBAG_WALL.to_raw_byte();
        raw[3] = OverlayType::GOLD1.to_raw_byte();
        raw[4] = OverlayType::WATER_CRATE.to_raw_byte();
        raw[5] = OverlayType::BRICK_WALL.to_raw_byte();
        raw[12] = OverlayType::GOLD1.to_raw_byte();
        let packed = compress_blocks(&raw, OVERLAY_PACK_BLOCK_SIZE).unwrap();
        let input = OverlayIniInput {
            new_ini_format: 2, packed_data: Some(&packed), game_normal: false,
            legacy_section_present: false, legacy_entries: &[], map_cell_total: 16, map_cell_width: 4,
        };
        let result = read_overlay_ini(input, &metadata, |cell| vec![
            BuildingOwnerDistance { owner: 9, distance: i32::from(cell) },
        ]).unwrap();
        assert_eq!(result, vec![
            OverlayPlacementRequest { cell: 5, kind: OverlayType::BRICK_WALL, wall_owner: Some(9) },
            OverlayPlacementRequest { cell: 12, kind: OverlayType::GOLD1, wall_owner: None },
        ]);
    }

    #[test]
    fn legacy_read_uses_atoi_prefix_and_format_section_gate() {
        let metadata = catalog();
        let entries = [
            LegacyOverlayEntry { key: " 5tail".into(), kind: OverlayType::SANDBAG_WALL },
            LegacyOverlayEntry { key: "0".into(), kind: OverlayType::GOLD1 },
            LegacyOverlayEntry { key: "15".into(), kind: OverlayType::FENCE },
            LegacyOverlayEntry { key: "7".into(), kind: OverlayType::NONE },
        ];
        let old = OverlayIniInput {
            new_ini_format: 1, packed_data: None, game_normal: true,
            legacy_section_present: false, legacy_entries: &entries, map_cell_total: 16, map_cell_width: 4,
        };
        let placements = read_overlay_ini(old, &metadata, |_| vec![]).unwrap();
        assert_eq!(placements.iter().map(|p| p.cell).collect::<Vec<_>>(), vec![5, 15]);
        let explicit = OverlayIniInput { new_ini_format: 2, legacy_section_present: true, ..old };
        assert_eq!(read_overlay_ini(explicit, &metadata, |_| vec![]).unwrap(), placements);
    }

    #[test]
    fn multiplayer_legacy_load_suppresses_crates_but_keeps_non_crates() {
        let metadata = catalog();
        let entries = [
            LegacyOverlayEntry { key: "5".into(), kind: OverlayType::WOOD_CRATE },
            LegacyOverlayEntry { key: "6".into(), kind: OverlayType::STEEL_CRATE },
            LegacyOverlayEntry { key: "7".into(), kind: OverlayType::WATER_CRATE },
            LegacyOverlayEntry { key: "8".into(), kind: OverlayType::GOLD1 },
        ];
        let input = OverlayIniInput {
            new_ini_format: 1, packed_data: None, game_normal: false,
            legacy_section_present: false, legacy_entries: &entries, map_cell_total: 16, map_cell_width: 4,
        };
        let result = read_overlay_ini(input, &metadata, |_| vec![]).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].kind, OverlayType::GOLD1);
    }

    #[test]
    fn write_round_trips_all_signed_enum_bytes_and_clears_both_sections() {
        let overlays: Vec<_> = (0..MAP_CELL_TOTAL).map(|i| {
            if i % 3 == 0 { OverlayType::NONE } else { OverlayType((i % OverlayType::COUNT) as i8) }
        }).collect();
        let plan = write_overlay_ini(&overlays, MAP_CELL_TOTAL, 32 * 1024).unwrap();
        assert!(plan.clear_legacy_section && plan.clear_packed_section);
        let packed = plan.packed_data.unwrap();
        let decoded = decompress_blocks(&packed, MAP_CELL_TOTAL).unwrap();
        assert_eq!(decoded, overlays.iter().map(|kind| kind.to_raw_byte()).collect::<Vec<_>>());
        assert!(write_overlay_ini(&overlays, MAP_CELL_TOTAL, 1).is_err());
    }

    #[test]
    fn write_rejects_wrong_map_cell_count() {
        assert_eq!(
            write_overlay_ini(&[OverlayType::NONE], MAP_CELL_TOTAL, 4096),
            Err(OverlayError::InvalidMapLength { expected: MAP_CELL_TOTAL, actual: 1 })
        );
    }
}
