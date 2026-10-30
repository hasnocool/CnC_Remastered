//! Portable state and text-layout port of both variants' WIN32LIB/WINDOWS.CPP.
//!
//! Rendering is emitted as ordered draw operations so the host can execute
//! them against its viewport. The legacy text controls, dipthong expansion,
//! printf-style %s/%d substitutions, cursor bookkeeping, word wrapping,
//! scrolling and more-prompt geometry are retained.

use crate::dipthong_tables::{COMMON, DIPTHONG};

pub const KA_MORE: u8 = 1;
pub const KA_SETBKGDCOL: u8 = 2;
pub const KA_SETFORECOL: u8 = 6;
pub const KA_FORMFEED: u8 = 12;
pub const KA_SPCTAB: u8 = 20;
pub const KA_SETX: u8 = 25;
pub const KA_SETY: u8 = 26;
pub const KA_TAB: u8 = 9;
pub const KA_RETURN: u8 = 13;
pub const KA_EXTEND: u8 = 27;
pub const KA_SPACE: u8 = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowDefinition {
    pub x: i32,
    pub y: i32,
    pub width_bytes: i32,
    pub height_pixels: i32,
    pub foreground: u8,
    pub background: u8,
    pub cursor_x: i32,
    pub cursor_y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct WindowFontMetrics<'a> {
    pub widths: &'a [u8],
    pub font_width: i32,
    pub font_height: i32,
    pub x_spacing: i32,
    pub y_spacing: i32,
}
impl WindowFontMetrics<'_> {
    fn char_width(&self, c: u8) -> i32 {
        self.widths.get(c as usize).copied().unwrap_or(0) as i32 + self.x_spacing
    }
    fn string_width(&self, bytes: &[u8]) -> i32 {
        let mut width = 0;
        let mut largest = 0;
        for c in bytes.iter().copied().take_while(|c| *c != 0) {
            if c == b'\r' { largest = largest.max(width); width = 0; }
            else { width = width.wrapping_add(self.char_width(c)); }
        }
        largest.max(width)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WindowDrawOp {
    Print { text: Vec<u8>, x: i32, y: i32, foreground: u8, background: u8 },
    FillRect { left: i32, top: i32, right: i32, bottom: i32, color: u8 },
    Blit { source_x: i32, source_y: i32, dest_x: i32, dest_y: i32, width: i32, height: i32 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WindowArg {
    Text(Option<Vec<u8>>),
    Integer(i32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowTextEngine {
    pub windows: Vec<WindowDefinition>,
    pub current_window: usize,
    pub window_columns: i32,
    pub window_lines: i32,
    pub window_width: i32,
    pub background: u8,
    pub foreground: u8,
    pub win_x: i32,
    pub win_y: i32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub win_height: i32,
    pub win_width: i32,
    pub more_on: bool,
    pub more_text: Vec<u8>,
    pub more_space: i32,
    pub more_foreground: u8,
    pub more_background: u8,
    pub more_callback_enabled: bool,
    scroll_counter: i32,
    word_wrap: bool,
    line: Vec<u8>,
    pos: usize,
    pixel_pos: i32,
    window_pos: i32,
}

impl WindowTextEngine {
    pub fn new(windows: Vec<WindowDefinition>) -> Self {
        Self {
            windows, current_window: 0, window_columns: 40, window_lines: 25,
            window_width: 40, background: 0, foreground: 1, win_x: 0, win_y: 0,
            cursor_x: 0, cursor_y: 0, win_height: 25, win_width: 40,
            more_on: true, more_text: b"--More--".to_vec(), more_space: 7,
            more_foreground: 0, more_background: 0, more_callback_enabled: true,
            scroll_counter: 0, word_wrap: false, line: Vec::with_capacity(84),
            pos: 0, pixel_pos: 0, window_pos: 0,
        }
    }

    pub fn set_more_prompt(&mut self, prompt: Option<&[u8]>, space: i32, foreground: u8, background: u8) {
        if let Some(prompt) = prompt {
            self.more_text = nul_terminated(prompt).to_vec();
            self.more_space = space;
            self.more_foreground = foreground;
            self.more_background = background;
        } else {
            self.more_text = b"<MORE>".to_vec();
            self.more_space = 7;
            self.more_foreground = 0;
            self.more_background = 0;
        }
    }

    pub fn set_more_on(&mut self) {
        self.more_on = true;
        self.scroll_counter = 0;
    }
    pub fn set_more_off(&mut self) { self.more_on = false; }

    pub fn change_window(&mut self, index: usize, metrics: WindowFontMetrics<'_>) -> Option<usize> {
        let next = self.windows.get(index)?.clone();
        let old = self.current_window;
        self.current_window = index;
        self.win_x = next.x;
        self.win_y = next.y;
        self.win_width = next.width_bytes;
        self.win_height = next.height_pixels;
        self.foreground = next.foreground;
        self.background = next.background;
        self.cursor_x = next.cursor_x;
        self.cursor_y = next.cursor_y;
        self.scroll_counter = 0;
        let char_width = metrics.font_width + metrics.x_spacing;
        let line_height = metrics.font_height + metrics.y_spacing;
        self.window_pos = if char_width != 0 { self.cursor_x / char_width } else { 0 };
        self.window_lines = if line_height != 0 { (self.win_height - metrics.y_spacing) / line_height } else { 0 };
        self.window_width = self.win_width.wrapping_shl(3);
        self.window_columns = if char_width != 0 { self.window_width / char_width } else { 0 };
        Some(old)
    }

    pub fn change_new_window(&mut self, index: usize, metrics: WindowFontMetrics<'_>) -> Option<(usize, Vec<WindowDrawOp>)> {
        let old = self.change_window(index, metrics)?;
        Some((old, self.new_window()))
    }

    pub fn new_window(&mut self) -> Vec<WindowDrawOp> {
        let x = self.win_x.wrapping_shl(3);
        let y = self.win_y;
        let right = self.win_x.wrapping_add(self.win_width).wrapping_shl(3) - 1;
        let bottom = self.win_y.wrapping_add(self.win_height) - 1;
        self.cursor_x = 0;
        self.window_pos = 0;
        self.cursor_y = 0;
        self.scroll_counter = 0;
        vec![WindowDrawOp::FillRect { left: x, top: y, right, bottom, color: self.background }]
    }

    pub fn window_int_print(&mut self, value: i32, metrics: WindowFontMetrics<'_>) -> Vec<WindowDrawOp> {
        self.window_print(value.to_string().as_bytes(), &[], metrics)
    }

    pub fn window_print(&mut self, source: &[u8], args: &[WindowArg], metrics: WindowFontMetrics<'_>) -> Vec<WindowDrawOp> {
        self.word_wrap = false;
        self.pos = 0;
        self.pixel_pos = 0;
        self.line.clear();
        let mut stream = LegacyCharStream::new(source, args);
        let old_foreground = self.foreground;
        let old_background = self.background;
        let mut ops = Vec::new();

        while let Some(mut c) = stream.fetch() {
            if c == b'%' {
                match stream.peek() {
                    Some(b's') | Some(b'S') => {
                        if stream.push_text_arg() {
                            c = match stream.fetch() { Some(v) => v, None => continue };
                        }
                    }
                    Some(b'd') | Some(b'D') => {
                        if stream.push_integer_arg() {
                            c = match stream.fetch() { Some(v) => v, None => continue };
                        }
                    }
                    _ => {}
                }
            }

            match c {
                KA_SETX => {
                    self.flush_line(metrics, &mut ops);
                    self.window_pos = stream.fetch().unwrap_or(0) as i8 as i32;
                    self.window_pos = self.window_pos.max(0).min(self.window_columns - 1);
                    self.cursor_x = self.window_pos.wrapping_mul(metrics.font_width + metrics.x_spacing);
                }
                KA_SETY => {
                    self.flush_line(metrics, &mut ops);
                    self.cursor_y = stream.fetch().unwrap_or(0) as i8 as i32;
                    self.cursor_y = self.cursor_y.min(self.window_lines - 1);
                }
                KA_MORE => {
                    self.flush_line(metrics, &mut ops);
                    self.emit_more_prompt(metrics, &mut ops);
                }
                KA_FORMFEED => ops.extend(self.new_window()),
                KA_RETURN => {
                    self.flush_line(metrics, &mut ops);
                    self.scroll_counter = self.scroll_counter.wrapping_add(1);
                    self.cursor_x = 0;
                    self.cursor_y = self.cursor_y.wrapping_add(1);
                }
                KA_SETBKGDCOL => {
                    self.flush_line(metrics, &mut ops);
                    self.background = stream.fetch().unwrap_or(0);
                }
                KA_SETFORECOL => {
                    self.flush_line(metrics, &mut ops);
                    self.foreground = stream.fetch().unwrap_or(0);
                }
                KA_TAB => {
                    self.flush_line(metrics, &mut ops);
                    self.window_pos = ((self.window_pos.wrapping_add(8)) & 0xfff8) - 1;
                    if self.window_pos >= self.window_columns { self.window_pos = 0; }
                    self.cursor_x = self.window_pos.wrapping_mul(metrics.font_width + metrics.x_spacing);
                }
                KA_SPCTAB => {
                    self.flush_line(metrics, &mut ops);
                    let old_x = self.cursor_x;
                    let x = self.win_x.wrapping_shl(3);
                    let y = self.win_y.wrapping_add(self.cursor_y.wrapping_mul(metrics.font_height + metrics.y_spacing));
                    self.window_pos = ((self.window_pos.wrapping_add(8)) & 0xfff8) - 1;
                    if self.window_pos >= self.window_columns {
                        self.cursor_x = 0;
                        self.window_pos = 0;
                        ops.push(WindowDrawOp::FillRect {
                            left: x + old_x, top: y, right: x + self.window_width - 1,
                            bottom: y + metrics.font_height + metrics.y_spacing - 1, color: self.background,
                        });
                        self.scroll_counter = self.scroll_counter.wrapping_add(1);
                        self.cursor_y = self.cursor_y.wrapping_add(1);
                    } else {
                        self.cursor_x = self.window_pos.wrapping_mul(metrics.font_width + metrics.x_spacing);
                        ops.push(WindowDrawOp::FillRect {
                            left: x + old_x, top: y, right: x + self.cursor_x - 1,
                            bottom: y + metrics.font_height + metrics.y_spacing - 1, color: self.background,
                        });
                    }
                }
                KA_EXTEND => self.append_char(127, metrics, &mut ops),
                _ => self.append_char(c, metrics, &mut ops),
            }
        }

        if self.pos != 0 { self.flush_line(metrics, &mut ops); }
        if let Some(window) = self.windows.get_mut(self.current_window) {
            window.cursor_x = self.cursor_x;
            window.cursor_y = self.cursor_y;
            window.foreground = old_foreground;
            window.background = old_background;
        }
        self.foreground = old_foreground;
        self.background = old_background;
        ops
    }

    fn append_char(&mut self, c: u8, metrics: WindowFontMetrics<'_>, ops: &mut Vec<WindowDrawOp>) {
        self.pixel_pos = self.pixel_pos.wrapping_add(metrics.char_width(c));
        if self.line.len() < 83 { self.line.push(c); }
        self.pos = self.line.len();
        if self.cursor_x.wrapping_add(self.pixel_pos) > self.window_width {
            self.flush_line(metrics, ops);
        }
    }

    fn emit_more_prompt(&self, metrics: WindowFontMetrics<'_>, ops: &mut Vec<WindowDrawOp>) {
        if !self.more_callback_enabled { return; }
        let line_height = metrics.font_height + metrics.y_spacing;
        let size = (self.more_space - 1).wrapping_mul(metrics.font_width + metrics.x_spacing);
        let x = (self.win_x.wrapping_add(self.win_width)).wrapping_shl(3).wrapping_sub(size);
        let y = self.win_y.wrapping_add((self.cursor_y - 1).wrapping_mul(line_height));
        let window = self.windows.get(self.current_window);
        let fg = if self.more_foreground != 0 { self.more_foreground } else { window.map_or(0, |w| w.background) };
        let bg = if self.more_background != 0 { self.more_background } else { window.map_or(0, |w| w.foreground) };
        ops.push(WindowDrawOp::Print { text: self.more_text.clone(), x, y, foreground: fg, background: bg });
        ops.push(WindowDrawOp::FillRect {
            left: x, top: y, right: x + size - 1, bottom: y + line_height - 1, color: self.background,
        });
    }

    fn scroll_window(&mut self, metrics: WindowFontMetrics<'_>, ops: &mut Vec<WindowDrawOp>) {
        if self.scroll_counter >= self.window_lines - 1 && self.more_on {
            self.scroll_counter = 0;
            self.emit_more_prompt(metrics, ops);
        }
        let line_height = metrics.font_height + metrics.y_spacing;
        let y = (self.win_height / line_height - 1).wrapping_mul(line_height);
        let x = self.win_x.wrapping_shl(3);
        ops.push(WindowDrawOp::Blit {
            source_x: x, source_y: self.win_y + line_height, dest_x: x, dest_y: self.win_y,
            width: self.win_width.wrapping_shl(3), height: self.win_height - line_height,
        });
        ops.push(WindowDrawOp::FillRect {
            left: x, top: self.win_y + y, right: (self.win_x + self.win_width).wrapping_shl(3) - 1,
            bottom: self.win_y + self.win_height - 1, color: self.background,
        });
    }

    fn flush_line(&mut self, metrics: WindowFontMetrics<'_>, ops: &mut Vec<WindowDrawOp>) {
        let line_height = metrics.font_height + metrics.y_spacing;
        while self.cursor_y >= self.window_lines {
            self.scroll_window(metrics, ops);
            if self.cursor_y >= self.window_lines { self.cursor_y -= 1; }
        }
        let x = self.win_x.wrapping_shl(3).wrapping_add(self.cursor_x);
        let y = self.win_y.wrapping_add(self.cursor_y.wrapping_mul(line_height));
        let mut break_width = self.window_width;
        let mut breakpoint = self.pos;
        let mut break_size = self.pixel_pos;

        if self.cursor_x.wrapping_add(break_size) > break_width {
            if self.scroll_counter >= self.window_lines - 1 && self.more_on {
                break_width = break_width.wrapping_sub(self.more_space.wrapping_mul(metrics.font_width + metrics.x_spacing));
            }
            break_width = break_width.wrapping_sub(self.cursor_x);
            let mut break_it = 0usize;
            let mut index = breakpoint.saturating_sub(1);
            while index > 0 {
                let break_char = self.line[index];
                break_size = break_size.wrapping_sub(metrics.char_width(break_char));
                if break_it == 0 && break_size <= break_width { break_it = index; }
                if break_it != 0 && break_char == KA_SPACE {
                    breakpoint = index;
                    self.word_wrap = false;
                    break;
                }
                index -= 1;
            }
            if index == 0 {
                if self.cursor_x != 0 && !self.word_wrap {
                    breakpoint = 0;
                    break_size = 0;
                    self.word_wrap = true;
                } else {
                    breakpoint = break_it;
                }
            }
        }

        let break_char = self.line.get(breakpoint).copied().unwrap_or(0);
        let printed = self.line[..breakpoint.min(self.line.len())].to_vec();
        ops.push(WindowDrawOp::Print { text: printed, x, y, foreground: self.foreground, background: self.background });
        self.cursor_x = self.cursor_x.wrapping_add(break_size);
        let mut next = breakpoint;
        if break_char == KA_SPACE { next += 1; }
        if self.line.get(next).copied() == Some(KA_SPACE) { next += 1; }
        let remainder = self.line.get(next..).unwrap_or(&[]).to_vec();
        self.line = remainder;
        self.pos = self.line.len();
        self.pixel_pos = metrics.string_width(&self.line);

        if self.pos != 0 || self.cursor_x >= self.window_width {
            self.cursor_x = 0;
            self.window_pos = 0;
            self.cursor_y = self.cursor_y.wrapping_add(1);
            self.flush_line(metrics, ops);
            self.scroll_counter = self.scroll_counter.wrapping_add(1);
        }
    }
}

struct LegacyCharStream<'a> {
    main: &'a [u8],
    main_pos: usize,
    alternate: Option<Vec<u8>>,
    alternate_pos: usize,
    stack: u8,
    pair: [u8; 2],
    args: &'a [WindowArg],
    arg_pos: usize,
}
impl<'a> LegacyCharStream<'a> {
    fn new(main: &'a [u8], args: &'a [WindowArg]) -> Self {
        Self { main: nul_terminated(main), main_pos: 0, alternate: None, alternate_pos: 0, stack: 0, pair: [0, 0], args, arg_pos: 0 }
    }
    fn peek(&self) -> Option<u8> {
        if self.pair[0] != 0 { Some(self.pair[0]) }
        else if let Some(alt) = &self.alternate { alt.get(self.alternate_pos).copied().filter(|b| *b != 0) }
        else { self.main.get(self.main_pos).copied().filter(|b| *b != 0) }
    }
    fn push_text_arg(&mut self) -> bool {
        let arg = self.args.get(self.arg_pos);
        self.arg_pos += 1;
        match arg {
            Some(WindowArg::Text(Some(text))) => {
                self.stack = self.pair[1];
                self.pair = [0, 0];
                self.alternate = Some(nul_terminated(text).to_vec());
                self.alternate_pos = 0;
                true
            }
            _ => false,
        }
    }
    fn push_integer_arg(&mut self) -> bool {
        let arg = self.args.get(self.arg_pos);
        self.arg_pos += 1;
        if let Some(WindowArg::Integer(value)) = arg {
            self.stack = self.pair[1];
            self.pair = [0, 0];
            self.alternate = Some(value.to_string().into_bytes());
            self.alternate_pos = 0;
            true
        } else { false }
    }
    fn in_char(&mut self) {
        let mut c = 0u8;
        if let Some(alt) = &self.alternate {
            if self.alternate_pos >= alt.len() {
                self.alternate = None;
                c = self.stack;
            } else {
                c = alt[self.alternate_pos];
                self.alternate_pos += 1;
            }
        }
        if c == 0 && self.main_pos < self.main.len() {
            c = self.main[self.main_pos];
            self.main_pos += 1;
        }
        let mut next = 0;
        if c & 0x80 != 0 {
            let packed = c & 0x7f;
            let row = ((packed & 0x78) >> 3) as usize;
            let col = (packed & 7) as usize;
            next = DIPTHONG[row][col];
            c = COMMON[row];
        }
        self.pair = [c, next];
    }
    fn fetch(&mut self) -> Option<u8> {
        if self.pair[0] == 0 { self.in_char(); }
        let c = self.pair[0];
        self.pair[0] = self.pair[1];
        self.pair[1] = 0;
        if self.pair[0] == 0 { self.in_char(); }
        if c == 0 { None } else { Some(c) }
    }
}
fn nul_terminated(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metrics<'a>(widths: &'a [u8]) -> WindowFontMetrics<'a> {
        WindowFontMetrics { widths, font_width: 8, font_height: 8, x_spacing: 0, y_spacing: 0 }
    }
    fn engine() -> WindowTextEngine {
        WindowTextEngine::new(vec![WindowDefinition {
            x: 2, y: 3, width_bytes: 10, height_pixels: 32,
            foreground: 7, background: 1, cursor_x: 0, cursor_y: 0,
        }])
    }

    #[test]
    fn change_and_clear_window_use_source_pixel_units() {
        let mut e = engine();
        let m = metrics(&[0; 256]);
        assert_eq!(e.change_window(0, m), Some(0));
        assert_eq!(e.window_width, 80);
        assert_eq!(e.window_columns, 10);
        let ops = e.new_window();
        assert_eq!(ops, vec![WindowDrawOp::FillRect { left: 16, top: 3, right: 95, bottom: 34, color: 1 }]);
        assert_eq!((e.cursor_x, e.cursor_y), (0, 0));
    }

    #[test]
    fn wraps_words_and_restores_window_colors() {
        let mut e = engine();
        let m = metrics(&[0; 256]);
        e.change_window(0, m);
        let ops = e.window_print(b"one two three", &[], m);
        assert!(ops.iter().any(|op| matches!(op, WindowDrawOp::Print { text, .. } if text == b"one two")));
        assert_eq!(e.foreground, 7);
        assert_eq!(e.background, 1);
    }

    #[test]
    fn supports_dipthongs_and_printf_substitutions() {
        let mut e = engine();
        let m = metrics(&[8; 256]);
        e.change_window(0, m);
        let ops = e.window_print(b"Hi %s %d", &[WindowArg::Text(Some(b"there".to_vec())), WindowArg::Integer(42)], m);
        let printed: Vec<u8> = ops.into_iter().filter_map(|op| match op { WindowDrawOp::Print { text, .. } => Some(text), _ => None }).flatten().collect();
        assert!(printed.windows(5).any(|w| w == b"there"));
        assert!(printed.windows(2).any(|w| w == b"42"));
    }

    #[test]
    fn handles_control_codes_and_more_prompt() {
        let mut e = engine();
        let m = metrics(&[8; 256]);
        e.change_window(0, m);
        let ops = e.window_print(&[b'a', KA_MORE, b'b', KA_RETURN, b'c', 0], &[], m);
        assert!(ops.iter().any(|op| matches!(op, WindowDrawOp::Print { text, .. } if text == b"--More--")));
        assert!(ops.iter().any(|op| matches!(op, WindowDrawOp::Print { text, .. } if text == b"a")));
        assert!(ops.iter().any(|op| matches!(op, WindowDrawOp::Print { text, .. } if text == b"bc")));
    }

    #[test]
    fn more_prompt_defaults_and_toggle_match_globals() {
        let mut e = engine();
        e.set_more_prompt(None, 0, 1, 1);
        assert_eq!(e.more_text, b"<MORE>");
        e.set_more_off();
        assert!(!e.more_on);
        e.set_more_on();
        assert!(e.more_on);
        assert_eq!(e.scroll_counter, 0);
    }
}
