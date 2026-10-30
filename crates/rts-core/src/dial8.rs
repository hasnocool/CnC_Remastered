//! Eight-direction mouse dial from both games' DIAL8.CPP/H.
//! Geometry and input behavior are shared; box theme and line colors vary.

use std::sync::atomic::{AtomicBool, Ordering};
use crate::facing::{DirType, Facing};
use crate::misc_asm::desired_facing8;

pub const LEFTPRESS: u32 = 0x0001;
pub const LEFTHELD: u32 = 0x0002;
pub const LEFTRELEASE: u32 = 0x0004;
pub const KN_NONE: u16 = 0;

static DIAL8_SELECTION_ACTIVE: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dial8Variant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DialBoxStyle { Down, Raised, GreenDown, GreenRaised }

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DialPoint { pub x: i32, pub y: i32 }

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DialLineColors { pub shadow: i32, pub highlight: i32 }

pub trait Dial8Host {
    fn sticky_process(&mut self, flags: u32);
    fn control_action(&mut self, flags: u32, key: &mut u16) -> bool;
    fn control_draw(&mut self, forced: bool) -> bool;
    fn get_mouse_x(&self) -> i32;
    fn get_mouse_y(&self) -> i32;
    fn is_seen_buffer_logic_page(&self) -> bool;
    fn hide_mouse(&mut self);
    fn show_mouse(&mut self);
    fn line_colors(&self, variant: Dial8Variant) -> DialLineColors;
    fn draw_box(&mut self, x: i32, y: i32, width: i32, height: i32, style: DialBoxStyle, fill: bool);
    fn draw_line(&mut self, from: DialPoint, to: DialPoint, color: i32);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dial8Controller {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    face_x: i32,
    face_y: i32,
    face_points: [DialPoint; 8],
    face_lines: [DialPoint; 8],
    direction: DirType,
    facing: Facing,
    old_facing: Facing,
    redraw_requested: bool,
}

impl Dial8Controller {
    pub fn new(id: i32, x: i32, y: i32, width: i32, height: i32, direction: DirType) -> Self {
        let face_x = x + width / 2;
        let face_y = y + height / 2;
        let mut points = [DialPoint::default(); 8];
        let mut lines = [DialPoint::default(); 8];

        points[0] = DialPoint { x: face_x, y: face_y - (height * 8 / 2) / 10 };
        points[1] = DialPoint { x: face_x + (width * 7 * 8 / 2) / 100, y: face_y - (height * 7 * 8 / 2) / 100 };
        points[2] = DialPoint { x: face_x + (width * 8 / 2) / 10, y: face_y };
        points[3] = DialPoint { x: face_x + (width * 7 * 8 / 2) / 100, y: face_y + (height * 7 * 8 / 2) / 100 };
        points[4] = DialPoint { x: face_x, y: face_y + (height * 8 / 2) / 10 };
        points[5] = DialPoint { x: face_x - (width * 7 * 8 / 2) / 100, y: face_y + (height * 7 * 8 / 2) / 100 };
        points[6] = DialPoint { x: face_x - (width * 8 / 2) / 10, y: face_y };
        points[7] = DialPoint { x: face_x - (width * 7 * 8 / 2) / 100, y: face_y - (height * 7 * 8 / 2) / 100 };

        lines[0] = DialPoint { x: face_x, y: face_y - (height * 6 / 2) / 10 };
        lines[1] = DialPoint { x: face_x + (width * 7 * 6 / 2) / 100, y: face_y - (height * 7 * 6 / 2) / 100 };
        lines[2] = DialPoint { x: face_x + (width * 6 / 2) / 10, y: face_y };
        lines[3] = DialPoint { x: face_x + (width * 7 * 6 / 2) / 100, y: face_y + (height * 7 * 6 / 2) / 100 };
        lines[4] = DialPoint { x: face_x, y: face_y + (height * 6 / 2) / 10 };
        lines[5] = DialPoint { x: face_x - (width * 7 * 6 / 2) / 100, y: face_y + (height * 7 * 6 / 2) / 100 };
        lines[6] = DialPoint { x: face_x - (width * 6 / 2) / 10, y: face_y };
        lines[7] = DialPoint { x: face_x - (width * 7 * 6 / 2) / 100, y: face_y - (height * 7 * 6 / 2) / 100 };

        let facing = direction.as_facing();
        Self {
            id, x, y, width, height, face_x, face_y,
            face_points: points,
            face_lines: lines,
            direction,
            facing,
            old_facing: facing,
            redraw_requested: false,
        }
    }

