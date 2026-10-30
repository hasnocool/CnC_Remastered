//! Red Alert WWMessageBox port from REDALERT/MSGBOX.CPP.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection at
//! f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! The modal C++ input loop is exposed as one non-blocking step per host
//! game-loop iteration. The layout, three-button 1-3-2 physical order,
//! keyboard/button-code selection mapping, surface-restoration redraw,
//! optional screen preservation, and the three Process overload conversions
//! are modeled separately from Tiberian Dawn's message-box implementation.
//! Font metrics, localization, drawing, keyboard input and mouse visibility
//! are injected through values/operations for the platform host.

pub const BUTTON_1: i32 = 1;
pub const BUTTON_2: i32 = 2;
pub const BUTTON_3: i32 = 3;
pub const BUTTON_FLAG: u16 = 0x8000;
pub const MESSAGE_BUFFER_SIZE: usize = 511;
pub const MESSAGE_COPY_LIMIT: usize = MESSAGE_BUFFER_SIZE - 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertMessageBoxRequest {
    /// NUL-terminated copy of the C++ 511-byte staging buffer.
    pub message: Vec<u8>,
    pub button_labels: [Option<String>; 3],
    pub preserve: bool,
}

impl RedAlertMessageBoxRequest {
    pub fn from_text(
        message: &[u8],
        button1: Option<&str>,
        button2: Option<&str>,
        button3: Option<&str>,
        preserve: bool,
    ) -> Self {
        let c_end = message.iter().position(|byte| *byte == 0).unwrap_or(message.len());
        let mut staged = message[..c_end.min(MESSAGE_COPY_LIMIT)].to_vec();
        staged.push(0);
        let labels = [button1, button2, button3].map(|value| {
            value.filter(|text| !text.is_empty()).map(str::to_owned)
        });
        Self { message: staged, button_labels: labels, preserve }
    }

    pub fn button_count(&self) -> usize {
        if self.button_labels[0].is_none() { 0 }
        else if self.button_labels[1].is_none() { 1 }
        else if self.button_labels[2].is_none() { 2 }
        else { 3 }
    }
}

/// Text_String adapter used by the int/int and char*/int Process overloads.
/// A missing resource (including TXT_NONE) is represented as None.
pub trait MessageTextResolver {
    fn text_string(&self, text_id: i32) -> Option<String>;
}

impl RedAlertMessageBoxRequest {
    /// Port of Process(int msg, int b1, int b2, int b3, bool preserve).
    pub fn from_resource_ids(
        resolver: &impl MessageTextResolver,
        message_id: i32,
        button_ids: [Option<i32>; 3],
        preserve: bool,
    ) -> Self {
        let message = resolver.text_string(message_id).unwrap_or_default();
        let labels = button_ids.map(|id| id.and_then(|value| resolver.text_string(value)));
        Self::from_text(
            message.as_bytes(),
            labels[0].as_deref(),
            labels[1].as_deref(),
            labels[2].as_deref(),
            preserve,
        )
    }

