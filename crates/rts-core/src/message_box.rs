//! Tiberian Dawn CCMessageBox::Process port from TIBERIANDAWN/MSGBOX.CPP.
//!
//! The legacy implementation blocks inside a modal loop. This port exposes
//! one callback/input iteration per step, allowing the host game loop to
//! remain responsive. Rendering, font metrics, localization and the timer
//! are supplied by the platform layer.

pub const BUTTON_1: i32 = 1;
pub const BUTTON_2: i32 = 2;
pub const BUTTON_3: i32 = 3;
pub const BUTTON_FLAG: u16 = 0x8000;
pub const MESSAGE_BUFFER_SIZE: usize = 511;
pub const MESSAGE_COPY_LIMIT: usize = MESSAGE_BUFFER_SIZE - 2;
pub const NO_BUTTON_DELAY_SECONDS: u32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxMode { Standard, JapanesePicture }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageBoxRequest {
    pub message: Vec<u8>,
    pub button_labels: [Option<String>; 3],
    pub preserve: bool,
}

impl MessageBoxRequest {
    pub fn from_text(
        message: &[u8],
        button1: Option<&str>,
        button2: Option<&str>,
        button3: Option<&str>,
        preserve: bool,
    ) -> Self {
        let c_end = message.iter().position(|byte| *byte == 0).unwrap_or(message.len());
        let mut text = message[..c_end.min(MESSAGE_COPY_LIMIT)].to_vec();
        text.push(0);

        let mut labels = [
            button1.filter(|value| !value.is_empty()).map(str::to_owned),
            button2.filter(|value| !value.is_empty()).map(str::to_owned),
            button3.filter(|value| !value.is_empty()).map(str::to_owned),
        ];
        if labels[0].is_none() {
            labels = [None, None, None];
        } else if labels[1].is_none() {
            labels[2] = None;
        }
        Self { message: text, button_labels: labels, preserve }
    }

    pub fn button_count(&self) -> usize {
        if self.button_labels[0].is_none() { 0 }
        else if self.button_labels[1].is_none() { 1 }
        else if self.button_labels[2].is_none() { 2 }
        else { 3 }
    }

    pub fn from_text_ids(
        resolver: &impl MessageTextResolver,
        message_id: i32,
        buttons: [Option<i32>; 3],
        preserve: bool,
    ) -> Self {
        let text = resolver.text_string(message_id);
        let labels = buttons.map(|id| id.map(|value| resolver.text_string(value)));
        Self::from_text(&text.into_bytes(), labels[0].as_deref(), labels[1].as_deref(), labels[2].as_deref(), preserve)
    }

    pub fn with_resource_buttons(
        message: &str,
        resolver: &impl MessageTextResolver,
        buttons: [Option<i32>; 3],
        preserve: bool,
    ) -> Self {
        let labels = buttons.map(|id| id.map(|value| resolver.text_string(value)));
        Self::from_text(message.as_bytes(), labels[0].as_deref(), labels[1].as_deref(), labels[2].as_deref(), preserve)
    }
}