    pub const fn accepted_mouse_flags(&self) -> u32 { LEFTPRESS | LEFTHELD | LEFTRELEASE }
    pub const fn direction(&self) -> DirType { self.direction }
    pub const fn facing(&self) -> Facing { self.facing }
    pub const fn old_facing(&self) -> Facing { self.old_facing }
    pub const fn face_center(&self) -> DialPoint { DialPoint { x: self.face_x, y: self.face_y } }
    pub const fn face_points(&self) -> &[DialPoint; 8] { &self.face_points }
    pub const fn face_lines(&self) -> &[DialPoint; 8] { &self.face_lines }
    pub const fn redraw_requested(&self) -> bool { self.redraw_requested }

    /// The original selection flag is a static local, shared by all dial instances.
    pub fn action(&mut self, host: &mut impl Dial8Host, flags: u32, key: &mut u16) -> bool {
        host.sticky_process(flags);
        if flags & LEFTPRESS != 0 {
            DIAL8_SELECTION_ACTIVE.store(true, Ordering::SeqCst);
        }

        if flags & LEFTPRESS != 0
            || ((flags & LEFTHELD != 0) && DIAL8_SELECTION_ACTIVE.load(Ordering::SeqCst))
        {
            self.direction = DirType(desired_facing8(
                self.face_x, self.face_y, host.get_mouse_x(), host.get_mouse_y(),
            ));
            self.facing = self.direction.as_facing();
            if self.facing != self.old_facing {
                self.old_facing = self.facing;
                self.redraw_requested = true;
                host.control_action(flags, key)
            } else {
                *key = KN_NONE;
                host.control_action(0, key);
                true
            }
        } else {
            if flags & LEFTRELEASE != 0 {
                *key = KN_NONE;
                DIAL8_SELECTION_ACTIVE.store(false, Ordering::SeqCst);
            }
            host.control_action(0, key)
        }
    }

    pub fn draw_me(&mut self, host: &mut impl Dial8Host, variant: Dial8Variant, forced: bool) -> bool {
        if !host.control_draw(forced) { return false; }
        let seen_buffer = host.is_seen_buffer_logic_page();
        if seen_buffer { host.hide_mouse(); }

        let (background, decoration) = match variant {
            Dial8Variant::RedAlert => (DialBoxStyle::Down, DialBoxStyle::Raised),
            Dial8Variant::TiberianDawn => (DialBoxStyle::GreenDown, DialBoxStyle::GreenRaised),
        };
        host.draw_box(self.x, self.y, self.width, self.height, background, true);
        for point in self.face_points {
            host.draw_box(point.x - 1, point.y - 1, 3, 3, decoration, false);
        }

        let colors = host.line_colors(variant);
        let line = self.face_lines[self.facing.ordinal() as usize];
        host.draw_line(
            DialPoint { x: self.face_x + 1, y: self.face_y + 1 },
            DialPoint { x: line.x + 1, y: line.y + 1 },
            colors.shadow,
        );
        host.draw_line(self.face_center(), line, colors.highlight);

        if seen_buffer { host.show_mouse(); }
        self.redraw_requested = false;
        true
    }

    pub fn set_direction(&mut self, direction: DirType) {
        self.direction = direction;
        self.facing = direction.as_facing();
        self.old_facing = self.facing;
        self.redraw_requested = true;
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
        mouse_x: i32,
        mouse_y: i32,
        actions: Vec<(u32, u16)>,
        draws: Vec<(i32, i32, i32, i32, DialBoxStyle, bool)>,
        lines: Vec<(DialPoint, DialPoint, i32)>,
        colors: DialLineColors,
        draw_parent: bool,
        seen: bool,
        hidden: usize,
        shown: usize,
    }

    impl Dial8Host for FakeHost {
        fn sticky_process(&mut self, _flags: u32) {}
        fn control_action(&mut self, flags: u32, key: &mut u16) -> bool {
            self.actions.push((flags, *key));
            true
        }
        fn control_draw(&mut self, _forced: bool) -> bool { self.draw_parent }
        fn get_mouse_x(&self) -> i32 { self.mouse_x }
        fn get_mouse_y(&self) -> i32 { self.mouse_y }
        fn is_seen_buffer_logic_page(&self) -> bool { self.seen }
        fn hide_mouse(&mut self) { self.hidden += 1; }
        fn show_mouse(&mut self) { self.shown += 1; }
        fn line_colors(&self, _variant: Dial8Variant) -> DialLineColors { self.colors }
        fn draw_box(&mut self, x: i32, y: i32, w: i32, h: i32, style: DialBoxStyle, fill: bool) {
            self.draws.push((x, y, w, h, style, fill));
        }
        fn draw_line(&mut self, from: DialPoint, to: DialPoint, color: i32) {
            self.lines.push((from, to, color));
        }
    }

