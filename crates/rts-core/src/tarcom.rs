//! Tiberian Dawn TarComClass::AI from TIBERIANDAWN/TARCOM.CPP/H.
//!
//! The controller delegates actual weapon, turret, target, facing, sound and
//! uncloak operations through TarComHost. It preserves the original branch
//! order, including secondary-weapon fallback and the post-fire facing update.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TarComUnitKind { Unit, FTank, Bike, Other }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TarComSpeed { Track, Wheeled, Hover, Other }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FireError {
    FireOk,
    FireFacing,
    FireCloaked,
    Other(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WeaponProfile {
    pub sound_id: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TarComConfig {
    pub primary_weapon: Option<WeaponProfile>,
    pub secondary_weapon: Option<WeaponProfile>,
    pub unit_kind: TarComUnitKind,
    pub is_fire_animation: bool,
    pub is_lock_turret: bool,
    pub is_turret_equipped: bool,
    pub speed: TarComSpeed,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TarComClass {
    pub is_firing: bool,
    pub is_driving: bool,
    pub is_rotating: bool,
    pub primary_facing_desired: Option<u16>,
    pub secondary_facing_desired: Option<u16>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TarComError {
    SecondaryCanFireWithoutWeapon,
}

pub trait TarComHost {
    fn turret_ai(&mut self);

    fn can_fire(&mut self, target: u32, weapon_slot: usize) -> FireError;
    fn fire_at(&mut self, target: u32, weapon_slot: usize) -> bool;
    fn play_weapon_sound(&mut self, sound_id: i32, source_coordinate: u32);

    fn target_is_legal(&mut self, target: u32) -> bool;
    fn direction_to_target(&mut self, target: u32) -> u16;

    /// Facing_Dir(Dir_Facing(direction)) is used by the Flame Tank.
    fn snap_flame_tank_direction(&mut self, direction: u16) -> u16;

    /// PrimaryFacing.Difference(direction) tests whether the hull is still
    /// facing away from the desired direction.
    fn primary_facing_differs(&mut self, direction: u16) -> bool;
    fn do_uncloak(&mut self);
}

impl TarComClass {
    pub fn new() -> Self { Self::default() }

    /// Port of TarComClass::AI. Target and navigation target are the current
    /// TarCom/NavCom values from the parent combat/drive classes.
    pub fn ai(
        &mut self,
        config: TarComConfig,
        target: u32,
        navigation_target: u32,
        coordinate: u32,
        host: &mut impl TarComHost,
    ) -> Result<(), TarComError> {
        // Always run the turret/base AI before target-computer behavior.
        host.turret_ai();

        if config.primary_weapon.is_some() {
            let mut selected_slot = 0usize;
            let mut selected_weapon = config.primary_weapon.ok_or(TarComError::SecondaryCanFireWithoutWeapon)?;
            let mut fire_status = host.can_fire(target, 0);

            if fire_status != FireError::FireOk && host.can_fire(target, 1) == FireError::FireOk {
                selected_slot = 1;
                selected_weapon = config.secondary_weapon
                    .ok_or(TarComError::SecondaryCanFireWithoutWeapon)?;
                fire_status = FireError::FireOk;
            }

            match fire_status {
                FireError::FireOk => {
                    if config.unit_kind != TarComUnitKind::Unit || !config.is_fire_animation {
                        self.is_firing = false;
                    }
                    if host.fire_at(target, selected_slot) {
                        host.play_weapon_sound(selected_weapon.sound_id, coordinate);
                    }
                }

                FireError::FireFacing => {
                    if config.is_lock_turret {
                        if !host.target_is_legal(navigation_target) && !self.is_driving {
                            let direction = host.direction_to_target(target);
                            self.primary_facing_desired = Some(direction);
                            self.secondary_facing_desired = Some(direction);
                        }
                    } else {
                        let direction = host.direction_to_target(target);
                        self.secondary_facing_desired = Some(if config.unit_kind == TarComUnitKind::FTank {
                            host.snap_flame_tank_direction(direction)
                        } else {
                            direction
                        });
                    }
                }

                FireError::FireCloaked => {
                    self.is_firing = false;
                    host.do_uncloak();
                }

                FireError::Other(_) => {}
            }
        }

        // Independent facing update for a legal target while not rotating.
        if host.target_is_legal(target) && !self.is_rotating {
            let direction = host.direction_to_target(target);
            if config.is_turret_equipped {
                self.secondary_facing_desired = Some(direction);
            } else {
                let maneuverable = config.speed == TarComSpeed::Track
                    || config.unit_kind == TarComUnitKind::Bike;
                if maneuverable
                    && !host.target_is_legal(navigation_target)
                    && !self.is_driving
                    && host.primary_facing_differs(direction)
                {
                    self.primary_facing_desired = Some(if config.unit_kind == TarComUnitKind::FTank {
                        host.snap_flame_tank_direction(direction)
                    } else {
                        direction
                    });
                }
            }
        }

        Ok(())
    }

    /// CHEAT_KEYS Debug_Dump only calls the parent TurretClass implementation.
    /// The pinned Tiberian Dawn project does not define CHEAT_KEYS.
    pub fn debug_dump_enabled(&self, cheat_keys_enabled: bool, host: &mut impl TarComDebugHost) {
        if cheat_keys_enabled { host.turret_debug_dump(); }
    }
}

pub trait TarComDebugHost { fn turret_debug_dump(&mut self); }

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeHost {
        calls: Vec<String>,
        primary_status: FireError,
        secondary_status: FireError,
        fire_result: bool,
        target_legal: bool,
        nav_legal: bool,
        direction: u16,
        snapped_direction: u16,
        facing_differs: bool,
    }

    impl TarComHost for FakeHost {
        fn turret_ai(&mut self) { self.calls.push("turret_ai".into()); }
        fn can_fire(&mut self, _target: u32, slot: usize) -> FireError {
            self.calls.push(format!("can_fire_{slot}"));
            if slot == 0 { self.primary_status } else { self.secondary_status }
        }
        fn fire_at(&mut self, _target: u32, slot: usize) -> bool {
            self.calls.push(format!("fire_at_{slot}"));
            self.fire_result
        }
        fn play_weapon_sound(&mut self, sound: i32, _coord: u32) {
            self.calls.push(format!("sound_{sound}"));
        }
        fn target_is_legal(&mut self, target: u32) -> bool {
            self.calls.push(format!("legal_{target}"));
            if target == 10 { self.target_legal } else { self.nav_legal }
        }
        fn direction_to_target(&mut self, _target: u32) -> u16 {
            self.calls.push("direction".into());
            self.direction
        }
        fn snap_flame_tank_direction(&mut self, _direction: u16) -> u16 {
            self.calls.push("snap_ftank".into());
            self.snapped_direction
        }
        fn primary_facing_differs(&mut self, _direction: u16) -> bool {
            self.calls.push("primary_difference".into());
            self.facing_differs
        }
        fn do_uncloak(&mut self) { self.calls.push("uncloak".into()); }
    }

    fn config() -> TarComConfig {
        TarComConfig {
            primary_weapon: Some(WeaponProfile { sound_id: 1 }),
            secondary_weapon: Some(WeaponProfile { sound_id: 2 }),
            unit_kind: TarComUnitKind::Unit,
            is_fire_animation: true,
            is_lock_turret: false,
            is_turret_equipped: true,
            speed: TarComSpeed::Track,
        }
    }

    #[test]
    fn primary_weapon_fires_and_plays_selected_weapon_sound() {
        let mut tarcom = TarComClass::new();
        let mut host = FakeHost {
            primary_status: FireError::FireOk,
            fire_result: true,
            target_legal: false,
            ..FakeHost::default()
        };
        tarcom.ai(config(), 10, 20, 100, &mut host).unwrap();
        assert_eq!(host.calls[..4], ["turret_ai", "can_fire_0", "fire_at_0", "sound_1"]);
    }

    #[test]
    fn secondary_weapon_is_selected_only_after_primary_failure() {
        let mut tarcom = TarComClass::new();
        let mut host = FakeHost {
            primary_status: FireError::Other(4),
            secondary_status: FireError::FireOk,
            fire_result: true,
            target_legal: false,
            ..FakeHost::default()
        };
        tarcom.ai(config(), 10, 20, 0, &mut host).unwrap();
        assert!(host.calls.contains(&"can_fire_0".to_owned()));
        assert!(host.calls.contains(&"can_fire_1".to_owned()));
        assert!(host.calls.contains(&"fire_at_1".to_owned()));
        assert!(host.calls.contains(&"sound_2".to_owned()));
    }

    #[test]
    fn secondary_is_not_checked_when_primary_can_fire() {
        let mut tarcom = TarComClass::new();
        let mut host = FakeHost {
            primary_status: FireError::FireOk,
            target_legal: false,
            ..FakeHost::default()
        };
        tarcom.ai(config(), 10, 20, 0, &mut host).unwrap();
        assert!(!host.calls.contains(&"can_fire_1".to_owned()));
    }

    #[test]
    fn non_animating_unit_and_non_unit_clear_is_firing_before_fire_call() {
        let mut tarcom = TarComClass { is_firing: true, ..TarComClass::default() };
        let mut cfg = config();
        cfg.is_fire_animation = false;
        let mut host = FakeHost { primary_status: FireError::FireOk, target_legal: false, ..FakeHost::default() };
        tarcom.ai(cfg, 10, 20, 0, &mut host).unwrap();
        assert!(!tarcom.is_firing);
        assert!(host.calls.contains(&"fire_at_0".to_owned()));

        tarcom.is_firing = true;
        cfg.is_fire_animation = true;
        cfg.unit_kind = TarComUnitKind::Other;
        let mut host = FakeHost { primary_status: FireError::FireOk, target_legal: false, ..FakeHost::default() };
        tarcom.ai(cfg, 10, 20, 0, &mut host).unwrap();
        assert!(!tarcom.is_firing);
    }

    #[test]
    fn facing_error_uses_lock_turret_navigation_and_flame_tank_snapping_rules() {
        let mut tarcom = TarComClass::new();
        let mut cfg = config();
        cfg.is_lock_turret = true;
        let mut host = FakeHost {
            primary_status: FireError::FireFacing,
            nav_legal: false,
            direction: 25,
            target_legal: false,
            ..FakeHost::default()
        };
        tarcom.ai(cfg, 10, 20, 0, &mut host).unwrap();
        assert_eq!(tarcom.primary_facing_desired, Some(25));
        assert_eq!(tarcom.secondary_facing_desired, Some(25));

        let mut cfg = config();
        cfg.unit_kind = TarComUnitKind::FTank;
        let mut host = FakeHost {
            primary_status: FireError::FireFacing,
            target_legal: false,
            direction: 30,
            snapped_direction: 24,
            ..FakeHost::default()
        };
        tarcom.ai(cfg, 10, 20, 0, &mut host).unwrap();
        assert_eq!(tarcom.secondary_facing_desired, Some(24));
    }

    #[test]
    fn cloaked_target_clears_firing_and_requests_uncloak() {
        let mut tarcom = TarComClass { is_firing: true, ..TarComClass::default() };
        let mut host = FakeHost { primary_status: FireError::FireCloaked, target_legal: false, ..FakeHost::default() };
        tarcom.ai(config(), 10, 20, 0, &mut host).unwrap();
        assert!(!tarcom.is_firing);
        assert!(host.calls.contains(&"uncloak".to_owned()));
    }

    #[test]
    fn legal_target_turns_turret_or_tracks_hull_with_vehicle_specific_rule() {
        let mut tarcom = TarComClass::new();
        let mut cfg = config();
        cfg.primary_weapon = None;
        let mut host = FakeHost {
            target_legal: true,
            direction: 45,
            ..FakeHost::default()
        };
        tarcom.ai(cfg, 10, 20, 0, &mut host).unwrap();
        assert_eq!(tarcom.secondary_facing_desired, Some(45));

        cfg.is_turret_equipped = false;
        let mut host = FakeHost {
            target_legal: true,
            nav_legal: false,
            direction: 90,
            facing_differs: true,
            ..FakeHost::default()
        };
        tarcom.ai(cfg, 10, 20, 0, &mut host).unwrap();
        assert_eq!(tarcom.primary_facing_desired, Some(90));

        cfg.unit_kind = TarComUnitKind::FTank;
        let mut host = FakeHost {
            target_legal: true,
            nav_legal: false,
            direction: 100,
            snapped_direction: 88,
            facing_differs: true,
            ..FakeHost::default()
        };
        tarcom.ai(cfg, 10, 20, 0, &mut host).unwrap();
        assert_eq!(tarcom.primary_facing_desired, Some(88));
    }

    #[test]
    fn rotating_driving_and_navigation_target_prevent_hull_turn() {
        let mut tarcom = TarComClass { is_driving: true, ..TarComClass::default() };
        let mut cfg = config();
        cfg.primary_weapon = None;
        cfg.is_turret_equipped = false;
        let mut host = FakeHost { target_legal: true, facing_differs: true, ..FakeHost::default() };
        tarcom.ai(cfg, 10, 20, 0, &mut host).unwrap();
        assert_eq!(tarcom.primary_facing_desired, None);

        tarcom.is_driving = false;
        tarcom.is_rotating = true;
        let mut host = FakeHost { target_legal: true, facing_differs: true, ..FakeHost::default() };
        tarcom.ai(cfg, 10, 20, 0, &mut host).unwrap();
        assert_eq!(tarcom.primary_facing_desired, None);
    }
}
