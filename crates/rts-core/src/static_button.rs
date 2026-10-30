//! Red Alert StaticButtonClass from REDALERT/STATBTN.CPP.
//!
//! The constructor's unsigned ID argument is intentionally ignored by the
//! upstream source. The host supplies gadget redraw, surface snapshot and
//! text rendering operations; owned text/background state lives here.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticButtonRect {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticButtonTextFlags {
    pub center: u32,
    pub right: u32,
    pub default_eight_point: u32,
}

pub const STATIC_BUTTON_BLACK_BACKGROUND: i32 = 0;

pub trait StaticButtonHost {
    fn string_pixel_width(&self, text: &str) -> i32;
    fn font_height(&self) -> i32;
    fn font_y_spacing(&self) -> i32;

    fn gadget_draw_me(&mut self, forced: bool) -> bool;
    fn logic_page_is_seen_buffer(&self) -> bool;
    fn conditional_hide_mouse(&mut self, rect: StaticButtonRect);
    fn conditional_show_mouse(&mut self);

    fn capture_page_region(&mut self, rect: StaticButtonRect, output: &mut [u8]);
    fn lock_logic_page(&mut self) -> bool;
    fn restore_region_to_logic_page(&mut self, rect: StaticButtonRect, background: &[u8]);
    fn unlock_logic_page(&mut self);

    fn color_scheme(&self) -> i32;
    fn fancy_text_print(&mut self, text: &str, x: i32, y: i32, foreground: i32, background: i32, flags: u32);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticButtonClass {
    pub rect: StaticButtonRect,
    pub print_flags: u32,
    pub text: Option<String>,
    background: Option<Vec<u8>>,
}

impl Default for StaticButtonClass {
    fn default() -> Self {
        Self {
            rect: StaticButtonRect { x: 0, y: 0, width: 0, height: 0 },
            print_flags: 0,
            text: None,
            background: None,
        }
    }
}

impl StaticButtonClass {
    /// StaticButtonClass(void): GadgetClass(0,0,0,0,flags=0),
    /// String=NULL, PrintFlags=TPF_8POINT.
    pub fn new_default(flags: StaticButtonTextFlags) -> Self {
        Self { print_flags: flags.default_eight_point, ..Self::default() }
    }

    /// Normal constructor. The first unsigned ID argument is unnamed and is
    /// discarded by the source implementation.
    pub fn new(
        _ignored_id: u32,
        text: Option<&str>,
        print_flags: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        metrics: &impl StaticButtonHost,
    ) -> Self {
        let mut button = Self {
            rect: StaticButtonRect { x, y, width, height },
            print_flags,
            text: text.map(str::to_owned),
            background: None,
        };
        if width == -1 {
            button.rect.width = button.text.as_deref().map_or(0, |value| metrics.string_pixel_width(value));
        }
        if height == -1 {
            button.rect.height = metrics.font_height();
        }
        button
    }

    /// Port of Set_Text. Text is owned as in the source's duplicated char array.
    /// On resize, Draw_Background is performed before the old dimensions are
    /// replaced; the background cache is then cleared for the new dimensions.
    pub fn set_text(
        &mut self,
        text: Option<&str>,
        resize: bool,
        host: &mut impl StaticButtonHost,
    ) {
        self.text = text.map(str::to_owned);
        self.background = None;

        if resize {
            if let Some(value) = self.text.as_deref() {
                self.draw_background(host);
                self.rect.width = host.string_pixel_width(value);
                self.rect.height = host.font_height() + host.font_y_spacing();
                self.background = None;
            }
        }
    }

    /// Draw_Background: capture the page the first time, then restore that
    /// snapshot only if the underlying logic page can be locked.
    pub fn draw_background(&mut self, host: &mut impl StaticButtonHost) {
        if self.background.is_none() && self.rect.width > 0 && self.rect.height > 0 {
            if let Some(size) = (self.rect.width as usize).checked_mul(self.rect.height as usize) {
                let mut background = Vec::new();
                if background.try_reserve_exact(size).is_ok() {
                    background.resize(size, 0);
                    host.capture_page_region(self.rect, &mut background);
                    self.background = Some(background);
                }
            }
        }

        if let Some(background) = self.background.as_deref() {
            if host.lock_logic_page() {
                host.restore_region_to_logic_page(self.rect, background);
                host.unlock_logic_page();
            }
        }
    }

    /// Draw_Text: align x using TPF_CENTER or TPF_RIGHT, then print the
    /// stored string with the active gadget color scheme and black background.
    pub fn draw_text(
        &self,
        host: &mut impl StaticButtonHost,
        flags: StaticButtonTextFlags,
    ) {
        let Some(text) = self.text.as_deref() else { return; };
        let mut x = self.rect.x;
        if self.print_flags & flags.center != 0 {
            x += self.rect.width / 2;
        }
        if self.print_flags & flags.right != 0 {
            x += self.rect.width - 1;
        }
        host.fancy_text_print(
            text,
            x,
            self.rect.y,
            host.color_scheme(),
            STATIC_BUTTON_BLACK_BACKGROUND,
            self.print_flags,
        );
    }

