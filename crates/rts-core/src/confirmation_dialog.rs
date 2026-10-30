//! ConfirmationClass::Process from REDALERT/CONFDLG.CPP and
//! TIBERIANDAWN/CONFDLG.CPP.
//!
//! Red Alert's active Process(int) implementation simply returns true; its
//! complete string-dialog body is under #if (0). Tiberian Dawn's modal dialog
//! is represented as an explicit state machine. Each step performs at most
//! one callback/main-loop tick and one input event, so the game host can keep
//! its outer loop responsive.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfirmationVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfirmationInput {
    None,
    YesButton,
    NoButton,
    Escape,
    Left,
    Right,
    Return,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfirmationDrawOp {
    InvokeGameCallback,
    InvokeMainLoop,
    HideMouse,
    DrawDialogBox { x: i32, y: i32, width: i32, height: i32 },
    DrawCaption { text_id: i32, x: i32, y: i32, width: i32 },
    DrawMessage {
        x: i32,
        y: i32,
        width_factor: i32,
        green_text: bool,
        gradient_palette: bool,
        no_shadow: bool,
    },
    DrawAllButtons,
    ShowMouse,
    TurnButtonOff { button_id: i32 },
    FlagButtonForRedraw { button_id: i32 },
    TurnButtonOn { button_id: i32 },
    ClearSurfacesRestored,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConfirmationLayout {
    pub factor: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub button_height: i32,
    pub button_width: i32,
    pub yes_x: i32,
    pub no_x: i32,
    pub button_y: i32,
    pub background_width: i32,
    pub background_height: i32,
}

impl ConfirmationLayout {
    /// Uses Formatted_Window_String width/height and the real font/button
    /// metrics measured by the platform host.
    pub fn new(
        seen_buffer_width: i32,
        seen_buffer_height: i32,
        formatted_width: i32,
        formatted_height: i32,
        yes_text_pixel_width: i32,
        font_height: i32,
        font_y_spacing: i32,
    ) -> Self {
        let factor = if seen_buffer_width == 320 { 1 } else { 2 };
        let width = formatted_width + 60 * factor;
        let height = formatted_height + 60 * factor;
        let x = (320 * factor - width) / 2;
        let y = (200 * factor - height) / 2;
        let button_height = font_height + font_y_spacing + 2;
        let button_width = (yes_text_pixel_width + 8).max(30);
        let button_y = y + height - (button_height + 5 * factor);
        Self {
            factor, x, y, width, height, button_height, button_width,
            yes_x: x + 10 * factor,
            no_x: x + width - (button_width + 10 * factor),
            button_y,
            background_width: seen_buffer_width,
            background_height: seen_buffer_height,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfirmationStep {
    pub operations: Vec<ConfirmationDrawOp>,
    pub complete: bool,
    pub result: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfirmationDialog {
    pub variant: ConfirmationVariant,
    pub text: Vec<u8>,
    pub layout: ConfirmationLayout,
    pub display: bool,
    pub process: bool,
    /// 0 = Yes, 1 = No. The legacy dialog initially focuses No.
    pub current_button: usize,
    pub result: bool,
    pub yes_button_id: i32,
    pub no_button_id: i32,
    pub caption_text_id: i32,
}

impl ConfirmationDialog {
    pub const BUTTON_YES: i32 = 1;
    pub const BUTTON_NO: i32 = 2;

    pub fn new_tiberian_dawn(
        text: &[u8],
        layout: ConfirmationLayout,
        caption_text_id: i32,
    ) -> Self {
        let mut text_copy = text.to_vec();
        if let Some(end) = text_copy.iter().position(|byte| *byte == 0) {
            text_copy.truncate(end);
        }
        Self {
            variant: ConfirmationVariant::TiberianDawn,
            text: text_copy,
            layout,
            display: true,
            process: true,
            current_button: 1,
            result: true,
            yes_button_id: Self::BUTTON_YES,
            no_button_id: Self::BUTTON_NO,
            caption_text_id,
        }
    }

    /// The active Red Alert Process(int) implementation discards its argument
    /// and returns true. The string overload's body is compiled out.
    pub const fn red_alert_process(_text_id: i32) -> bool { true }

    fn selected_button_id(&self) -> i32 {
        if self.current_button == 0 { self.yes_button_id } else { self.no_button_id }
    }

    /// Process one modal-loop iteration. If game_normal is false, the caller
    /// supplies Main_Loop's return value; true ends the dialog with false.
    pub fn step(
        &mut self,
        game_normal: bool,
        main_loop_requested_exit: bool,
        surfaces_restored: bool,
        input: ConfirmationInput,
    ) -> ConfirmationStep {
        if !self.process {
            return ConfirmationStep { operations: Vec::new(), complete: true, result: Some(self.result) };
        }

        let mut operations = Vec::new();
        if game_normal {
            operations.push(ConfirmationDrawOp::InvokeGameCallback);
        } else {
            operations.push(ConfirmationDrawOp::InvokeMainLoop);
            if main_loop_requested_exit {
                self.process = false;
                self.result = false;
            }
        }

        if !self.process {
            return ConfirmationStep { operations, complete: true, result: Some(false) };
        }

        if surfaces_restored {
            operations.push(ConfirmationDrawOp::ClearSurfacesRestored);
            self.display = true;
        }

        if self.display {
            operations.push(ConfirmationDrawOp::HideMouse);
            operations.push(ConfirmationDrawOp::DrawDialogBox {
                x: self.layout.x,
                y: self.layout.y,
                width: self.layout.width,
                height: self.layout.height,
            });
            operations.push(ConfirmationDrawOp::DrawCaption {
                text_id: self.caption_text_id,
                x: self.layout.x,
                y: self.layout.y,
                width: self.layout.width,
            });
            operations.push(ConfirmationDrawOp::DrawMessage {
                x: self.layout.x + 20 * self.layout.factor,
                y: self.layout.y + 30 * self.layout.factor,
                width_factor: self.layout.factor,
                green_text: true,
                gradient_palette: true,
                no_shadow: true,
            });
            operations.push(ConfirmationDrawOp::DrawAllButtons);
            operations.push(ConfirmationDrawOp::ShowMouse);
            self.display = false;
        }

        let selected = match input {
            ConfirmationInput::YesButton => Some(true),
            ConfirmationInput::NoButton | ConfirmationInput::Escape => Some(false),
            ConfirmationInput::Left => {
                let old = self.selected_button_id();
                operations.push(ConfirmationDrawOp::TurnButtonOff { button_id: old });
                operations.push(ConfirmationDrawOp::FlagButtonForRedraw { button_id: old });
                self.current_button = (self.current_button + 1) % 2;
                let new = self.selected_button_id();
                operations.push(ConfirmationDrawOp::TurnButtonOn { button_id: new });
                operations.push(ConfirmationDrawOp::FlagButtonForRedraw { button_id: new });
                None
            }
            ConfirmationInput::Right => {
                let old = self.selected_button_id();
                operations.push(ConfirmationDrawOp::TurnButtonOff { button_id: old });
                operations.push(ConfirmationDrawOp::FlagButtonForRedraw { button_id: old });
                self.current_button = (self.current_button + 1) % 2;
                let new = self.selected_button_id();
                operations.push(ConfirmationDrawOp::TurnButtonOn { button_id: new });
                operations.push(ConfirmationDrawOp::FlagButtonForRedraw { button_id: new });
                None
            }
            ConfirmationInput::Return => Some(self.current_button == 0),
            ConfirmationInput::None => None,
        };

        if let Some(result) = selected {
            self.result = result;
            self.process = false;
        }

        ConfirmationStep {
            operations,
            complete: !self.process,
            result: if self.process { None } else { Some(self.result) },
        }
    }

    pub fn flag_for_redraw(&mut self) { self.display = true; }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout() -> ConfirmationLayout {
        ConfirmationLayout::new(320, 200, 180, 60, 42, 8, 2)
    }

    #[test]
    fn red_alert_active_integer_overload_always_returns_true() {
        assert!(ConfirmationDialog::red_alert_process(0));
        assert!(ConfirmationDialog::red_alert_process(1234));
    }

    #[test]
    fn tiberian_dawn_layout_uses_seen_buffer_factor_and_font_metrics() {
        let value = layout();
        assert_eq!(value.factor, 1);
        assert_eq!((value.width, value.height), (240, 120));
        assert_eq!((value.x, value.y), (40, 40));
        assert_eq!(value.button_height, 12);
        assert_eq!(value.button_width, 50);
        assert_eq!((value.yes_x, value.no_x, value.button_y), (50, 230, 143));

        let large = ConfirmationLayout::new(640, 400, 360, 120, 42, 8, 2);
        assert_eq!(large.factor, 2);
        assert_eq!((large.x, large.y), (80, 80));
    }

    #[test]
    fn initial_focus_is_no_and_return_confirms_no() {
        let mut dialog = ConfirmationDialog::new_tiberian_dawn(b"Delete saved game?", layout(), 77);
        assert_eq!(dialog.current_button, 1);
        let step = dialog.step(true, false, false, ConfirmationInput::Return);
        assert!(step.complete);
        assert_eq!(step.result, Some(false));
        assert!(step.operations.contains(&ConfirmationDrawOp::DrawDialogBox {
            x: 40, y: 40, width: 240, height: 120,
        }));
    }

    #[test]
    fn arrow_keys_toggle_button_focus_and_yes_button_confirms() {
        let mut dialog = ConfirmationDialog::new_tiberian_dawn(b"Confirm", layout(), 77);
        let initial = dialog.step(true, false, false, ConfirmationInput::Left);
        assert!(!initial.complete);
        assert_eq!(dialog.current_button, 0);
        assert!(initial.operations.contains(&ConfirmationDrawOp::TurnButtonOff { button_id: 2 }));
        assert!(initial.operations.contains(&ConfirmationDrawOp::TurnButtonOn { button_id: 1 }));
        let yes = dialog.step(true, false, false, ConfirmationInput::YesButton);
        assert_eq!(yes.result, Some(true));
    }

    #[test]
    fn escape_and_no_button_cancel_and_main_loop_can_abort() {
        let mut escape = ConfirmationDialog::new_tiberian_dawn(b"Confirm", layout(), 77);
        assert_eq!(escape.step(true, false, false, ConfirmationInput::Escape).result, Some(false));

        let mut main_loop = ConfirmationDialog::new_tiberian_dawn(b"Confirm", layout(), 77);
        let step = main_loop.step(false, true, false, ConfirmationInput::None);
        assert_eq!(step.result, Some(false));
        assert!(step.operations.contains(&ConfirmationDrawOp::InvokeMainLoop));
    }

    #[test]
    fn restored_surface_forces_a_full_redraw() {
        let mut dialog = ConfirmationDialog::new_tiberian_dawn(b"Confirm", layout(), 77);
        let first = dialog.step(true, false, false, ConfirmationInput::None);
        assert!(!first.complete);
        let restored = dialog.step(true, false, true, ConfirmationInput::None);
        assert!(restored.operations.contains(&ConfirmationDrawOp::ClearSurfacesRestored));
        assert!(restored.operations.contains(&ConfirmationDrawOp::DrawAllButtons));
    }
}
