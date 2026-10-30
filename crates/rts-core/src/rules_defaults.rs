//! Default rules and difficulty constructors from TIBERIANDAWN/RULES.CPP.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DifficultyDefaults {
    pub firepower_bias: f32,
    pub groundspeed_bias: f32,
    pub airspeed_bias: f32,
    pub armor_bias: f32,
    pub rate_of_fire_bias: f32,
    pub cost_bias: f32,
    pub build_speed_bias: f32,
    pub repair_delay: f32,
    pub build_delay: f32,
    pub build_slowdown: bool,
    pub wall_destroyer: bool,
    pub content_scan: bool,
}

impl Default for DifficultyDefaults {
    fn default() -> Self {
        Self {
            firepower_bias: 1.0,
            groundspeed_bias: 1.0,
            airspeed_bias: 1.0,
            armor_bias: 1.0,
            rate_of_fire_bias: 1.0,
            cost_bias: 1.0,
            build_speed_bias: 1.0,
            repair_delay: 0.02,
            build_delay: 0.03,
            build_slowdown: false,
            wall_destroyer: true,
            content_scan: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RulesDefaults {
    pub attack_interval: i32,
    pub base_size_add: i32,
    pub power_surplus: i32,
    pub attack_delay: i32,
    pub power_emergency_fraction: u16,
    pub helipad_ratio: u16,
    pub helipad_limit: i32,
    pub aa_ratio: u16,
    pub aa_limit: i32,
    pub defense_ratio: u16,
    pub defense_limit: i32,
    pub war_ratio: u16,
    pub war_limit: i32,
    pub barracks_ratio: u16,
    pub barracks_limit: i32,
    pub refinery_limit: i32,
    pub refinery_ratio: u16,
    pub airstrip_ratio: u16,
    pub airstrip_limit: i32,
    pub max_iq: i32,
    pub iq_super_weapons: i32,
    pub iq_production: i32,
    pub iq_guard_area: i32,
    pub iq_repair_sell: i32,
    pub iq_crush: i32,
    pub iq_scatter: i32,
    pub iq_content_scan: i32,
    pub iq_aircraft: i32,
    pub iq_harvester: i32,
    pub iq_sell_back: i32,
    pub infantry_reserve: i32,
    pub infantry_base_multiplier: i32,
    pub computer_paranoid: bool,
    pub allow_super_weapons: bool,
}

impl Default for RulesDefaults {
    fn default() -> Self {
        Self {
            attack_interval: 3,
            base_size_add: 3,
            power_surplus: 50,
            attack_delay: 5,
            power_emergency_fraction: 0x00c0,
            helipad_ratio: 0x001e,
            helipad_limit: 5,
            aa_ratio: 0x0024,
            aa_limit: 10,
            defense_ratio: 0x0066,
            defense_limit: 40,
            war_ratio: 0x0019,
            war_limit: 2,
            barracks_ratio: 0x0028,
            barracks_limit: 2,
            refinery_limit: 4,
            refinery_ratio: 0x0028,
            airstrip_ratio: 0x001e,
            airstrip_limit: 5,
            max_iq: 5,
            iq_super_weapons: 4,
            iq_production: 5,
            iq_guard_area: 4,
            iq_repair_sell: 1,
            iq_crush: 2,
            iq_scatter: 3,
            iq_content_scan: 4,
            iq_aircraft: 4,
            iq_harvester: 2,
            iq_sell_back: 2,
            infantry_reserve: 3000,
            infantry_base_multiplier: 1,
            computer_paranoid: true,
            allow_super_weapons: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difficulty_defaults_match_legacy_constructor() {
        assert_eq!(DifficultyDefaults::default(), DifficultyDefaults {
            firepower_bias: 1.0, groundspeed_bias: 1.0, airspeed_bias: 1.0,
            armor_bias: 1.0, rate_of_fire_bias: 1.0, cost_bias: 1.0,
            build_speed_bias: 1.0, repair_delay: 0.02, build_delay: 0.03,
            build_slowdown: false, wall_destroyer: true, content_scan: false,
        });
    }

    #[test]
    fn rules_defaults_match_legacy_constructor_values() {
        let r = RulesDefaults::default();
        assert_eq!((r.attack_interval, r.base_size_add, r.power_surplus, r.attack_delay), (3, 3, 50, 5));
        assert_eq!((r.helipad_ratio, r.helipad_limit, r.aa_ratio, r.aa_limit), (30, 5, 36, 10));
        assert_eq!((r.defense_ratio, r.defense_limit, r.war_ratio, r.war_limit), (102, 40, 25, 2));
        assert_eq!((r.refinery_limit, r.refinery_ratio, r.airstrip_ratio, r.airstrip_limit), (4, 40, 30, 5));
        assert_eq!((r.max_iq, r.infantry_reserve, r.infantry_base_multiplier), (5, 3000, 1));
        assert!(r.computer_paranoid && r.allow_super_weapons);
    }
}
