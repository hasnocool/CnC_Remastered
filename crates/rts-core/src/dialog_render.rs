//! Dialog boxes, list/window boxes and text rendering from both variants'
//! of DIALOG.CPP. This module preserves the source's layout/font/palette
//! arithmetic and exposes DirectDraw/LogicPage operations through DialogHost.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DialogVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DialogFont {
    Default, Score, Metal12, Map, Vcr, Gradient6, Point3, Point6, Editor, Point8,
    Led, Type, Green12, Green12Gradient,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RemapScheme {
    pub color: u8,
    pub bright: u8,
    pub bright_color: u8,
    pub background: u8,
    pub shadow: u8,
    pub highlight: u8,
    pub corners: u8,
    pub box_color: u8,
    pub font_remap: [u8; 16],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoxStyleColors {
    pub filler: i32,
    pub shadow: i32,
    pub highlight: i32,
    pub corner: i32,
    pub use_green_texture: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontCodes {
    pub score: u32,
    pub metal12: u32,
    pub map: u32,
    pub vcr: u32,
    pub gradient6: u32,
    pub point3: u32,
    pub point6: u32,
    pub editor: u32,
    pub point8: u32,
    pub led: u32,
    pub font_type: u32,
    pub green12: u32,
    pub green12_gradient: u32,
    pub center: u32,
    pub right: u32,
    pub no_shadow: u32,
    pub drop_shadow: u32,
    pub full_shadow: u32,
    pub light_shadow: u32,
    pub use_gradient_palette: u32,
    pub medium_color: u32,
    pub bright_color: u32,
    pub vcr_palette_type: u32,
    pub six_point_gradient: u32,
    pub metal_gradient: u32,
    pub editor_gradient: u32,
    pub type_gradient: u32,
    pub text_caption: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub trait DialogHost {
    fn font_codes(&self) -> FontCodes;
    fn red_alert_default_remap(&self) -> RemapScheme;
    fn box_style_colors(&self, variant: DialogVariant, style: usize) -> BoxStyleColors;
    fn window_rect(&self, window: usize) -> WindowRect;
    fn current_logic_page_is_seen_buffer(&self) -> bool;
    fn conditional_hide_mouse(&mut self, left: i32, top: i32, right: i32, bottom: i32);
    fn conditional_show_mouse(&mut self);

    fn draw_shape_asset(&mut self, name: &'static str, frame: usize, x: i32, y: i32);
    fn draw_red_alert_dialog_background(&mut self, x: i32, y: i32, width: i32, height: i32, resolution_factor: i32);
    fn texture_fill_green(&mut self, x: i32, y: i32, width: i32, height: i32);
    fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: i32);
    fn draw_rect(&mut self, left: i32, top: i32, right: i32, bottom: i32, color: i32);
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: i32);
    fn put_pixel(&mut self, x: i32, y: i32, color: i32);

    fn font_height(&self) -> i32;
    fn font_y_spacing(&self) -> i32;
    fn char_pixel_width(&self, byte: u8) -> i32;
    fn string_pixel_width(&self, text: &str) -> i32;
    fn text_string(&self, text_id: i32) -> Option<String>;
    fn set_font(&mut self, font: DialogFont);
    fn set_font_spacing(&mut self, x_spacing: i32, y_spacing: i32);
    fn set_font_palette(&mut self, palette: [u8; 16]);
    fn logic_page_size(&self) -> (i32, i32);
    fn print_text(&mut self, text: &str, x: i32, y: i32, foreground: i32, background: i32);
    fn color_remap_foreground(&self, scheme: &RemapScheme) -> i32;
    fn font_palette(&self, color: i32) -> Option<[u8; 16]>;
    fn debug_map_enabled(&self) -> bool;
}

pub const TIBERIAN_DAWN_FONT_PALETTE: [[u8; 16]; 16] = [
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,26,0,0,0,0,0,0,0,0,0,27,27,26,25,24],
    [0,135,0,0,0,0,0,0,0,0,0,136,136,135,119,2],
    [0,159,0,0,0,0,0,0,0,0,0,142,143,159,41,167],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,157,0,0,0,0,0,0,0,0,0,180,180,157,158,5],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,179,0,0,0,0,0,0,0,0,0,180,180,179,178,176],
    [0,123,0,0,0,0,0,0,0,0,0,122,122,123,125,127],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,203,0,0,0,0,0,0,0,0,0,204,204,203,202,201],
    [0,1,4,166,41,3,0,0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,203,0,0,0,0,0,0,0,0,0,204,204,203,202,201],
    [0,203,0,0,0,0,0,0,0,0,0,204,204,203,202,201],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
];
pub const TIBERIAN_DAWN_MEDIUM_PALETTE: [u8; 16] =
    [0,25,119,41,0,158,0,178,125,0,202,0,0,0,0,0];
