//! Red Alert ProgressBarClass from REDALERT/BAR.CPP.
//!
//! This module intentionally preserves the source rectangle endpoints and
//! unsigned Q16.16 arithmetic. The render target is supplied by the host.

use crate::fixed16::Fixed16;

pub const PROGRESS_REDRAW_THRESHOLD: Fixed16 = Fixed16::from_raw(6_553);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgressBarRect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

pub trait ProgressBarCanvas {
    fn hide_mouse(&mut self);
    fn draw_line(&mut self, rect: ProgressBarRect, color: i32);
    fn fill_rect(&mut self, rect: ProgressBarRect, color: i32);
    fn show_mouse(&mut self);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressBarClass {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub bar_color: i32,
    pub back_color: i32,
    pub border_color: i32,
    pub current_value: Fixed16,
    pub last_display_current: Fixed16,
    pub is_drawn: bool,
}

impl ProgressBarClass {
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        forecolor: i32,
        backcolor: i32,
        bordercolor: i32,
    ) -> Self {
        Self {
            x, y, width, height,
            bar_color: forecolor,
            back_color: backcolor,
            border_color: bordercolor,
            current_value: Fixed16::ZERO,
            last_display_current: Fixed16::ZERO,
            is_drawn: false,
        }
    }

    pub const fn is_horizontal(&self) -> bool {
        self.width > self.height
    }

    pub const fn is_outlined(&self) -> bool {
        self.border_color != 0
    }

    fn outline(&self, canvas: &mut impl ProgressBarCanvas) {
        if !self.is_outlined() {
            return;
        }
        canvas.draw_line(
            ProgressBarRect { x1: self.x, y1: self.y, x2: self.x + self.width, y2: self.y },
            self.border_color,
        );
        canvas.draw_line(
            ProgressBarRect { x1: self.x, y1: self.y, x2: self.x, y2: self.y + self.height },
            self.border_color,
        );
        // The original third Draw_Line call is a point at the lower-left corner.
        canvas.draw_line(
            ProgressBarRect {
                x1: self.x,
                y1: self.y + self.height,
                x2: self.x,
                y2: self.y + self.height,
            },
            self.border_color,
        );
        canvas.draw_line(
            ProgressBarRect {
                x1: self.x + self.width,
                y1: self.y,
                x2: self.x + self.width,
                y2: self.y + self.height,
            },
            self.border_color,
        );
    }

    /// ProgressBarClass::Redraw. The Q16.16 value is multiplied by the
    /// horizontal width or vertical height, then converted to unsigned using
    /// Fixed16's source-compatible nearest-integer rounding.
    pub fn redraw(&mut self, canvas: &mut impl ProgressBarCanvas) {
        canvas.hide_mouse();
        self.outline(canvas);

        let mut x = self.x;
        let mut y = self.y;
        let mut width = self.width;
        let mut height = self.height;
        if self.is_outlined() {
            x += 1;
            y += 1;
            width -= 2;
            height -= 2;
        }

        let horizontal = self.is_horizontal();
        let size = if horizontal { width } else { height };
        let fill = self.current_value.mul_integer(size).to_unsigned() as i32;

        if fill > 0 {
            if horizontal {
                canvas.fill_rect(
                    ProgressBarRect { x1: x, y1: y, x2: x + fill, y2: y + height },
                    self.bar_color,
                );
            } else {
                canvas.fill_rect(
                    ProgressBarRect { x1: x, y1: y + fill, x2: x + width, y2: y + height },
                    self.bar_color,
                );
            }
        }

        // This condition uses width for both orientations in the original.
        if width - fill > 0 {
            if horizontal {
                canvas.fill_rect(
                    ProgressBarRect { x1: x + fill, y1: y, x2: x + width, y2: y + height },
                    self.back_color,
                );
            } else {
                canvas.fill_rect(
                    ProgressBarRect { x1: x, y1: y, x2: x + width, y2: y + fill - 1 },
                    self.back_color,
                );
            }
        }

        canvas.show_mouse();
        self.last_display_current = self.current_value;
        self.is_drawn = true;
    }