pub trait MessageTextResolver {
    fn text_string(&self, text_id: i32) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormattedMessage { pub width: i32, pub height: i32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageBoxMetrics {
    pub seen_width: i32,
    pub seen_height: i32,
    pub formatted: FormattedMessage,
    pub button1_text_width: i32,
    pub button2_text_width: i32,
    pub button3_default_width: i32,
    pub font_height: i32,
    pub font_y_spacing: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageBoxLayout {
    pub factor: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub button_height: i32,
    pub button_width: i32,
    pub button1_x: i32,
    pub button2_x: i32,
    pub button3_x: i32,
    pub button_y: i32,
    pub message_x: i32,
    pub message_y: i32,
}

impl MessageBoxLayout {
    pub fn new(metrics: MessageBoxMetrics, button_count: usize, mode: MessageBoxMode) -> Self {
        let factor = if metrics.seen_width == 320 { 1 } else { 2 };
        let (formatted_width, formatted_height) = match mode {
            MessageBoxMode::Standard => (metrics.formatted.width, metrics.formatted.height),
            MessageBoxMode::JapanesePicture => (90, 80),
        };
        let first_width = (metrics.button1_text_width + 8 * factor).max(30 * factor);
        let button_width = if button_count >= 2 {
            first_width.max(metrics.button2_text_width + 8 * factor)
        } else { first_width };
        let button_height = metrics.font_height + metrics.font_y_spacing + 2 * factor;
        let width = formatted_width.max(50 * factor) + 40 * factor;
        let height = formatted_height + if button_count == 0 { 30 * factor } else { 60 * factor };
        let x = (metrics.seen_width - width) / 2;
        let y = (metrics.seen_height - height) / 2;
        Self {
            factor, x, y, width, height, button_height, button_width,
            button1_x: x + if button_count == 1 { (width - button_width) >> 1 } else { 10 * factor },
            button2_x: x + width - (button_width + 10 * factor),
            button3_x: x + ((width - metrics.button3_default_width) >> 1),
            button_y: y + height - (button_height + 5 * factor),
            message_x: x + 20 * factor,
            message_y: y + 25 * factor,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageBoxInput {
    None,
    ButtonCode(u16),
    Escape,
    Left,
    Right,
    Return,
    Other(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageBoxDrawOp {
    SetLogicPageToSeen,
    SaveVisiblePage,
    HideMouse,
    CaptureDialogRect { x: i32, y: i32, width: i32, height: i32 },
    DrawDialogBox { x: i32, y: i32, width: i32, height: i32 },
    DrawCaption { text_id: i32, x: i32, y: i32, width: i32 },
    DrawMessage { text: Vec<u8>, x: i32, y: i32, green_text: bool, gradient_palette: bool, no_shadow: bool },
    DrawJapanesePicture { path: Vec<u8>, center_x: i32, center_y: i32 },
    DrawAllButtons { physical_order: Vec<i32>, current_button: usize },
    ShowMouse,
    ClearSurfacesRestored,
    RestoreSavedVisiblePage,
    InvokeGameCallback,
    TurnButtonOff { button_id: i32 },
    FlagButtonForRedraw { button_id: i32 },
    TurnButtonOn { button_id: i32 },
    ClearKeyboard,
    RestoreDialogRect { x: i32, y: i32, width: i32, height: i32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageBoxStep {
    pub operations: Vec<MessageBoxDrawOp>,
    pub complete: bool,
    pub result: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageBoxDialog {
    pub caption_id: i32,
    pub request: MessageBoxRequest,
    pub layout: MessageBoxLayout,
    pub mode: MessageBoxMode,
    pub current_button: usize,
    pub display: bool,
    pub started: bool,
    pub complete: bool,
    pub result: Option<i32>,
    real_values: [i32; 3],
}

impl MessageBoxDialog {
    pub fn new(caption_id: i32, request: MessageBoxRequest, metrics: MessageBoxMetrics, mode: MessageBoxMode) -> Self {
        let count = request.button_count();
        let real_values = match count {
            0 => [0, 0, 0],
            1 => [BUTTON_1, 0, 0],
            2 => [BUTTON_1, BUTTON_2, 0],
            _ => [BUTTON_1, BUTTON_3, BUTTON_2],
        };
        Self {
            caption_id, layout: MessageBoxLayout::new(metrics, count, mode), request, mode,
            current_button: 0, display: true, started: false, complete: false,
            result: None, real_values,
        }
    }

    fn physical_order(&self) -> Vec<i32> {
        match self.request.button_count() {
            0 => vec![],
            1 => vec![BUTTON_1],
            2 => vec![BUTTON_1, BUTTON_2],
            _ => vec![BUTTON_1, BUTTON_3, BUTTON_2],
        }
    }

    fn draw_operations(&self) -> Vec<MessageBoxDrawOp> {
        let mut ops = vec![MessageBoxDrawOp::HideMouse];
        if self.request.preserve {
            ops.push(MessageBoxDrawOp::CaptureDialogRect {
                x: self.layout.x, y: self.layout.y, width: self.layout.width, height: self.layout.height,
            });
        }
        match self.mode {
            MessageBoxMode::JapanesePicture => ops.push(MessageBoxDrawOp::DrawJapanesePicture {
                path: self.request.message.clone(), center_x: 160, center_y: 100,
            }),
            MessageBoxMode::Standard => {
                ops.push(MessageBoxDrawOp::DrawDialogBox {
                    x: self.layout.x, y: self.layout.y, width: self.layout.width, height: self.layout.height,
                });
                ops.push(MessageBoxDrawOp::DrawCaption {
                    text_id: self.caption_id, x: self.layout.x, y: self.layout.y, width: self.layout.width,
                });
                ops.push(MessageBoxDrawOp::DrawMessage {
                    text: self.request.message.clone(), x: self.layout.message_x, y: self.layout.message_y,
                    green_text: true, gradient_palette: true, no_shadow: true,
                });
            }
        }
        if self.request.button_count() > 0 {
            ops.push(MessageBoxDrawOp::DrawAllButtons {
                physical_order: self.physical_order(), current_button: self.current_button,
            });
        }
        ops.push(MessageBoxDrawOp::ShowMouse);
        ops
    }

    pub fn start(&mut self) -> MessageBoxStep {
        if self.started { return self.snapshot(Vec::new()); }
        self.started = true;
        let mut ops = vec![MessageBoxDrawOp::SetLogicPageToSeen, MessageBoxDrawOp::SaveVisiblePage];
        ops.extend(self.draw_operations());
        self.display = false;
        self.snapshot(ops)
    }

    fn snapshot(&self, operations: Vec<MessageBoxDrawOp>) -> MessageBoxStep {
        MessageBoxStep { operations, complete: self.complete, result: self.result }
    }

    fn finish(&mut self, result: i32, ops: &mut Vec<MessageBoxDrawOp>) {
        self.result = Some(result);
        self.complete = true;
        if self.request.preserve {
            ops.push(MessageBoxDrawOp::HideMouse);
            ops.push(MessageBoxDrawOp::RestoreDialogRect {
                x: self.layout.x, y: self.layout.y, width: self.layout.width, height: self.layout.height,
            });
            ops.push(MessageBoxDrawOp::ShowMouse);
        }
    }

    fn selection_result(selection: i32) -> Option<i32> {
        match selection { BUTTON_1 => Some(0), BUTTON_2 => Some(1), BUTTON_3 => Some(2), _ => None }
    }

    fn selection_from_input(&self, input: MessageBoxInput) -> Option<i32> {
        let count = self.request.button_count();
        match input {
            MessageBoxInput::ButtonCode(code) if code == (BUTTON_1 as u16 | BUTTON_FLAG) => Some(self.real_values[0]),
            MessageBoxInput::ButtonCode(code) if code == (BUTTON_2 as u16 | BUTTON_FLAG) && count >= 2 => Some(self.real_values[1]),
            MessageBoxInput::ButtonCode(code) if code == (BUTTON_3 as u16 | BUTTON_FLAG) && count >= 3 => Some(self.real_values[2]),
            MessageBoxInput::Escape if count >= 3 => Some(self.real_values[1]),
            // The C++ source reads an uninitialized realval[2] for 1/2 buttons.
            // Choose the last declared button as the deterministic compatibility fallback.
            MessageBoxInput::Escape if count == 2 => Some(self.real_values[1]),
            MessageBoxInput::Escape if count == 1 => Some(self.real_values[0]),
            MessageBoxInput::Return => Some(self.current_button as i32 + BUTTON_1),
            _ => None,
        }
    }

    fn focused_button_id(&self, index: usize) -> i32 {
        match (self.request.button_count(), index) {
            (1, 0) | (2, 0) | (3, 0) => BUTTON_1,
            (2, 1) | (3, 2) => BUTTON_2,
            (3, 1) => BUTTON_3,
            _ => BUTTON_1,
        }
    }

    /// Executes one callback/input iteration. The host owns the 4-second timer
    /// on the zero-button path and signals its expiration with timer_expired.
    pub fn step(&mut self, surfaces_restored: bool, input: MessageBoxInput, timer_expired: bool) -> MessageBoxStep {
        if self.complete { return self.snapshot(Vec::new()); }
        if !self.started { return self.start(); }
        let mut ops = Vec::new();

        if self.request.button_count() == 0 {
            ops.push(MessageBoxDrawOp::InvokeGameCallback);
            if timer_expired {
                ops.push(MessageBoxDrawOp::ClearKeyboard);
                self.finish(0, &mut ops);
            }
            return self.snapshot(ops);
        }

        if surfaces_restored {
            ops.push(MessageBoxDrawOp::ClearSurfacesRestored);
            ops.push(MessageBoxDrawOp::RestoreSavedVisiblePage);
            self.display = true;
        }
        if self.display {
            ops.extend(self.draw_operations());
            self.display = false;
        }
        ops.push(MessageBoxDrawOp::InvokeGameCallback);

        match input {
            MessageBoxInput::Left | MessageBoxInput::Right if self.request.button_count() > 1 => {
                let old = self.focused_button_id(self.current_button);
                ops.push(MessageBoxDrawOp::TurnButtonOff { button_id: old });
                ops.push(MessageBoxDrawOp::FlagButtonForRedraw { button_id: old });
                let count = self.request.button_count();
                self.current_button = if input == MessageBoxInput::Left {
                    if self.current_button == 0 { count - 1 } else { self.current_button - 1 }
                } else { (self.current_button + 1) % count };
                let new = self.focused_button_id(self.current_button);
                ops.push(MessageBoxDrawOp::TurnButtonOn { button_id: new });
                ops.push(MessageBoxDrawOp::FlagButtonForRedraw { button_id: new });
            }
            _ => {
                if let Some(selection) = self.selection_from_input(input) {
                    if let Some(result) = Self::selection_result(selection) {
                        self.finish(result, &mut ops);
                    }
                }
            }
        }
        self.snapshot(ops)
    }

    pub fn flag_for_redraw(&mut self) {
        if !self.complete { self.display = true; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Texts;
    impl MessageTextResolver for Texts {
        fn text_string(&self, id: i32) -> String { format!("TEXT-{id}") }
    }

    fn metrics() -> MessageBoxMetrics {
        MessageBoxMetrics {
            seen_width: 320, seen_height: 200,
            formatted: FormattedMessage { width: 100, height: 30 },
            button1_text_width: 42, button2_text_width: 35,
            button3_default_width: 45, font_height: 8, font_y_spacing: 2,
        }
    }

    fn request(buttons: [Option<&str>; 3], preserve: bool) -> MessageBoxRequest {
        MessageBoxRequest::from_text(b"Test message", buttons[0], buttons[1], buttons[2], preserve)
    }

    #[test]
    fn c_string_staging_and_empty_button_rules_match_source() {
        let long = vec![b'A'; 700];
        let bounded = MessageBoxRequest::from_text(&long, Some("OK"), None, None, false);
        assert_eq!(bounded.message.len(), MESSAGE_COPY_LIMIT + 1);
        assert_eq!(bounded.message.last(), Some(&0));
        assert_eq!(request([Some(""), Some("Cancel"), Some("Third")], false).button_count(), 0);
        let one = request([Some("OK"), Some(""), Some("Ignored")], false);
        assert_eq!(one.button_count(), 1);
        assert_eq!(one.button_labels[2], None);
    }

    #[test]
    fn resource_overloads_resolve_message_and_button_strings() {
        let req = MessageBoxRequest::from_text_ids(&Texts, 10, [Some(11), Some(12), None], false);
        assert_eq!(req.message, b"TEXT-10\0");
        assert_eq!(req.button_labels, [Some("TEXT-11".into()), Some("TEXT-12".into()), None]);
        let other = MessageBoxRequest::with_resource_buttons("Literal", &Texts, [Some(21), None, None], true);
        assert_eq!(other.button_labels[0].as_deref(), Some("TEXT-21"));
        assert!(other.preserve);
    }

    #[test]
    fn layout_preserves_standard_and_japanese_picture_geometry() {
        let zero = MessageBoxLayout::new(metrics(), 0, MessageBoxMode::Standard);
        assert_eq!((zero.width, zero.height, zero.x, zero.y), (140, 60, 90, 70));
        assert_eq!((zero.message_x, zero.message_y), (110, 95));
        let one = MessageBoxLayout::new(metrics(), 1, MessageBoxMode::Standard);
        assert_eq!((one.button_width, one.button1_x, one.button_y), (50, 135, 128));
        let three = MessageBoxLayout::new(metrics(), 3, MessageBoxMode::Standard);
        assert_eq!((three.button1_x, three.button2_x, three.button3_x), (100, 170, 137));
        let picture = MessageBoxLayout::new(metrics(), 0, MessageBoxMode::JapanesePicture);
        assert_eq!((picture.width, picture.height, picture.x, picture.y), (130, 110, 95, 45));
    }

    #[test]
    fn three_button_order_and_physical_input_mapping_match_legacy_arrays() {
        let mut dialog = MessageBoxDialog::new(55, request([Some("OK"), Some("No"), Some("Cancel")], false), metrics(), MessageBoxMode::Standard);
        let start = dialog.start();
        assert!(start.operations.contains(&MessageBoxDrawOp::DrawAllButtons {
            physical_order: vec![1, 3, 2], current_button: 0,
        }));
        assert_eq!(dialog.step(false, MessageBoxInput::ButtonCode(0x8002), false).result, Some(2));
        let mut second = MessageBoxDialog::new(55, request([Some("OK"), Some("No"), Some("Cancel")], false), metrics(), MessageBoxMode::Standard);
        second.start();
        assert_eq!(second.step(false, MessageBoxInput::ButtonCode(0x8003), false).result, Some(1));
    }

    #[test]
    fn keyboard_focus_cycles_in_visual_order_and_enter_uses_index_plus_one() {
        let mut dialog = MessageBoxDialog::new(55, request([Some("OK"), Some("No"), Some("Cancel")], false), metrics(), MessageBoxMode::Standard);
        dialog.start();
        let left = dialog.step(false, MessageBoxInput::Left, false);
        assert_eq!(dialog.current_button, 2);
        assert!(left.operations.contains(&MessageBoxDrawOp::TurnButtonOff { button_id: 1 }));
        assert!(left.operations.contains(&MessageBoxDrawOp::TurnButtonOn { button_id: 2 }));
        assert_eq!(dialog.step(false, MessageBoxInput::Return, false).result, Some(2));
    }

    #[test]
    fn escape_selects_last_defined_button_and_preserve_restores_the_region() {
        let mut dialog = MessageBoxDialog::new(55, request([Some("OK"), Some("Cancel"), None], true), metrics(), MessageBoxMode::Standard);
        dialog.start();
        let step = dialog.step(false, MessageBoxInput::Escape, false);
        assert_eq!(step.result, Some(1));
        assert!(step.operations.iter().any(|op| matches!(op, MessageBoxDrawOp::RestoreDialogRect { .. })));
    }

    #[test]
    fn focus_restoration_restores_saved_page_before_redrawing() {
        let mut dialog = MessageBoxDialog::new(55, request([Some("OK"), Some("Cancel"), None], false), metrics(), MessageBoxMode::Standard);
        dialog.start();
        let step = dialog.step(true, MessageBoxInput::None, false);
        let restored = step.operations.iter().position(|op| *op == MessageBoxDrawOp::RestoreSavedVisiblePage).unwrap();
        let redrawn = step.operations.iter().position(|op| matches!(op, MessageBoxDrawOp::DrawDialogBox { .. })).unwrap();
        assert!(restored < redrawn);
        assert!(step.operations.contains(&MessageBoxDrawOp::ClearSurfacesRestored));
    }

    #[test]
    fn no_button_dialog_calls_back_until_host_timer_expires() {
        let mut dialog = MessageBoxDialog::new(55, request([None, None, None], true), metrics(), MessageBoxMode::Standard);
        dialog.start();
        for _ in 0..100 {
            let step = dialog.step(false, MessageBoxInput::None, false);
            assert!(!step.complete);
            assert_eq!(step.operations, vec![MessageBoxDrawOp::InvokeGameCallback]);
        }
        let final_step = dialog.step(false, MessageBoxInput::None, true);
        assert!(final_step.complete);
        assert_eq!(final_step.result, Some(0));
        assert!(final_step.operations.contains(&MessageBoxDrawOp::ClearKeyboard));
        assert!(final_step.operations.iter().any(|op| matches!(op, MessageBoxDrawOp::RestoreDialogRect { .. })));
    }

    #[test]
    fn finished_dialog_is_idempotent_and_cannot_be_flagged_for_redraw() {
        let mut dialog = MessageBoxDialog::new(55, request([Some("OK"), None, None], false), metrics(), MessageBoxMode::Standard);
        dialog.start();
        assert_eq!(dialog.step(false, MessageBoxInput::Return, false).result, Some(0));
        dialog.flag_for_redraw();
        assert!(!dialog.display);
        assert!(dialog.step(false, MessageBoxInput::Escape, false).operations.is_empty());
    }
}