    /// Port of Process(char const* msg, int b1, int b2, int b3, bool preserve).
    pub fn with_resource_buttons(
        message: &str,
        resolver: &impl MessageTextResolver,
        button_ids: [Option<i32>; 3],
        preserve: bool,
    ) -> Self {
        let labels = button_ids.map(|id| id.and_then(|value| resolver.text_string(value)));
        Self::from_text(
            message.as_bytes(),
            labels[0].as_deref(),
            labels[1].as_deref(),
            labels[2].as_deref(),
            preserve,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormattedMessage {
    pub width: i32,
    pub height: i32,
    pub lines: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageBoxMetrics {
    pub seen_width: i32,
    pub seen_height: i32,
    pub resolution_factor: i32,
    pub formatted: FormattedMessage,
    pub button1_text_width: i32,
    pub button2_text_width: i32,
    /// Actual TextButtonClass width for button 3, which is centered separately.
    pub button3_width: i32,
    pub font_height: i32,
    pub font_y_spacing: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreserveRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageBoxLayout {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub button_height: i32,
    pub shared_button_width: i32,
    pub button1_x: i32,
    pub button2_x: i32,
    pub button3_x: i32,
    pub button_y: i32,
    pub message_x: i32,
    pub message_y: i32,
    pub message_centered: bool,
    pub format_max_width: i32,
    pub preserve_rect: PreserveRect,
}

impl MessageBoxLayout {
    pub fn new(metrics: MessageBoxMetrics, button_count: usize, windows_build: bool) -> Self {
        let rf = metrics.resolution_factor.max(1);
        let first_width = (metrics.button1_text_width + 8 * rf).max(30 * rf);
        let shared_button_width = if button_count >= 2 {
            first_width.max(metrics.button2_text_width + 8 * rf)
        } else {
            first_width
        };
        let button_height = metrics.font_height + metrics.font_y_spacing + 2 * rf;
        let width = metrics.formatted.width.max(90 * rf) + 40 * rf;
        let height = metrics.formatted.height + if button_count == 0 { 40 * rf } else { 60 * rf };
        let x = (metrics.seen_width - width) / 2;
        let y = (metrics.seen_height - height) / 2;
        let message_centered = metrics.formatted.lines == 1;
        let message_x = if message_centered { x + width / 2 } else { x + 20 * rf };
        let preserve_rect = if windows_build {
            PreserveRect { x, y, width, height }
        } else {
            let preserve_x = (x - 4).max(0);
            let preserve_y = (y - 4).max(0);
            PreserveRect {
                x: preserve_x,
                y: preserve_y,
                width: (width + 8).min(320 - preserve_x).max(0),
                height: (height + 8).min(200 - preserve_y).max(0),
            }
        };
        Self {
            x,
            y,
            width,
            height,
            button_height,
            shared_button_width,
            button1_x: x + if button_count == 1 {
                (width - shared_button_width) >> 1
            } else {
                20 * rf
            },
            button2_x: x + width - (shared_button_width + 20 * rf),
            button3_x: x + ((width - metrics.button3_width) >> 1),
            button_y: y + height - (button_height + 15 * rf),
            message_x,
            message_y: y + 20 * rf,
            message_centered,
            format_max_width: 255 * rf,
            preserve_rect,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageBoxPlatform {
    pub windows_build: bool,
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
    CapturePreservedRect(PreserveRect),
    DrawDialogBox { x: i32, y: i32, width: i32, height: i32 },
    DrawCaption { text_id: i32, x: i32, y: i32, width: i32 },
    DrawMessage {
        text: Vec<u8>,
        x: i32,
        y: i32,
        centered: bool,
        format_flags: u8,
    },
    DrawButtons { physical_order: Vec<i32>, current_button: usize },
    RestoreSavedVisiblePage,
    ClearSurfacesRestored,
    InvokeGameCallback,
    TurnButtonOff { button_id: i32 },
    ClearButtonPressed { button_id: i32 },
    FlagButtonForRedraw { button_id: i32 },
    TurnButtonOn { button_id: i32 },
    SetButtonPressed { button_id: i32 },
    DrawButtonsPressed,
    ClearKeyboard,
    RestorePreservedRect(PreserveRect),
    ShowMouse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageBoxStep {
    pub operations: Vec<MessageBoxDrawOp>,
    pub complete: bool,
    pub result: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertMessageBox {
    pub caption_id: i32,
    pub request: RedAlertMessageBoxRequest,
    pub layout: MessageBoxLayout,
    pub platform: MessageBoxPlatform,
    pub current_button: usize,
    pub started: bool,
    pub display: bool,
    pub complete: bool,
    pub result: Option<i32>,
    real_values: [i32; 5],
}

impl RedAlertMessageBox {
    pub fn new(
        caption_id: i32,
        request: RedAlertMessageBoxRequest,
        metrics: MessageBoxMetrics,
        platform: MessageBoxPlatform,
    ) -> Self {
        let count = request.button_count();
        // The C++ realval array stores actual button IDs in the visual order
        // [BUTTON_1, BUTTON_3, BUTTON_2] for three-button dialogs.
        let real_values = match count {
            0 => [0; 5],
            1 => [BUTTON_1, 0, 0, 0, 0],
            2 => [BUTTON_1, BUTTON_2, 0, 0, 0],
            _ => [BUTTON_1, BUTTON_3, BUTTON_2, 0, 0],
        };
        Self {
            caption_id,
            layout: MessageBoxLayout::new(metrics, count, platform.windows_build),
            request,
            platform,
            current_button: 0,
            started: false,
            display: false,
            complete: false,
            result: None,
            real_values,
        }
    }

    pub fn button_count(&self) -> usize { self.request.button_count() }

    fn physical_order(&self) -> Vec<i32> {
        match self.button_count() {
            0 => vec![],
            1 => vec![BUTTON_1],
            2 => vec![BUTTON_1, BUTTON_2],
            _ => vec![BUTTON_1, BUTTON_3, BUTTON_2],
        }
    }

    fn draw_operations(&self) -> Vec<MessageBoxDrawOp> {
        let mut ops = vec![MessageBoxDrawOp::HideMouse];
        if self.request.preserve {
            ops.push(MessageBoxDrawOp::CapturePreservedRect(self.layout.preserve_rect));
        }
        ops.push(MessageBoxDrawOp::DrawDialogBox {
            x: self.layout.x,
            y: self.layout.y,
            width: self.layout.width,
            height: self.layout.height,
        });
        ops.push(MessageBoxDrawOp::DrawCaption {
            text_id: self.caption_id,
            x: self.layout.x,
            y: self.layout.y,
            width: self.layout.width,
        });
        // TPF_TEXT plus TPF_CENTER for one-line strings.
        ops.push(MessageBoxDrawOp::DrawMessage {
            text: self.request.message.clone(),
            x: self.layout.message_x,
            y: self.layout.message_y,
            centered: self.layout.message_centered,
            format_flags: if self.layout.message_centered { 0b11 } else { 0b01 },
        });
        if self.button_count() > 0 {
            ops.push(MessageBoxDrawOp::DrawButtons {
                physical_order: self.physical_order(),
                current_button: self.current_button,
            });
        }
        ops.push(MessageBoxDrawOp::ShowMouse);
        ops
    }

    pub fn start(&mut self) -> MessageBoxStep {
        if self.started { return self.snapshot(Vec::new()); }
        self.started = true;
        let mut ops = vec![MessageBoxDrawOp::SetLogicPageToSeen];
        if self.platform.windows_build {
            ops.push(MessageBoxDrawOp::SaveVisiblePage);
        }
        ops.extend(self.draw_operations());
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
            ops.push(MessageBoxDrawOp::RestorePreservedRect(self.layout.preserve_rect));
            ops.push(MessageBoxDrawOp::ShowMouse);
        }
    }

    fn button_id_at(&self, index: usize) -> Option<i32> {
        (index < self.button_count()).then_some(self.real_values[index])
    }

    fn selected_button_for_input(&self, input: MessageBoxInput) -> Option<i32> {
        let count = self.button_count();
        match input {
            MessageBoxInput::Escape if count > 0 => Some(self.real_values[count - 1]),
            MessageBoxInput::Return if count > 0 => self.button_id_at(self.current_button),
            MessageBoxInput::ButtonCode(code) if code == (BUTTON_1 as u16 | BUTTON_FLAG) => {
                Some(self.real_values[0])
            }
            MessageBoxInput::ButtonCode(code) if code == (BUTTON_2 as u16 | BUTTON_FLAG) && count > 1 => {
                Some(if count > 2 { self.real_values[2] } else { self.real_values[1] })
            }
            MessageBoxInput::ButtonCode(code) if code == (BUTTON_3 as u16 | BUTTON_FLAG) && count > 1 => {
                Some(self.real_values[1])
            }
            _ => None,
        }
    }

    fn result_for_selection(selection: i32) -> Option<i32> {
        match selection {
            BUTTON_1 => Some(0),
            BUTTON_2 => Some(1),
            BUTTON_3 => Some(2),
            _ => None,
        }
    }

    fn focused_button_id(&self, index: usize) -> Option<i32> {
        self.button_id_at(index)
    }

    fn redraw_selected_button(&self, selection: i32, ops: &mut Vec<MessageBoxDrawOp>) {
        for button_id in [BUTTON_1, BUTTON_2, BUTTON_3] {
            if self.physical_order().contains(&button_id) {
                ops.push(MessageBoxDrawOp::TurnButtonOff { button_id });
                ops.push(MessageBoxDrawOp::ClearButtonPressed { button_id });
            }
        }
        if self.physical_order().contains(&selection) {
            ops.push(MessageBoxDrawOp::TurnButtonOn { button_id: selection });
            ops.push(MessageBoxDrawOp::SetButtonPressed { button_id: selection });
        }
        ops.push(MessageBoxDrawOp::HideMouse);
        ops.push(MessageBoxDrawOp::DrawButtonsPressed);
        ops.push(MessageBoxDrawOp::ShowMouse);
    }

    /// One callback/input iteration of the original modal loop. On the
    /// no-button path the C++ code immediately clears the keyboard and skips
    /// the modal loop; its retval is uninitialized, so Rust deterministically
    /// returns zero rather than reproducing undefined behavior.
    pub fn step(
        &mut self,
        surfaces_restored: bool,
        input: MessageBoxInput,
    ) -> MessageBoxStep {
        if self.complete { return self.snapshot(Vec::new()); }
        if !self.started { return self.start(); }

        let mut ops = Vec::new();
        if self.button_count() == 0 {
            ops.push(MessageBoxDrawOp::ClearKeyboard);
            self.finish(0, &mut ops);
            return self.snapshot(ops);
        }

        if self.platform.windows_build && surfaces_restored {
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
            MessageBoxInput::Left | MessageBoxInput::Right if self.button_count() > 1 => {
                if let Some(old_button) = self.focused_button_id(self.current_button) {
                    ops.push(MessageBoxDrawOp::TurnButtonOff { button_id: old_button });
                    ops.push(MessageBoxDrawOp::FlagButtonForRedraw { button_id: old_button });
                }
                let count = self.button_count();
                self.current_button = if input == MessageBoxInput::Left {
                    if self.current_button == 0 { count - 1 } else { self.current_button - 1 }
                } else {
                    (self.current_button + 1) % count
                };
                if let Some(new_button) = self.focused_button_id(self.current_button) {
                    ops.push(MessageBoxDrawOp::TurnButtonOn { button_id: new_button });
                    ops.push(MessageBoxDrawOp::FlagButtonForRedraw { button_id: new_button });
                }
            }
            _ => {
                if let Some(selection) = self.selected_button_for_input(input) {
                    if let Some(result) = Self::result_for_selection(selection) {
                        self.redraw_selected_button(selection, &mut ops);
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
        fn text_string(&self, id: i32) -> Option<String> {
            (id >= 0).then(|| format!("TEXT-{id}"))
        }
    }

    fn metrics(lines: usize) -> MessageBoxMetrics {
        MessageBoxMetrics {
            seen_width: 320,
            seen_height: 200,
            resolution_factor: 1,
            formatted: FormattedMessage { width: 100, height: 30, lines },
            button1_text_width: 42,
            button2_text_width: 35,
            button3_width: 45,
            font_height: 8,
            font_y_spacing: 2,
        }
    }

    fn request(buttons: [Option<&str>; 3], preserve: bool) -> RedAlertMessageBoxRequest {
        RedAlertMessageBoxRequest::from_text(b"Test message", buttons[0], buttons[1], buttons[2], preserve)
    }

    fn dialog(buttons: [Option<&str>; 3], preserve: bool) -> RedAlertMessageBox {
        RedAlertMessageBox::new(
            55,
            request(buttons, preserve),
            metrics(2),
            MessageBoxPlatform { windows_build: true },
        )
    }

    #[test]
    fn c_string_staging_uses_510_bytes_plus_terminator() {
        let long = vec![b'A'; 700];
        let staged = RedAlertMessageBoxRequest::from_text(&long, Some("OK"), None, None, false);
        assert_eq!(staged.message.len(), MESSAGE_BUFFER_SIZE);
        assert_eq!(staged.message.last(), Some(&0));
        assert_eq!(staged.message[..MESSAGE_COPY_LIMIT].len(), 510);
    }

    #[test]
    fn empty_button_strings_are_null_and_button_count_requires_button_one() {
        assert_eq!(request([Some(""), Some("Cancel"), Some("Third")], false).button_count(), 0);
        assert_eq!(request([Some("OK"), Some(""), Some("Ignored")], false).button_count(), 1);
        assert_eq!(request([Some("OK"), Some("No"), Some("")], false).button_count(), 2);
    }

    #[test]
    fn resource_process_overloads_resolve_string_and_numeric_button_ids() {
        let numeric = RedAlertMessageBoxRequest::from_resource_ids(&Texts, 10, [Some(11), Some(12), Some(-1)], false);
        assert_eq!(numeric.message, b"TEXT-10\0");
        assert_eq!(numeric.button_labels, [Some("TEXT-11".into()), Some("TEXT-12".into()), None]);
        let mixed = RedAlertMessageBoxRequest::with_resource_buttons("Literal", &Texts, [Some(21), None, None], true);
        assert_eq!(mixed.message, b"Literal\0");
        assert_eq!(mixed.button_labels[0].as_deref(), Some("TEXT-21"));
    }

    #[test]
    fn layout_matches_red_alert_size_margins_and_one_line_centering() {
        let one_line = MessageBoxLayout::new(metrics(1), 1, true);
        assert_eq!((one_line.width, one_line.height, one_line.x, one_line.y), (140, 90, 90, 55));
        assert_eq!((one_line.message_x, one_line.message_y, one_line.message_centered), (160, 75, true));
        assert_eq!(one_line.button1_x, 135);
        assert_eq!(one_line.button_y, 112);

        let three = MessageBoxLayout::new(metrics(2), 3, false);
        assert_eq!((three.button1_x, three.button2_x, three.button3_x), (110, 160, 137));
        assert_eq!(three.preserve_rect, PreserveRect { x: 86, y: 51, width: 148, height: 98 });
    }

    #[test]
    fn three_button_visual_order_is_one_three_two() {
        let mut dialog = dialog([Some("OK"), Some("No"), Some("Cancel")], false);
        let start = dialog.start();
        assert!(start.operations.contains(&MessageBoxDrawOp::DrawButtons {
            physical_order: vec![BUTTON_1, BUTTON_3, BUTTON_2],
            current_button: 0,
        }));
    }

    #[test]
    fn button_code_mapping_matches_realval_array_for_three_buttons() {
        let mut second = dialog([Some("OK"), Some("No"), Some("Cancel")], false);
        second.start();
        assert_eq!(second.step(false, MessageBoxInput::ButtonCode(0x8002)).result, Some(1));

        let mut third = dialog([Some("OK"), Some("No"), Some("Cancel")], false);
        third.start();
        assert_eq!(third.step(false, MessageBoxInput::ButtonCode(0x8003)).result, Some(2));
    }

    #[test]
    fn escape_selects_realval_last_entry_and_return_selects_current_visual_item() {
        let mut escape = dialog([Some("OK"), Some("No"), Some("Cancel")], true);
        escape.start();
        let step = escape.step(false, MessageBoxInput::Escape);
        assert_eq!(step.result, Some(1));
        assert!(step.operations.iter().any(|op| matches!(op, MessageBoxDrawOp::RestorePreservedRect(_))));

        let mut enter = dialog([Some("OK"), Some("No"), Some("Cancel")], false);
        enter.start();
        enter.step(false, MessageBoxInput::Right);
        enter.step(false, MessageBoxInput::Right);
        assert_eq!(enter.step(false, MessageBoxInput::Return).result, Some(1));
    }

    #[test]
    fn focus_restoration_redraws_saved_visible_page_before_dialog() {
        let mut dialog = dialog([Some("OK"), Some("Cancel"), None], false);
        dialog.start();
        let step = dialog.step(true, MessageBoxInput::None);
        let restore = step.operations.iter().position(|op| *op == MessageBoxDrawOp::RestoreSavedVisiblePage).unwrap();
        let redraw = step.operations.iter().position(|op| matches!(op, MessageBoxDrawOp::DrawDialogBox { .. })).unwrap();
        assert!(restore < redraw);
        assert!(step.operations.contains(&MessageBoxDrawOp::ClearSurfacesRestored));
    }

    #[test]
    fn no_button_dialog_clears_keyboard_without_entering_a_modal_loop() {
        let mut dialog = dialog([None, None, None], true);
        dialog.start();
        let step = dialog.step(false, MessageBoxInput::None);
        assert!(step.complete);
        assert_eq!(step.result, Some(0));
        assert!(step.operations.contains(&MessageBoxDrawOp::ClearKeyboard));
        assert!(step.operations.iter().any(|op| matches!(op, MessageBoxDrawOp::RestorePreservedRect(_))));
        assert!(!step.operations.contains(&MessageBoxDrawOp::InvokeGameCallback));
    }

    #[test]
    fn finished_dialog_is_idempotent_and_redraw_requests_are_ignored() {
        let mut dialog = dialog([Some("OK"), None, None], false);
        dialog.start();
        assert_eq!(dialog.step(false, MessageBoxInput::Return).result, Some(0));
        dialog.flag_for_redraw();
        assert!(!dialog.display);
        assert!(dialog.step(false, MessageBoxInput::Escape).operations.is_empty());
    }

    #[test]
    fn non_windows_preservation_uses_clipped_four_pixel_border() {
        let layout = MessageBoxLayout::new(metrics(2), 2, false);
        assert_eq!(layout.preserve_rect.x, (layout.x - 4).max(0));
        assert_eq!(layout.preserve_rect.y, (layout.y - 4).max(0));
        assert_eq!(layout.preserve_rect.width, (layout.width + 8).min(320 - layout.preserve_rect.x));
        assert_eq!(layout.preserve_rect.height, (layout.height + 8).min(200 - layout.preserve_rect.y));
    }
}
