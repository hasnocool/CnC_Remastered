//! Red Alert CrateClass from REDALERT/CRATE.CPP and CRATE.H.

pub type CrateCell = i32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrateOverlay {
    None,
    WoodCrate,
    SteelCrate,
    WaterCrate,
    Other(u16),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrateMovementSpeed { Float, Foot, Track }

pub trait CrateMapHost {
    fn scenario_init(&self) -> i32;
    fn set_scenario_init(&mut self, value: i32);

    fn in_radar(&self, cell: CrateCell) -> bool;
    fn overlay(&self, cell: CrateCell) -> CrateOverlay;
    fn is_clear_to_build(&self, cell: CrateCell, speed: CrateMovementSpeed) -> bool;

    /// Map::Pick_Random_Location. None means the map adapter cannot provide
    /// another candidate; the source assumes the global map can always do so.
    fn pick_random_location(&mut self) -> Option<CrateCell>;
    fn nearby_location(&mut self, cell: CrateCell, speed: CrateMovementSpeed) -> CrateCell;
    fn percent_chance(&mut self, percent: u32) -> bool;

    fn place_overlay(&mut self, cell: CrateCell, overlay: CrateOverlay);
    fn clear_overlay(&mut self, cell: CrateCell);
    fn redraw_objects(&mut self, cell: CrateCell);

    fn pick_random_timer(&mut self, minimum: u32, maximum: u32) -> u32;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrateCreateError {
    InvalidTimerRange,
    TimerOverflow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrateClass {
    cell: CrateCell,
    timer_expiration: Option<u32>,
}

impl Default for CrateClass {
    fn default() -> Self {
        Self { cell: -1, timer_expiration: None }
    }
}

impl CrateClass {
    pub const fn new() -> Self { Self { cell: -1, timer_expiration: None } }
    pub const fn cell(&self) -> CrateCell { self.cell }
    pub const fn is_valid(&self) -> bool { self.cell != -1 }
    pub const fn is_here(&self, cell: CrateCell) -> bool { self.is_valid() && self.cell == cell }

    /// Init and Make_Invalid clear the tracked cell and stop the timer.
    pub fn init(&mut self) { self.make_invalid(); }
    fn make_invalid(&mut self) {
        self.cell = -1;
        self.timer_expiration = None;
    }

    /// Is_Expired: valid crates expire when their FrameTimer reaches zero.
    /// Signed wrapping subtraction supports the ordinary u32 tick rollover.
    pub fn is_expired(&self, now_tick: u32) -> bool {
        self.is_valid()
            && self.timer_expiration.is_some_and(|deadline| (now_tick.wrapping_sub(deadline) as i32) >= 0)
    }

    pub fn timer_expiration(&self) -> Option<u32> { self.timer_expiration }

    /// Remove_It always invalidates a valid tracked crate, even if the cell's
    /// overlay no longer contains a recognized crate type.
    pub fn remove_it(&mut self, host: &mut impl CrateMapHost) -> bool {
        if !self.is_valid() { return false; }
        let old_cell = self.cell;
        let _ = Self::get_crate(host, old_cell);
        self.make_invalid();
        true
    }

    /// Create_Crate first removes the old tracked crate, tries to place a new
    /// overlay, then arms a random lifetime in the source's half-to-double
    /// CrateTime range.
    pub fn create_crate(
        &mut self,
        requested_cell: CrateCell,
        crate_time: u32,
        ticks_per_minute: u32,
        now_tick: u32,
        water_crate_chance: u32,
        host: &mut impl CrateMapHost,
    ) -> Result<bool, CrateCreateError> {
        self.remove_it(host);

        let minimum = crate_time.checked_mul(ticks_per_minute / 2)
            .ok_or(CrateCreateError::InvalidTimerRange)?;
        let maximum = crate_time.checked_mul(ticks_per_minute.saturating_mul(2))
            .ok_or(CrateCreateError::InvalidTimerRange)?;
        if minimum > maximum { return Err(CrateCreateError::InvalidTimerRange); }

        let mut actual_cell = requested_cell;
        if !Self::put_crate(host, &mut actual_cell, water_crate_chance) {
            return Ok(false);
        }

        self.cell = actual_cell;
        let duration = host.pick_random_timer(minimum, maximum);
        self.timer_expiration = Some(now_tick.checked_add(duration).ok_or(CrateCreateError::TimerOverflow)?);
        Ok(true)
    }

    /// Put_Crate saves ScenarioInit, clears it for placement, retries blocked
    /// overlay cells using a random location and water/track approach, then
    /// restores ScenarioInit on every normal return path.
    fn put_crate(
        host: &mut impl CrateMapHost,
        cell: &mut CrateCell,
        water_crate_chance: u32,
    ) -> bool {
        let previous_scenario_init = host.scenario_init();
        host.set_scenario_init(0);

        if !host.in_radar(*cell) {
            host.set_scenario_init(previous_scenario_init);
            return false;
        }

        while host.overlay(*cell) != CrateOverlay::None
            && !host.is_clear_to_build(*cell, CrateMovementSpeed::Float)
            && !host.is_clear_to_build(*cell, CrateMovementSpeed::Foot)
        {
            let Some(random_cell) = host.pick_random_location() else {
                host.set_scenario_init(previous_scenario_init);
                return false;
            };
            *cell = random_cell;
            let speed = if host.percent_chance(water_crate_chance.saturating_mul(100)) {
                CrateMovementSpeed::Float
            } else {
                CrateMovementSpeed::Track
            };
            *cell = host.nearby_location(*cell, speed);
        }

        let overlay = if host.is_clear_to_build(*cell, CrateMovementSpeed::Float) {
            CrateOverlay::WaterCrate
        } else {
            CrateOverlay::WoodCrate
        };
        host.place_overlay(*cell, overlay);
        host.set_scenario_init(previous_scenario_init);
        true
    }

    /// Get_Crate removes only recognized crate overlays and redraws objects.
    fn get_crate(host: &mut impl CrateMapHost, cell: CrateCell) -> bool {
        if !host.in_radar(cell) { return false; }
        match host.overlay(cell) {
            CrateOverlay::WoodCrate | CrateOverlay::SteelCrate | CrateOverlay::WaterCrate => {
                host.clear_overlay(cell);
                host.redraw_objects(cell);
                true
            }
            CrateOverlay::None | CrateOverlay::Other(_) => false,
        }
    }

    pub fn get_crate_at(host: &mut impl CrateMapHost, cell: CrateCell) -> bool {
        Self::get_crate(host, cell)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    struct FakeMap {
        scenario_init: i32,
        radar: HashMap<CrateCell, bool>,
        overlays: HashMap<CrateCell, CrateOverlay>,
        clear_float: HashMap<CrateCell, bool>,
        clear_foot: HashMap<CrateCell, bool>,
        random_cells: Vec<CrateCell>,
        percent_result: bool,
        timer_duration: u32,
        placed: Vec<(CrateCell, CrateOverlay)>,
        cleared: Vec<CrateCell>,
        redrawn: Vec<CrateCell>,
        nearby: Vec<(CrateCell, CrateMovementSpeed)>,
        chance_calls: Vec<u32>,
    }

    impl CrateMapHost for FakeMap {
        fn scenario_init(&self) -> i32 { self.scenario_init }
        fn set_scenario_init(&mut self, value: i32) { self.scenario_init = value; }
        fn in_radar(&self, cell: CrateCell) -> bool { self.radar.get(&cell).copied().unwrap_or(false) }
        fn overlay(&self, cell: CrateCell) -> CrateOverlay {
            self.overlays.get(&cell).copied().unwrap_or(CrateOverlay::None)
        }
        fn is_clear_to_build(&self, cell: CrateCell, speed: CrateMovementSpeed) -> bool {
            match speed {
                CrateMovementSpeed::Float => self.clear_float.get(&cell).copied().unwrap_or(false),
                CrateMovementSpeed::Foot => self.clear_foot.get(&cell).copied().unwrap_or(false),
                CrateMovementSpeed::Track => false,
            }
        }
        fn pick_random_location(&mut self) -> Option<CrateCell> {
            if self.random_cells.is_empty() { None } else { Some(self.random_cells.remove(0)) }
        }
        fn nearby_location(&mut self, cell: CrateCell, speed: CrateMovementSpeed) -> CrateCell {
            self.nearby.push((cell, speed));
            cell
        }
        fn percent_chance(&mut self, percent: u32) -> bool {
            self.chance_calls.push(percent);
            self.percent_result
        }
        fn place_overlay(&mut self, cell: CrateCell, overlay: CrateOverlay) {
            self.placed.push((cell, overlay));
            self.overlays.insert(cell, overlay);
        }
        fn clear_overlay(&mut self, cell: CrateCell) {
            self.cleared.push(cell);
            self.overlays.insert(cell, CrateOverlay::None);
        }
        fn redraw_objects(&mut self, cell: CrateCell) { self.redrawn.push(cell); }
        fn pick_random_timer(&mut self, _minimum: u32, _maximum: u32) -> u32 { self.timer_duration }
    }

    #[test]
    fn constructor_and_init_use_invalid_cell_and_stopped_timer() {
        let mut crate_class = CrateClass::new();
        assert!(!crate_class.is_valid());
        assert_eq!(crate_class.timer_expiration(), None);
        crate_class.cell = 42;
        crate_class.timer_expiration = Some(500);
        crate_class.init();
        assert_eq!(crate_class.cell(), -1);
        assert_eq!(crate_class.timer_expiration(), None);
    }

    #[test]
    fn create_places_water_crate_and_starts_random_lifetime() {
        let mut host = FakeMap { scenario_init: 9, timer_duration: 1200, ..FakeMap::default() };
        host.radar.insert(10, true);
        host.clear_float.insert(10, true);
        let mut crate_class = CrateClass::new();
        assert!(crate_class.create_crate(10, 5, 60, 100, 25, &mut host).unwrap());
        assert_eq!(crate_class.cell(), 10);
        assert_eq!(crate_class.timer_expiration(), Some(1300));
        assert_eq!(host.placed, vec![(10, CrateOverlay::WaterCrate)]);
        assert_eq!(host.scenario_init, 9);
    }

    #[test]
    fn blocked_cell_retries_nearby_track_location_and_places_wood_crate() {
        let mut host = FakeMap { scenario_init: 7, timer_duration: 10, ..FakeMap::default() };
        host.radar.insert(1, true);
        host.overlays.insert(1, CrateOverlay::Other(4));
        host.random_cells.push(2);
        host.radar.insert(2, true);
        host.overlays.insert(2, CrateOverlay::None);
        host.clear_foot.insert(2, true);
        host.percent_result = false;
        let mut crate_class = CrateClass::new();
        assert!(crate_class.create_crate(1, 1, 60, 0, 50, &mut host).unwrap());
        assert_eq!(crate_class.cell(), 2);
        assert_eq!(host.nearby, vec![(2, CrateMovementSpeed::Track)]);
        assert_eq!(host.chance_calls, vec![5000]);
        assert_eq!(host.placed, vec![(2, CrateOverlay::WoodCrate)]);
        assert_eq!(host.scenario_init, 7);
    }

    #[test]
    fn create_rejects_out_of_radar_location_and_restores_scenario_state() {
        let mut host = FakeMap { scenario_init: 11, ..FakeMap::default() };
        let mut crate_class = CrateClass::new();
        assert!(!crate_class.create_crate(99, 1, 60, 0, 20, &mut host).unwrap());
        assert_eq!(host.scenario_init, 11);
        assert!(!crate_class.is_valid());
    }

    #[test]
    fn remove_invalidates_even_if_the_crate_overlay_is_already_missing() {
        let mut host = FakeMap::default();
        host.radar.insert(5, true);
        let mut crate_class = CrateClass { cell: 5, timer_expiration: Some(500) };
        assert!(crate_class.remove_it(&mut host));
        assert!(!crate_class.is_valid());
        assert_eq!(crate_class.timer_expiration(), None);
        assert!(host.cleared.is_empty());
    }

    #[test]
    fn recognized_crate_overlay_is_removed_and_objects_redrawn() {
        let mut host = FakeMap::default();
        host.radar.insert(5, true);
        host.overlays.insert(5, CrateOverlay::SteelCrate);
        let mut crate_class = CrateClass { cell: 5, timer_expiration: Some(500) };
        assert!(crate_class.remove_it(&mut host));
        assert_eq!(host.cleared, vec![5]);
        assert_eq!(host.redrawn, vec![5]);
    }

    #[test]
    fn expiration_requires_valid_crate_and_uses_wrapping_tick_comparison() {
        let mut crate_class = CrateClass { cell: 1, timer_expiration: Some(5) };
        assert!(!crate_class.is_expired(4));
        assert!(crate_class.is_expired(5));
        crate_class.timer_expiration = Some(u32::MAX - 2);
        assert!(!crate_class.is_expired(0));
        assert!(crate_class.is_expired(3));
        crate_class.init();
        assert!(!crate_class.is_expired(u32::MAX));
    }
}
