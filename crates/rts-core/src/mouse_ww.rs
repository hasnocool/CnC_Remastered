//! Portable active-behavior port of the shared
//! REDALERT/WIN32LIB/MOUSEWW.CPP and TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP.
//!
//! The upstream cursor drawing, timer callback installation, clipping and
//! screen-shadow operations are disabled or return early in the current
//! source. This module preserves the live nesting/state logic and exposes
//! host-provided cursor coordinates instead of calling Win32 directly.

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MousePoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MouseRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl MouseRect {
    pub fn contains_inclusive(self, point: MousePoint) -> bool {
        point.x >= self.left && point.x <= self.right
            && point.y >= self.top && point.y <= self.bottom
    }
}

pub const CONDHIDE: u8 = 1;
pub const CONDHIDDEN: u8 = 2;

/// State retained by the legacy singleton mouse controller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyMouse {
    pub max_width: i32,
    pub max_height: i32,
    pub cursor_width: i32,
    pub cursor_height: i32,
    pub hotspot_x: i32,
    pub hotspot_y: i32,
    pub saved_mouse_position: MousePoint,
    pub conditional_rect: MouseRect,
    pub conditional_flags: u8,
    pub conditional_count: i8,
    pub update_depth: i32,
    pub state: i32,
    pub screen_width: i32,
    pub screen_height: i32,
    pub screen_buffer_token: Option<usize>,
    pub critical_section_depth: u32,
    pub cursor_position: MousePoint,
    pub force_x: Option<i32>,
    pub force_y: Option<i32>,
}

impl LegacyMouse {
    /// Mirrors constructor defaults; screen dimensions and identity are
    /// supplied by the portable host integration.
    pub fn new(screen_width: i32, screen_height: i32, max_width: i32, max_height: i32) -> Self {
        Self {
            max_width,
            max_height,
            cursor_width: 0,
            cursor_height: 0,
            hotspot_x: 0,
            hotspot_y: 0,
            saved_mouse_position: MousePoint { x: -1, y: -1 },
            conditional_rect: MouseRect::default(),
            conditional_flags: 0,
            conditional_count: 0,
            update_depth: 0,
            state: 1,
            screen_width,
            screen_height,
            screen_buffer_token: None,
            critical_section_depth: 0,
            cursor_position: MousePoint::default(),
            force_x: None,
            force_y: None,
        }
    }

    /// Process_Mouse currently returns immediately in upstream.
    pub fn process_mouse(&mut self) {}

    /// The active Low_Hide_Mouse increments State; shadow restoration is
    /// inside a disabled preprocessor block.
    pub fn low_hide_mouse(&mut self) {
        self.state = self.state.wrapping_add(1);
    }

    pub fn hide_mouse(&mut self) {
        self.update_depth = self.update_depth.wrapping_add(1);
        self.low_hide_mouse();
        self.update_depth = self.update_depth.wrapping_sub(1);
    }

    /// The active Low_Show_Mouse decrements State only when it is nonzero.
    pub fn low_show_mouse(&mut self, _x: i32, _y: i32) {
        if self.state == 0 {
            return;
        }
        self.state -= 1;
    }

    pub fn show_mouse(&mut self) {
        self.update_depth = self.update_depth.wrapping_add(1);
        self.low_show_mouse(self.cursor_position.x, self.cursor_position.y);
        self.update_depth = self.update_depth.wrapping_sub(1);
    }

    /// Preserve legacy hotspot expansion, screen clipping, nested-region
    /// union, and the saved-background-position conditional test.
    pub fn conditional_hide_mouse(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.update_depth = self.update_depth.wrapping_add(1);

        let left = (x1.wrapping_sub(self.cursor_width.wrapping_sub(self.hotspot_x))).max(0);
        let top = (y1.wrapping_sub(self.cursor_height.wrapping_sub(self.hotspot_y))).max(0);
        let right = x2.wrapping_add(self.hotspot_x).min(self.screen_width);
        let bottom = y2.wrapping_add(self.hotspot_y).min(self.screen_height);

        if self.conditional_count == 0 {
            self.conditional_rect = MouseRect { left, top, right, bottom };
        } else {
            self.conditional_rect.left = left.min(self.conditional_rect.left);
            self.conditional_rect.top = top.min(self.conditional_rect.top);
            self.conditional_rect.right = right.max(self.conditional_rect.right);
            self.conditional_rect.bottom = bottom.max(self.conditional_rect.bottom);
        }

        if self.conditional_flags & CONDHIDDEN == 0
            && self.conditional_rect.contains_inclusive(self.saved_mouse_position)
        {
            self.low_hide_mouse();
            self.conditional_flags |= CONDHIDDEN;
        }

        self.conditional_flags |= CONDHIDE;
        self.conditional_count = self.conditional_count.wrapping_add(1);
        self.update_depth = self.update_depth.wrapping_sub(1);
    }

    pub fn conditional_show_mouse(&mut self) {
        self.update_depth = self.update_depth.wrapping_add(1);
        if self.conditional_count != 0 {
            self.conditional_count = self.conditional_count.wrapping_sub(1);
            if self.conditional_count == 0 {
                if self.conditional_flags & CONDHIDDEN != 0 {
                    self.show_mouse();
                }
                self.conditional_flags = 0;
            }
        }
        self.update_depth = self.update_depth.wrapping_sub(1);
    }