    /// Subtraction is wrapping unsigned Q16.16, matching FIXED.H raw storage.
    /// Consequently, a decrease wraps to a large delta and also redraws.
    pub fn update(&mut self, value: Fixed16, canvas: &mut impl ProgressBarCanvas) -> bool {
        self.current_value = value;
        let delta = self.current_value.sub(self.last_display_current);
        if !self.is_drawn || delta >= PROGRESS_REDRAW_THRESHOLD {
            self.redraw(canvas);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct RecordingCanvas {
        events: Vec<String>,
        lines: Vec<(ProgressBarRect, i32)>,
        fills: Vec<(ProgressBarRect, i32)>,
    }

    impl ProgressBarCanvas for RecordingCanvas {
        fn hide_mouse(&mut self) { self.events.push("hide".to_owned()); }
        fn draw_line(&mut self, rect: ProgressBarRect, color: i32) {
            self.lines.push((rect, color));
            self.events.push("line".to_owned());
        }
        fn fill_rect(&mut self, rect: ProgressBarRect, color: i32) {
            self.fills.push((rect, color));
            self.events.push("fill".to_owned());
        }
        fn show_mouse(&mut self) { self.events.push("show".to_owned()); }
    }

    fn ratio(numerator: i32, denominator: i32) -> Fixed16 {
        Fixed16::from_ratio(numerator, denominator)
    }

    #[test]
    fn constructor_and_orientation_match_source() {
        let bar = ProgressBarClass::new(10, 20, 100, 8, 3, 4, 0);
        assert_eq!((bar.x, bar.y, bar.width, bar.height), (10, 20, 100, 8));
        assert_eq!((bar.bar_color, bar.back_color, bar.border_color), (3, 4, 0));
        assert_eq!(bar.current_value, Fixed16::ZERO);
        assert!(!bar.is_drawn);
        assert!(bar.is_horizontal());
        assert!(!ProgressBarClass::new(0, 0, 8, 100, 1, 2, 0).is_horizontal());
        assert!(!ProgressBarClass::new(0, 0, 10, 10, 1, 2, 0).is_horizontal());
    }

    #[test]
    fn first_update_draws_then_small_increments_are_suppressed() {
        let mut bar = ProgressBarClass::new(0, 0, 100, 10, 1, 2, 0);
        let mut canvas = RecordingCanvas::default();
        assert!(bar.update(Fixed16::ZERO, &mut canvas));
        assert!(!bar.update(ratio(5, 100), &mut canvas));
        assert_eq!(canvas.events.first().map(String::as_str), Some("hide"));
        assert_eq!(canvas.events.last().map(String::as_str), Some("show"));
        assert_eq!(bar.last_display_current, Fixed16::ZERO);
        assert!(bar.is_drawn);
    }

    #[test]
    fn exactly_one_tenth_increment_redraws() {
        let mut bar = ProgressBarClass::new(0, 0, 100, 10, 1, 2, 0);
        let mut canvas = RecordingCanvas::default();
        bar.update(Fixed16::ZERO, &mut canvas);
        assert!(bar.update(ratio(1, 10), &mut canvas));
        assert_eq!(bar.last_display_current, ratio(1, 10));
    }

    #[test]
    fn unsigned_fixed_subtraction_makes_decreases_redraw() {
        let mut bar = ProgressBarClass::new(0, 0, 100, 10, 1, 2, 0);
        let mut canvas = RecordingCanvas::default();
        bar.update(ratio(1, 2), &mut canvas);
        assert!(bar.update(ratio(2, 5), &mut canvas));
        assert_eq!(bar.last_display_current, ratio(2, 5));
    }

    #[test]
    fn horizontal_bar_uses_source_inclusive_fill_and_background_rectangles() {
        let mut bar = ProgressBarClass::new(10, 20, 10, 5, 7, 8, 0);
        let mut canvas = RecordingCanvas::default();
        bar.current_value = ratio(1, 2);
        bar.redraw(&mut canvas);
        assert_eq!(canvas.fills, vec![
            (ProgressBarRect { x1: 10, y1: 20, x2: 15, y2: 25 }, 7),
            (ProgressBarRect { x1: 15, y1: 20, x2: 20, y2: 25 }, 8),
        ]);
    }

    #[test]
    fn vertical_bar_retains_original_bottom_fill_and_width_based_background_test() {
        let mut bar = ProgressBarClass::new(10, 20, 5, 10, 7, 8, 0);
        let mut canvas = RecordingCanvas::default();
        bar.current_value = ratio(1, 2);
        bar.redraw(&mut canvas);
        assert_eq!(canvas.fills, vec![
            (ProgressBarRect { x1: 10, y1: 25, x2: 15, y2: 30 }, 7),
            (ProgressBarRect { x1: 10, y1: 20, x2: 15, y2: 24 }, 8),
        ]);

        let mut empty = ProgressBarClass::new(0, 0, 5, 10, 7, 8, 0);
        let mut empty_canvas = RecordingCanvas::default();
        empty.redraw(&mut empty_canvas);
        assert_eq!(
            empty_canvas.fills,
            vec![(ProgressBarRect { x1: 0, y1: 0, x2: 5, y2: -1 }, 8)]
        );
    }

    #[test]
    fn outlined_bar_uses_the_four_source_draw_line_calls() {
        let mut bar = ProgressBarClass::new(2, 3, 10, 8, 1, 2, 9);
        let mut canvas = RecordingCanvas::default();
        bar.redraw(&mut canvas);
        assert_eq!(canvas.lines, vec![
            (ProgressBarRect { x1: 2, y1: 3, x2: 12, y2: 3 }, 9),
            (ProgressBarRect { x1: 2, y1: 3, x2: 2, y2: 11 }, 9),
            (ProgressBarRect { x1: 2, y1: 11, x2: 2, y2: 11 }, 9),
            (ProgressBarRect { x1: 12, y1: 3, x2: 12, y2: 11 }, 9),
        ]);
        assert_eq!(canvas.fills[0].0, ProgressBarRect { x1: 3, y1: 4, x2: 8, y2: 9 });
    }

    #[test]
    fn full_value_draws_once_and_equal_value_is_suppressed() {
        let mut bar = ProgressBarClass::new(0, 0, 10, 4, 1, 2, 0);
        let mut canvas = RecordingCanvas::default();
        bar.current_value = Fixed16::ONE;
        bar.redraw(&mut canvas);
        assert_eq!(canvas.fills.len(), 1);
        assert_eq!(canvas.fills[0].0, ProgressBarRect { x1: 0, y1: 0, x2: 10, y2: 4 });
        let old_len = canvas.fills.len();
        assert!(!bar.update(Fixed16::ONE, &mut canvas));
        assert_eq!(canvas.fills.len(), old_len);
    }
}
