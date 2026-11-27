//! IFF-backed icon-set loading from both variants' WIN32LIB/ICONSET.CPP.
//!
//! The loader consumes ICON/SINF/SSET/TRNS/MAP IFF chunks and builds the
//! packed 40-byte IControl_Type header followed by icon, map and transparency
//! data. Pointer-valued legacy fields are represented as checked relative
//! offsets so the structure is relocatable and safe on 64-bit hosts.

use std::fmt;
use crate::iff::{IffError, IffFile, FORM_HEADER_LEN};
use crate::palette_rgba::{Palette256, PaletteError, RgbaImage};

pub const ICON_CONTROL_SIZE: usize = 40;
pub const FORM_ICON: [u8; 4] = *b"ICON";
pub const FORM_SINF: [u8; 4] = *b"SINF";
pub const FORM_SSET: [u8; 4] = *b"SSET";
pub const FORM_TRNS: [u8; 4] = *b"TRNS";
pub const FORM_MAP: [u8; 4] = *b"MAP ";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IconSetError {
    Iff(IffError),
    InvalidForm,
    MissingInfoChunk,
    InvalidIconDimensions,
    MissingIconChunk,
    BufferTooSmall { required: usize, supplied: usize },
    SizeOverflow,
    InvalidHeader,
    InvalidIconDataLength { bytes: usize, bytes_per_icon: usize },
    UnsupportedBitplanes { bitplanes: u8 },
    InvalidMapIndex { logical_index: usize, physical_index: usize, physical_count: usize },
    Palette(PaletteError),
}
impl fmt::Display for IconSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for IconSetError {}
impl From<PaletteError> for IconSetError {
    fn from(value: PaletteError) -> Self { Self::Palette(value) }
}
impl From<IffError> for IconSetError {
    fn from(value: IffError) -> Self { Self::Iff(value) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IconSetControl {
    pub width: i16,
    pub height: i16,
    pub count: i16,
    pub allocated: bool,
    pub map_width: i16,
    pub map_height: i16,
    pub size: i32,
    pub icons_offset: usize,
    pub palettes_offset: usize,
    pub remaps_offset: usize,
    pub transparency_offset: usize,
    pub color_map_offset: usize,
    pub map_offset: usize,
}

impl IconSetControl {
    fn encode(self) -> Result<[u8; ICON_CONTROL_SIZE], IconSetError> {
        let mut out = [0u8; ICON_CONTROL_SIZE];
        let shorts = [self.width, self.height, self.count, i16::from(self.allocated), self.map_width, self.map_height];
        for (i, value) in shorts.into_iter().enumerate() {
            out[i * 2..i * 2 + 2].copy_from_slice(&value.to_le_bytes());
        }
        let fields = [
            self.size,
            i32::try_from(self.icons_offset).map_err(|_| IconSetError::SizeOverflow)?,
            i32::try_from(self.palettes_offset).map_err(|_| IconSetError::SizeOverflow)?,
            i32::try_from(self.remaps_offset).map_err(|_| IconSetError::SizeOverflow)?,
            i32::try_from(self.transparency_offset).map_err(|_| IconSetError::SizeOverflow)?,
            i32::try_from(self.color_map_offset).map_err(|_| IconSetError::SizeOverflow)?,
            i32::try_from(self.map_offset).map_err(|_| IconSetError::SizeOverflow)?,
        ];
        for (i, value) in fields.into_iter().enumerate() {
            let at = 12 + i * 4;
            out[at..at + 4].copy_from_slice(&value.to_le_bytes());
        }
        Ok(out)
    }

    fn decode(bytes: &[u8]) -> Result<Self, IconSetError> {
        let h = bytes.get(..ICON_CONTROL_SIZE).ok_or(IconSetError::InvalidHeader)?;
        let short = |i: usize| i16::from_le_bytes([h[i * 2], h[i * 2 + 1]]);
        let long = |i: usize| i32::from_le_bytes([h[12 + i * 4], h[13 + i * 4], h[14 + i * 4], h[15 + i * 4]]);
        let offset = |i: usize| -> Result<usize, IconSetError> {
            usize::try_from(long(i)).map_err(|_| IconSetError::InvalidHeader)
        };
        Ok(Self {
            width: short(0), height: short(1), count: short(2), allocated: short(3) != 0,
            map_width: short(4), map_height: short(5), size: long(0),
            icons_offset: offset(1)?, palettes_offset: offset(2)?, remaps_offset: offset(3)?,
            transparency_offset: offset(4)?, color_map_offset: offset(5)?, map_offset: offset(6)?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IconSet {
    storage: Vec<u8>,
    control: IconSetControl,
    reported_size: usize,
    allocated_by_loader: bool,
    physical_icon_count: usize,
    bitplanes: u8,
}

impl IconSet {
    /// Load a packed IFF icon set. supplied_capacity=None mirrors the
    /// legacy loader allocating its own block; Some(n) models caller storage.
    pub fn load(file: &[u8], supplied_capacity: Option<usize>) -> Result<Self, IconSetError> {
        let mut iff = IffFile::open(file.to_vec())?;
        if file.get(8..12) != Some(FORM_ICON.as_slice()) { return Err(IconSetError::InvalidForm); }

        let sinf = iff.read_chunk(FORM_SINF, 4)?.ok_or(IconSetError::MissingInfoChunk)?;
        if sinf.len() < 4 { return Err(IconSetError::MissingInfoChunk); }
        let icon_width = (sinf[0] as i8 as i32).wrapping_shl(3);
        let icon_height = (sinf[1] as i8 as i32).wrapping_shl(3);
        let bitplanes = sinf[3];
        let bytes_per_icon = icon_width.checked_mul(icon_height)
            .and_then(|pixels| pixels.checked_mul(i32::from(bitplanes)))
            .map(|bits| bits / 8)
            .filter(|bytes| *bytes > 0)
            .ok_or(IconSetError::InvalidIconDimensions)? as usize;

        let icon_size = iff.chunk_size(FORM_SSET)?.ok_or(IconSetError::MissingIconChunk)?;
        let transparency_size = iff.chunk_size(FORM_TRNS)?.unwrap_or(0);
        let map_size = iff.chunk_size(FORM_MAP)?.unwrap_or(0);
        let required = ICON_CONTROL_SIZE.checked_add(icon_size)
            .and_then(|n| n.checked_add(transparency_size))
            .and_then(|n| n.checked_add(map_size)).ok_or(IconSetError::SizeOverflow)?;

        let allocated_by_loader = supplied_capacity.is_none();
        let capacity = supplied_capacity.unwrap_or(required);
        if capacity < required {
            return Err(IconSetError::BufferTooSmall { required, supplied: capacity });
        }
        let mut storage = vec![0u8; required];

        // Read_Iff_Chunk returns min(requested, actual chunk size), and the
        // caller requests exactly the size reported by Get_Iff_Chunk_Size.
        let icon_data = iff.read_chunk(FORM_SSET, icon_size)?.unwrap_or_default();
        let copied_icons = icon_data.len().min(icon_size);
        storage[ICON_CONTROL_SIZE..ICON_CONTROL_SIZE + copied_icons].copy_from_slice(&icon_data[..copied_icons]);

        if copied_icons % bytes_per_icon != 0 {
            return Err(IconSetError::InvalidIconDataLength {
                bytes: copied_icons,
                bytes_per_icon,
            });
        }
        let physical_icon_count = copied_icons / bytes_per_icon;
        let mut icon_count = physical_icon_count as u32;
        if map_size != 0 { icon_count = map_size as u32; }

        let map_offset = ICON_CONTROL_SIZE.checked_add(icon_size).ok_or(IconSetError::SizeOverflow)?;
        let transparency_offset = map_offset.checked_add(map_size).ok_or(IconSetError::SizeOverflow)?;
        let transparency = iff.read_chunk(FORM_TRNS, transparency_size)?.unwrap_or_default();
        let trans_copy = transparency.len().min(transparency_size);
        storage[transparency_offset..transparency_offset + trans_copy].copy_from_slice(&transparency[..trans_copy]);

        let map = iff.read_chunk(FORM_MAP, map_size)?.unwrap_or_default();
        let map_copy = map.len().min(map_size);
        storage[map_offset..map_offset + map_copy].copy_from_slice(&map[..map_copy]);

        let control = IconSetControl {
            width: icon_width as i16,
            height: icon_height as i16,
            count: icon_count as i16,
            allocated: allocated_by_loader,
            map_width: 0,
            map_height: 0,
            // The C++ code records the original caller capacity in Size before
            // reducing only its local buffsize variable.
            size: i32::try_from(capacity).map_err(|_| IconSetError::SizeOverflow)?,
            icons_offset: ICON_CONTROL_SIZE,
            palettes_offset: 0,
            remaps_offset: 0,
            transparency_offset,
            color_map_offset: 0,
            map_offset,
        };
        storage[..ICON_CONTROL_SIZE].copy_from_slice(&control.encode()?);

        Ok(Self { storage, control, reported_size: capacity, allocated_by_loader, physical_icon_count, bitplanes })
    }

    pub fn control(&self) -> IconSetControl { self.control }
    pub fn size(&self) -> usize { self.reported_size }
    pub fn width(&self) -> i16 { self.control.width }
    pub fn height(&self) -> i16 { self.control.height }
    pub fn count(&self) -> i16 { self.control.count }
    pub fn allocated_by_loader(&self) -> bool { self.allocated_by_loader }
    /// Number of physical images stored in SSET, before MAP indirection.
    pub fn physical_icon_count(&self) -> usize { self.physical_icon_count }
    /// SINF bitplane count. The TD/RA indexed renderer path currently requires 8.
    pub fn bitplanes(&self) -> u8 { self.bitplanes }

    /// Return one physical row-major indexed image. This deliberately refuses
    /// non-8-bit sets rather than misinterpreting planar data as palette indices.
    pub fn physical_icon_pixels(&self, index: usize) -> Result<Option<&[u8]>, IconSetError> {
        if self.bitplanes != 8 {
            return Err(IconSetError::UnsupportedBitplanes { bitplanes: self.bitplanes });
        }
        if index >= self.physical_icon_count { return Ok(None); }
        let pixels = usize::try_from(self.width()).ok()
            .and_then(|w| usize::try_from(self.height()).ok().and_then(|h| w.checked_mul(h)))
            .ok_or(IconSetError::InvalidIconDimensions)?;
        let start = index.checked_mul(pixels).ok_or(IconSetError::SizeOverflow)?;
        let end = start.checked_add(pixels).ok_or(IconSetError::SizeOverflow)?;
        Ok(self.icon_data().get(start..end))
    }

    /// Resolve a logical icon through MAP, or use identity mapping when MAP
    /// is absent. 0xFF is the legacy empty-icon sentinel.
    pub fn logical_icon_pixels(&self, logical_index: usize) -> Result<Option<&[u8]>, IconSetError> {
        if logical_index >= self.count().max(0) as usize { return Ok(None); }
        let physical = if self.map_data().is_empty() {
            logical_index
        } else {
            let Some(&mapped) = self.map_data().get(logical_index) else { return Ok(None); };
            if mapped == u8::MAX { return Ok(None); }
            usize::from(mapped)
        };
        if physical >= self.physical_icon_count {
            return Err(IconSetError::InvalidMapIndex {
                logical_index,
                physical_index: physical,
                physical_count: self.physical_icon_count,
            });
        }
        self.physical_icon_pixels(physical)
    }
    /// Expand one physical icon through an explicitly selected game palette.
    /// Transparency is opt-in because TD/RA terrain pixels are normally opaque;
    /// empty logical cells are represented by MAP=0xFF and return None instead.
    pub fn physical_icon_rgba(
        &self,
        index: usize,
        palette: &Palette256,
        transparent_index: Option<u8>,
    ) -> Result<Option<RgbaImage>, IconSetError> {
        self.physical_icon_pixels(index)?.map(|pixels| {
            Ok(palette.expand_rgba(
                u32::try_from(self.width()).map_err(|_| IconSetError::InvalidIconDimensions)?,
                u32::try_from(self.height()).map_err(|_| IconSetError::InvalidIconDimensions)?,
                pixels,
                transparent_index,
            )?)
        }).transpose()
    }

    /// Resolve MAP indirection and expand the selected physical icon.
    /// Returns None for an out-of-range logical index or the 0xFF empty marker.
    pub fn logical_icon_rgba(
        &self,
        logical_index: usize,
        palette: &Palette256,
        transparent_index: Option<u8>,
    ) -> Result<Option<RgbaImage>, IconSetError> {
        if self.map_data().is_empty() && logical_index >= self.physical_icon_count {
            return Ok(None);
        }
        let Some(pixels) = self.logical_icon_pixels(logical_index)? else { return Ok(None); };
        Ok(Some(palette.expand_rgba(
            u32::try_from(self.width()).map_err(|_| IconSetError::InvalidIconDimensions)?,
            u32::try_from(self.height()).map_err(|_| IconSetError::InvalidIconDimensions)?,
            pixels,
            transparent_index,
        )?))
    }

    pub fn storage(&self) -> &[u8] { &self.storage }
    pub fn icon_data(&self) -> &[u8] { self.slice_at(self.control.icons_offset, self.control.map_offset.saturating_sub(self.control.icons_offset)) }
    pub fn transparency_data(&self) -> &[u8] { self.slice_at(self.control.transparency_offset, self.storage.len().saturating_sub(self.control.transparency_offset)) }
    pub fn map_data(&self) -> &[u8] { self.slice_at(self.control.map_offset, self.control.transparency_offset.saturating_sub(self.control.map_offset)) }

    fn slice_at(&self, offset: usize, len: usize) -> &[u8] {
        self.storage.get(offset..offset.saturating_add(len)).unwrap_or(&[])
    }

    pub fn from_packed(storage: Vec<u8>) -> Result<Self, IconSetError> {
        let control = IconSetControl::decode(&storage)?;
        let reported_size = usize::try_from(control.size).map_err(|_| IconSetError::InvalidHeader)?;
        for offset in [control.icons_offset, control.map_offset, control.transparency_offset] {
            if offset > storage.len() { return Err(IconSetError::InvalidHeader); }
        }
        let pixels = usize::try_from(control.width).ok()
            .and_then(|w| usize::try_from(control.height).ok().and_then(|h| w.checked_mul(h)))
            .filter(|pixels| *pixels > 0).ok_or(IconSetError::InvalidHeader)?;
        let icon_bytes = control.map_offset.checked_sub(control.icons_offset)
            .ok_or(IconSetError::InvalidHeader)?;
        if icon_bytes % pixels != 0 { return Err(IconSetError::InvalidHeader); }
        let physical_icon_count = icon_bytes / pixels;
        Ok(Self { storage, control, reported_size, allocated_by_loader: control.allocated, physical_icon_count, bitplanes: 8 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chunk(id: [u8; 4], data: &[u8], out: &mut Vec<u8>) {
        out.extend_from_slice(&id);
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        out.extend_from_slice(data);
        if data.len() & 1 != 0 { out.push(0); }
    }

    fn sample_iff() -> Vec<u8> {
        let mut body = b"ICON".to_vec();
        chunk(FORM_SINF, &[1, 1, 0, 1], &mut body);
        chunk(FORM_SSET, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut body);
        chunk(FORM_TRNS, &[9, 10], &mut body);
        chunk(FORM_MAP, &[1, 255, 0], &mut body);
        let mut file = b"FORM".to_vec();
        file.extend_from_slice(&(body.len() as u32).to_be_bytes());
        file.extend_from_slice(&body);
        file
    }

    #[test]
    fn loads_icon_set_chunks_and_initializes_control_block() {
        let icon = IconSet::load(&sample_iff(), None).unwrap();
        assert_eq!((icon.width(), icon.height()), (8, 8));
        assert_eq!(icon.count(), 3); // MAP chunk size overrides icon count.
        assert!(icon.allocated_by_loader());
        assert_eq!(icon.icon_data(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(icon.transparency_data(), &[9, 10]);
        assert_eq!(icon.map_data(), &[0, 1, 2]);
        assert_eq!(icon.size(), ICON_CONTROL_SIZE + 16 + 2 + 3);
        assert_eq!(IconSet::from_packed(icon.storage().to_vec()).unwrap().control(), icon.control());
    }

    #[test]
    fn decodes_physical_tiles_and_resolves_logical_map() {
        let icon = IconSet::load(&sample_iff(), None).unwrap();
        assert_eq!(icon.physical_icon_count(), 2);
        assert_eq!(icon.bitplanes(), 8);
        assert_eq!(icon.physical_icon_pixels(1).unwrap(), Some(&[9, 10, 11, 12, 13, 14, 15, 16][..]));
        assert_eq!(icon.logical_icon_pixels(0).unwrap(), icon.physical_icon_pixels(1).unwrap());
        assert_eq!(icon.logical_icon_pixels(1).unwrap(), None); // 0xFF empty sentinel
        assert_eq!(icon.logical_icon_pixels(2).unwrap(), icon.physical_icon_pixels(0).unwrap());
        assert!(matches!(icon.physical_icon_pixels(2).unwrap(), None));
    }

    #[test]
    fn expands_logical_terrain_icon_to_rgba_without_assuming_transparency() {
        use crate::palette_rgba::{PaletteEncoding, PALETTE_BYTES};
        let icon = IconSet::load(&sample_iff(), None).unwrap();
        let mut raw = vec![0u8; PALETTE_BYTES];
        raw[3..6].copy_from_slice(&[11, 22, 33]);
        raw[6..9].copy_from_slice(&[44, 55, 66]);
        let palette = Palette256::parse(&raw, PaletteEncoding::Rgb8).unwrap();
        let image = icon.logical_icon_rgba(0, &palette, None).unwrap().unwrap();
        assert_eq!((image.width, image.height), (8, 8));
        assert_eq!(&image.pixels[..8], &[44, 55, 66, 255, 11, 22, 33, 255]);
        assert!(icon.logical_icon_rgba(1, &palette, None).unwrap().is_none());
    }

    #[test]
    fn caller_capacity_is_preserved_and_checked() {
        let file = sample_iff();
        assert!(matches!(IconSet::load(&file, Some(40)), Err(IconSetError::BufferTooSmall { .. })));
        let icon = IconSet::load(&file, Some(128)).unwrap();
        assert_eq!(icon.size(), 128);
        assert!(!icon.allocated_by_loader());
    }

    #[test]
    fn rejects_non_icon_and_bad_sinf() {
        let mut file = sample_iff();
        file[8..12].copy_from_slice(b"ILBM");
        assert_eq!(IconSet::load(&file, None), Err(IconSetError::InvalidForm));
    }
}
