// Filename: crates/rts-formats/src/palette_rgba.rs
//! Explicit palette-indexed image conversion for the Westwood 8-bit path.
//! Upstream LOADPAL.CPP loads 256 RGB triples (768 bytes) on IBM. Encoding is
//! caller-selected; this module never guesses whether components are RGB6/8.

use std::fmt;
pub const PALETTE_COLORS: usize = 256;
pub const PALETTE_BYTES: usize = PALETTE_COLORS * 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaletteEncoding { Rgb8, Rgb6 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgb8 { pub r: u8, pub g: u8, pub b: u8 }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Palette256 { colors: [Rgb8; PALETTE_COLORS] }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaletteError {
    InvalidLength { expected: usize, actual: usize },
    InvalidRgb6Component { byte_index: usize, value: u8 },
    DimensionOverflow,
    PixelCountMismatch { expected: usize, actual: usize },
}
impl fmt::Display for PaletteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for PaletteError {}

impl Palette256 {
    pub fn parse(bytes: &[u8], encoding: PaletteEncoding) -> Result<Self, PaletteError> {
        if bytes.len() != PALETTE_BYTES {
            return Err(PaletteError::InvalidLength { expected: PALETTE_BYTES, actual: bytes.len() });
        }
        let mut colors = [Rgb8 { r: 0, g: 0, b: 0 }; PALETTE_COLORS];
        for (index, rgb) in bytes.chunks_exact(3).enumerate() {
            let convert = |value: u8, byte_index: usize| match encoding {
                PaletteEncoding::Rgb8 => Ok(value),
                PaletteEncoding::Rgb6 if value <= 63 => Ok((value << 2) | (value >> 4)),
                PaletteEncoding::Rgb6 => Err(PaletteError::InvalidRgb6Component { byte_index, value }),
            };
            colors[index] = Rgb8 {
                r: convert(rgb[0], index * 3)?,
                g: convert(rgb[1], index * 3 + 1)?,
                b: convert(rgb[2], index * 3 + 2)?,
            };
        }
        Ok(Self { colors })
    }

    pub fn color(&self, index: u8) -> Rgb8 { self.colors[index as usize] }
    pub fn colors(&self) -> &[Rgb8; PALETTE_COLORS] { &self.colors }

    /// Convert row-major indexed pixels to tightly packed RGBA8. Transparency
    /// is opt-in: terrain can remain opaque while sprite index zero is clear.
    pub fn expand_rgba(&self, width: u32, height: u32, indices: &[u8],
        transparent_index: Option<u8>) -> Result<RgbaImage, PaletteError> {
        let count = (width as usize).checked_mul(height as usize)
            .ok_or(PaletteError::DimensionOverflow)?;
        if indices.len() != count {
            return Err(PaletteError::PixelCountMismatch { expected: count, actual: indices.len() });
        }
        let capacity = count.checked_mul(4).ok_or(PaletteError::DimensionOverflow)?;
        let mut pixels = Vec::with_capacity(capacity);
        for &index in indices {
            let c = self.color(index);
            pixels.extend_from_slice(&[c.r, c.g, c.b,
                if transparent_index == Some(index) { 0 } else { 255 }]);
        }
        Ok(RgbaImage { width, height, pixels })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RgbaImage {
    pub width: u32,
    pub height: u32,
    /// Row-major RGBA8, directly suitable for the renderer texture upload API.
    pub pixels: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rgb8_expansion_and_transparency_are_explicit() {
        let mut bytes = vec![0; PALETTE_BYTES];
        bytes[3..6].copy_from_slice(&[12, 34, 56]);
        let palette = Palette256::parse(&bytes, PaletteEncoding::Rgb8).unwrap();
        let image = palette.expand_rgba(2, 1, &[1, 0], Some(0)).unwrap();
        assert_eq!(image.pixels, [12, 34, 56, 255, 0, 0, 0, 0]);
    }
    #[test]
    fn rgb6_expands_to_the_full_eight_bit_range() {
        let mut bytes = vec![0; PALETTE_BYTES];
        bytes[..3].copy_from_slice(&[0, 32, 63]);
        let p = Palette256::parse(&bytes, PaletteEncoding::Rgb6).unwrap();
        assert_eq!(p.color(0), Rgb8 { r: 0, g: 130, b: 255 });
    }
    #[test]
    fn malformed_palette_and_pixel_buffers_are_rejected() {
        assert!(matches!(Palette256::parse(&[0; 3], PaletteEncoding::Rgb8),
            Err(PaletteError::InvalidLength { .. })));
        let p = Palette256::parse(&vec![0; PALETTE_BYTES], PaletteEncoding::Rgb8).unwrap();
        assert!(matches!(p.expand_rgba(2, 2, &[0; 3], None),
            Err(PaletteError::PixelCountMismatch { .. })));
    }
}
