//! Red Alert LogicClass from REDALERT/LOGIC.CPP.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection commit
//! f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! This module preserves the Red Alert frame order, mission timer triggers and
//! voice reminders, shadow growth, Chronal Vortex/time-quake processing,
//! mutation-safe object iteration, factory/house order, auto-sonar, trigger
//! detach, and recently-created cleanup. Runtime-owned objects are accessed
//! through a synchronous simulation-thread adapter.

use crate::fixed16::Fixed16;

pub const TICKS_PER_SECOND: u32 = 15;
pub const TICKS_PER_MINUTE: u32 = TICKS_PER_SECOND * 60;
pub const TICKS_PER_HOUR: u32 = TICKS_PER_MINUTE * 60;
pub const TIMER_SECOND: u32 = 60;
pub const HOUSE_FIRST: u8 = 0;
pub const HOUSE_MULTI1: u8 = 4;
pub const HOUSE_COUNT: u8 = 10;
pub const AUTO_SONAR_PERIOD: u32 = TICKS_PER_SECOND * 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaSessionType {
    Normal,
    GlyphxMultiplayer,
    OtherMultiplayer,
}

impl RaSessionType {
    pub const fn is_normal(self) -> bool { matches!(self, Self::Normal) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicTriggerEvent {
    GlobalSet,
    GlobalClear,
    AllBridgesDestroyed,
    TimeExpired,
    MissionTimerExpired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionTimerVoice {
    OneMinute,
    TwoMinutes,
    ThreeMinutes,
    FourMinutes,
    FiveMinutes,
    TenMinutes,
    TwentyMinutes,
    ThirtyMinutes,
    FortyMinutes,
}

pub fn mission_timer_voice(ticks_remaining: u32) -> Option<MissionTimerVoice> {
    Some(match ticks_remaining {
        value if value == TICKS_PER_MINUTE => MissionTimerVoice::OneMinute,
        value if value == 2 * TICKS_PER_MINUTE => MissionTimerVoice::TwoMinutes,
        value if value == 3 * TICKS_PER_MINUTE => MissionTimerVoice::ThreeMinutes,
        value if value == 4 * TICKS_PER_MINUTE => MissionTimerVoice::FourMinutes,
        value if value == 5 * TICKS_PER_MINUTE => MissionTimerVoice::FiveMinutes,
        value if value == 10 * TICKS_PER_MINUTE => MissionTimerVoice::TenMinutes,
        value if value == 20 * TICKS_PER_MINUTE => MissionTimerVoice::TwentyMinutes,
        value if value == 30 * TICKS_PER_MINUTE => MissionTimerVoice::ThirtyMinutes,
        value if value == 40 * TICKS_PER_MINUTE => MissionTimerVoice::FortyMinutes,
        _ => return None,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicSessionPlayer {
    pub house: u8,
    pub is_human: bool,
    pub house_exists: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeQuakeConfig {
    pub center_present: bool,
    pub damage_percent: Fixed16,
    pub mtank_distance: i32,
    pub infantry_damage: i32,
    pub building_damage: i32,
    pub unit_damage: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicFrameState {
    pub global_changed: bool,
    pub bridge_changed: bool,
    pub mission_timer_active: bool,
    pub mission_timer_ticks: u32,
    pub shadow_grow: bool,
    pub shroud_rate: u32,
    pub shroud_timer: u32,
    pub session: RaSessionType,
    pub local_player_house: Option<u8>,
    pub session_players: Vec<LogicSessionPlayer>,
    pub time_quake: Option<TimeQuakeConfig>,
    pub fixit_csii: bool,
    pub fixit_version_3: bool,
    pub auto_sonar_timer: u32,
    pub auto_sonar_pulse: bool,
}

impl Default for LogicFrameState {
    fn default() -> Self {
        Self {
            global_changed: false,
            bridge_changed: false,
            mission_timer_active: false,
            mission_timer_ticks: 0,
            shadow_grow: false,
            shroud_rate: 0,
            shroud_timer: 0,
            session: RaSessionType::Normal,
            local_player_house: None,
            session_players: Vec::new(),
            time_quake: None,
            fixit_csii: false,
            fixit_version_3: false,
            auto_sonar_timer: 0,
            auto_sonar_pulse: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicObjectKind {
    Infantry,
    Building,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicObjectState {
    pub active: bool,
    pub in_limbo: bool,
    pub strength: i32,
    pub max_strength: i32,
    pub kind: LogicObjectKind,
    /// Source Distance(obj->As_Target(), TimeQuakeCenter), in leptons.
    pub distance_to_quake: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuakeDamagePlan {
    pub damage: i32,
    pub clicker_house: Option<u8>,
    pub spawn_mine_explosion: bool,
    pub forced: bool,
    pub armor_piercing: bool,
}

/// Source-compatible quake calculation after an object's AI has run.
/// Returns None when the object is not eligible or is outside the configured
/// FIXIT_CSII quake radius.
pub fn plan_time_quake_damage(
    state: LogicObjectState,
    quake: TimeQuakeConfig,
    fixit_csii: bool,
) -> Option<QuakeDamagePlan> {
    if !state.active || state.in_limbo || state.strength == 0 {
        return None;
    }

    let (damage, special_center_hit) = if fixit_csii && quake.center_present {
        let distance = state.distance_to_quake?;
        if distance / 256 >= quake.mtank_distance {
            return None;
        }
        let damage = match state.kind {
            LogicObjectKind::Infantry => quake.infantry_damage,
            LogicObjectKind::Building => quake.building_damage.wrapping_mul(state.max_strength),
            LogicObjectKind::Other => quake.unit_damage.wrapping_mul(state.max_strength),
        };
        (damage, true)
    } else {
        let damage = Fixed16::from_unsigned(state.max_strength.max(0) as u32)
            .mul_fixed(quake.damage_percent)
            .to_unsigned() as i32;
        (damage, false)
    };

    let credit_and_animation = special_center_hit && damage != 0;
    Some(QuakeDamagePlan {
        damage,
        clicker_house: credit_and_animation.then_some(HOUSE_COUNT),
        spawn_mine_explosion: credit_and_animation,
        forced: true,
        armor_piercing: true,
    })
}

pub trait RedAlertLogicWorld {
    fn frame_state(&self) -> LogicFrameState { LogicFrameState::default() }

    fn do_fade_ai(&mut self) {}
    fn logic_trigger_count(&self) -> usize { 0 }
    fn logic_trigger_at(&self, _index: usize) -> Option<u32> { None }
    fn spring_trigger(&mut self, _trigger: u32, _event: LogicTriggerEvent) -> bool { false }
    fn stop_mission_timer(&mut self) {}
    fn clear_global_and_bridge_changed(&mut self) {}
    fn speak_mission_timer(&mut self, _voice: MissionTimerVoice) {}
    fn set_map_flasher_timer(&mut self, _frames: u8) {}
    fn flag_map_to_redraw(&mut self, _full: bool) {}
    fn set_shroud_timer(&mut self, _ticks: u32) {}
    fn encroach_shadow(&mut self, _house: u8) {}

    fn team_count(&self) -> usize { 0 }
    fn team_at(&self, _index: usize) -> Option<u32> { None }
    fn team_ai(&mut self, _team: u32) {}

    fn play_time_quake_sound(&mut self) {}
    fn shake_screen(&mut self, _strength: i32) {}
    fn chronal_vortex_ai(&mut self) {}

    fn logic_object_count(&self) -> usize { 0 }
    fn logic_object_at(&self, _index: usize) -> Option<u32> { None }
    fn object_ai(&mut self, _object: u32) {}
    fn object_state(&self, _object: u32) -> Option<LogicObjectState> { None }
    /// Must preserve source order: target-click credit, optional mine-exp1
    /// animation creation, then forced AP damage application.
    fn apply_time_quake_damage(&mut self, _object: u32, _plan: QuakeDamagePlan) {}

    fn recalculate_house_attributes(&mut self) {}
    fn map_logic(&mut self) {}

    fn factory_count(&self) -> usize { 0 }
    fn factory_at(&self, _index: usize) -> Option<u32> { None }
    fn factory_ai(&mut self, _factory: u32) {}

    fn house_is_active(&self, _house: u8) -> bool { false }
    fn house_ai(&mut self, _house: u8) {}

    fn activate_sonar_pulse(&mut self) {}
    fn play_sonar_sound(&mut self) {}
    fn set_auto_sonar_pulse(&mut self, _pending: bool) {}
    fn set_auto_sonar_timer(&mut self, _ticks: u32) {}

    fn trigger_from_target(&self, _target: u32) -> Option<u32> { None }
    fn remove_logic_trigger_at(&mut self, _index: usize) {}
    fn set_object_recently_created(&mut self, _object: u32, _recently_created: bool) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LogicFrameReport {
    pub frames_per_second: u32,
    pub triggers_visited: usize,
    pub mission_timer_voice: Option<MissionTimerVoice>,
    pub shadow_updates: usize,
    pub teams_processed: usize,
    pub objects_processed: usize,
    pub quake_hits: usize,
    pub factories_processed: usize,
    pub houses_processed: usize,
    pub auto_sonar_activated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RedAlertLogicRuntime {
    /// Static FramesPerSecond in LOGIC.CPP, accumulated across AI calls.
    pub frames_per_second: u32,
}

impl RedAlertLogicRuntime {
    pub fn ai(&mut self, world: &mut impl RedAlertLogicWorld) -> LogicFrameReport {
        self.frames_per_second = self.frames_per_second.wrapping_add(1);
        let mut report = LogicFrameReport {
            frames_per_second: self.frames_per_second,
            ..LogicFrameReport::default()
        };

        world.do_fade_ai();
        // Do_Fade_AI may advance timers or update scenario flags; C++ reads
        // those globals only after this call.
        let state = world.frame_state();

        // Trigger order and the continue statements match LogicClass::AI.
        for index in 0..world.logic_trigger_count() {
            let Some(trigger) = world.logic_trigger_at(index) else { continue };
            report.triggers_visited += 1;
            if state.global_changed {
                if world.spring_trigger(trigger, LogicTriggerEvent::GlobalSet) { continue; }
                if world.spring_trigger(trigger, LogicTriggerEvent::GlobalClear) { continue; }
            }
            if state.bridge_changed
                && world.spring_trigger(trigger, LogicTriggerEvent::AllBridgesDestroyed)
            {
                continue;
            }
            if world.spring_trigger(trigger, LogicTriggerEvent::TimeExpired) { continue; }
            if state.mission_timer_active && state.mission_timer_ticks == 0 {
                let _ = world.spring_trigger(trigger, LogicTriggerEvent::MissionTimerExpired);
            }
        }

        // Trigger callbacks can mutate the mission timer, so sample it again
        // after the full trigger scan as the C++ code does.
        let timer_state = world.frame_state();
        if timer_state.mission_timer_active {
            if let Some(voice) = mission_timer_voice(timer_state.mission_timer_ticks) {
                world.speak_mission_timer(voice);
                world.set_map_flasher_timer(7);
                report.mission_timer_voice = Some(voice);
            }
            if timer_state.mission_timer_ticks == 0 {
                world.stop_mission_timer();
                world.flag_map_to_redraw(true);
            }
        }
        world.clear_global_and_bridge_changed();

        // Read the shadow/session globals after clearing the trigger flags.
        let shadow_state = world.frame_state();
        if shadow_state.shadow_grow && shadow_state.shroud_rate != 0 && shadow_state.shroud_timer == 0 {
            world.set_shroud_timer(TICKS_PER_MINUTE.wrapping_mul(shadow_state.shroud_rate));
            if shadow_state.session == RaSessionType::GlyphxMultiplayer {
                for player in &shadow_state.session_players {
                    if player.house_exists && player.is_human {
                        world.encroach_shadow(player.house);
                        report.shadow_updates += 1;
                    }
                }
            } else if let Some(house) = shadow_state.local_player_house {
                world.encroach_shadow(house);
                report.shadow_updates += 1;
            }
        }

        for index in 0..world.team_count() {
            if let Some(team) = world.team_at(index) {
                world.team_ai(team);
                report.teams_processed += 1;
            }
        }

        if state.time_quake.is_some() {
            world.play_time_quake_sound();
            world.shake_screen(8);
        }
        world.chronal_vortex_ai();
        // Vortex and team updates may change the quake globals consumed by
        // subsequent object AI; read the current values for that pass.
        let quake_state = world.frame_state();

        // The engine's LayerClass count may shrink while object AI or quake
        // damage deletes objects. Adjust the index by the signed count delta
        // so the next shifted object is not skipped.
        let mut object_index: i64 = 0;
        while object_index >= 0 && (object_index as usize) < world.logic_object_count() {
            let before_count = world.logic_object_count() as i64;
            let Some(object) = world.logic_object_at(object_index as usize) else {
                object_index += 1;
                continue;
            };
            world.object_ai(object);
            report.objects_processed += 1;

            if let Some(quake) = quake_state.time_quake {
                if let Some(object_state) = world.object_state(object) {
                    if let Some(plan) = plan_time_quake_damage(object_state, quake, state.fixit_csii) {
                        world.apply_time_quake_damage(object, plan);
                        report.quake_hits += 1;
                    }
                }
            }

            let after_count = world.logic_object_count() as i64;
            let count_diff = after_count - before_count;
            if count_diff < 0 {
                object_index = (object_index + count_diff + 1).max(0);
            } else {
                object_index += 1;
            }
        }

        world.recalculate_house_attributes();
        world.map_logic();

        for index in 0..world.factory_count() {
            if let Some(factory) = world.factory_at(index) {
                world.factory_ai(factory);
                report.factories_processed += 1;
            }
        }

        let house_state = world.frame_state();
        if house_state.fixit_version_3 && !house_state.session.is_normal() {
            for house in HOUSE_MULTI1..HOUSE_COUNT {
                if world.house_is_active(house) {
                    world.house_ai(house);
                    report.houses_processed += 1;
                }
            }
        } else {
            for house in HOUSE_FIRST..HOUSE_COUNT {
                if world.house_is_active(house) {
                    world.house_ai(house);
                    report.houses_processed += 1;
                }
            }
        }

        let sonar_state = world.frame_state();
        if sonar_state.fixit_version_3 && !sonar_state.session.is_normal() && sonar_state.auto_sonar_timer == 0 {
            if sonar_state.auto_sonar_pulse {
                world.activate_sonar_pulse();
                world.play_sonar_sound();
                world.set_auto_sonar_pulse(false);
                report.auto_sonar_activated = true;
            }
            world.set_auto_sonar_timer(AUTO_SONAR_PERIOD);
        }

        report
    }

    /// Port of LogicClass::Detach. Every occurrence of a matching trigger is
    /// removed; the index is backed up after deletion just like C++.
    pub fn detach(&self, target: u32, world: &mut impl RedAlertLogicWorld) -> usize {
        let Some(trigger_id) = world.trigger_from_target(target) else { return 0 };
        let mut removed = 0;
        let mut index = 0;
        while index < world.logic_trigger_count() {
            if world.logic_trigger_at(index) == Some(trigger_id) {
                world.remove_logic_trigger_at(index);
                removed += 1;
            } else {
                index += 1;
            }
        }
        removed
    }

    /// Port of LogicClass::Clear_Recently_Created_Bits.
    pub fn clear_recently_created_bits(&self, world: &mut impl RedAlertLogicWorld) -> usize {
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

    /// CHEAT_KEYS-only Debug_Dump consumes the rolling FPS counter once per
    /// second. The actual monochrome drawing is left to the optional host UI.
    pub fn consume_debug_fps_sample(&mut self) -> u32 {
        let frames = self.frames_per_second;
        self.frames_per_second = 0;
        frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[derive(Default)]
    struct MockWorld {
        state: LogicFrameState,
        triggers: Vec<u32>,
        sprung: Vec<(u32, LogicTriggerEvent)>,
        spring_true: BTreeSet<(u32, LogicTriggerEvent)>,
        stopped_timer: usize,
        cleared_scene_flags: usize,
        voices: Vec<MissionTimerVoice>,
        flash_timers: Vec<u8>,
        redraws: Vec<bool>,
        shroud_timers: Vec<u32>,
        encroached: Vec<u8>,
        teams: Vec<u32>,
        team_calls: Vec<u32>,
        sounds: Vec<&'static str>,
        shakes: Vec<i32>,
        vortex_calls: usize,
        objects: Vec<u32>,
        object_calls: Vec<u32>,
        object_states: Vec<(u32, LogicObjectState)>,
        removed_by_ai: BTreeSet<u32>,
        quake_effects: Vec<(u32, QuakeDamagePlan)>,
        factories: Vec<u32>,
        factory_calls: Vec<u32>,
        active_houses: BTreeSet<u8>,
        house_calls: Vec<u8>,
        sonar_pulses: usize,
        sonar_pending: Option<bool>,
        sonar_timer: Option<u32>,
        trigger_target: Option<(u32, u32)>,
        recently_cleared: Vec<u32>,
        order: Vec<String>,
    }

    impl RedAlertLogicWorld for MockWorld {
        fn frame_state(&self) -> LogicFrameState { self.state.clone() }
        fn do_fade_ai(&mut self) { self.order.push("fade".into()); }
        fn logic_trigger_count(&self) -> usize { self.triggers.len() }
        fn logic_trigger_at(&self, index: usize) -> Option<u32> { self.triggers.get(index).copied() }
        fn spring_trigger(&mut self, trigger: u32, event: LogicTriggerEvent) -> bool {
            self.sprung.push((trigger, event));
            self.spring_true.contains(&(trigger, event))
        }
        fn stop_mission_timer(&mut self) { self.stopped_timer += 1; }
        fn clear_global_and_bridge_changed(&mut self) { self.cleared_scene_flags += 1; }
        fn speak_mission_timer(&mut self, voice: MissionTimerVoice) { self.voices.push(voice); }
        fn set_map_flasher_timer(&mut self, frames: u8) { self.flash_timers.push(frames); }
        fn flag_map_to_redraw(&mut self, full: bool) { self.redraws.push(full); }
        fn set_shroud_timer(&mut self, ticks: u32) { self.shroud_timers.push(ticks); }
        fn encroach_shadow(&mut self, house: u8) { self.encroached.push(house); }
        fn team_count(&self) -> usize { self.teams.len() }
        fn team_at(&self, index: usize) -> Option<u32> { self.teams.get(index).copied() }
        fn team_ai(&mut self, team: u32) { self.team_calls.push(team); self.order.push(format!("team:{team}")); }
        fn play_time_quake_sound(&mut self) { self.sounds.push("quake"); }
        fn shake_screen(&mut self, strength: i32) { self.shakes.push(strength); }
        fn chronal_vortex_ai(&mut self) { self.vortex_calls += 1; self.order.push("vortex".into()); }
        fn logic_object_count(&self) -> usize { self.objects.len() }
        fn logic_object_at(&self, index: usize) -> Option<u32> { self.objects.get(index).copied() }
        fn object_ai(&mut self, object: u32) {
            self.object_calls.push(object);
            self.order.push(format!("object:{object}"));
            if self.removed_by_ai.contains(&object) {
                if let Some(index) = self.objects.iter().position(|item| *item == object) {
                    self.objects.remove(index);
                }
            }
        }
        fn object_state(&self, object: u32) -> Option<LogicObjectState> {
            self.object_states.iter().find(|(id, _)| *id == object).map(|(_, state)| *state)
        }
        fn apply_time_quake_damage(&mut self, object: u32, plan: QuakeDamagePlan) {
            self.quake_effects.push((object, plan));
            self.order.push(format!("quake:{object}"));
            if self.removed_by_ai.contains(&object) {
                if let Some(index) = self.objects.iter().position(|item| *item == object) {
                    self.objects.remove(index);
                }
            }
        }
        fn recalculate_house_attributes(&mut self) { self.order.push("recalc".into()); }
        fn map_logic(&mut self) { self.order.push("map".into()); }
        fn factory_count(&self) -> usize { self.factories.len() }
        fn factory_at(&self, index: usize) -> Option<u32> { self.factories.get(index).copied() }
        fn factory_ai(&mut self, factory: u32) { self.factory_calls.push(factory); self.order.push(format!("factory:{factory}")); }
        fn house_is_active(&self, house: u8) -> bool { self.active_houses.contains(&house) }
        fn house_ai(&mut self, house: u8) { self.house_calls.push(house); self.order.push(format!("house:{house}")); }
        fn activate_sonar_pulse(&mut self) { self.sonar_pulses += 1; }
        fn play_sonar_sound(&mut self) { self.sounds.push("sonar"); }
        fn set_auto_sonar_pulse(&mut self, pending: bool) { self.sonar_pending = Some(pending); }
        fn set_auto_sonar_timer(&mut self, ticks: u32) { self.sonar_timer = Some(ticks); }
        fn trigger_from_target(&self, target: u32) -> Option<u32> {
            self.trigger_target.filter(|(value, _)| *value == target).map(|(_, id)| id)
        }
        fn remove_logic_trigger_at(&mut self, index: usize) { self.triggers.remove(index); }
        fn set_object_recently_created(&mut self, object: u32, recently_created: bool) {
            if !recently_created { self.recently_cleared.push(object); }
        }
    }

    fn quake_state() -> LogicFrameState {
        LogicFrameState {
            time_quake: Some(TimeQuakeConfig {
                center_present: false,
                damage_percent: Fixed16::from_ratio(1, 4),
                mtank_distance: 10,
                infantry_damage: 20,
                building_damage: 2,
                unit_damage: 3,
            }),
            ..LogicFrameState::default()
        }
    }

    #[test]
    fn mission_timer_triggers_run_in_source_order_and_timer_zero_stops_after_springing() {
        let mut world = MockWorld {
            state: LogicFrameState {
                global_changed: true,
                bridge_changed: true,
                mission_timer_active: true,
                mission_timer_ticks: 0,
                ..LogicFrameState::default()
            },
            triggers: vec![1],
            spring_true: BTreeSet::from([(1, LogicTriggerEvent::GlobalSet)]),
            ..MockWorld::default()
        };
        let report = RedAlertLogicRuntime::default().ai(&mut world);
        assert_eq!(world.sprung, vec![(1, LogicTriggerEvent::GlobalSet)]);
        assert_eq!(world.stopped_timer, 1);
        assert_eq!(world.cleared_scene_flags, 1);
        assert_eq!(world.redraws, vec![true]);
        assert_eq!(report.triggers_visited, 1);
    }

    #[test]
    fn mission_timer_voice_reminders_use_exact_tick_values() {
        assert_eq!(mission_timer_voice(TICKS_PER_MINUTE), Some(MissionTimerVoice::OneMinute));
        assert_eq!(mission_timer_voice(5 * TICKS_PER_MINUTE), Some(MissionTimerVoice::FiveMinutes));
        assert_eq!(mission_timer_voice(40 * TICKS_PER_MINUTE), Some(MissionTimerVoice::FortyMinutes));
        assert_eq!(mission_timer_voice(TICKS_PER_MINUTE + 1), None);
        let mut world = MockWorld {
            state: LogicFrameState {
                mission_timer_active: true,
                mission_timer_ticks: 10 * TICKS_PER_MINUTE,
                ..LogicFrameState::default()
            },
            ..MockWorld::default()
        };
        RedAlertLogicRuntime::default().ai(&mut world);
        assert_eq!(world.voices, vec![MissionTimerVoice::TenMinutes]);
        assert_eq!(world.flash_timers, vec![7]);
    }

    #[test]
    fn shadow_growth_targets_local_player_or_each_human_glyphx_player() {
        let mut local = MockWorld {
            state: LogicFrameState {
                shadow_grow: true, shroud_rate: 2, shroud_timer: 0,
                local_player_house: Some(1),
                ..LogicFrameState::default()
            },
            ..MockWorld::default()
        };
        RedAlertLogicRuntime::default().ai(&mut local);
        assert_eq!(local.shroud_timers, vec![2 * TICKS_PER_MINUTE]);
        assert_eq!(local.encroached, vec![1]);

        let mut glyphx = MockWorld {
            state: LogicFrameState {
                shadow_grow: true, shroud_rate: 1, shroud_timer: 0,
                session: RaSessionType::GlyphxMultiplayer,
                session_players: vec![
                    LogicSessionPlayer { house: 4, is_human: true, house_exists: true },
                    LogicSessionPlayer { house: 5, is_human: false, house_exists: true },
                    LogicSessionPlayer { house: 6, is_human: true, house_exists: false },
                    LogicSessionPlayer { house: 7, is_human: true, house_exists: true },
                ],
                ..LogicFrameState::default()
            },
            ..MockWorld::default()
        };
        let report = RedAlertLogicRuntime::default().ai(&mut glyphx);
        assert_eq!(glyphx.encroached, vec![4, 7]);
        assert_eq!(report.shadow_updates, 2);
    }

    #[test]
    fn frame_order_places_teams_quake_vortex_objects_recalc_map_factory_house() {
        let mut world = MockWorld {
            state: LogicFrameState {
                time_quake: Some(TimeQuakeConfig {
                    center_present: false, damage_percent: Fixed16::ZERO, mtank_distance: 10,
                    infantry_damage: 0, building_damage: 0, unit_damage: 0,
                }),
                ..LogicFrameState::default()
            },
            teams: vec![1],
            objects: vec![10],
            object_states: vec![(10, LogicObjectState {
                active: true, in_limbo: false, strength: 1, max_strength: 10,
                kind: LogicObjectKind::Other, distance_to_quake: None,
            })],
            factories: vec![20],
            active_houses: BTreeSet::from([0]),
            ..MockWorld::default()
        };
        RedAlertLogicRuntime::default().ai(&mut world);
        let at = |name: &str| world.order.iter().position(|s| s == name).unwrap();
        assert!(at("fade") < at("team:1"));
        assert!(at("team:1") < at("vortex"));
        assert!(at("vortex") < at("object:10"));
        assert!(at("object:10") < at("recalc"));
        assert!(at("recalc") < at("map"));
        assert!(at("map") < at("factory:20"));
        assert!(at("factory:20") < at("house:0"));
    }

    #[test]
    fn timequake_damage_uses_base_fixed_percent_or_fixit_center_damage() {
        let object = LogicObjectState {
            active: true, in_limbo: false, strength: 50, max_strength: 80,
            kind: LogicObjectKind::Infantry, distance_to_quake: Some(256),
        };
        let base = plan_time_quake_damage(object, TimeQuakeConfig {
            center_present: true, damage_percent: Fixed16::from_ratio(1, 4),
            mtank_distance: 1, infantry_damage: 18, building_damage: 2, unit_damage: 3,
        }, false).unwrap();
        assert_eq!(base.damage, 20);
        assert_eq!(base.clicker_house, None);
        let center = plan_time_quake_damage(object, TimeQuakeConfig {
            center_present: true, damage_percent: Fixed16::ZERO,
            mtank_distance: 2, infantry_damage: 18, building_damage: 2, unit_damage: 3,
        }, true).unwrap();
        assert_eq!(center.damage, 18);
        assert_eq!(center.clicker_house, Some(HOUSE_COUNT));
        assert!(center.spawn_mine_explosion);
        assert_eq!(plan_time_quake_damage(LogicObjectState { distance_to_quake: Some(512), ..object }, TimeQuakeConfig {
            center_present: true, damage_percent: Fixed16::ZERO,
            mtank_distance: 2, infantry_damage: 18, building_damage: 2, unit_damage: 3,
        }, true), None);
    }

    #[test]
    fn object_loop_adjusts_for_deletions_without_skipping_shifted_objects() {
        let mut world = MockWorld {
            state: quake_state(),
            objects: vec![1, 2, 3, 4],
            removed_by_ai: BTreeSet::from([1, 3]),
            ..MockWorld::default()
        };
        let report = RedAlertLogicRuntime::default().ai(&mut world);
        assert_eq!(world.object_calls, vec![1, 2, 3, 4]);
        assert_eq!(world.objects, vec![2, 4]);
        assert_eq!(report.objects_processed, 4);
    }

    #[test]
    fn house_ranges_follow_fixit_version_three_compile_branch() {
        let active = BTreeSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut old_build = MockWorld {
            state: LogicFrameState { session: RaSessionType::OtherMultiplayer, fixit_version_3: false, ..Default::default() },
            active_houses: active.clone(),
            ..Default::default()
        };
        RedAlertLogicRuntime::default().ai(&mut old_build);
        assert_eq!(old_build.house_calls, (0..HOUSE_COUNT).collect::<Vec<_>>());

        let mut v3 = MockWorld {
            state: LogicFrameState { session: RaSessionType::OtherMultiplayer, fixit_version_3: true, ..Default::default() },
            active_houses: active,
            ..Default::default()
        };
        RedAlertLogicRuntime::default().ai(&mut v3);
        assert_eq!(v3.house_calls, (HOUSE_MULTI1..HOUSE_COUNT).collect::<Vec<_>>());
    }

    #[test]
    fn fixit_version_three_autosonar_activates_and_resets_even_without_pending_pulse() {
        let mut active = MockWorld {
            state: LogicFrameState {
                session: RaSessionType::OtherMultiplayer,
                fixit_version_3: true,
                auto_sonar_timer: 0,
                auto_sonar_pulse: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let report = RedAlertLogicRuntime::default().ai(&mut active);
        assert!(report.auto_sonar_activated);
        assert_eq!(active.sonar_pulses, 1);
        assert_eq!(active.sounds, vec!["sonar"]);
        assert_eq!(active.sonar_pending, Some(false));
        assert_eq!(active.sonar_timer, Some(AUTO_SONAR_PERIOD));

        let mut not_pending = MockWorld {
            state: LogicFrameState {
                session: RaSessionType::OtherMultiplayer,
                fixit_version_3: true,
                auto_sonar_timer: 0,
                auto_sonar_pulse: false,
                ..Default::default()
            },
            ..Default::default()
        };
        RedAlertLogicRuntime::default().ai(&mut not_pending);
        assert_eq!(not_pending.sonar_pulses, 0);
        assert_eq!(not_pending.sonar_timer, Some(AUTO_SONAR_PERIOD));
    }

    #[test]
    fn detach_removes_all_matching_trigger_references() {
        let mut world = MockWorld {
            triggers: vec![1, 7, 1, 2, 1],
            trigger_target: Some((99, 1)),
            ..Default::default()
        };
        let removed = RedAlertLogicRuntime::default().detach(99, &mut world);
        assert_eq!(removed, 3);
        assert_eq!(world.triggers, vec![7, 2]);
        assert_eq!(RedAlertLogicRuntime::default().detach(100, &mut world), 0);
    }

    #[test]
    fn clear_recently_created_bits_covers_each_logic_object() {
        let mut world = MockWorld { objects: vec![1, 2, 3], ..Default::default() };
        let count = RedAlertLogicRuntime::default().clear_recently_created_bits(&mut world);
        assert_eq!(count, 3);
        assert_eq!(world.recently_cleared, vec![1, 2, 3]);
    }

    #[test]
    fn frame_counter_wraps_and_debug_sample_consumes_it() {
        let mut runtime = RedAlertLogicRuntime { frames_per_second: u32::MAX };
        assert_eq!(runtime.ai(&mut MockWorld::default()).frames_per_second, 0);
        runtime.frames_per_second = 15;
        assert_eq!(runtime.consume_debug_fps_sample(), 15);
        assert_eq!(runtime.frames_per_second, 0);
    }
}
