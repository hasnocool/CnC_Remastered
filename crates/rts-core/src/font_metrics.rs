//! Font width and text-print cursor helpers from WIN32LIB/FONT.CPP.

/// Width lookup plus the global horizontal character spacing used by Set_Font.
#[derive(Debug, Clone, Copy)]
pub struct FontMetrics<'a> {
    pub widths: &'a [u8],
    pub x_spacing: i32,
}

impl FontMetrics<'_> {
    /// Equivalent to Char_Pixel_Width: unsigned byte width plus FontXSpacing.
    pub fn char_pixel_width(&self, character: u8) -> i32 {
        self.widths
            .get(character as usize)
            .copied()
            .unwrap_or(0) as i32
            + self.x_spacing
    }

    /// Equivalent to String_Pixel_Width. CR starts a new line; the maximum
    /// line width is returned. The original accumulator is a 16-bit WORD.
    pub fn string_pixel_width(&self, string: Option<&[u8]>) -> u16 {
        let Some(bytes) = string else { return 0 };
        let mut width = 0u16;
        let mut largest = 0u16;
        for byte in bytes.iter().copied().take_while(|b| *b != 0) {
            if byte == b'\r' {
                largest = largest.max(width);
                width = 0;
            } else {
                width = width.wrapping_add(self.char_pixel_width(byte) as u16);
            }
        }
        largest.max(width)
    }
}

/// The viewport values used by Get_Next_Text_Print_XY.
pub trait TextPrintViewport {
    fn width(&self) -> u32;
    fn x_add(&self) -> u32;
    fn offset(&self) -> u32;
}

/// Convert Text_Print's returned buffer offset into viewport-relative X/Y.
/// Arithmetic follows the source's unsigned 32-bit offset calculations.
pub fn get_next_text_print_xy<V: TextPrintViewport>(
    viewport: &V,
    offset: u32,
) -> (i32, i32) {
    if offset == 0 {
        return (0, 0);
    }
    let buffer_width = viewport.width().wrapping_add(viewport.x_add());
    if buffer_width == 0 {
        return (0, 0);
    }
    let relative = offset.wrapping_sub(viewport.offset());
    ((relative % buffer_width) as i32, (relative / buffer_width) as i32)
}
