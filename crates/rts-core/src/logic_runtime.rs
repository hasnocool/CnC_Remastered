//! Tiberian Dawn LogicClass from TIBERIANDAWN/LOGIC.CPP.
//!
//! A single AI frame is executed in source order. Engine collections, object
//! AI, map logic and per-house bit fields are supplied through LogicWorld.

pub const HOUSE_GOOD: u8 = 0;
pub const HOUSE_NEUTRAL: u8 = 2;
pub const HOUSE_JURASSIC_PARK: u8 = 3;
pub const HOUSE_MULTI1: u8 = 4;
pub const HOUSE_COUNT: u8 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicGameMode { Normal, Multiplayer, Skirmish }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicScanCategory { Unit, Infantry, Aircraft, Building }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicScanObject {
    pub house: u8,
    pub is_locked: bool,
    pub house_is_human: bool,
    pub discovered_by_player: bool,
    pub is_in_limbo: bool,
    pub type_index: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicConfig {
    pub game_mode: LogicGameMode,
    pub crate_maker_enabled: bool,
    pub use_ra_ai: bool,
    pub ticks_per_minute: u32,
}

pub trait LogicWorld {
    type TeamId: Copy;
    type ObjectId: Copy;
    type FactoryId: Copy;

    fn crate_timer_expired(&mut self) -> bool;
    fn place_random_crate(&mut self);
    fn random_pick(&mut self, minimum: u32, maximum: u32) -> u32;
    fn set_crate_timer(&mut self, ticks: u32);

    fn team_count(&self) -> usize;
    fn team_at(&self, index: usize) -> Option<Self::TeamId>;
    fn team_ai(&mut self, team: Self::TeamId);

    fn logic_object_count(&self) -> usize;
    fn logic_object_at(&self, index: usize) -> Option<Self::ObjectId>;
    fn object_ai(&mut self, object: Self::ObjectId);

    fn scan_object_count(&self, category: LogicScanCategory) -> usize;
    fn scan_object_at(&self, category: LogicScanCategory, index: usize) -> Option<LogicScanObject>;
    fn add_house_scan_bits(
        &mut self,
        house: u8,
        category: LogicScanCategory,
        type_bit: u32,
        active: bool,
    );

    fn recalculate_house_attributes(&mut self);
    fn map_logic(&mut self);

    fn factory_count(&self) -> usize;
    fn factory_at(&self, index: usize) -> Option<Self::FactoryId>;
    fn factory_ai(&mut self, factory: Self::FactoryId);

    fn house_is_active(&self, house: u8) -> bool;
    fn house_ai(&mut self, house: u8);

    fn set_object_recently_created(&mut self, object: Self::ObjectId, recently_created: bool);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LogicFrameReport {
    pub frame_rate_counter: u32,
    pub crate_placed: bool,
    pub teams_processed: usize,
    pub objects_processed: usize,
    pub scan_bits_added: usize,
    pub factories_processed: usize,
    pub houses_processed: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LogicClass {
    pub frames_per_second: u32,
}

impl LogicClass {
    /// One ordered LogicClass::AI pass.
    pub fn ai(&mut self, config: LogicConfig, world: &mut impl LogicWorld) -> LogicFrameReport {
        self.frames_per_second = self.frames_per_second.wrapping_add(1);
        let mut report = LogicFrameReport {
            frame_rate_counter: self.frames_per_second,
            ..LogicFrameReport::default()
        };

        // Crate regeneration.
        if config.game_mode != LogicGameMode::Normal
            && config.crate_maker_enabled
            && world.crate_timer_expired()
        {
            world.place_random_crate();
            let minutes = world.random_pick(7, 15);
            world.set_crate_timer(config.ticks_per_minute.wrapping_mul(minutes));
            report.crate_placed = true;
        }

        // Team AI pass.
        let mut index = 0;
        while index < world.team_count() {
            if let Some(team) = world.team_at(index) {
                world.team_ai(team);
                report.teams_processed += 1;
            }
            index += 1;
        }

        // Object AI pass. The source snapshots Count before calling AI and
        // adjusts the current index by the count difference if objects were
        // deleted. This prevents the next object shifting into the current
        // index from being skipped.
        let mut object_index: i64 = 0;
        while object_index >= 0 && (object_index as usize) < world.logic_object_count() {
            let before = world.logic_object_count() as i64;
            if let Some(object) = world.logic_object_at(object_index as usize) {
                world.object_ai(object);
                report.objects_processed += 1;
            }
            let after = world.logic_object_count() as i64;
            let count_diff = after - before;
            object_index = (object_index + count_diff + 1).max(0);
        }

        // Locked-object scan masks are accumulated only for houses eligible
        // for the current session, and active masks exclude objects in limbo.
        for category in [
            LogicScanCategory::Unit,
            LogicScanCategory::Infantry,
            LogicScanCategory::Aircraft,
            LogicScanCategory::Building,
        ] {
            for index in 0..world.scan_object_count(category) {
                let Some(object) = world.scan_object_at(category, index) else { continue };
                let owner_is_visible = config.game_mode != LogicGameMode::Normal
                    || !object.house_is_human
                    || object.discovered_by_player;
                if !object.is_locked || !owner_is_visible {
                    continue;
                }
                if object.type_index < 32 {
                    let bit = 1_u32 << object.type_index;
                    world.add_house_scan_bits(object.house, category, bit, false);
                    report.scan_bits_added += 1;
                    if !object.is_in_limbo {
                        world.add_house_scan_bits(object.house, category, bit, true);
                        report.scan_bits_added += 1;
                    }
                }
            }
        }

        if config.use_ra_ai {
            world.recalculate_house_attributes();
        }

        // Global map logic follows object scan accumulation and RA recalc.
        world.map_logic();

        // Factories are processed after map logic.
        let mut factory_index = 0;
        while factory_index < world.factory_count() {
            if let Some(factory) = world.factory_at(factory_index) {
                world.factory_ai(factory);
                report.factories_processed += 1;
            }
            factory_index += 1;
        }

        // In non-normal games only multiplayer houses, neutral and Jurassic
        // Park run AI. Normal games process the full house ID range.
        match config.game_mode {
            LogicGameMode::Normal => {
                for house in HOUSE_GOOD..HOUSE_COUNT {
                    if world.house_is_active(house) {
                        world.house_ai(house);
                        report.houses_processed += 1;
                    }
                }
            }
            LogicGameMode::Multiplayer | LogicGameMode::Skirmish => {
                for house in HOUSE_MULTI1..HOUSE_COUNT {
                    if world.house_is_active(house) {
                        world.house_ai(house);
                        report.houses_processed += 1;
                    }
                }
                for house in [HOUSE_NEUTRAL, HOUSE_JURASSIC_PARK] {
                    if world.house_is_active(house) {
                        world.house_ai(house);
                        report.houses_processed += 1;
                    }
                }
            }
        }

        report
    }

    /// Port of Clear_Recently_Created_Bits.
    pub fn clear_recently_created_bits(&self, world: &mut impl LogicWorld) -> usize {
        let count = world.logic_object_count();
        let mut cleared = 0;
        for index in 0..count {
            if let Some(object) = world.logic_object_at(index) {
                world.set_object_recently_created(object, false);
                cleared += 1;
            }
        }
        cleared
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[derive(Default)]
    struct MockWorld {
        crate_expired: bool,
        crates_placed: usize,
        crate_timer: u32,
        random_value: u32,
        team_ids: Vec<u32>,
        team_calls: Vec<u32>,
        objects: Vec<u32>,
        removed_on_ai: BTreeSet<u32>,
        object_calls: Vec<u32>,
        scan: Vec<(LogicScanCategory, LogicScanObject)>,
        scan_updates: Vec<(u8, LogicScanCategory, u32, bool)>,
        ra_recalculations: usize,
        map_calls: usize,
        factories: Vec<u32>,
        factory_calls: Vec<u32>,
        active_houses: BTreeSet<u8>,
        house_calls: Vec<u8>,
        recently_created: BTreeSet<u32>,
        recently_cleared: Vec<u32>,
        operation_order: Vec<String>,
    }

    impl LogicWorld for MockWorld {
        type TeamId = u32;
        type ObjectId = u32;
        type FactoryId = u32;

        fn crate_timer_expired(&mut self) -> bool {
            let value = self.crate_expired;
            self.crate_expired = false;
            value
        }
        fn place_random_crate(&mut self) {
            self.crates_placed += 1;
            self.operation_order.push("crate".into());
        }
        fn random_pick(&mut self, minimum: u32, maximum: u32) -> u32 {
            self.random_value.clamp(minimum, maximum)
        }
        fn set_crate_timer(&mut self, ticks: u32) { self.crate_timer = ticks; }

        fn team_count(&self) -> usize { self.team_ids.len() }
        fn team_at(&self, index: usize) -> Option<u32> { self.team_ids.get(index).copied() }
        fn team_ai(&mut self, team: u32) {
            self.team_calls.push(team);
            self.operation_order.push(format!("team:{team}"));
        }

        fn logic_object_count(&self) -> usize { self.objects.len() }
        fn logic_object_at(&self, index: usize) -> Option<u32> { self.objects.get(index).copied() }
        fn object_ai(&mut self, object: u32) {
            self.object_calls.push(object);
            self.operation_order.push(format!("object:{object}"));
            if self.removed_on_ai.contains(&object) {
                let index = self.objects.iter().position(|item| *item == object).unwrap();
                self.objects.remove(index);
            }
        }

        fn scan_object_count(&self, category: LogicScanCategory) -> usize {
            self.scan.iter().filter(|(kind, _)| *kind == category).count()
        }
        fn scan_object_at(&self, category: LogicScanCategory, index: usize) -> Option<LogicScanObject> {
            self.scan.iter().filter(|(kind, _)| *kind == category).nth(index).map(|(_, item)| *item)
        }
        fn add_house_scan_bits(&mut self, house: u8, category: LogicScanCategory, bit: u32, active: bool) {
            self.scan_updates.push((house, category, bit, active));
            self.operation_order.push(if active { "scan-active".into() } else { "scan-new".into() });
        }

        fn recalculate_house_attributes(&mut self) {
            self.ra_recalculations += 1;
            self.operation_order.push("recalc".into());
        }
        fn map_logic(&mut self) {
            self.map_calls += 1;
            self.operation_order.push("map".into());
        }

        fn factory_count(&self) -> usize { self.factories.len() }
        fn factory_at(&self, index: usize) -> Option<u32> { self.factories.get(index).copied() }
        fn factory_ai(&mut self, factory: u32) {
            self.factory_calls.push(factory);
            self.operation_order.push(format!("factory:{factory}"));
        }

        fn house_is_active(&self, house: u8) -> bool { self.active_houses.contains(&house) }
        fn house_ai(&mut self, house: u8) {
            self.house_calls.push(house);
            self.operation_order.push(format!("house:{house}"));
        }

        fn set_object_recently_created(&mut self, object: u32, recently_created: bool) {
            if recently_created { self.recently_created.insert(object); }
            else {
                self.recently_created.remove(&object);
                self.recently_cleared.push(object);
            }
        }
    }

    fn config(mode: LogicGameMode) -> LogicConfig {
        LogicConfig {
            game_mode: mode,
            crate_maker_enabled: true,
            use_ra_ai: true,
            ticks_per_minute: 60,
        }
    }

    #[test]
    fn crate_regeneration_runs_only_for_non_normal_game_and_resets_timer() {
        let mut world = MockWorld { crate_expired: true, random_value: 11, ..MockWorld::default() };
        let mut logic = LogicClass::default();
        let normal = logic.ai(config(LogicGameMode::Normal), &mut world);
        assert!(!normal.crate_placed);
        assert_eq!(world.crates_placed, 0);

        world.crate_expired = true;
        let multiplayer = logic.ai(config(LogicGameMode::Multiplayer), &mut world);
        assert!(multiplayer.crate_placed);
        assert_eq!(world.crates_placed, 1);
        assert_eq!(world.crate_timer, 660);
    }

    #[test]
    fn team_and_object_ai_run_in_source_order() {
        let mut world = MockWorld {
            team_ids: vec![1, 2],
            objects: vec![10, 11, 12],
            ..MockWorld::default()
        };
        let mut logic = LogicClass::default();
        let result = logic.ai(config(LogicGameMode::Normal), &mut world);
        assert_eq!(world.team_calls, vec![1, 2]);
        assert_eq!(world.object_calls, vec![10, 11, 12]);
        assert_eq!(result.teams_processed, 2);
        assert_eq!(result.objects_processed, 3);
        assert!(world.operation_order.iter().position(|item| item == "team:2")
            < world.operation_order.iter().position(|item| item == "object:10"));
    }

    #[test]
    fn object_iteration_does_not_skip_after_an_ai_deletion() {
        let mut world = MockWorld {
            objects: vec![10, 11, 12, 13],
            removed_on_ai: BTreeSet::from([10, 12]),
            ..MockWorld::default()
        };
        let mut logic = LogicClass::default();
        let result = logic.ai(config(LogicGameMode::Normal), &mut world);
        assert_eq!(world.object_calls, vec![10, 11, 12, 13]);
        assert_eq!(world.objects, vec![11, 13]);
        assert_eq!(result.objects_processed, 4);
    }

    #[test]
    fn scan_masks_require_locked_object_and_session_visibility() {
        let mut world = MockWorld {
            scan: vec![
                (LogicScanCategory::Unit, LogicScanObject { house: 0, is_locked: true, house_is_human: true, discovered_by_player: false, is_in_limbo: false, type_index: 2 }),
                (LogicScanCategory::Unit, LogicScanObject { house: 1, is_locked: true, house_is_human: true, discovered_by_player: true, is_in_limbo: true, type_index: 3 }),
                (LogicScanCategory::Unit, LogicScanObject { house: 2, is_locked: false, house_is_human: false, discovered_by_player: true, is_in_limbo: false, type_index: 4 }),
            ],
            ..MockWorld::default()
        };
        let mut logic = LogicClass::default();
        let normal = logic.ai(config(LogicGameMode::Normal), &mut world);
        assert_eq!(normal.scan_bits_added, 1);
        let multi = logic.ai(config(LogicGameMode::Multiplayer), &mut world);
        assert_eq!(multi.scan_bits_added, 3);
        assert_eq!(world.scan_updates, vec![
            (1, LogicScanCategory::Unit, 1 << 3, false),
            (0, LogicScanCategory::Unit, 1 << 2, false),
            (0, LogicScanCategory::Unit, 1 << 2, true),
            (1, LogicScanCategory::Unit, 1 << 3, false),
        ]);
    }

    #[test]
    fn scan_active_mask_is_skipped_for_limbo_objects() {
        let mut world = MockWorld {
            scan: vec![(LogicScanCategory::Infantry, LogicScanObject {
                house: 2, is_locked: true, house_is_human: false, discovered_by_player: false,
                is_in_limbo: true, type_index: 1,
            })],
            ..MockWorld::default()
        };
        LogicClass::default().ai(config(LogicGameMode::Multiplayer), &mut world);
        assert_eq!(world.scan_updates, vec![(2, LogicScanCategory::Infantry, 2, false)]);
    }

    #[test]
    fn ra_attribute_recalculation_occurs_before_map_logic() {
        let mut world = MockWorld::default();
        LogicClass::default().ai(config(LogicGameMode::Normal), &mut world);
        assert_eq!(world.ra_recalculations, 1);
        let recalc = world.operation_order.iter().position(|item| item == "recalc").unwrap();
        let map = world.operation_order.iter().position(|item| item == "map").unwrap();
        assert!(recalc < map);

        let mut disabled = MockWorld::default();
        let mut cfg = config(LogicGameMode::Normal);
        cfg.use_ra_ai = false;
        LogicClass::default().ai(cfg, &mut disabled);
        assert_eq!(disabled.ra_recalculations, 0);
    }

    #[test]
    fn factories_run_after_map_logic() {
        let mut world = MockWorld { factories: vec![1, 2], ..MockWorld::default() };
        let result = LogicClass::default().ai(config(LogicGameMode::Normal), &mut world);
        assert_eq!(world.factory_calls, vec![1, 2]);
        assert_eq!(result.factories_processed, 2);
        let map = world.operation_order.iter().position(|item| item == "map").unwrap();
        let factory = world.operation_order.iter().position(|item| item == "factory:1").unwrap();
        assert!(map < factory);
    }

    #[test]
    fn normal_and_multiplayer_house_ai_use_different_source_ranges() {
        let active_houses = BTreeSet::from([0, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut normal = MockWorld { active_houses: active_houses.clone(), ..MockWorld::default() };
        LogicClass::default().ai(config(LogicGameMode::Normal), &mut normal);
        assert_eq!(normal.house_calls, vec![0, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut multiplayer = MockWorld { active_houses, ..MockWorld::default() };
        LogicClass::default().ai(config(LogicGameMode::Multiplayer), &mut multiplayer);
        assert_eq!(multiplayer.house_calls, vec![4, 5, 6, 7, 8, 9, 2, 3]);
    }

    #[test]
    fn recently_created_flags_are_cleared_for_each_live_logic_object() {
        let mut world = MockWorld {
            objects: vec![1, 2, 3],
            recently_created: BTreeSet::from([1, 2, 3]),
            ..MockWorld::default()
        };
        let cleared = LogicClass::default().clear_recently_created_bits(&mut world);
        assert_eq!(cleared, 3);
        assert!(world.recently_created.is_empty());
        assert_eq!(world.recently_cleared, vec![1, 2, 3]);
    }

    #[test]
    fn frame_rate_counter_wraps_as_unsigned_source_storage() {
        let mut logic = LogicClass { frames_per_second: u32::MAX };
        let report = logic.ai(
            LogicConfig {
                game_mode: LogicGameMode::Normal, crate_maker_enabled: false,
                use_ra_ai: false, ticks_per_minute: 60,
            },
            &mut MockWorld::default(),
        );
        assert_eq!(report.frame_rate_counter, 0);
    }
}
