//! Portable BMP8 image/palette loader from REDALERT/BMP8.CPP.
//!
//! Win32 GDI handles are represented by owned decoded data and a render plan.
//! The legacy palette-channel assignment is preserved exactly: the source
//! reads RGBQUAD memory bytes (blue, green, red, reserved) into PALETTEENTRY
//! red, green, blue fields in that same order.

use std::fmt;

pub const BMP_FILE_HEADER_SIZE: usize = 14;
pub const BMP_INFO_HEADER_SIZE: usize = 40;
pub const BMP_PALETTE_ENTRIES: usize = 256;
pub const BMP_PALETTE_ENTRY_SIZE: usize = 4;
pub const BI_RGB: u32 = 0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PaletteEntry {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub flags: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Bmp8Error {
    InvalidSignature,
    TruncatedFileHeader,
    TruncatedInfoHeader,
    UnsupportedBitDepth(u16),
    UnsupportedCompression(u32),
    MissingPalette,
    InvalidPixelOffset,
    TruncatedPixelData,
    SizeOverflow,
}
impl fmt::Display for Bmp8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for Bmp8Error {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bmp8Image {
    pub file_size: usize,
    pub pixel_offset: usize,
    pub width: i32,
    pub height: i32,
    pub top_down: bool,
    pub planes: u16,
    pub bit_count: u16,
    pub compression: u32,
    pub image_size: u32,
    pub x_pixels_per_meter: i32,
    pub y_pixels_per_meter: i32,
    pub colors_used: u32,
    pub colors_important: u32,
    pub palette: [PaletteEntry; BMP_PALETTE_ENTRIES],
    pub pixels: Vec<u8>,
}

impl Bmp8Image {
    pub fn parse(file: &[u8]) -> Result<Self, Bmp8Error> {
        let fh = file.get(..BMP_FILE_HEADER_SIZE).ok_or(Bmp8Error::TruncatedFileHeader)?;
        if &fh[0..2] != b"BM" { return Err(Bmp8Error::InvalidSignature); }
        let file_size = u32::from_le_bytes([fh[2], fh[3], fh[4], fh[5]]) as usize;
        let pixel_offset = u32::from_le_bytes([fh[10], fh[11], fh[12], fh[13]]) as usize;

        let ih = file.get(BMP_FILE_HEADER_SIZE..BMP_FILE_HEADER_SIZE + BMP_INFO_HEADER_SIZE)
            .ok_or(Bmp8Error::TruncatedInfoHeader)?;
        let header_size = u32::from_le_bytes([ih[0], ih[1], ih[2], ih[3]]);
        if header_size < BMP_INFO_HEADER_SIZE as u32 { return Err(Bmp8Error::TruncatedInfoHeader); }
        let width = i32::from_le_bytes([ih[4], ih[5], ih[6], ih[7]]);
        let height = i32::from_le_bytes([ih[8], ih[9], ih[10], ih[11]]);
        let planes = u16::from_le_bytes([ih[12], ih[13]]);
        let bit_count = u16::from_le_bytes([ih[14], ih[15]]);
        if bit_count != 8 { return Err(Bmp8Error::UnsupportedBitDepth(bit_count)); }
        let compression = u32::from_le_bytes([ih[16], ih[17], ih[18], ih[19]]);
        if compression != BI_RGB { return Err(Bmp8Error::UnsupportedCompression(compression)); }
        let image_size = u32::from_le_bytes([ih[20], ih[21], ih[22], ih[23]]);
        let x_pixels_per_meter = i32::from_le_bytes([ih[24], ih[25], ih[26], ih[27]]);
        let y_pixels_per_meter = i32::from_le_bytes([ih[28], ih[29], ih[30], ih[31]]);
        let colors_used = u32::from_le_bytes([ih[32], ih[33], ih[34], ih[35]]);
        let colors_important = u32::from_le_bytes([ih[36], ih[37], ih[38], ih[39]]);

        let palette_bytes = BMP_PALETTE_ENTRIES.checked_mul(BMP_PALETTE_ENTRY_SIZE).ok_or(Bmp8Error::SizeOverflow)?;
        let palette_end = BMP_FILE_HEADER_SIZE.checked_add(BMP_INFO_HEADER_SIZE)
            .and_then(|n| n.checked_add(palette_bytes)).ok_or(Bmp8Error::SizeOverflow)?;
        let palette_data = file.get(BMP_FILE_HEADER_SIZE + BMP_INFO_HEADER_SIZE..palette_end)
            .ok_or(Bmp8Error::MissingPalette)?;
        let mut palette = [PaletteEntry { red: 0, green: 0, blue: 0, flags: 0 }; BMP_PALETTE_ENTRIES];
        for (index, entry) in palette.iter_mut().enumerate() {
            let at = index * BMP_PALETTE_ENTRY_SIZE;
            // Preserve the original byte-wise RGBQUAD -> PALETTEENTRY assignment.
            entry.red = palette_data[at];
            entry.green = palette_data[at + 1];
            entry.blue = palette_data[at + 2];
            entry.flags = palette_data[at + 3];
        }

        if pixel_offset < palette_end || pixel_offset > file_size || file_size > file.len() {
            return Err(Bmp8Error::InvalidPixelOffset);
        }
        let pixels = file.get(pixel_offset..file_size).ok_or(Bmp8Error::TruncatedPixelData)?.to_vec();
        Ok(Self {
            file_size, pixel_offset, width, height, top_down: height < 0,
            planes, bit_count, compression, image_size, x_pixels_per_meter,
            y_pixels_per_meter, colors_used, colors_important, palette, pixels,
        })
    }

    pub fn row_stride(&self) -> Result<usize, Bmp8Error> {
        let width = self.width.unsigned_abs() as usize;
        width.checked_add(3).map(|n| n & !3).ok_or(Bmp8Error::SizeOverflow)
    }

    pub fn absolute_height(&self) -> u32 { self.height.unsigned_abs() }
    pub fn absolute_width(&self) -> u32 { self.width.unsigned_abs() }

    /// The active StretchBlt path uses COLORONCOLOR and scales the complete
    /// source bitmap to the destination client rectangle.
    pub fn draw_plan(&self, client_width: i32, client_height: i32) -> Bmp8DrawPlan {
        Bmp8DrawPlan {
            source_width: self.absolute_width() as i32,
            source_height: self.absolute_height() as i32,
            destination_width: client_width,
            destination_height: client_height,
            color_on_color: true,
            realize_palette: true,
            stretch: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bmp8DrawPlan {
    pub source_width: i32,
    pub source_height: i32,
    pub destination_width: i32,
    pub destination_height: i32,
    pub color_on_color: bool,
    pub realize_palette: bool,
    pub stretch: bool,
}

/// RAII equivalent of BMP8's hBMP/hPal/hWnd ownership.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Bmp8 {
    pub image: Option<Bmp8Image>,
    pub window_handle: Option<usize>,
}
impl Bmp8 {
    pub fn init_from_bytes(&mut self, file: &[u8], window_handle: usize) -> bool {
        match Bmp8Image::parse(file) {
            Ok(image) => {
                self.image = Some(image);
                self.window_handle = Some(window_handle);
                true
            }
            Err(_) => false,
        }
    }

    pub fn draw(&self, client_width: i32, client_height: i32) -> Option<Bmp8DrawPlan> {
        self.image.as_ref().map(|image| image.draw_plan(client_width, client_height))
    }

    pub fn free_resources(&mut self) {
        self.image = None;
        self.window_handle = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bmp8() -> Vec<u8> {
        let mut file = vec![0u8; 14 + 40 + 1024 + 8];
        file[0..2].copy_from_slice(b"BM");
        let size = file.len() as u32;
        file[2..6].copy_from_slice(&size.to_le_bytes());
        file[10..14].copy_from_slice(&(1078u32).to_le_bytes());
        file[14..18].copy_from_slice(&40u32.to_le_bytes());
        file[18..22].copy_from_slice(&2i32.to_le_bytes());
        file[22..26].copy_from_slice(&(-2i32).to_le_bytes());
        file[26..28].copy_from_slice(&1u16.to_le_bytes());
        file[28..30].copy_from_slice(&8u16.to_le_bytes());
        file[30..34].copy_from_slice(&BI_RGB.to_le_bytes());
        file[34..38].copy_from_slice(&8u32.to_le_bytes());
        file[54..58].copy_from_slice(&[1, 2, 3, 4]);
        file[1078..].copy_from_slice(&[1, 2, 0, 0, 3, 4, 0, 0]);
        file
    }

    #[test]
    fn parses_8bit_headers_palette_and_pixel_payload() {
        let image = Bmp8Image::parse(&bmp8()).unwrap();
        assert_eq!((image.width, image.height), (2, -2));
        assert!(image.top_down);
        assert_eq!(image.row_stride().unwrap(), 4);
        assert_eq!(image.palette[0], PaletteEntry { red: 1, green: 2, blue: 3, flags: 4 });
        assert_eq!(image.pixels, [1, 2, 0, 0, 3, 4, 0, 0]);
    }

    #[test]
    fn draw_scales_to_client_rectangle_with_color_on_color() {
        let image = Bmp8Image::parse(&bmp8()).unwrap();
        assert_eq!(image.draw_plan(640, 480), Bmp8DrawPlan {
            source_width: 2, source_height: 2, destination_width: 640, destination_height: 480,
            color_on_color: true, realize_palette: true, stretch: true,
        });
    }

    #[test]
    fn rejects_invalid_and_non_8bit_bitmaps() {
        assert_eq!(Bmp8Image::parse(b"bad"), Err(Bmp8Error::TruncatedFileHeader));
        let mut file = bmp8();
        file[28..30].copy_from_slice(&24u16.to_le_bytes());
        assert_eq!(Bmp8Image::parse(&file), Err(Bmp8Error::UnsupportedBitDepth(24)));
    }

    #[test]
    fn class_init_and_free_manage_owned_resources() {
        let mut bmp = Bmp8::default();
        assert!(bmp.init_from_bytes(&bmp8(), 77));
        assert_eq!(bmp.window_handle, Some(77));
        assert!(bmp.draw(320, 200).is_some());
        bmp.free_resources();
        assert!(bmp.image.is_none());
        assert!(bmp.window_handle.is_none());
    }
}
