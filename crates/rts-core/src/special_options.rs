//! Tiberian Dawn SpecialClass defaults and Special_Dialog from SPECIAL.H/CPP.
//!
//! The dialog edits a local copy of SpecialClass. OK and Escape enqueue an
//! EventClass containing the updated copy; Cancel discards it. The outer game
//! consumes that event to update the real global settings.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HealthBarDisplayMode { Damaged, Always, Selected }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceBarDisplayMode { Never, Always, Selected }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpecialClass {
    pub is_scroll_mod: bool,
    pub is_gross: bool,
    pub is_easy: bool,
    pub is_difficult: bool,
    pub is_speed_build: bool,
    pub is_defender_advantage: bool,
    pub is_visible_target: bool,
    pub is_variation: bool,
    pub is_jurassic: bool,
    pub is_juvenile: bool,
    pub is_smart_defense: bool,
    pub is_tree_target: bool,
    pub is_mcv_deploy: bool,
    pub is_visceroids: bool,
    pub is_mono_enabled: bool,
    pub is_inert: bool,
    pub is_show_path: bool,
    pub is_three_point: bool,
    pub is_tiberium_growth: bool,
    pub is_tiberium_spread: bool,
    pub is_tiberium_fast: bool,
    pub is_road: bool,
    pub is_scatter: bool,
    pub is_capture_the_flag: bool,
    pub is_named: bool,
    pub is_from_install: bool,
    pub is_separate_helipad: bool,
    pub is_from_wchat: bool,
    pub is_early_win: bool,
    pub health_bar_display_mode: HealthBarDisplayMode,
    pub resource_bar_display_mode: ResourceBarDisplayMode,
    pub modern_balance: bool,
}