    /// Draw_Me: the base gadget decides whether anything needs drawing. Mouse
    /// hiding/showing is conditional on the logic page being SeenBuff.
    pub fn draw_me(
        &mut self,
        forced: bool,
        host: &mut impl StaticButtonHost,
        flags: StaticButtonTextFlags,
    ) -> bool {
        if !host.gadget_draw_me(forced) { return false; }
        let on_seen_buffer = host.logic_page_is_seen_buffer();
        if on_seen_buffer {
            host.conditional_hide_mouse(self.rect);
        }
        self.draw_background(host);
        self.draw_text(host, flags);
        if on_seen_buffer {
            host.conditional_show_mouse();
        }
        true
    }

    pub fn background_size(&self) -> Option<usize> {
        self.background.as_ref().map(Vec::len)
    }

    pub fn clear_background(&mut self) {
        self.background = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockHost {
        width_per_character: i32,
        font_height_value: i32,
        font_spacing: i32,
        should_draw: bool,
        seen_buffer: bool,
        lock: bool,
        calls: Vec<String>,
        captured: Vec<u8>,
        restored: Vec<u8>,
        prints: Vec<(String, i32, i32, i32, i32, u32)>,
    }

    impl StaticButtonHost for MockHost {
        fn string_pixel_width(&self, text: &str) -> i32 {
            text.chars().count() as i32 * self.width_per_character
        }
        fn font_height(&self) -> i32 { self.font_height_value }
        fn font_y_spacing(&self) -> i32 { self.font_spacing }
        fn gadget_draw_me(&mut self, forced: bool) -> bool {
            self.calls.push(format!("base-draw:{forced}"));
            self.should_draw
        }
        fn logic_page_is_seen_buffer(&self) -> bool { self.seen_buffer }
        fn conditional_hide_mouse(&mut self, rect: StaticButtonRect) {
            self.calls.push(format!("hide:{}:{}:{}:{}", rect.x, rect.y, rect.width, rect.height));
        }
        fn conditional_show_mouse(&mut self) { self.calls.push("show".into()); }
        fn capture_page_region(&mut self, _rect: StaticButtonRect, output: &mut [u8]) {
            self.calls.push("capture".into());
            for (index, byte) in output.iter_mut().enumerate() {
                *byte = index as u8;
            }
            self.captured = output.to_vec();
        }
        fn lock_logic_page(&mut self) -> bool {
            self.calls.push("lock".into());
            self.lock
        }
        fn restore_region_to_logic_page(&mut self, _rect: StaticButtonRect, background: &[u8]) {
            self.calls.push("restore".into());
            self.restored = background.to_vec();
        }
        fn unlock_logic_page(&mut self) { self.calls.push("unlock".into()); }
        fn color_scheme(&self) -> i32 { 42 }
        fn fancy_text_print(&mut self, text: &str, x: i32, y: i32, foreground: i32, background: i32, flags: u32) {
            self.prints.push((text.to_owned(), x, y, foreground, background, flags));
        }
    }

    fn flags() -> StaticButtonTextFlags {
        StaticButtonTextFlags { center: 1, right: 2, default_eight_point: 4 }
    }

    fn host() -> MockHost {
        MockHost {
            width_per_character: 6,
            font_height_value: 8,
            font_spacing: 2,
            should_draw: true,
            seen_buffer: true,
            lock: true,
            ..MockHost::default()
        }
    }

    #[test]
    fn default_constructor_uses_zero_geometry_and_default_print_flags() {
        let button = StaticButtonClass::new_default(flags());
        assert_eq!(button.rect, StaticButtonRect { x: 0, y: 0, width: 0, height: 0 });
        assert_eq!(button.text, None);
        assert_eq!(button.print_flags, 4);
        assert_eq!(button.background_size(), None);
    }

    #[test]
    fn normal_constructor_ignores_id_and_autosizes_only_minus_one_dimensions() {
        let metrics = host();
        let button = StaticButtonClass::new(99, Some("TEST"), 0, 10, 20, -1, -1, &metrics);
        assert_eq!(button.rect, StaticButtonRect { x: 10, y: 20, width: 24, height: 8 });
        let fixed = StaticButtonClass::new(100, Some("LONG"), 0, 1, 2, 30, 12, &metrics);
        assert_eq!(fixed.rect, StaticButtonRect { x: 1, y: 2, width: 30, height: 12 });
    }

    #[test]
    fn null_text_does_not_autosize_constructor_dimensions() {
        let metrics = host();
        let button = StaticButtonClass::new(1, None, 0, 0, 0, -1, -1, &metrics);
        assert_eq!((button.rect.width, button.rect.height), (0, 8));
    }

    #[test]
    fn set_text_duplicates_string_and_flags_for_redraw_without_resizing() {
        let mut button = StaticButtonClass::new_default(flags());
        let mut host = host();
        button.set_text(Some("Alpha"), false, &mut host);
        assert_eq!(button.text.as_deref(), Some("Alpha"));
        assert!(button.background_size().is_none());
        assert!(host.calls.is_empty());
    }

    #[test]
    fn resize_set_text_draws_old_background_then_updates_size_and_invalidates_cache() {
        let mut button = StaticButtonClass::new(0, Some("A"), 0, 3, 4, 10, 6, &host());
        let mut host = host();
        button.set_text(Some("Long"), true, &mut host);
        assert_eq!(button.rect, StaticButtonRect { x: 3, y: 4, width: 24, height: 10 });
        assert!(button.background_size().is_none());
        assert_eq!(host.calls.iter().filter(|call| call.as_str() == "capture").count(), 1);
        assert_eq!(host.calls[0], "capture");
    }

    #[test]
    fn draw_background_captures_once_and_restores_only_when_page_locks() {
        let mut button = StaticButtonClass::new(0, Some("Hi"), 0, 0, 0, 4, 3, &host());
        let mut host = host();
        button.draw_background(&mut host);
        assert_eq!(button.background_size(), Some(12));
        assert_eq!(host.restored, host.captured);
        let capture_count = host.calls.iter().filter(|call| call.as_str() == "capture").count();
        button.draw_background(&mut host);
        assert_eq!(host.calls.iter().filter(|call| call.as_str() == "capture").count(), capture_count);
    }

    #[test]
    fn draw_background_keeps_snapshot_when_page_lock_fails() {
        let mut button = StaticButtonClass::new(0, Some("Hi"), 0, 0, 0, 4, 3, &host());
        let mut host = MockHost { lock: false, ..host() };
        button.draw_background(&mut host);
        assert_eq!(button.background_size(), Some(12));
        assert!(!host.calls.contains(&"restore".to_owned()));
    }

    #[test]
    fn draw_text_supports_left_center_right_and_both_flag_combination() {
        let button = StaticButtonClass::new(0, Some("X"), 0, 10, 5, 21, 9, &host());
        let mut centered = button.clone();
        centered.print_flags = 1;
        let mut right = button.clone();
        right.print_flags = 2;
        let mut both = button.clone();
        both.print_flags = 3;
        let mut host = host();
        centered.draw_text(&mut host, flags());
        right.draw_text(&mut host, flags());
        both.draw_text(&mut host, flags());
        assert_eq!(host.prints[0].1, 20);
        assert_eq!(host.prints[1].1, 30);
        assert_eq!(host.prints[2].1, 40);
        assert_eq!(host.prints[0].3, 42);
        assert_eq!(host.prints[0].4, STATIC_BUTTON_BLACK_BACKGROUND);
    }

    #[test]
    fn draw_me_hides_and_shows_mouse_only_on_seen_buffer() {
        let mut button = StaticButtonClass::new(0, Some("Go"), 0, 1, 2, 20, 10, &host());
        let mut seen = host();
        assert!(button.draw_me(false, &mut seen, flags()));
        assert!(seen.calls.iter().any(|call| call.starts_with("hide:")));
        assert_eq!(seen.calls.last().map(String::as_str), Some("show"));

        let mut other_page = MockHost { seen_buffer: false, ..host() };
        assert!(button.draw_me(true, &mut other_page, flags()));
        assert!(!other_page.calls.iter().any(|call| call.starts_with("hide:")));
        assert!(!other_page.calls.iter().any(|call| call == "show"));
    }

    #[test]
    fn draw_me_returns_false_without_touching_background_when_base_declines() {
        let mut button = StaticButtonClass::new(0, Some("Go"), 0, 1, 2, 20, 10, &host());
        let mut host = MockHost { should_draw: false, ..host() };
        assert!(!button.draw_me(false, &mut host, flags()));
        assert_eq!(host.calls, vec!["base-draw:false"]);
        assert!(button.background_size().is_none());
    }

    #[test]
    fn set_text_null_releases_string_and_resize_does_not_change_dimensions() {
        let mut button = StaticButtonClass::new(0, Some("Go"), 0, 1, 2, 20, 10, &host());
        let mut host = host();
        button.set_text(None, true, &mut host);
        assert_eq!(button.text, None);
        assert_eq!((button.rect.width, button.rect.height), (20, 10));
    }
}