pub const TIBERIAN_DAWN_BRIGHT_PALETTE: [u8; 16] =
    [0,24,2,4,0,5,0,176,127,0,201,0,0,0,0,0];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DialogPrintState {
    pub font: DialogFont,
    pub x_spacing: i32,
    pub y_spacing: i32,
    pub foreground: i32,
    pub background: i32,
    pub palette: [u8; 16],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormatWindowResult {
    pub lines: usize,
    pub width: i32,
    pub height: i32,
}

pub fn draw_box(
    host: &mut impl DialogHost,
    variant: DialogVariant,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    style: usize,
    filled: bool,
) {
    let w = width - 1;
    let h = height - 1;
    let colors = host.box_style_colors(variant, style);
    if filled {
        if colors.use_green_texture {
            host.texture_fill_green(x, y, w, h);
        } else {
            host.fill_rect(x, y, w, h, colors.filler);
        }
    }

    let (box_style, border_style) = match variant {
        DialogVariant::RedAlert => (4usize, 5usize),
        DialogVariant::TiberianDawn => (10usize, 11usize),
    };
    if style == box_style {
        host.draw_rect(x, y, x + w, y + h, colors.highlight);
    } else if style == border_style {
        host.draw_rect(x + 1, y + 1, x + w - 1, y + h - 1, colors.highlight);
    } else {
        host.draw_line(x, y + h, x + w, y + h, colors.shadow);
        host.draw_line(x + w, y, x + w, y + h, colors.shadow);
        host.draw_line(x, y, x + w, y, colors.highlight);
        host.draw_line(x, y, x, y + h, colors.highlight);
        host.put_pixel(x, y + h, colors.corner);
        host.put_pixel(x + w, y, colors.corner);
    }
}

pub fn dialog_box(
    host: &mut impl DialogHost,
    variant: DialogVariant,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    resolution_factor: i32,
) {
    match variant {
        DialogVariant::RedAlert => {
            host.draw_red_alert_dialog_background(x, y, width, height, resolution_factor);
        }
        DialogVariant::TiberianDawn => {
            draw_box(host, variant, x, y, width, height, 11, true);
        }
    }
}

pub fn window_box(
    host: &mut impl DialogHost,
    variant: DialogVariant,
    window: usize,
    style: usize,
) {
    let bounds = host.window_rect(window);
    let (x, y, width, height) = match variant {
        DialogVariant::RedAlert => (bounds.x, bounds.y, bounds.width, bounds.height),
        DialogVariant::TiberianDawn => (
            bounds.x << 3,
            bounds.y,
            bounds.width << 3,
            bounds.height,
        ),
    };

    let seen = host.current_logic_page_is_seen_buffer();
    if seen {
        host.conditional_hide_mouse(x, y, x + width, y + height);
    }
    draw_box(host, variant, x, y, width, height, style, true);

    if variant == DialogVariant::TiberianDawn {
        const BORDERS: [(i32, i32); 11] = [
            (0,0), (2,4), (1,1), (2,1), (0,0), (20,0),
            (0,0), (2,4), (0,0), (20,0), (0,1),
        ];
        if let Some((border, _)) = BORDERS.get(style).copied() {
            if border != 0 {
                draw_box(host, variant, x + border, y + border,
                    width - (border << 1), height - (border << 1), style, false);
            }
        }
    }
    if seen { host.conditional_show_mouse(); }
}

/// Format_Window_String modifies '@' into CR, wraps at the last preceding
/// space, and reports the maximum rendered line width and total font height.
/// The source edits the supplied C string in place; this byte-slice form does too.
pub fn format_window_string(
    host: &impl DialogHost,
    variant: DialogVariant,
    text: &mut [u8],
    max_line_length: i32,
) -> FormatWindowResult {
    let mut width = 0i32;
    let mut height = 0i32;
    let mut lines = 0usize;
    let mut cursor = 0usize;

    while cursor < text.len() && text[cursor] != 0 {
        if text[cursor] == 0 { break; }
        let mut line_width = 0i32;
        height += host.font_height() + host.font_y_spacing();
        lines += 1;

        if variant == DialogVariant::RedAlert && text[cursor] == b'@' {
            text[cursor] = b'\r';
        }

        while cursor < text.len()
            && text[cursor] != 0
            && text[cursor] != b'\r'
            && (variant == DialogVariant::TiberianDawn || text[cursor] != b'@')
            && line_width < max_line_length
        {
            line_width += host.char_pixel_width(text[cursor]);
            cursor += 1;
        }

        if line_width >= max_line_length {
            while cursor > 0
                && cursor < text.len()
                && text[cursor] != b' '
                && text[cursor] != b'\r'
                && text[cursor] != 0
                && (variant == DialogVariant::TiberianDawn || text[cursor] != b'@')
            {
                cursor -= 1;
                line_width -= host.char_pixel_width(text[cursor]);
            }
        }

        width = width.max(line_width);
        if cursor < text.len() && text[cursor] != 0 {
            text[cursor] = b'\r';
            cursor += 1;
        }
    }

    FormatWindowResult { lines, width, height }
}

fn select_font(point: u32, variant: DialogVariant, codes: FontCodes) -> (DialogFont, i32, i32, bool) {
    let mut x_spacing = 1;
    let mut y_spacing = 0;
    let mut strip_shadows = false;
    let font = if variant == DialogVariant::TiberianDawn {
        if point == codes.green12 {
            DialogFont::Green12
        } else if point == codes.green12_gradient {
            DialogFont::Green12Gradient
        } else if point == codes.map {
            x_spacing -= 1; DialogFont::Map
        } else if point == codes.vcr {
            DialogFont::Vcr
        } else if point == codes.gradient6 {
            x_spacing -= 1; DialogFont::Gradient6
        } else if point == codes.point3 {
            x_spacing += 1; strip_shadows = true; DialogFont::Point3
        } else if point == codes.point6 {
            x_spacing -= 1; DialogFont::Point6
        } else if point == codes.point8 {
            x_spacing -= 2; y_spacing -= 4; DialogFont::Point8
        } else if point == codes.led {
            x_spacing -= 4; DialogFont::Led
        } else { DialogFont::Default }
    } else if point == codes.score {
        DialogFont::Score
    } else if point == codes.metal12 {
        DialogFont::Metal12
    } else if point == codes.map {
        x_spacing -= 1; DialogFont::Map
    } else if point == codes.vcr {
        DialogFont::Vcr
    } else if point == codes.gradient6 {
        x_spacing -= 1; DialogFont::Gradient6
    } else if point == codes.point3 {
        x_spacing += 1; strip_shadows = true; DialogFont::Point3
    } else if point == codes.point6 {
        x_spacing -= 1; DialogFont::Point6
    } else if point == codes.editor {
        #[cfg(windows)] { y_spacing += 1; x_spacing -= 1; }
        x_spacing -= 1; DialogFont::Editor
    } else if point == codes.point8 {
        #[cfg(windows)] { x_spacing -= 2; y_spacing -= 4; }
        #[cfg(not(windows))] { x_spacing -= 1; y_spacing -= 2; }
        DialogFont::Point8
    } else if point == codes.led {
        #[cfg(windows)] { x_spacing -= 4; }
        #[cfg(not(windows))] { x_spacing -= 2; }
        DialogFont::Led
    } else if point == codes.font_type {
        x_spacing -= 1;
        DialogFont::Type
    } else {
        DialogFont::Default
    };
    if variant == DialogVariant::RedAlert && point == codes.font_type {
        #[cfg(windows)] { x_spacing -= 2; y_spacing += 2; }
    }
    (font, x_spacing, y_spacing, strip_shadows)
}

fn build_palette(
    variant: DialogVariant,
    flags: u32,
    background: u8,
    foreground: u8,
    scheme: RemapScheme,
    codes: FontCodes,
) -> (i32, [u8; 16]) {
    let mut palette = [background; 16];
    let mut forecolor = foreground as i32;
    let point = flags & 0x0f;
    if variant == DialogVariant::RedAlert {
        let gradient_type = point == codes.vcr_palette_type
            || point == codes.gradient6
            || point == codes.metal_gradient
            || point == codes.editor_gradient
            || point == codes.type_gradient;
        if gradient_type {
            if flags & codes.use_gradient_palette != 0 {
                palette = scheme.font_remap;
                forecolor = scheme.color as i32;
                if point == codes.font_type { forecolor = palette[1] as i32; }
            } else {
                palette[4..].fill(scheme.color);
                forecolor = scheme.color as i32;
            }
            if flags & codes.medium_color != 0 {
                forecolor = scheme.color as i32;
                palette[4..].fill(scheme.color);
            }
            if flags & codes.bright_color != 0 {
                forecolor = scheme.bright as i32;
                palette[4..].fill(scheme.bright_color);
            }
        }
    } else {
        if point == codes.vcr_palette_type {
            palette[3] = 12; palette[9] = 15;
            palette[10] = 200; palette[11] = 201; palette[12] = 202;
            palette[13] = 203; palette[14] = 204; palette[15] = 205;
        }
        if point == codes.gradient6 || point == codes.metal_gradient || point == codes.editor_gradient {
            if flags & codes.use_gradient_palette != 0 {
                palette = TIBERIAN_DAWN_FONT_PALETTE[(foreground & 0x0f) as usize];
            } else {
                palette[4..].fill(foreground);
            }
            if flags & codes.medium_color != 0 {
                forecolor = TIBERIAN_DAWN_MEDIUM_PALETTE[(foreground & 0x0f) as usize] as i32;
                palette[4..].fill(forecolor as u8);
            } else if flags & codes.bright_color != 0 {
                forecolor = TIBERIAN_DAWN_BRIGHT_PALETTE[(foreground & 0x0f) as usize] as i32;
                palette[4..].fill(forecolor as u8);
            } else {
                forecolor = palette[1] as i32;
            }
        }
    }
    (forecolor, palette)
}

pub fn simple_text_print(
    host: &mut impl DialogHost,
    variant: DialogVariant,
    text: Option<&str>,
    x: u32,
    y: u32,
    foreground: u32,
    background: u32,
    flags: u32,
    scheme: Option<RemapScheme>,
) -> DialogPrintState {
    let codes = host.font_codes();
    let scheme = scheme.unwrap_or_else(|| host.red_alert_default_remap());
    let (mut forecolor, mut palette) = build_palette(
        variant, flags, background as u8, foreground as u8, scheme, codes,
    );
    let point = flags & 0x0f;
    let (font, mut x_spacing, mut y_spacing, strip_shadows) = select_font(point, variant, codes);
    let mut effective_flags = if strip_shadows {
        flags & !(codes.no_shadow | codes.drop_shadow | codes.full_shadow)
    } else {
        flags
    };

    let shadow = effective_flags & (codes.no_shadow | codes.drop_shadow | codes.full_shadow | codes.light_shadow);
    if shadow == codes.no_shadow {
        palette[2] = background as u8;
        palette[3] = background as u8;
        x_spacing -= 1;
        y_spacing -= if variant == DialogVariant::RedAlert { 2 } else { 2 };
    } else if shadow == codes.drop_shadow {
        palette[2] = 0;
        palette[3] = background as u8;
        x_spacing -= 1;
    } else if shadow == codes.light_shadow {
        palette[2] = 232;
        palette[3] = background as u8;
        x_spacing -= 1;
    } else if shadow == codes.full_shadow {
        palette[2] = 0;
        palette[3] = 0;
        x_spacing -= 1;
    }

    if variant == DialogVariant::RedAlert {
        if point != codes.font_type {
            palette[0] = background as u8;
            palette[1] = scheme.color;
        }
    } else {
        palette[0] = background as u8;
        palette[1] = forecolor as u8;
    }

    host.set_font_spacing(x_spacing, y_spacing);
    host.set_font(font);
    host.set_font_palette(palette);

    if let Some(text) = text.filter(|text| !text.is_empty()) {
        let mut draw_x = x as i32;
        match effective_flags & (codes.center | codes.right) {
            value if value == codes.center => draw_x -= host.string_pixel_width(text) >> 1,
            value if value == codes.right => draw_x -= host.string_pixel_width(text),
            _ => {}
        }
        let (page_width, page_height) = host.logic_page_size();
        if draw_x < page_width && (y as i32) < page_height {
            if variant == DialogVariant::TiberianDawn {
                let cleaned: String = text.chars().filter(|ch| *ch != '\u{00ff}').collect();
                if !cleaned.is_empty() {
                    host.print_text(&cleaned, draw_x, y as i32, forecolor, background as i32);
                }
            } else {
                host.print_text(text, draw_x, y as i32, host.color_remap_foreground(&scheme), background as i32);
            }
        }
    }

    DialogPrintState {
        font,
        x_spacing,
        y_spacing,
        foreground: forecolor,
        background: background as i32,
        palette,
    }
}

pub fn fancy_text_print(
    host: &mut impl DialogHost,
    variant: DialogVariant,
    text: Option<&str>,
    x: u32,
    y: u32,
    foreground: u32,
    background: u32,
    flags: u32,
    scheme: Option<RemapScheme>,
) -> DialogPrintState {
    simple_text_print(host, variant, text, x, y, foreground, background, flags, scheme)
}

pub fn fancy_text_print_id(
    host: &mut impl DialogHost,
    variant: DialogVariant,
    text_id: i32,
    x: u32,
    y: u32,
    foreground: u32,
    background: u32,
    flags: u32,
    scheme: Option<RemapScheme>,
) -> DialogPrintState {
    let text = if text_id == -1 { None } else { host.text_string(text_id) };
    simple_text_print(host, variant, text.as_deref(), x, y, foreground, background, flags, scheme)
}

pub fn plain_text_print(
    host: &mut impl DialogHost,
    text: Option<&str>,
    x: u32,
    y: u32,
    foreground: u32,
    background: u32,
    flags: u32,
) -> DialogPrintState {
    let mut scheme = RemapScheme::default();
    scheme.color = foreground as u8;
    scheme.bright = foreground as u8;
    scheme.bright_color = foreground as u8;
    scheme.background = foreground as u8;
    scheme.shadow = foreground as u8;
    scheme.highlight = foreground as u8;
    scheme.corners = foreground as u8;
    scheme.box_color = foreground as u8;
    scheme.font_remap[4..].fill(foreground as u8);
    fancy_text_print(host, DialogVariant::RedAlert, text, x, y, foreground, background, flags, Some(scheme))
}

/// Conquer_Clip_Text_Print divides a string at tabs, clips by character width,
/// and advances to the supplied absolute tab stops. The fallback stop
/// expression preserves the original operator precedence.
pub fn conquer_clip_text_print(
    host: &mut impl DialogHost,
    variant: DialogVariant,
    text: Option<&str>,
    x: u32,
    y: u32,
    foreground: u32,
    background: u32,
    flags: u32,
    max_width: u32,
    tabs: Option<&[i32]>,
    scheme: Option<RemapScheme>,
) -> Vec<(String, u32)> {
    let Some(text) = text else { return Vec::new() };
    if variant == DialogVariant::TiberianDawn {
        simple_text_print(host, variant, None, 0, 0, foreground, background, flags, scheme);
    }

    let bytes = text.as_bytes();
    let mut cursor = 0usize;
    let mut offset = 0u32;
    let mut tab_index = 0usize;
    let mut fragments = Vec::new();

    while cursor < bytes.len() && offset < max_width {
        let start = cursor;
        while cursor < bytes.len() && bytes[cursor] != b'\t' { cursor += 1; }
        let tab_at = (cursor < bytes.len()).then_some(cursor);
        let mut end = cursor;

        if end > start {
            let mut fragment_width = 0i32;
            let mut scan = start;
            while scan < end && (offset as i32 + fragment_width) < max_width as i32 {
                fragment_width += host.char_pixel_width(bytes[scan]);
                scan += 1;
            }
            if offset as i32 + fragment_width >= max_width as i32 {
                if scan > start {
                    scan -= 1;
                    fragment_width -= host.char_pixel_width(bytes[scan]);
                }
                end = scan;
            }
            if end > start {
                let fragment = String::from_utf8_lossy(&bytes[start..end]).into_owned();
                simple_text_print(
                    host, variant, Some(&fragment), x + offset, y,
                    foreground, background, flags, scheme,
                );
                fragments.push((fragment, x + offset));
                offset = offset.saturating_add(fragment_width.max(0) as u32);
            }
        }

        if let Some(tab) = tab_at {
            if let Some(tab_stops) = tabs {
                while tab_index < tab_stops.len() && (offset as i32) > tab_stops[tab_index] {
                    tab_index += 1;
                }
                if let Some(stop) = tab_stops.get(tab_index) {
                    offset = (*stop).max(0) as u32;
                }
            } else {
                offset = ((offset + (1 / 50) + 1) * 50) / 1;
            }
            cursor = tab + 1;
        } else {
            break;
        }
        if end < cursor && end < bytes.len() && bytes[end] != b'\t' {
            break;
        }
    }
    fragments
}

pub fn draw_caption(
    host: &mut impl DialogHost,
    text: Option<&str>,
    x: i32,
    y: i32,
    width: i32,
    resolution_factor: i32,
) {
    let Some(text) = text.filter(|value| !value.is_empty()) else { return };
    if host.debug_map_enabled() {
        let flags = host.font_codes().center | host.font_codes().editor | host.font_codes().use_gradient_palette | host.font_codes().no_shadow;
        fancy_text_print(host, DialogVariant::RedAlert, Some(text), (width / 2 + x) as u32,
            (2 * resolution_factor + y) as u32, 0, 0, flags, None);
    } else {
        let flags = host.font_codes().center | host.font_codes().text_caption;
        fancy_text_print(host, DialogVariant::RedAlert, Some(text), (width / 2 + x) as u32,
            (8 * resolution_factor + y) as u32, 0, 0, flags, None);
        let length = host.string_pixel_width(text);
        let scheme = host.red_alert_default_remap();
        host.draw_line(
            (x + (width / 2)) - (length / 2),
            y + host.font_height() + host.font_y_spacing() + (8 * resolution_factor),
            (x + (width / 2)) + (length / 2),
            y + host.font_height() + host.font_y_spacing() + (8 * resolution_factor),
            scheme.box_color as i32,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeHost {
        codes: FontCodes,
        scheme: RemapScheme,
        boxes: Vec<(i32,i32,i32,i32)>,
        rects: Vec<(i32,i32,i32,i32,i32)>,
        fills: Vec<(i32,i32,i32,i32,i32)>,
        lines: Vec<(i32,i32,i32,i32,i32)>,
        pixels: Vec<(i32,i32,i32)>,
        printed: Vec<(String,i32,i32,i32,i32)>,
        fonts: Vec<DialogFont>,
        spacing: (i32,i32),
        palette: [u8;16],
        font_h: i32,
        font_space: i32,
        page: (i32,i32),
        seen: bool,
        hidden: usize,
        shown: usize,
    }
    impl DialogHost for FakeHost {
        fn font_codes(&self)->FontCodes { self.codes }
        fn red_alert_default_remap(&self)->RemapScheme { self.scheme }
        fn box_style_colors(&self,_:DialogVariant,_:usize)->BoxStyleColors { BoxStyleColors{filler:1,shadow:2,highlight:3,corner:4,use_green_texture:false} }
        fn window_rect(&self,_:usize)->WindowRect { WindowRect{x:2,y:3,width:20,height:12} }
        fn current_logic_page_is_seen_buffer(&self)->bool { self.seen }
        fn conditional_hide_mouse(&mut self,l:i32,t:i32,r:i32,b:i32){self.hidden+=1;self.rects.push((l,t,r,b,0));}
        fn conditional_show_mouse(&mut self){self.shown+=1;}
        fn draw_shape_asset(&mut self,_:&'static str,_:usize,_:i32,_:i32){}
        fn draw_red_alert_dialog_background(&mut self,_:i32,_:i32,_:i32,_:i32,_:i32){}
        fn texture_fill_green(&mut self,_:i32,_:i32,_:i32,_:i32){}
        fn fill_rect(&mut self,x:i32,y:i32,w:i32,h:i32,c:i32){self.fills.push((x,y,w,h,c));}
        fn draw_rect(&mut self,l:i32,t:i32,r:i32,b:i32,c:i32){self.rects.push((l,t,r,b,c));}
        fn draw_line(&mut self,x1:i32,y1:i32,x2:i32,y2:i32,c:i32){self.lines.push((x1,y1,x2,y2,c));}
        fn put_pixel(&mut self,x:i32,y:i32,c:i32){self.pixels.push((x,y,c));}
        fn font_height(&self)->i32{self.font_h.max(8)}
        fn font_y_spacing(&self)->i32{self.font_space}
        fn char_pixel_width(&self,_:u8)->i32{8}
        fn string_pixel_width(&self,s:&str)->i32{s.len() as i32*8}
        fn text_string(&self,_:i32)->Option<String>{Some("hello".into())}
        fn set_font(&mut self,font:DialogFont){self.fonts.push(font);}
        fn set_font_spacing(&mut self,x:i32,y:i32){self.spacing=(x,y);}
        fn set_font_palette(&mut self,p:[u8;16]){self.palette=p;}
        fn logic_page_size(&self)->(i32,i32){self.page}
        fn print_text(&mut self,s:&str,x:i32,y:i32,f:i32,b:i32){self.printed.push((s.into(),x,y,f,b));}
        fn color_remap_foreground(&self,s:&RemapScheme)->i32{s.color as i32}
        fn font_palette(&self,_:i32)->Option<[u8;16]>{None}
        fn debug_map_enabled(&self)->bool{false}
    }

    #[test]
    fn draw_box_matches_bevel_line_and_corner_sequence() {
        let mut host=FakeHost::default();
        draw_box(&mut host,DialogVariant::RedAlert,10,20,30,40,0,true);
        assert_eq!(host.fills,vec![(10,20,29,39,1)]);
        assert_eq!(host.lines.len(),4);
        assert_eq!(host.pixels.len(),2);
    }
    #[test]
    fn tiberian_dawn_dialog_box_uses_green_border_style() {
        let mut host=FakeHost::default();
        dialog_box(&mut host,DialogVariant::TiberianDawn,1,2,80,50,2);
        assert_eq!(host.rects.last(),Some(&(2,3,78,51,3)));
    }
    #[test]
    fn format_window_string_wraps_and_converts_red_alert_at_sign() {
        let mut host=FakeHost::default();
        let mut text=b"one@two\0".to_vec();
        let result=format_window_string(&host,DialogVariant::RedAlert,&mut text,200);
        assert_eq!(result.lines,2);
        assert!(text.contains(&b'\r'));
    }
    #[test]
    fn window_box_tiberian_coordinates_are_scaled_and_have_secondary_border() {
        let mut host=FakeHost{seen:true,..FakeHost::default()};
        window_box(&mut host,DialogVariant::TiberianDawn,0,1);
        assert_eq!(host.hidden,1);
        assert_eq!(host.shown,1);
        assert_eq!(host.fills[0].0,16);
        assert_eq!(host.fills[0].1,3);
    }
    #[test]
    fn tiberian_font_path_removes_ff_bytes_and_sets_green_gradient_font() {
        let mut host=FakeHost{page:(320,200),..FakeHost::default()};
        let codes=FontCodes{gradient6:1,six_point_gradient:1,use_gradient_palette:2,medium_color:4,bright_color:8,center:16,..FontCodes::default()};
        host.codes=codes;
        simple_text_print(&mut host,DialogVariant::TiberianDawn,Some("A\u{00ff}B"),20,10,3,0,1|2|16,None);
        assert_eq!(host.fonts.last(),Some(&DialogFont::Gradient6));
        assert_eq!(host.printed.last().map(|x|x.0.as_str()),Some("AB"));
    }
}