impl Default for SpecialClass {
    fn default() -> Self {
        Self {
            is_scroll_mod: false,
            is_gross: false,
            is_easy: false,
            is_difficult: false,
            is_speed_build: false,
            is_defender_advantage: true,
            is_visible_target: false,
            is_variation: false,
            is_jurassic: false,
            is_juvenile: false,
            is_smart_defense: false,
            is_tree_target: false,
            is_mcv_deploy: false,
            is_visceroids: false,
            is_mono_enabled: false,
            is_inert: false,
            is_show_path: false,
            is_three_point: false,
            is_tiberium_growth: true,
            is_tiberium_spread: true,
            is_tiberium_fast: true,
            is_road: false,
            is_scatter: false,
            is_capture_the_flag: false,
            is_named: false,
            is_from_install: false,
            is_separate_helipad: false,
            is_from_wchat: false,
            is_early_win: false,
            health_bar_display_mode: HealthBarDisplayMode::Selected,
            resource_bar_display_mode: ResourceBarDisplayMode::Selected,
            modern_balance: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpecialOption {
    SeparateHelipad,
    VisibleTarget,
    TreeTarget,
    McvDeploy,
    SmartDefense,
    ThreePoint,
    TiberiumFast,
    RoadPieces,
    Scatter,
    ShowNames,
}

pub const SPECIAL_OPTIONS: [SpecialOption; 10] = [
    SpecialOption::SeparateHelipad,
    SpecialOption::VisibleTarget,
    SpecialOption::TreeTarget,
    SpecialOption::McvDeploy,
    SpecialOption::SmartDefense,
    SpecialOption::ThreePoint,
    SpecialOption::TiberiumFast,
    SpecialOption::RoadPieces,
    SpecialOption::Scatter,
    SpecialOption::ShowNames,
];

impl SpecialClass {
    pub fn option_value(&self, option: SpecialOption) -> bool {
        match option {
            SpecialOption::SeparateHelipad => self.is_separate_helipad,
            SpecialOption::VisibleTarget => self.is_visible_target,
            SpecialOption::TreeTarget => self.is_tree_target,
            SpecialOption::McvDeploy => self.is_mcv_deploy,
            SpecialOption::SmartDefense => self.is_smart_defense,
            SpecialOption::ThreePoint => self.is_three_point,
            SpecialOption::TiberiumFast => self.is_tiberium_fast,
            SpecialOption::RoadPieces => self.is_road,
            SpecialOption::Scatter => self.is_scatter,
            SpecialOption::ShowNames => self.is_named,
        }
    }

    pub fn set_option_value(&mut self, option: SpecialOption, value: bool) {
        match option {
            SpecialOption::SeparateHelipad => self.is_separate_helipad = value,
            SpecialOption::VisibleTarget => self.is_visible_target = value,
            SpecialOption::TreeTarget => self.is_tree_target = value,
            SpecialOption::McvDeploy => self.is_mcv_deploy = value,
            SpecialOption::SmartDefense => self.is_smart_defense = value,
            SpecialOption::ThreePoint => self.is_three_point = value,
            SpecialOption::TiberiumFast => self.is_tiberium_fast = value,
            SpecialOption::RoadPieces => self.is_road = value,
            SpecialOption::Scatter => self.is_scatter = value,
            SpecialOption::ShowNames => self.is_named = value,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpecialDialogInput {
    None,
    Ok,
    Cancel,
    Escape,
    Toggle(usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpecialDialogOperation {
    OverrideNormalMouseShape,
    SetLogicPageSeenBuffer,
    InvokeGameCallback,
    InvokeMainLoop,
    ClearSurfacesRestored,
    HideMouse,
    DrawDialogBox { x: i32, y: i32, width: i32, height: i32 },
    DrawCaption { x: i32, y: i32, width: i32 },
    DrawOptionLabel { index: usize, option: SpecialOption, x: i32, y: i32 },
    DrawAllGadgets,
    ShowMouse,
    TurnCheckboxOn { index: usize },
    TurnCheckboxOff { index: usize },
    QueueSpecialEvent(SpecialClass),
    RevertMouseShape,
    ClearHiddenPage,
    FlagMapForRedraw { full: bool },
    RenderMap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpecialDialogStep {
    pub operations: Vec<SpecialDialogOperation>,
    pub complete: bool,
    pub committed: bool,
    pub result: Option<SpecialClass>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpecialDialog {
    original: SpecialClass,
    edited: SpecialClass,
    pub process: bool,
    pub display: bool,
    pub factor: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl SpecialDialog {
    pub const OK_BUTTON_ID: i32 = 200;
    pub const CANCEL_BUTTON_ID: i32 = 201;
    pub const CHECKBOX_ID_BASE: i32 = 100;

    pub fn new(current: &SpecialClass) -> Self {
        let width = 236;
        let height = 162;
        Self {
            original: current.clone(),
            edited: current.clone(),
            process: true,
            display: true,
            factor: 1,
            x: (320 - width) / 2,
            y: (200 - height) / 2,
            width,
            height,
        }
    }

    pub fn original(&self) -> &SpecialClass { &self.original }
    pub fn edited(&self) -> &SpecialClass { &self.edited }

    pub fn initial_operations(&self) -> Vec<SpecialDialogOperation> {
        let mut operations = vec![
            SpecialDialogOperation::OverrideNormalMouseShape,
            SpecialDialogOperation::SetLogicPageSeenBuffer,
        ];
        for (index, option) in SPECIAL_OPTIONS.iter().copied().enumerate() {
            if self.edited.option_value(option) {
                operations.push(SpecialDialogOperation::TurnCheckboxOn { index });
            } else {
                operations.push(SpecialDialogOperation::TurnCheckboxOff { index });
            }
        }
        operations
    }

    fn draw_operations(&self) -> Vec<SpecialDialogOperation> {
        let mut operations = vec![
            SpecialDialogOperation::HideMouse,
            SpecialDialogOperation::DrawDialogBox {
                x: self.x, y: self.y, width: self.width, height: self.height,
            },
            SpecialDialogOperation::DrawCaption { x: self.x, y: self.y, width: self.width },
        ];
        for (index, option) in SPECIAL_OPTIONS.iter().copied().enumerate() {
            operations.push(SpecialDialogOperation::DrawOptionLabel {
                index,
                option,
                x: self.x + 7 + 10,
                y: self.y + 20 + (index as i32 * 10),
            });
        }
        operations.push(SpecialDialogOperation::DrawAllGadgets);
        operations.push(SpecialDialogOperation::ShowMouse);
        operations
    }

    fn finish_operations(&self) -> Vec<SpecialDialogOperation> {
        vec![
            SpecialDialogOperation::RevertMouseShape,
            SpecialDialogOperation::ClearHiddenPage,
            SpecialDialogOperation::FlagMapForRedraw { full: true },
            SpecialDialogOperation::RenderMap,
        ]
    }

    /// Execute one iteration of the modal options loop. OK and Escape commit
    /// an event containing the edited copy; Cancel and Main_Loop termination
    /// abandon it, matching the source's local oldspecial snapshot.
    pub fn step(
        &mut self,
        game_normal: bool,
        main_loop_requested_exit: bool,
        surfaces_restored: bool,
        input: SpecialDialogInput,
    ) -> SpecialDialogStep {
        if !self.process {
            return SpecialDialogStep { operations: Vec::new(), complete: true, committed: false, result: None };
        }
        let mut operations = Vec::new();

        if surfaces_restored {
            operations.push(SpecialDialogOperation::ClearSurfacesRestored);
            self.display = true;
        }

        if game_normal {
            operations.push(SpecialDialogOperation::InvokeGameCallback);
        } else {
            operations.push(SpecialDialogOperation::InvokeMainLoop);
            if main_loop_requested_exit {
                self.process = false;
                operations.extend(self.finish_operations());
                return SpecialDialogStep {
                    operations, complete: true, committed: false, result: None,
                };
            }
        }

        if self.display {
            operations.extend(self.draw_operations());
            self.display = false;
        }

        match input {
            SpecialDialogInput::None => {}
            SpecialDialogInput::Toggle(index) if index < SPECIAL_OPTIONS.len() => {
                let option = SPECIAL_OPTIONS[index];
                let value = !self.edited.option_value(option);
                self.edited.set_option_value(option, value);
                if value {
                    operations.push(SpecialDialogOperation::TurnCheckboxOn { index });
                } else {
                    operations.push(SpecialDialogOperation::TurnCheckboxOff { index });
                }
            }
            SpecialDialogInput::Toggle(_) => {}
            SpecialDialogInput::Cancel => {
                self.process = false;
                operations.extend(self.finish_operations());
                return SpecialDialogStep {
                    operations, complete: true, committed: false, result: None,
                };
            }
            SpecialDialogInput::Ok | SpecialDialogInput::Escape => {
                self.process = false;
                // The C++ code copies the ten setting values into oldspecial,
                // preserving every other field from the initial snapshot.
                let mut committed = self.original.clone();
                for option in SPECIAL_OPTIONS {
                    committed.set_option_value(option, self.edited.option_value(option));
                }
                operations.push(SpecialDialogOperation::QueueSpecialEvent(committed.clone()));
                operations.extend(self.finish_operations());
                return SpecialDialogStep {
                    operations, complete: true, committed: true, result: Some(committed),
                };
            }
        }

        SpecialDialogStep { operations, complete: false, committed: false, result: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_init_defaults_match_the_header_initializer() {
        let special = SpecialClass::default();
        assert!(special.is_defender_advantage);
        assert!(special.is_tiberium_growth);
        assert!(special.is_tiberium_spread);
        assert!(special.is_tiberium_fast);
        assert_eq!(special.health_bar_display_mode, HealthBarDisplayMode::Selected);
        assert_eq!(special.resource_bar_display_mode, ResourceBarDisplayMode::Selected);
        assert!(!special.modern_balance);
    }

    #[test]
    fn option_table_contains_the_ten_enabled_checkbox_rows_in_source_order() {
        assert_eq!(SPECIAL_OPTIONS, [
            SpecialOption::SeparateHelipad, SpecialOption::VisibleTarget,
            SpecialOption::TreeTarget, SpecialOption::McvDeploy,
            SpecialOption::SmartDefense, SpecialOption::ThreePoint,
            SpecialOption::TiberiumFast, SpecialOption::RoadPieces,
            SpecialOption::Scatter, SpecialOption::ShowNames,
        ]);
    }

    #[test]
    fn toggling_options_only_changes_the_local_snapshot_until_commit() {
        let current = SpecialClass::default();
        let mut dialog = SpecialDialog::new(&current);
        assert!(!dialog.edited().is_separate_helipad);
        let step = dialog.step(true, false, false, SpecialDialogInput::Toggle(0));
        assert!(!step.complete);
        assert!(dialog.edited().is_separate_helipad);
        assert!(!current.is_separate_helipad);
    }

    #[test]
    fn ok_and_escape_queue_an_event_and_preserve_non_dialog_fields() {
        let mut current = SpecialClass::default();
        current.is_defender_advantage = false;
        current.is_tiberium_growth = false;
        let mut dialog = SpecialDialog::new(&current);
        dialog.step(true, false, false, SpecialDialogInput::Toggle(0));
        let step = dialog.step(true, false, false, SpecialDialogInput::Ok);
        assert!(step.complete && step.committed);
        let committed = step.result.unwrap();
        assert!(committed.is_separate_helipad);
        assert!(!committed.is_defender_advantage);
        assert!(!committed.is_tiberium_growth);
        assert!(step.operations.iter().any(|operation| matches!(operation, SpecialDialogOperation::QueueSpecialEvent(_))));
        assert!(step.operations.contains(&SpecialDialogOperation::RevertMouseShape));
    }

    #[test]
    fn cancel_discards_local_changes_without_queueing_event() {
        let mut dialog = SpecialDialog::new(&SpecialClass::default());
        dialog.step(true, false, false, SpecialDialogInput::Toggle(1));
        let step = dialog.step(true, false, false, SpecialDialogInput::Cancel);
        assert!(step.complete);
        assert!(!step.committed);
        assert!(step.result.is_none());
        assert!(!step.operations.iter().any(|operation| matches!(operation, SpecialDialogOperation::QueueSpecialEvent(_))));
        assert!(step.operations.contains(&SpecialDialogOperation::RenderMap));
    }

    #[test]
    fn main_loop_exit_abandons_dialog_and_surface_restore_redraws() {
        let mut dialog = SpecialDialog::new(&SpecialClass::default());
        let restored = dialog.step(true, false, true, SpecialDialogInput::None);
        assert!(restored.operations.contains(&SpecialDialogOperation::ClearSurfacesRestored));
        assert!(restored.operations.contains(&SpecialDialogOperation::DrawDialogBox {
            x: 42, y: 19, width: 236, height: 162,
        }));

        let exit = dialog.step(false, true, false, SpecialDialogInput::None);
        assert!(exit.complete);
        assert!(!exit.committed);
        assert!(exit.operations.contains(&SpecialDialogOperation::InvokeMainLoop));
    }
}
