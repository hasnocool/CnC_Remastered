//! TextButtonClass from REDALERT/TEXTBTN.CPP and TIBERIANDAWN/TEXTBTN.CPP.
//!
//! Sizing, localized text assignment, ToggleClass state and draw gating are
//! shared. Red Alert prints using its active remap color scheme and ordinary
//! box styles; Tiberian Dawn supports the six-point green-gradient style and
//! pressed/on color differences. UI/platform rendering is supplied by host.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextButtonVariant {
    RedAlert,
    TiberianDawn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextButtonBoxStyle {
    Down,
    Raised,
    DisabledRaised,
    GreenDown,
    GreenRaised,
    GreenDisabledRaised,
    WolApiDisabledBox,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextButtonForeground {
    RedAlertRemapScheme,
    PaletteColor(i32),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextButtonState {
    pub disabled: bool,
    pub pressed: bool,
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextButtonPrintFlags {
    pub six_point_gradient: u32,
    pub no_shadow: u32,
    pub use_gradient_palette: u32,
    pub bright_color: u32,
    pub medium_color: u32,
    pub center: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextButtonPalette {
    pub black: i32,
    pub white: i32,
    pub light_gray: i32,
    pub dark_gray: i32,
    pub red: i32,
    pub cc_green: i32,
}

pub trait TextButtonHost {
    /// String_Pixel_Width, FontHeight and FontYSpacing come from the active
    /// font; String_Pixel_Width is called only when the source requests sizing.
    fn string_pixel_width(&self, text: &str) -> i32;
    fn font_height(&self) -> i32;
    fn font_y_spacing(&self) -> i32;

    /// Resolves Text_String(text). The returned owned string replaces the ID;
    /// the legacy TextButtonClass does not retain the integer resource ID.
    fn text_string(&self, text_id: i32) -> Option<String>;

    /// Tiberian Dawn invokes Fancy_Text_Print(TXT_NONE, ...) before measuring
    /// localized constructor text. Red Alert has that call commented out.
    fn prepare_tiberian_dawn_text_measurement(&mut self, _print_flags: u32) {}

    /// Parent ControlClass::Draw_Me redraw gate.
    fn control_draw(&mut self, forced: bool, bounds: (i32, i32, i32, i32)) -> bool;
    fn is_seen_buffer_logic_page(&self) -> bool;
    fn conditional_hide_mouse(&mut self, left: i32, top: i32, right: i32, bottom: i32);
    fn conditional_show_mouse(&mut self);

    /// Draw_Rect on LogicPage for the optional black outer border.
    fn draw_rect(&mut self, left: i32, top: i32, right: i32, bottom: i32, color: i32);
    fn draw_box(&mut self, x: i32, y: i32, width: i32, height: i32, style: TextButtonBoxStyle, filled: bool);
    fn palette(&self) -> TextButtonPalette;
    fn print_flags(&self) -> TextButtonPrintFlags;
    fn wolapi_integration_enabled(&self) -> bool { false }

    fn fancy_text_print(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        foreground: TextButtonForeground,
        background: i32,
        flags: u32,
    );
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextButtonController {
    pub variant: TextButtonVariant,
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    string: Option<String>,
    print_flags: u32,
    black_border: bool,
    state: TextButtonState,
    redraw_requested: bool,
}

impl TextButtonController {
    pub const TXT_NONE: i32 = -1;

    /// Default TextButtonClass constructor: ToggleClass(0,0,0,0,0),
    /// no string, no black border and TPF_8POINT print flags.
    pub fn new(variant: TextButtonVariant) -> Self {
        Self {
            variant,
            id: 0,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            string: None,
            print_flags: 0,
            black_border: false,
            state: TextButtonState::default(),
            redraw_requested: false,
        }
    }

    /// String constructor. A width or height of -1 is text-sized independently.
    pub fn with_string(
        host: &impl TextButtonHost,
        variant: TextButtonVariant,
        id: u32,
        text: Option<String>,
        print_flags: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        black_border: bool,
    ) -> Self {
        let mut button = Self {
            variant,
            id,
            x,
            y,
            width,
            height,
            string: text,
            print_flags,
            black_border,
            state: TextButtonState::default(),
            redraw_requested: false,
        };
        button.resize_constructor(host, false);
        button
    }

    /// Resource-ID constructor. The source first calls Set_Text(text), then
    /// measures the resolved String when width/height are automatic.
    pub fn with_text_id(
        host: &mut impl TextButtonHost,
        variant: TextButtonVariant,
        id: u32,
        text_id: i32,
        print_flags: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        black_border: bool,
    ) -> Self {
        let mut button = Self {
            variant,
            id,
            x,
            y,
            width,
            height,
            string: None,
            print_flags,
            black_border,
            state: TextButtonState::default(),
            redraw_requested: false,
        };
        button.set_text_id(host, text_id, false);
        if (button.width == -1 || button.height == -1)
            && variant == TextButtonVariant::TiberianDawn
        {
            host.prepare_tiberian_dawn_text_measurement(print_flags);
        }
        button.resize_constructor(host, true);
        button
    }

    fn resize_constructor(&mut self, host: &impl TextButtonHost, localized_constructor: bool) {
        if self.width == -1 || self.height == -1 {
            if localized_constructor && self.variant == TextButtonVariant::TiberianDawn {
                // The call is performed by the host before this pure geometry step.
                // (The mutable host hook is invoked in with_text_id below.)
            }
            if self.width == -1 {
                self.width = host.string_pixel_width(self.string.as_deref().unwrap_or("")) + 8;
            }
            if self.height == -1 {
                self.height = host.font_height() + host.font_y_spacing() + 2;
            }
        }
    }

    pub const fn text(&self) -> Option<&String> { self.string.as_ref() }
    pub const fn state(&self) -> TextButtonState { self.state }
    pub const fn print_style(&self) -> u32 { self.print_flags }
    pub const fn has_black_border(&self) -> bool { self.black_border }
    pub const fn redraw_requested(&self) -> bool { self.redraw_requested }

    pub fn set_state(&mut self, state: TextButtonState) {
        if self.state != state {
            self.state = state;
            self.redraw_requested = true;
        }
    }

    pub fn set_style(&mut self, style: u32) {
        self.print_flags = style;
        self.redraw_requested = true;
    }

    /// Set_Text(char const*, resize): stores the pointer/string, flags redraw,
    /// and optionally recalculates both dimensions if String is non-null.
    pub fn set_text(
        &mut self,
        host: &impl TextButtonHost,
        text: Option<String>,
        resize: bool,
    ) {
        self.string = text;
        self.redraw_requested = true;
        if resize {
            if let Some(string) = self.string.as_deref() {
                self.width = host.string_pixel_width(string) + 8;
                self.height = host.font_height() + host.font_y_spacing() + 2;
            }
        }
    }

    /// Set_Text(int text): TXT_NONE leaves the existing String unchanged.
    pub fn set_text_id(
        &mut self,
        host: &impl TextButtonHost,
        text_id: i32,
        resize: bool,
    ) -> bool {
        if text_id == Self::TXT_NONE { return false; }
        let Some(text) = host.text_string(text_id) else { return false };
        self.set_text(host, Some(text), resize);
        true
    }

    pub fn set_black_border(&mut self, enabled: bool) {
        if self.black_border != enabled {
            self.black_border = enabled;
            self.redraw_requested = true;
        }
    }

    pub fn set_bounds(&mut self, x: i32, y: i32, width: i32, height: i32) {
        if (self.x, self.y, self.width, self.height) != (x, y, width, height) {
            self.x = x;
            self.y = y;
            self.width = width;
            self.height = height;
            self.redraw_requested = true;
        }
    }

    /// TextButtonClass::Draw_Me: draw only when parent ControlClass approves;
    /// hide pointer, draw background + text, restore pointer and return true.
    pub fn draw_me(&mut self, host: &mut impl TextButtonHost, forced: bool) -> bool {
        let bounds = (self.x, self.y, self.x + self.width, self.y + self.height);
        if !host.control_draw(forced, bounds) { return false; }

        if host.is_seen_buffer_logic_page() {
            let (right, bottom) = match self.variant {
                TextButtonVariant::RedAlert => (self.x + self.width - 1, self.y + self.height - 1),
                TextButtonVariant::TiberianDawn => (self.x + self.width, self.y + self.height),
            };
            host.conditional_hide_mouse(self.x, self.y, right, bottom);
        }

        self.draw_background(host);
        self.draw_text(host);

        if host.is_seen_buffer_logic_page() {
            host.conditional_show_mouse();
        }
        self.redraw_requested = false;
        true
    }

    pub fn draw_background(&self, host: &mut impl TextButtonHost) {
        let palette = host.palette();
        if self.black_border {
            host.draw_rect(
                self.x - 1,
                self.y - 1,
                self.x + self.width + 2,
                self.y + self.height + 2,
                palette.black,
            );
        }

        let gradient = self.print_flags & host.print_flags().six_point_gradient != 0;
        let style = match self.variant {
            TextButtonVariant::RedAlert => {
                if self.state.disabled {
                    if host.wolapi_integration_enabled() {
                        TextButtonBoxStyle::WolApiDisabledBox
                    } else {
                        TextButtonBoxStyle::DisabledRaised
                    }
                } else if self.state.pressed {
                    TextButtonBoxStyle::Down
                } else {
                    TextButtonBoxStyle::Raised
                }
            }
            TextButtonVariant::TiberianDawn if gradient => {
                if self.state.disabled {
                    TextButtonBoxStyle::GreenDisabledRaised
                } else if self.state.pressed {
                    TextButtonBoxStyle::GreenDown
                } else {
                    TextButtonBoxStyle::GreenRaised
                }
            }
            TextButtonVariant::TiberianDawn => {
                if self.state.disabled {
                    TextButtonBoxStyle::DisabledRaised
                } else if self.state.pressed {
                    TextButtonBoxStyle::Down
                } else {
                    TextButtonBoxStyle::Raised
                }
            }
        };
        host.draw_box(self.x, self.y, self.width, self.height, style, true);
    }

    pub fn draw_text(&self, host: &mut impl TextButtonHost) {
        let Some(text) = self.string.as_deref() else { return };
        let palette = host.palette();
        let bits = host.print_flags();
        let text_x = self.x + (self.width >> 1) - 1;
        let text_y = self.y + 1;

        match self.variant {
            TextButtonVariant::RedAlert => {
                let mut flags = if self.state.disabled {
                    0
                } else if self.state.pressed || self.state.on {
                    bits.use_gradient_palette | bits.bright_color
                } else {
                    bits.use_gradient_palette | bits.medium_color
                };
                flags |= self.print_flags | bits.center;
                host.fancy_text_print(
                    text,
                    text_x,
                    text_y,
                    TextButtonForeground::RedAlertRemapScheme,
                    palette.black,
                    flags,
                );
            }
            TextButtonVariant::TiberianDawn if self.print_flags & bits.six_point_gradient != 0 => {
                let extra_flags = if self.state.disabled {
                    0
                } else if self.state.pressed || self.state.on {
                    bits.use_gradient_palette | bits.bright_color
                } else {
                    bits.use_gradient_palette | bits.medium_color
                };
                host.fancy_text_print(
                    text,
                    text_x,
                    text_y,
                    TextButtonForeground::PaletteColor(palette.cc_green),
                    palette.black,
                    self.print_flags | extra_flags | bits.center,
                );
            }
            TextButtonVariant::TiberianDawn => {
                let color = if self.state.on {
                    palette.red
                } else if self.state.disabled {
                    palette.light_gray
                } else if self.state.pressed {
                    if self.print_flags & bits.no_shadow != 0 {
                        palette.dark_gray
                    } else {
                        palette.light_gray
                    }
                } else {
                    palette.white
                };
                host.fancy_text_print(
                    text,
                    text_x,
                    text_y,
                    TextButtonForeground::PaletteColor(color),
                    palette.black,
                    self.print_flags | bits.center,
                );
            }
        }
    }

    pub fn consume_redraw_request(&mut self) -> bool {
        let pending = self.redraw_requested;
        self.redraw_requested = false;
        pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeHost {
        text_width: i32,
        font_height: i32,
        font_spacing: i32,
        localized: Option<String>,
        prepare_calls: usize,
        draw_gate: bool,
        seen: bool,
        hidden: Vec<(i32, i32, i32, i32)>,
        shown: usize,
        rects: Vec<(i32, i32, i32, i32, i32)>,
        boxes: Vec<(i32, i32, i32, i32, TextButtonBoxStyle, bool)>,
        text_calls: Vec<(String, i32, i32, TextButtonForeground, i32, u32)>,
    }

    impl TextButtonHost for FakeHost {
        fn string_pixel_width(&self, text: &str) -> i32 {
            if self.text_width != 0 { self.text_width } else { text.len() as i32 * 8 }
        }
        fn font_height(&self) -> i32 { self.font_height.max(8) }
        fn font_y_spacing(&self) -> i32 { self.font_spacing }
        fn text_string(&self, _id: i32) -> Option<String> { self.localized.clone() }
        fn prepare_tiberian_dawn_text_measurement(&mut self, _flags: u32) { self.prepare_calls += 1; }
        fn control_draw(&mut self, _forced: bool, _bounds: (i32, i32, i32, i32)) -> bool { self.draw_gate }
        fn is_seen_buffer_logic_page(&self) -> bool { self.seen }
        fn conditional_hide_mouse(&mut self, l: i32, t: i32, r: i32, b: i32) {
            self.hidden.push((l, t, r, b));
        }
        fn conditional_show_mouse(&mut self) { self.shown += 1; }
        fn draw_rect(&mut self, l: i32, t: i32, r: i32, b: i32, c: i32) { self.rects.push((l, t, r, b, c)); }
        fn draw_box(&mut self, x: i32, y: i32, w: i32, h: i32, s: TextButtonBoxStyle, f: bool) {
            self.boxes.push((x, y, w, h, s, f));
        }
        fn palette(&self) -> TextButtonPalette {
            TextButtonPalette { black: 0, white: 1, light_gray: 2, dark_gray: 3, red: 4, cc_green: 5 }
        }
        fn wolapi_integration_enabled(&self) -> bool { false }
        fn print_flags(&self) -> TextButtonPrintFlags {
            TextButtonPrintFlags {
                six_point_gradient: 1,
                no_shadow: 2,
                use_gradient_palette: 4,
                bright_color: 8,
                medium_color: 16,
                center: 32,
            }
        }
        fn fancy_text_print(
            &mut self, text: &str, x: i32, y: i32, fg: TextButtonForeground, bg: i32, flags: u32,
        ) {
            self.text_calls.push((text.to_string(), x, y, fg, bg, flags));
        }
    }

    #[test]
    fn string_constructor_sizes_only_dimensions_marked_minus_one() {
        let host = FakeHost { text_width: 40, font_height: 10, font_spacing: 2, ..FakeHost::default() };
        let ra = TextButtonController::with_string(
            &host, TextButtonVariant::RedAlert, 4, Some("test".into()), 0, 10, 20, -1, -1, true,
        );
        assert_eq!((ra.width, ra.height), (48, 14));
        assert!(ra.has_black_border());
    }

    #[test]
    fn text_id_constructor_resolves_string_and_none_preserves_previous_text() {
        let mut host = FakeHost {
            localized: Some("Start".into()),
            text_width: 32,
            font_height: 8,
            font_spacing: 1,
            ..FakeHost::default()
        };
        let mut button = TextButtonController::with_text_id(
            &mut host, TextButtonVariant::TiberianDawn, 9, 12, 0, 0, 0, -1, -1, false,
        );
        assert_eq!(button.text().map(String::as_str), Some("Start"));
        assert!(!button.set_text_id(&host, TextButtonController::TXT_NONE, false));
        assert_eq!(button.text().map(String::as_str), Some("Start"));
        assert_eq!((button.width, button.height), (40, 11));
    }

    #[test]
    fn setting_text_resizes_and_flags_redraw_only_when_string_present() {
        let host = FakeHost { text_width: 25, font_height: 9, font_spacing: 2, ..FakeHost::default() };
        let mut button = TextButtonController::new(TextButtonVariant::RedAlert);
        button.set_text(&host, Some("abc".into()), true);
        assert_eq!((button.width, button.height), (33, 13));
        assert!(button.redraw_requested());
        button.consume_redraw_request();
        button.set_text(&host, None, true);
        assert_eq!((button.width, button.height), (33, 13));
    }

    #[test]
    fn red_alert_background_uses_standard_styles_and_remap_text_scheme() {
        let host = FakeHost::default();
        let mut button = TextButtonController::with_string(
            &host, TextButtonVariant::RedAlert, 1, Some("OK".into()), 0, 4, 5, 20, 10, true,
        );
        button.set_state(TextButtonState { disabled: false, pressed: true, on: false });
        let mut draw_host = FakeHost::default();
        button.draw_background(&mut draw_host);
        assert_eq!(draw_host.rects, vec![(3, 4, 26, 17, 0)]);
        assert_eq!(draw_host.boxes[0].4, TextButtonBoxStyle::Down);
        button.draw_text(&mut draw_host);
        assert_eq!(draw_host.text_calls[0].3, TextButtonForeground::RedAlertRemapScheme);
    }

    #[test]
    fn tiberian_dawn_gradient_and_plain_text_paths_choose_variant_colors() {
        let host = FakeHost::default();
        let mut button = TextButtonController::with_string(
            &host, TextButtonVariant::TiberianDawn, 1, Some("Launch".into()), 1, 0, 0, 40, 14, false,
        );
        button.set_state(TextButtonState { disabled: false, pressed: true, on: false });
        let mut draw_host = FakeHost::default();
        button.draw_background(&mut draw_host);
        assert_eq!(draw_host.boxes[0].4, TextButtonBoxStyle::GreenDown);
        button.draw_text(&mut draw_host);
        assert_eq!(draw_host.text_calls[0].3, TextButtonForeground::PaletteColor(5));
        assert_ne!(draw_host.text_calls[0].5 & 8, 0);

        button.set_style(0);
        button.set_state(TextButtonState { disabled: false, pressed: true, on: false });
        button.draw_text(&mut draw_host);
        assert_eq!(draw_host.text_calls.last().unwrap().3, TextButtonForeground::PaletteColor(2));
    }

    #[test]
    fn draw_gate_and_seen_buffer_mouse_guard_match_source() {
        let host = FakeHost::default();
        let mut button = TextButtonController::with_string(
            &host, TextButtonVariant::TiberianDawn, 2, Some("Go".into()), 0, 1, 2, 10, 8, false,
        );
        let mut draw_host = FakeHost { draw_gate: true, seen: true, ..FakeHost::default() };
        assert!(button.draw_me(&mut draw_host, false));
        assert_eq!(draw_host.hidden, vec![(1, 2, 11, 10)]);
        assert_eq!(draw_host.shown, 1);
        assert_eq!(draw_host.boxes.len(), 1);
        assert_eq!(draw_host.text_calls.len(), 1);
    }
}