    /// Cursor installation is compiled out; the input pointer is returned.
    pub fn set_cursor(&mut self, _hotspot_x: i32, _hotspot_y: i32, cursor: usize) -> usize {
        cursor
    }

    /// Draw_Mouse and Erase_Mouse both return immediately in active source.
    pub fn draw_mouse(&mut self) {}
    pub fn erase_mouse(&mut self, _forced: bool) {}

    pub fn mouse_state(&self) -> i32 {
        self.state
    }

    pub fn mouse_x(&self) -> i32 {
        self.force_x.unwrap_or(self.cursor_position.x)
    }

    pub fn mouse_y(&self) -> i32 {
        self.force_y.unwrap_or(self.cursor_position.y)
    }

    pub fn mouse_xy(&self) -> MousePoint {
        MousePoint { x: self.mouse_x(), y: self.mouse_y() }
    }

    /// Block_Mouse/Unblock_Mouse only lock when the supplied buffer is the
    /// controller's screen buffer.
    pub fn block_mouse(&mut self, buffer_token: usize) {
        if self.screen_buffer_token == Some(buffer_token) {
            self.critical_section_depth = self.critical_section_depth.saturating_add(1);
        }
    }

    pub fn unblock_mouse(&mut self, buffer_token: usize) {
        if self.screen_buffer_token == Some(buffer_token) {
            self.critical_section_depth = self.critical_section_depth.saturating_sub(1);
        }
    }

    pub fn set_cursor_clip(&mut self) {}
    pub fn clear_cursor_clip(&mut self) {}
}

/// Global API behavior when the legacy singleton has not been constructed.
pub fn global_mouse_state(mouse: Option<&LegacyMouse>) -> i32 {
    mouse.map_or(0, LegacyMouse::mouse_state)
}
pub fn global_mouse_x(mouse: Option<&LegacyMouse>, forced_x: i32) -> i32 {
    if forced_x >= 0 { forced_x } else { mouse.map_or(0, LegacyMouse::mouse_x) }
}
pub fn global_mouse_y(mouse: Option<&LegacyMouse>, forced_y: i32) -> i32 {
    if forced_y >= 0 { forced_y } else { mouse.map_or(0, LegacyMouse::mouse_y) }
}
pub fn global_set_cursor(mouse: Option<&mut LegacyMouse>, hot_x: i32, hot_y: i32, cursor: usize) -> usize {
    mouse.map_or(0, |m| m.set_cursor(hot_x, hot_y, cursor))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_and_hide_show_match_live_state() {
        let mut mouse = LegacyMouse::new(640, 480, 64, 64);
        assert_eq!(mouse.mouse_state(), 1);
        mouse.hide_mouse();
        assert_eq!(mouse.mouse_state(), 2);
        mouse.show_mouse();
        assert_eq!(mouse.mouse_state(), 1);
        mouse.low_show_mouse(0, 0);
        assert_eq!(mouse.mouse_state(), 0);
        mouse.low_show_mouse(0, 0);
        assert_eq!(mouse.mouse_state(), 0);
    }

    #[test]
    fn conditional_regions_union_and_nest() {
        let mut mouse = LegacyMouse::new(640, 480, 64, 64);
        mouse.saved_mouse_position = MousePoint { x: 20, y: 20 };
        mouse.conditional_hide_mouse(10, 10, 30, 30);
        assert_eq!(mouse.conditional_count, 1);
        assert_eq!(mouse.conditional_flags, CONDHIDE | CONDHIDDEN);
        assert_eq!(mouse.state, 2);
        mouse.conditional_hide_mouse(40, 40, 50, 50);
        assert_eq!(mouse.conditional_rect, MouseRect { left: 10, top: 10, right: 50, bottom: 50 });
        assert_eq!(mouse.conditional_count, 2);
        mouse.conditional_show_mouse();
        assert_eq!(mouse.state, 2);
        mouse.conditional_show_mouse();
        assert_eq!(mouse.state, 1);
        assert_eq!(mouse.conditional_flags, 0);
    }

    #[test]
    fn force_coordinates_override_host_cursor_and_uninstalled_api_is_zero() {
        let mut mouse = LegacyMouse::new(800, 600, 32, 32);
        mouse.cursor_position = MousePoint { x: 12, y: 34 };
        mouse.force_x = Some(90);
        assert_eq!(mouse.mouse_xy(), MousePoint { x: 90, y: 34 });
        assert_eq!(global_mouse_state(None), 0);
        assert_eq!(global_mouse_x(None, -1), 0);
        assert_eq!(global_mouse_y(None, 44), 44);
    }

    #[test]
    fn block_and_unblock_only_affect_the_screen_buffer() {
        let mut mouse = LegacyMouse::new(320, 200, 32, 32);
        mouse.screen_buffer_token = Some(7);
        mouse.block_mouse(8);
        assert_eq!(mouse.critical_section_depth, 0);
        mouse.block_mouse(7);
        assert_eq!(mouse.critical_section_depth, 1);
        mouse.unblock_mouse(7);
        assert_eq!(mouse.critical_section_depth, 0);
    }
}
