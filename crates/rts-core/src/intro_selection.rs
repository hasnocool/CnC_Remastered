//! Active Choose_Side behavior from REDALERT/INTRO.CPP and
//! TIBERIANDAWN/INTRO.CPP.
//!
//! The legacy cinematic/briefing code is compiled out by #if (0) or
//! #ifdef OBSOLETE. The pinned Tiberian Dawn project does not define DEMO.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HouseChoice { Good, Bad }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScenarioPlayer { Gdi, Nod }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntroVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntroGlobals {
    pub whom: HouseChoice,
    pub scenario_player: Option<ScenarioPlayer>,
}

impl Default for IntroGlobals {
    fn default() -> Self {
        Self { whom: HouseChoice::Good, scenario_player: None }
    }
}

/// Choose_Side's live branch in the pinned Red Alert project assigns only
/// Whom=HOUSE_GOOD; the obsolete movie/side-selection branch does not run.
pub fn choose_side_red_alert(globals: &mut IntroGlobals) {
    globals.whom = HouseChoice::Good;
}

/// Choose_Side's live branch in the pinned Tiberian Dawn project assigns
/// Whom=HOUSE_GOOD and ScenPlayer=SCEN_PLAYER_GDI, then returns immediately.
pub fn choose_side_tiberian_dawn(globals: &mut IntroGlobals) {
    globals.whom = HouseChoice::Good;
    globals.scenario_player = Some(ScenarioPlayer::Gdi);
}

/// Variant-dispatched form of the two active translation-unit implementations.
pub fn choose_side(variant: IntroVariant, globals: &mut IntroGlobals) {
    match variant {
        IntroVariant::RedAlert => choose_side_red_alert(globals),
        IntroVariant::TiberianDawn => choose_side_tiberian_dawn(globals),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_alert_selects_good_house_without_changing_scenario_player() {
        let mut globals = IntroGlobals {
            whom: HouseChoice::Bad,
            scenario_player: Some(ScenarioPlayer::Nod),
        };
        choose_side_red_alert(&mut globals);
        assert_eq!(globals.whom, HouseChoice::Good);
        assert_eq!(globals.scenario_player, Some(ScenarioPlayer::Nod));
    }

    #[test]
    fn tiberian_dawn_selects_good_house_and_gdi_scenario_player() {
        let mut globals = IntroGlobals {
            whom: HouseChoice::Bad,
            scenario_player: Some(ScenarioPlayer::Nod),
        };
        choose_side_tiberian_dawn(&mut globals);
        assert_eq!(globals.whom, HouseChoice::Good);
        assert_eq!(globals.scenario_player, Some(ScenarioPlayer::Gdi));
    }

    #[test]
    fn variant_dispatch_calls_the_correct_active_branch() {
        let mut red_alert = IntroGlobals::default();
        choose_side(IntroVariant::RedAlert, &mut red_alert);
        assert_eq!(red_alert.scenario_player, None);

        let mut tiberian_dawn = IntroGlobals::default();
        choose_side(IntroVariant::TiberianDawn, &mut tiberian_dawn);
        assert_eq!(tiberian_dawn.scenario_player, Some(ScenarioPlayer::Gdi));
    }
}