    #[test]
    fn constructor_uses_original_integer_geometry() {
        let dial = Dial8Controller::new(7, 10, 20, 100, 80, DirType::NORTH);
        assert_eq!(dial.face_center(), DialPoint { x: 60, y: 60 });
        assert_eq!(dial.face_points()[0], DialPoint { x: 60, y: 28 });
        assert_eq!(dial.face_points()[2], DialPoint { x: 100, y: 60 });
        assert_eq!(dial.face_points()[4], DialPoint { x: 60, y: 92 });
        assert_eq!(dial.face_points()[6], DialPoint { x: 20, y: 60 });
        assert_eq!(dial.face_lines()[0], DialPoint { x: 60, y: 36 });
        assert_eq!(dial.face_lines()[2], DialPoint { x: 90, y: 60 });
        assert_eq!(dial.accepted_mouse_flags(), 7);
    }

    #[test]
    fn press_and_held_updates_direction_then_suppresses_unchanged_event() {
        DIAL8_SELECTION_ACTIVE.store(false, Ordering::SeqCst);
        let mut dial = Dial8Controller::new(1, 0, 0, 100, 100, DirType::NORTH);
        let mut host = FakeHost { mouse_x: 100, mouse_y: 50, ..FakeHost::default() };
        let mut key = 123;
        assert!(dial.action(&mut host, LEFTPRESS, &mut key));
        assert_eq!(dial.direction(), DirType::EAST);
        assert_eq!(dial.facing(), Facing::East);
        assert_eq!(dial.old_facing(), Facing::East);
        assert_eq!(host.actions.last().unwrap().0, LEFTPRESS);

        key = 999;
        assert!(dial.action(&mut host, LEFTHELD, &mut key));
        assert_eq!(key, KN_NONE);
        assert_eq!(host.actions.last().unwrap().0, 0);
    }

    #[test]
    fn release_clears_the_shared_selection_flag() {
        DIAL8_SELECTION_ACTIVE.store(true, Ordering::SeqCst);
        let mut dial = Dial8Controller::new(1, 0, 0, 40, 40, DirType::NORTH);
        let mut host = FakeHost::default();
        let mut key = 22;
        assert!(dial.action(&mut host, LEFTRELEASE, &mut key));
        assert_eq!(key, KN_NONE);
        assert!(!DIAL8_SELECTION_ACTIVE.load(Ordering::SeqCst));
    }

    #[test]
    fn set_direction_updates_face_and_requests_redraw() {
        let mut dial = Dial8Controller::new(1, 0, 0, 100, 100, DirType::NORTH);
        dial.set_direction(DirType::SOUTH_WEST);
        assert_eq!(dial.direction(), DirType::SOUTH_WEST);
        assert_eq!(dial.facing(), Facing::SouthWest);
        assert_eq!(dial.old_facing(), Facing::SouthWest);
        assert!(dial.consume_redraw_request());
        assert!(!dial.consume_redraw_request());
    }

    #[test]
    fn draw_uses_per_variant_box_styles_and_mouse_visibility_guard() {
        let mut dial = Dial8Controller::new(1, 5, 6, 80, 60, DirType::EAST);
        let mut ra = FakeHost {
            draw_parent: true,
            seen: true,
            colors: DialLineColors { shadow: 2, highlight: 3 },
            ..FakeHost::default()
        };
        assert!(dial.draw_me(&mut ra, Dial8Variant::RedAlert, false));
        assert_eq!(ra.draws.len(), 9);
        assert_eq!(ra.draws[0].4, DialBoxStyle::Down);
        assert_eq!(ra.draws[1].4, DialBoxStyle::Raised);
        assert_eq!((ra.hidden, ra.shown), (1, 1));
        assert_eq!(ra.lines.len(), 2);
        assert_eq!((ra.lines[0].2, ra.lines[1].2), (2, 3));

        let mut td = FakeHost { draw_parent: true, ..FakeHost::default() };
        assert!(dial.draw_me(&mut td, Dial8Variant::TiberianDawn, false));
        assert_eq!(td.draws[0].4, DialBoxStyle::GreenDown);
        assert_eq!(td.draws[1].4, DialBoxStyle::GreenRaised);
    }

    #[test]
    fn parent_redraw_gate_prevents_drawing_and_mouse_side_effects() {
        let mut dial = Dial8Controller::new(1, 0, 0, 50, 50, DirType::NORTH);
        let mut host = FakeHost { draw_parent: false, seen: true, ..FakeHost::default() };
        assert!(!dial.draw_me(&mut host, Dial8Variant::RedAlert, false));
        assert!(host.draws.is_empty());
        assert_eq!((host.hidden, host.shown), (0, 0));
    }
}
