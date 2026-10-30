//! Tiberian Dawn VisualControlsClass from TIBERIANDAWN/VISUDLG.CPP.
//!
//! Each call to step represents one outer modal-loop pass. UI rendering,
//! pointer interaction and the game callback remain host operations, while
//! slider values, option updates, focus navigation and reset semantics are
//! deterministic Rust state.

pub const BUTTON_BRIGHTNESS: i32 = 1;
pub const BUTTON_COLOR: i32 = 2;
pub const BUTTON_CONTRAST: i32 = 3;
pub const BUTTON_TINT: i32 = 4;
pub const BUTTON_RESET: i32 = 5;
pub const BUTTON_OPTIONS: i32 = 6;
pub const SLIDER_THUMB_UNITS: u8 = 20;
pub const SLIDER_MAXIMUM: u8 = 255;
pub const SLIDER_EFFECTIVE_MAXIMUM: u8 = SLIDER_MAXIMUM - SLIDER_THUMB_UNITS;
pub const RESET_VALUE: u8 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisualControl {
    Brightness,
    Color,
    Contrast,
    Tint,
}

impl VisualControl {
    pub const ALL: [Self; 4] = [Self::Brightness, Self::Color, Self::Contrast, Self::Tint];

    pub const fn index(self) -> usize {
        match self {
            Self::Brightness => 0,
            Self::Color => 1,
            Self::Contrast => 2,
            Self::Tint => 3,
        }
    }

    pub const fn button_id(self) -> i32 {
        self.index() as i32 + BUTTON_BRIGHTNESS
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisualControlsLayout {
    pub factor: i32,
    pub option_width: i32,
    pub option_height: i32,
    pub option_x: i32,
    pub option_y: i32,
    pub text_x: i32,
    pub text_y: i32,
    pub slider_x: i32,
    pub slider_y: i32,
    pub slider_width: i32,
    pub slider_height: i32,
    pub slider_y_spacing: i32,
    pub button_x: i32,
    pub button_y: i32,
}

impl VisualControlsLayout {
    /// Exact VisualControlsClass::Init geometry.
    pub const fn new(seen_width: i32, seen_height: i32) -> Self {
        let factor = if seen_width == 320 { 1 } else { 2 };
        let option_width = 216 * factor;
        let option_height = 122 * factor;
        let option_x = (seen_width - option_width) / 2;
        let option_y = (seen_height - option_height) / 2;
        Self {
            factor,
            option_width,
            option_height,
            option_x,
            option_y,
            text_x: option_x + 28 * factor,
            text_y: option_y + 30 * factor,
            slider_x: option_x + 105 * factor,
            slider_y: option_y + 30 * factor,
            slider_width: 70 * factor,
            slider_height: 5 * factor,
            slider_y_spacing: 11 * factor,
            button_x: option_x + 63 * factor,
            button_y: option_y + 102 * factor,
        }
    }

    pub const fn slider_position(&self, control: VisualControl) -> (i32, i32) {
        (self.slider_x, self.slider_y + control.index() as i32 * self.slider_y_spacing)
    }

    pub const fn title_position(&self, control: VisualControl) -> (i32, i32) {
        (self.slider_x - 8, self.text_y + control.index() as i32 * self.slider_y_spacing)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualTitleStyle {
    BrightColor,
    GradientPalette,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualControlsInput {
    None,
    SliderChanged { control: VisualControl, value: u8 },
    ResetButton,
    OptionsButton,
    Escape,
    Left,
    Right,
    Up,
    Down,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualControlsOp {
    InvokeGameCallback,
    InvokeMainLoop,
    ClearSurfacesRestored,
    HideMouse,
    DrawDialogBox { x: i32, y: i32, width: i32, height: i32 },
    DrawCaption { x: i32, y: i32, width: i32 },
    ShowMouse,
    DrawTitle {
        control: VisualControl,
        x: i32,
        y: i32,
        style: VisualTitleStyle,
    },
    DrawAllGadgets,
    TurnButtonOff { button_id: i32 },
    FlagButtonForRedraw { button_id: i32 },
    TurnButtonOn { button_id: i32 },
    SetSliderValue { control: VisualControl, value: u8 },
    SetOption { control: VisualControl, value: u8 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualControlsStep {
    pub operations: Vec<VisualControlsOp>,
    pub complete: bool,
    pub slider_values: [u8; 4],
    pub option_values: [u8; 4],
    pub current_button: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualControlsDialog {
    pub layout: VisualControlsLayout,
    /// Values held by the four SliderClass controls (constructor caps at max-thumb).
    pub slider_values: [u8; 4],
    /// Persisted Options values; not changed until user input or Reset.
    pub option_values: [u8; 4],
    /// 0..3 are sliders, 4 is Reset, 5 is Options.
    pub current_button: usize,
    pub display: bool,
    pub partial: bool,
    pub process: bool,
}

impl VisualControlsDialog {
    pub fn new(
        seen_width: i32,
        seen_height: i32,
        initial_values: [u8; 4],
    ) -> Self {
        let slider_values = initial_values.map(|value| value.min(SLIDER_EFFECTIVE_MAXIMUM));
        Self {
            layout: VisualControlsLayout::new(seen_width, seen_height),
            slider_values,
            option_values: initial_values,
            current_button: 0,
            display: true,
            partial: true,
            process: true,
        }
    }

    fn snapshot(&self, operations: Vec<VisualControlsOp>) -> VisualControlsStep {
        VisualControlsStep {
            operations,
            complete: !self.process,
            slider_values: self.slider_values,
            option_values: self.option_values,
            current_button: self.current_button,
        }
    }

    fn draw_full(&mut self, operations: &mut Vec<VisualControlsOp>) {
        operations.push(VisualControlsOp::HideMouse);
        operations.push(VisualControlsOp::DrawDialogBox {
            x: self.layout.option_x,
            y: self.layout.option_y,
            width: self.layout.option_width,
            height: self.layout.option_height,
        });
        operations.push(VisualControlsOp::DrawCaption {
            x: self.layout.option_x,
            y: self.layout.option_y,
            width: self.layout.option_width,
        });
        operations.push(VisualControlsOp::ShowMouse);
        self.display = false;
        self.partial = true;
    }

    fn draw_partial(&mut self, operations: &mut Vec<VisualControlsOp>) {
        operations.push(VisualControlsOp::HideMouse);
        for control in VisualControl::ALL {
            let (x, y) = self.layout.title_position(control);
            operations.push(VisualControlsOp::DrawTitle {
                control,
                x,
                y,
                style: if self.current_button == control.index() {
                    VisualTitleStyle::BrightColor
                } else {
                    VisualTitleStyle::GradientPalette
                },
            });
        }
        operations.push(VisualControlsOp::DrawAllGadgets);
        operations.push(VisualControlsOp::ShowMouse);
        self.partial = false;
    }

    fn set_slider_value(
        &mut self,
        control: VisualControl,
        value: u8,
        operations: &mut Vec<VisualControlsOp>,
    ) {
        let value = value.min(SLIDER_EFFECTIVE_MAXIMUM);
        let index = control.index();
        self.slider_values[index] = value;
        self.option_values[index] = value;
        operations.push(VisualControlsOp::SetSliderValue { control, value });
        operations.push(VisualControlsOp::SetOption { control, value });
    }

    fn reset(&mut self, operations: &mut Vec<VisualControlsOp>) {
        for control in VisualControl::ALL {
            let index = control.index();
            self.slider_values[index] = RESET_VALUE;
            self.option_values[index] = RESET_VALUE;
            operations.push(VisualControlsOp::SetSliderValue { control, value: RESET_VALUE });
            operations.push(VisualControlsOp::SetOption { control, value: RESET_VALUE });
        }
    }

    fn focused_button_id(&self) -> i32 {
        if self.current_button == 4 { BUTTON_RESET } else { BUTTON_OPTIONS }
    }

    fn move_horizontal(&mut self, operations: &mut Vec<VisualControlsOp>, to_right: bool) {
        if self.current_button <= 3 {
            let control = VisualControl::ALL[self.current_button];
            // SliderClass::Bump(1) subtracts Thumb; Bump(0) adds Thumb.
            let current = self.slider_values[self.current_button] as i16;
            let next = if to_right {
                current + SLIDER_THUMB_UNITS as i16
            } else {
                current - SLIDER_THUMB_UNITS as i16
            }
            .clamp(0, SLIDER_EFFECTIVE_MAXIMUM as i16) as u8;
            self.set_slider_value(control, next, operations);
            return;
        }

        let old = self.focused_button_id();
        operations.push(VisualControlsOp::TurnButtonOff { button_id: old });
        operations.push(VisualControlsOp::FlagButtonForRedraw { button_id: old });
        self.current_button = if self.current_button == 4 { 5 } else { 4 };
        let new = self.focused_button_id();
        operations.push(VisualControlsOp::TurnButtonOn { button_id: new });
        operations.push(VisualControlsOp::FlagButtonForRedraw { button_id: new });
    }

    fn move_vertical(&mut self, operations: &mut Vec<VisualControlsOp>, down: bool) {
        let old_button = self.current_button;
        if old_button <= 3 {
            self.partial = true;
        } else {
            let old_id = self.focused_button_id();
            operations.push(VisualControlsOp::TurnButtonOff { button_id: old_id });
            operations.push(VisualControlsOp::FlagButtonForRedraw { button_id: old_id });
        }

        if down {
            self.current_button += 1;
            // The source's DOWN path wraps after Reset (index 4), not Options.
            if self.current_button > 4 {
                self.current_button = 0;
            }
        } else {
            if self.current_button == 0 {
                self.current_button = 4;
            } else {
                self.current_button -= 1;
                // Up from Options (index 5) skips Reset (index 4).
                if self.current_button == 4 && old_button == 5 {
                    self.current_button -= 1;
                }
            }
        }

        if self.current_button <= 3 {
            self.partial = true;
        } else {
            let new_id = self.focused_button_id();
            operations.push(VisualControlsOp::TurnButtonOn { button_id: new_id });
            operations.push(VisualControlsOp::FlagButtonForRedraw { button_id: new_id });
        }
    }

    /// One source modal-loop pass. The host performs callbacks and renders the
    /// returned operations; this method never blocks or performs I/O.
    pub fn step(
        &mut self,
        game_normal: bool,
        main_loop_requested_exit: bool,
        surfaces_restored: bool,
        input: VisualControlsInput,
    ) -> VisualControlsStep {
        if !self.process {
            return self.snapshot(Vec::new());
        }

        let mut operations = Vec::new();
        if game_normal {
            operations.push(VisualControlsOp::InvokeGameCallback);
        } else {
            operations.push(VisualControlsOp::InvokeMainLoop);
            if main_loop_requested_exit {
                self.process = false;
            }
        }

        if surfaces_restored {
            operations.push(VisualControlsOp::ClearSurfacesRestored);
            self.display = true;
        }
        if self.display {
            self.draw_full(&mut operations);
        }
        if self.partial {
            self.draw_partial(&mut operations);
        }

        match input {
            VisualControlsInput::SliderChanged { control, value } => {
                self.set_slider_value(control, value, &mut operations);
            }
            VisualControlsInput::ResetButton => self.reset(&mut operations),
            VisualControlsInput::OptionsButton | VisualControlsInput::Escape => {
                self.process = false;
            }
            VisualControlsInput::Left => self.move_horizontal(&mut operations, false),
            VisualControlsInput::Right => self.move_horizontal(&mut operations, true),
            VisualControlsInput::Up => self.move_vertical(&mut operations, false),
            VisualControlsInput::Down => self.move_vertical(&mut operations, true),
            VisualControlsInput::Return => match self.current_button + 1 {
                BUTTON_RESET => self.reset(&mut operations),
                BUTTON_OPTIONS => self.process = false,
                _ => {}
            },
            VisualControlsInput::None => {}
        }

        self.snapshot(operations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_geometry_matches_320_and_640_resolution_paths() {
        let low = VisualControlsLayout::new(320, 200);
        assert_eq!((low.factor, low.option_width, low.option_height), (1, 216, 122));
        assert_eq!((low.option_x, low.option_y), (52, 39));
        assert_eq!((low.text_x, low.text_y), (80, 69));
        assert_eq!((low.slider_x, low.slider_y, low.slider_width, low.slider_height), (157, 69, 70, 5));
        assert_eq!((low.slider_y_spacing, low.button_x, low.button_y), (11, 115, 141));

        let high = VisualControlsLayout::new(640, 400);
        assert_eq!((high.factor, high.option_width, high.option_height), (2, 432, 244));
        assert_eq!((high.option_x, high.option_y), (104, 78));
        assert_eq!(high.slider_position(VisualControl::Tint), (314, 138));
    }

    #[test]
    fn constructor_caps_slider_positions_but_preserves_initial_option_values() {
        let dialog = VisualControlsDialog::new(320, 200, [10, 128, 240, 255]);
        assert_eq!(dialog.slider_values, [10, 128, 235, 235]);
        assert_eq!(dialog.option_values, [10, 128, 240, 255]);
        assert_eq!(dialog.current_button, 0);
    }

    #[test]
    fn left_and_right_bump_by_the_thumb_size_and_update_the_option() {
        let mut dialog = VisualControlsDialog::new(320, 200, [128; 4]);
        let left = dialog.step(true, false, false, VisualControlsInput::Left);
        assert_eq!(left.slider_values[0], 108);
        assert!(left.operations.contains(&VisualControlsOp::SetOption {
            control: VisualControl::Brightness, value: 108,
        }));
        let right = dialog.step(true, false, false, VisualControlsInput::Right);
        assert_eq!(right.slider_values[0], 128);
        assert_eq!(right.option_values[0], 128);
    }

    #[test]
    fn slider_bumps_respect_zero_and_effective_upper_limit() {
        let mut dialog = VisualControlsDialog::new(320, 200, [0, 255, 0, 0]);
        let lower = dialog.step(true, false, false, VisualControlsInput::Left);
        assert_eq!(lower.slider_values[0], 0);
        let upper = dialog.step(true, false, false, VisualControlsInput::Right);
        assert_eq!(upper.slider_values[0], 20);
        dialog.slider_values[0] = SLIDER_EFFECTIVE_MAXIMUM;
        let clipped = dialog.step(true, false, false, VisualControlsInput::Right);
        assert_eq!(clipped.slider_values[0], SLIDER_EFFECTIVE_MAXIMUM);
    }

    #[test]
    fn mouse_slider_events_update_only_the_selected_option_value() {
        let mut dialog = VisualControlsDialog::new(320, 200, [128; 4]);
        let step = dialog.step(true, false, false, VisualControlsInput::SliderChanged {
            control: VisualControl::Contrast, value: 77,
        });
        assert_eq!(step.option_values, [128, 128, 77, 128]);
        assert!(step.operations.contains(&VisualControlsOp::SetOption {
            control: VisualControl::Contrast, value: 77,
        }));
    }

    #[test]
    fn vertical_navigation_matches_slider_reset_and_options_path() {
        let mut dialog = VisualControlsDialog::new(320, 200, [128; 4]);
        assert_eq!(dialog.step(true, false, false, VisualControlsInput::Up).current_button, 4);
        assert_eq!(dialog.step(true, false, false, VisualControlsInput::Down).current_button, 0);

        dialog.current_button = 3;
        assert_eq!(dialog.step(true, false, false, VisualControlsInput::Down).current_button, 4);
        assert_eq!(dialog.step(true, false, false, VisualControlsInput::Down).current_button, 0);

        dialog.current_button = 5;
        assert_eq!(dialog.step(true, false, false, VisualControlsInput::Up).current_button, 3);
        assert_eq!(dialog.step(true, false, false, VisualControlsInput::Down).current_button, 4);
    }

    #[test]
    fn horizontal_navigation_toggles_reset_and_options_buttons() {
        let mut dialog = VisualControlsDialog::new(320, 200, [128; 4]);
        dialog.current_button = 4;
        let step = dialog.step(true, false, false, VisualControlsInput::Right);
        assert_eq!(step.current_button, 5);
        assert!(step.operations.contains(&VisualControlsOp::TurnButtonOff { button_id: BUTTON_RESET }));
        assert!(step.operations.contains(&VisualControlsOp::TurnButtonOn { button_id: BUTTON_OPTIONS }));
    }

    #[test]
    fn reset_button_and_enter_on_reset_restore_all_four_values_to_128() {
        let mut dialog = VisualControlsDialog::new(320, 200, [20, 40, 60, 80]);
        let reset = dialog.step(true, false, false, VisualControlsInput::ResetButton);
        assert_eq!(reset.option_values, [128; 4]);
        assert_eq!(reset.slider_values, [128; 4]);
        dialog.option_values = [1, 2, 3, 4];
        dialog.slider_values = [1, 2, 3, 4];
        dialog.current_button = 4;
        assert_eq!(dialog.step(true, false, false, VisualControlsInput::Return).option_values, [128; 4]);
    }

    #[test]
    fn surfaces_restored_forces_full_then_partial_redraw() {
        let mut dialog = VisualControlsDialog::new(320, 200, [128; 4]);
        let step = dialog.step(true, false, true, VisualControlsInput::None);
        assert!(step.operations.contains(&VisualControlsOp::ClearSurfacesRestored));
        assert!(step.operations.iter().any(|op| matches!(op, VisualControlsOp::DrawDialogBox { .. })));
        assert_eq!(step.operations.iter().filter(|op| matches!(op, VisualControlsOp::DrawTitle { .. })).count(), 4);
        assert!(!step.operations.last().is_some_and(|op| matches!(op, VisualControlsOp::ShowMouse)));
    }

    #[test]
    fn options_escape_and_main_loop_exit_close_dialog() {
        let mut escape = VisualControlsDialog::new(320, 200, [128; 4]);
        assert!(escape.step(true, false, false, VisualControlsInput::Escape).complete);

        let mut options = VisualControlsDialog::new(320, 200, [128; 4]);
        options.current_button = 5;
        assert!(options.step(true, false, false, VisualControlsInput::Return).complete);

        let mut main_loop = VisualControlsDialog::new(320, 200, [128; 4]);
        assert!(main_loop.step(false, true, false, VisualControlsInput::None).complete);
    }
}
