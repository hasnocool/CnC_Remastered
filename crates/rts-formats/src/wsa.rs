//! Westwood WSA animation reader/playback from TIBERIANDAWN/WIN32LIB/WSA.CPP.
//!
//! This port parses the 16-byte WSA file header plus its (frames+2) offset
//! table, selects the legacy resident/file-backed storage policy, decompresses
//! frame zero and subsequent LCW delta blocks, and preserves the source's
//! shortest-direction frame stepping and wrap behavior. Direct viewport delta
//! application and indirect-buffer blitting are backend operations.
//!
//! The original Open_File/Seek_File/Read_File path is represented by an
//! immutable Arc<[u8]> source. Resident mode builds the source's compact offset
//! table + post-frame-zero data buffer; file mode retains the immutable source
//! and resolves each delta range on demand.

use std::sync::Arc;

use crate::keyframe::{self, KeyFrameError};
use crate::lcw::{self, LcwError};

pub const WSA_FILE_HEADER_SIZE: usize = 16;
pub const WSA_PALETTE_SIZE: usize = 768;
pub const WSA_EXTRA_ALLOC_BYTES: usize = 6;

pub const WSA_OPEN_FROM_MEM: u16 = 0x0000;
pub const WSA_OPEN_INDIRECT: u16 = 0x0000;
pub const WSA_OPEN_FROM_DISK: u16 = 0x0001;
pub const WSA_OPEN_DIRECT: u16 = 0x0002;

pub const WSA_NORMAL: u16 = 0x0000;
pub const WSA_GHOST: u16 = 0x1000;
pub const WSA_PRIORITY2: u16 = 0x2000;
pub const WSA_TRANS: u16 = 0x4000;
pub const WSA_PRIORITY: u16 = 0x8000;

const WSA_SYS_ALLOCATED: u16 = 0x0002;
const WSA_FILE: u16 = 0x0004;
const WSA_RESIDENT: u16 = 0x0008;
const WSA_TARGET_IN_BUFFER: u16 = 0x0010;
const WSA_LINEAR_ONLY: u16 = 0x0020;
const WSA_FRAME_0_ON_PAGE: u16 = 0x0040;
const WSA_AMIGA_ANIMATION: u16 = 0x0080;
const WSA_PALETTE_PRESENT: u16 = 0x0100;
const WSA_FRAME_0_IS_DELTA: u16 = 0x0200;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WsaHeader {
    pub total_frames: u16,
    pub pixel_x: u16,
    pub pixel_y: u16,
    pub pixel_width: u16,
    pub pixel_height: u16,
    pub largest_frame_size: u16,
    pub flags: u16,
}

impl WsaHeader {
    pub fn pixel_count(self) -> Result<usize, WsaError> {
        (self.pixel_width as usize).checked_mul(self.pixel_height as usize)
            .ok_or(WsaError::SizeOverflow)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WsaStorageMode { Resident, FileBacked }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WsaOpenOptions {
    pub flags: u16,
    /// Some(capacity) models a caller-supplied buffer; None asks the legacy
    /// allocator to choose the minimum or maximum configuration.
    pub user_buffer_capacity: Option<usize>,
    /// Equivalent to Ram_Free(MEM_NORMAL) for the legacy allocation decision.
    pub available_memory: usize,
    /// Set when the caller supplied a palette pointer to Open_Animation.
    pub capture_palette: bool,
}

impl Default for WsaOpenOptions {
    fn default() -> Self {
        Self {
            flags: WSA_OPEN_FROM_MEM | WSA_OPEN_INDIRECT,
            user_buffer_capacity: None,
            available_memory: usize::MAX,
            capture_palette: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WsaError {
    FileTooShort,
    InvalidFrameCount,
    InvalidOffsetTable,
    InvalidFrameZeroRange,
    InvalidFrameRange { frame: usize, start: usize, end: usize },
    InvalidResidentOffset { frame: usize, offset: u32 },
    InvalidFrameNumber(i32),
    BufferTooSmall { required: usize, provided: usize },
    InsufficientMemory { minimum: usize, available: usize },
    SizeOverflow,
    Closed,
    Lcw(LcwError),
    Delta(KeyFrameError),
    PaletteTooShort,
    ViewLockFailed,
    ViewportError(i32),
}

impl From<LcwError> for WsaError {
    fn from(value: LcwError) -> Self { Self::Lcw(value) }
}
impl From<KeyFrameError> for WsaError {
    fn from(value: KeyFrameError) -> Self { Self::Delta(value) }
}

pub trait WsaFrameHost {
    /// GraphicViewPortClass::Lock / Unlock.
    fn lock_view(&mut self) -> bool;
    fn unlock_view(&mut self);

    /// Apply_XOR_Delta_To_Page_Or_Viewport. The host applies the decoded WSA
    /// delta command stream at (x,y), with source frame dimensions and the
    /// destination row stride. copy=true is used only for a non-delta frame 0.
    fn apply_delta_to_viewport(
        &mut self,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        destination_stride: usize,
        delta: &[u8],
        copy: bool,
    ) -> Result<(), i32>;

    /// Buffer_To_Page, which is the active #if TRUE indirect rendering path.
    fn buffer_to_page(
        &mut self,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        pixels: &[u8],
    ) -> Result<(), i32>;
}

#[derive(Clone, Debug)]
pub struct WsaAnimation {
    file_bytes: Arc<[u8]>,
    header: WsaHeader,
    offsets: Vec<u32>,
    resident_data: Option<Vec<u8>>,
    storage_mode: WsaStorageMode,
    flags: u16,
    current_frame: u16,
    frame0_size: usize,
    palette_adjust: usize,
    initial_delta: Vec<u8>,
    frame_buffer: Option<Vec<u8>>,
    palette: Option<[u8; WSA_PALETTE_SIZE]>,
    allocation_size: usize,
    minimum_buffer_size: usize,
    maximum_buffer_size: usize,
    closed: bool,
}

fn read_u16(bytes: &[u8], at: usize) -> Option<u16> {
    let b = bytes.get(at..at.checked_add(2)?)?;
    Some(u16::from_le_bytes([b[0], b[1]]))
}
fn read_u32(bytes: &[u8], at: usize) -> Option<u32> {
    let b = bytes.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn parse_header(bytes: &[u8]) -> Result<WsaHeader, WsaError> {
    if bytes.len() < WSA_FILE_HEADER_SIZE { return Err(WsaError::FileTooShort); }
    Ok(WsaHeader {
        total_frames: read_u16(bytes, 0).ok_or(WsaError::FileTooShort)?,
        pixel_x: read_u16(bytes, 2).ok_or(WsaError::FileTooShort)?,
        pixel_y: read_u16(bytes, 4).ok_or(WsaError::FileTooShort)?,
        pixel_width: read_u16(bytes, 6).ok_or(WsaError::FileTooShort)?,
        pixel_height: read_u16(bytes, 8).ok_or(WsaError::FileTooShort)?,
        largest_frame_size: read_u16(bytes, 10).ok_or(WsaError::FileTooShort)?,
        flags: read_u16(bytes, 12).ok_or(WsaError::FileTooShort)?,
    })
}

impl WsaAnimation {
    pub fn open(file_bytes: impl Into<Arc<[u8]>>, options: WsaOpenOptions) -> Result<Self, WsaError> {
        let file_bytes = file_bytes.into();
        let bytes = file_bytes.as_ref();
        let header = parse_header(bytes)?;
        if header.total_frames == 0 { return Err(WsaError::InvalidFrameCount); }

        // The first two values are part of WSA_FileHeaderType in C++, but
        // WSA_FILE_HEADER_SIZE backs up to byte 16 so the complete table is read.
        let offset_count = header.total_frames as usize + 2;
        let offsets_size = offset_count.checked_mul(4).ok_or(WsaError::SizeOverflow)?;
        let table_end = WSA_FILE_HEADER_SIZE.checked_add(offsets_size).ok_or(WsaError::SizeOverflow)?;
        if bytes.len() < table_end { return Err(WsaError::InvalidOffsetTable); }

        let mut offsets = Vec::with_capacity(offset_count);
        for index in 0..offset_count {
            offsets.push(read_u32(bytes, WSA_FILE_HEADER_SIZE + index * 4)
                .ok_or(WsaError::InvalidOffsetTable)?);
        }

        let frame0_size = if offsets[0] != 0 {
            offsets[1].checked_sub(offsets[0]).ok_or(WsaError::InvalidFrameZeroRange)? as usize
        } else {
            0
        };

        let palette_adjust = if header.flags & 1 != 0 { WSA_PALETTE_SIZE } else { 0 };
        let palette_start = table_end;
        let frame0_start = table_end.checked_add(palette_adjust).ok_or(WsaError::SizeOverflow)?;
        let frame0_end = frame0_start.checked_add(frame0_size).ok_or(WsaError::SizeOverflow)?;
        if frame0_end > bytes.len() { return Err(WsaError::InvalidFrameZeroRange); }

        let file_buffer_size = bytes.len()
            .checked_sub(palette_adjust)
            .and_then(|size| size.checked_sub(frame0_size))
            .and_then(|size| size.checked_sub(WSA_FILE_HEADER_SIZE))
            .ok_or(WsaError::InvalidOffsetTable)?;

        let pixel_count = header.pixel_count()?;
        let target_buffer_size = if options.flags & WSA_OPEN_DIRECT != 0 { 0 } else { pixel_count };
        // The original allocator reserves six compatibility bytes because
        // SysAnimHeaderType grew after the ANIMATE.EXE format was established.
        let delta_buffer_size = (header.largest_frame_size as usize)
            .checked_add(WSA_EXTRA_ALLOC_BYTES).ok_or(WsaError::SizeOverflow)?;
        let minimum_buffer_size = target_buffer_size.checked_add(delta_buffer_size)
            .ok_or(WsaError::SizeOverflow)?;
        let maximum_buffer_size = minimum_buffer_size.checked_add(file_buffer_size)
            .ok_or(WsaError::SizeOverflow)?;

        let disk_requested = options.flags & WSA_OPEN_FROM_DISK != 0;
        let allocation_size = if let Some(capacity) = options.user_buffer_capacity {
            if capacity < minimum_buffer_size {
                return Err(WsaError::BufferTooSmall { required: minimum_buffer_size, provided: capacity });
            }
            if disk_requested || capacity < maximum_buffer_size {
                minimum_buffer_size
            } else {
                maximum_buffer_size
            }
        } else {
            let desired = if disk_requested { minimum_buffer_size } else { maximum_buffer_size };
            if desired > options.available_memory {
                if minimum_buffer_size > options.available_memory {
                    return Err(WsaError::InsufficientMemory {
                        minimum: minimum_buffer_size,
                        available: options.available_memory,
                    });
                }
                minimum_buffer_size
            } else {
                desired
            }
        };

        let resident = allocation_size == maximum_buffer_size;
        let mut flags = 0u16;
        flags |= if options.user_buffer_capacity.is_some() { 0x0001 } else { WSA_SYS_ALLOCATED };
        if target_buffer_size != 0 { flags |= WSA_TARGET_IN_BUFFER; }
        if header.flags & 1 != 0 { flags |= WSA_PALETTE_PRESENT; }
        if header.flags & 2 != 0 { flags |= WSA_FRAME_0_IS_DELTA; }
        if offsets[0] == 0 { flags |= WSA_FRAME_0_ON_PAGE; }

        if resident {
            flags |= WSA_RESIDENT;
            if offsets[header.total_frames as usize + 1] == 0 { flags |= WSA_LINEAR_ONLY; }
        } else {
            flags |= WSA_FILE;
            if offsets[header.total_frames as usize + 1] == 0 { flags |= WSA_LINEAR_ONLY; }
        }

        let palette = if options.capture_palette && palette_adjust != 0 {
            let palette_end = palette_start.checked_add(WSA_PALETTE_SIZE).ok_or(WsaError::SizeOverflow)?;
            let src = bytes.get(palette_start..palette_end).ok_or(WsaError::PaletteTooShort)?;
            let mut value = [0u8; WSA_PALETTE_SIZE];
            value.copy_from_slice(src);
            Some(value)
        } else {
            None
        };

        let resident_data = if resident {
            let mut data = Vec::with_capacity(file_buffer_size);
            data.extend_from_slice(&bytes[WSA_FILE_HEADER_SIZE..table_end]);
            data.extend_from_slice(&bytes[frame0_end..]);
            Some(data)
        } else {
            None
        };

        let initial_delta = if frame0_size != 0 {
            lcw::decompress(&bytes[frame0_start..frame0_end], header.largest_frame_size as usize)?
        } else {
            Vec::new()
        };
        let frame_buffer = if target_buffer_size != 0 {
            Some(vec![0; target_buffer_size])
        } else {
            None
        };

        Ok(Self {
            file_bytes,
            header,
            offsets,
            resident_data,
            storage_mode: if resident { WsaStorageMode::Resident } else { WsaStorageMode::FileBacked },
            flags,
            current_frame: header.total_frames,
            frame0_size,
            palette_adjust,
            initial_delta,
            frame_buffer,
            palette,
            allocation_size,
            minimum_buffer_size,
            maximum_buffer_size,
            closed: false,
        })
    }

    pub fn header(&self) -> WsaHeader { self.header }
    pub fn current_frame(&self) -> u16 { self.current_frame }
    pub fn storage_mode(&self) -> WsaStorageMode { self.storage_mode }
    pub fn flags(&self) -> u16 { self.flags }
    pub fn allocation_size(&self) -> usize { self.allocation_size }
    pub fn minimum_buffer_size(&self) -> usize { self.minimum_buffer_size }
    pub fn maximum_buffer_size(&self) -> usize { self.maximum_buffer_size }
    pub fn palette(&self) -> Option<&[u8; WSA_PALETTE_SIZE]> { self.palette.as_ref() }
    pub fn is_linear_only(&self) -> bool { self.flags & WSA_LINEAR_ONLY != 0 }
    pub fn is_direct_to_destination(&self) -> bool { self.flags & WSA_TARGET_IN_BUFFER == 0 }
    pub fn get_animation_frame_count(&self) -> i32 { self.header.total_frames as i16 as i32 }
    pub fn get_animation_x(&self) -> i32 { self.header.pixel_x as i32 }
    pub fn get_animation_y(&self) -> i32 { self.header.pixel_y as i32 }
    pub fn get_animation_width(&self) -> i32 { self.header.pixel_width as i32 }
    pub fn get_animation_height(&self) -> i32 { self.header.pixel_height as i32 }
    pub fn get_animation_palette(&self) -> bool { self.flags & WSA_PALETTE_PRESENT != 0 }
    pub fn get_animation_size(&self) -> usize { self.allocation_size }

    /// Close_Animation: closes the legacy file handle and frees only a
    /// system-allocated animation block. Rust drops the owned backing storage
    /// when the animation value is dropped; this marks the handle unusable.
    pub fn close_animation(&mut self) {
        self.closed = true;
        self.resident_data = None;
        self.frame_buffer = None;
        self.initial_delta.clear();
    }

    fn frame_offset(&self, frame: usize) -> Result<usize, WsaError> {
        let raw = *self.offsets.get(frame).ok_or(WsaError::InvalidOffsetTable)?;
        if raw == 0 { return Ok(0); }
        match self.storage_mode {
            WsaStorageMode::Resident => {
                let base = self.frame0_size.checked_add(WSA_FILE_HEADER_SIZE).ok_or(WsaError::SizeOverflow)?;
                (raw as usize).checked_sub(base)
                    .ok_or(WsaError::InvalidResidentOffset { frame, offset: raw })
            }
            WsaStorageMode::FileBacked => (raw as usize)
                .checked_add(self.palette_adjust).ok_or(WsaError::SizeOverflow),
        }
    }

    fn read_delta_block(&self, frame: usize) -> Result<Vec<u8>, WsaError> {
        let start = self.frame_offset(frame)?;
        let end = self.frame_offset(frame + 1)?;
        if start == 0 || end <= start {
            return Err(WsaError::InvalidFrameRange { frame, start, end });
        }

        let compressed = match self.storage_mode {
            WsaStorageMode::Resident => {
                let data = self.resident_data.as_ref().ok_or(WsaError::Closed)?;
                data.get(start..end).ok_or(WsaError::InvalidFrameRange { frame, start, end })?
            }
            WsaStorageMode::FileBacked => {
                self.file_bytes.get(start..end).ok_or(WsaError::InvalidFrameRange { frame, start, end })?
            }
        };
        Ok(lcw::decompress(compressed, self.header.largest_frame_size as usize)?)
    }

    /// Animate_Frame. The host locks the destination view, applies direct-mode
    /// deltas or receives the completed indirect frame buffer, then unlocks.
    pub fn animate_frame(
        &mut self,
        frame_number: i32,
        x_pixel: i32,
        y_pixel: i32,
        host: &mut impl WsaFrameHost,
    ) -> Result<bool, WsaError> {
        if self.closed { return Err(WsaError::Closed); }
        let total_frames = self.header.total_frames as usize;
        if frame_number < 0 || frame_number as usize >= total_frames {
            return Ok(false);
        }
        if !host.lock_view() { return Err(WsaError::ViewLockFailed); }

        let result = self.animate_frame_locked(frame_number as usize, x_pixel, y_pixel, host);
        host.unlock_view();
        result.map(|_| true)
    }

    fn animate_frame_locked(
        &mut self,
        frame_number: usize,
        x_pixel: i32,
        y_pixel: i32,
        host: &mut impl WsaFrameHost,
    ) -> Result<(), WsaError> {
        let total = self.header.total_frames as usize;
        let width = self.header.pixel_width as usize;
        let height = self.header.pixel_height as usize;
        let x = x_pixel.wrapping_add(self.header.pixel_x as i16 as i32);
        let y = y_pixel.wrapping_add(self.header.pixel_y as i16 as i32);
        let direct = self.flags & WSA_TARGET_IN_BUFFER == 0;
        let stride = host.destination_stride();

        // The original initial-frame state uses current_frame == total_frames.
        if self.current_frame as usize == total {
            if self.flags & WSA_FRAME_0_ON_PAGE == 0 {
                if direct {
                    host.apply_delta_to_viewport(
                        x, y, width, height, stride, &self.initial_delta,
                        self.flags & WSA_FRAME_0_IS_DELTA == 0,
                    ).map_err(WsaError::ViewportError)?;
                } else {
                    let buffer = self.frame_buffer.as_mut().ok_or(WsaError::Closed)?;
                    keyframe::apply_xor_delta(buffer, &self.initial_delta)?;
                }
            }
            self.current_frame = 0;
        }

        let mut current = self.current_frame as usize;
        let distance = current.abs_diff(frame_number);
        let mut search_direction = 1i32;
        let mut search_frames: usize;

        if frame_number > current {
            search_frames = total - frame_number + current;
            if search_frames < distance && self.flags & WSA_LINEAR_ONLY == 0 {
                search_direction = -1;
            } else {
                search_frames = distance;
            }
        } else {
            search_frames = total - current + frame_number;
            if search_frames >= distance || self.flags & WSA_LINEAR_ONLY != 0 {
                search_direction = -1;
                search_frames = distance;
            }
        }

        for _ in 0..search_frames {
            if search_direction > 0 {
                current += 1;
                let delta = self.read_delta_block(current)?;
                self.apply_delta(&delta, direct, x, y, width, height, stride, host)?;
                if current == total { current = 0; }
            } else {
                if current == 0 { current = total; }
                let delta = self.read_delta_block(current)?;
                self.apply_delta(&delta, direct, x, y, width, height, stride, host)?;
                current -= 1;
            }
        }

        self.current_frame = frame_number as u16;
        if let Some(buffer) = self.frame_buffer.as_ref() {
            host.buffer_to_page(x, y, width, height, buffer).map_err(WsaError::ViewportError)?;
        }
        Ok(())
    }

    fn apply_delta(
        &mut self,
        delta: &[u8],
        direct: bool,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        stride: usize,
        host: &mut impl WsaFrameHost,
    ) -> Result<(), WsaError> {
        if direct {
            host.apply_delta_to_viewport(x, y, width, height, stride, delta, false)
                .map_err(WsaError::ViewportError)?;
        } else {
            let buffer = self.frame_buffer.as_mut().ok_or(WsaError::Closed)?;
            keyframe::apply_xor_delta(buffer, delta)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeView {
        stride: usize,
        locked: bool,
        lock_fail: bool,
        applied: Vec<(i32, i32, bool, Vec<u8>)>,
        blits: Vec<(i32, i32, usize, usize, Vec<u8>)>,
    }

    impl WsaFrameHost for FakeView {
        fn lock_view(&mut self) -> bool {
            if self.lock_fail { return false; }
            self.locked = true;
            true
        }
        fn unlock_view(&mut self) { self.locked = false; }
        fn destination_stride(&self) -> usize { self.stride }
        fn apply_delta_to_viewport(
            &mut self, x: i32, y: i32, _width: usize, _height: usize,
            _stride: usize, delta: &[u8], copy: bool,
        ) -> Result<(), i32> {
            self.applied.push((x, y, copy, delta.to_vec()));
            Ok(())
        }
        fn buffer_to_page(
            &mut self, x: i32, y: i32, width: usize, height: usize, pixels: &[u8],
        ) -> Result<(), i32> {
            self.blits.push((x, y, width, height, pixels.to_vec()));
            Ok(())
        }
    }

    fn simple_wsa(frame0_delta: bool, palette: bool) -> Vec<u8> {
        // 16-byte header, three offsets for one frame, then LCW frame-zero.
        // The decoded first frame is an XOR-delta short dump: 4 pixels.
        let delta_stream = [4, 1, 2, 3, 4, 0x80, 0, 0];
        let mut compressed = vec![0x88];
        compressed.extend_from_slice(&delta_stream);
        compressed.push(0x80);
        let total_frames = 1u16;
        let palette_bytes = if palette { WSA_PALETTE_SIZE } else { 0 };
        let table_end = WSA_FILE_HEADER_SIZE + (total_frames as usize + 2) * 4;
        let frame0_offset = (table_end + palette_bytes) as u32;
        let frame0_end = frame0_offset + compressed.len() as u32;
        let mut bytes = vec![0u8; table_end];
        bytes[0..2].copy_from_slice(&total_frames.to_le_bytes());
        bytes[2..4].copy_from_slice(&10u16.to_le_bytes());
        bytes[4..6].copy_from_slice(&12u16.to_le_bytes());
        bytes[6..8].copy_from_slice(&2u16.to_le_bytes());
        bytes[8..10].copy_from_slice(&2u16.to_le_bytes());
        bytes[10..12].copy_from_slice(&16u16.to_le_bytes());
        let flags = (if palette { 1 } else { 0 }) | (if frame0_delta { 2 } else { 0 });
        bytes[12..14].copy_from_slice(&(flags as u16).to_le_bytes());
        bytes[16..20].copy_from_slice(&frame0_offset.to_le_bytes());
        bytes[20..24].copy_from_slice(&frame0_end.to_le_bytes());
        bytes[24..28].copy_from_slice(&0u32.to_le_bytes());
        if palette { bytes.extend(vec![0x3f; WSA_PALETTE_SIZE]); }
        bytes.extend(compressed);
        bytes
    }

    #[test]
    fn opens_resident_animation_and_decodes_first_frame_to_indirect_buffer() {
        let bytes = simple_wsa(false, false);
        let mut animation = WsaAnimation::open(bytes, WsaOpenOptions::default()).unwrap();
        assert_eq!(animation.storage_mode(), WsaStorageMode::Resident);
        assert_eq!(animation.get_animation_frame_count(), 1);
        assert_eq!((animation.get_animation_x(), animation.get_animation_y()), (10, 12));
        let mut view = FakeView::default();
        assert!(animation.animate_frame(0, 1, 2, &mut view).unwrap());
        assert!(!view.locked);
        assert_eq!(view.blits[0], (11, 14, 2, 2, vec![1, 2, 3, 4]));
    }

    #[test]
    fn direct_mode_passes_frame_zero_copy_or_xor_flag_to_viewport() {
        let bytes = simple_wsa(false, false);
        let options = WsaOpenOptions { flags: WSA_OPEN_DIRECT, ..WsaOpenOptions::default() };
        let mut animation = WsaAnimation::open(bytes, options).unwrap();
        let mut view = FakeView { stride: 640, ..FakeView::default() };
        animation.animate_frame(0, 0, 0, &mut view).unwrap();
        assert_eq!(view.applied.len(), 1);
        assert!(view.applied[0].2); // frame zero is not marked as a delta
        assert_eq!((view.applied[0].0, view.applied[0].1), (10, 12));
    }

    #[test]
    fn user_buffer_size_selects_resident_or_file_backed_storage() {
        let bytes = simple_wsa(false, false);
        let mut resident_options = WsaOpenOptions::default();
        resident_options.user_buffer_capacity = Some(10_000);
        let resident = WsaAnimation::open(bytes.clone(), resident_options).unwrap();
        assert_eq!(resident.storage_mode(), WsaStorageMode::Resident);

        let mut disk_options = WsaOpenOptions::default();
        disk_options.flags |= WSA_OPEN_FROM_DISK;
        let file_backed = WsaAnimation::open(bytes, disk_options).unwrap();
        assert_eq!(file_backed.storage_mode(), WsaStorageMode::FileBacked);
        assert!(file_backed.flags() & WSA_FILE != 0);
    }

    #[test]
    fn captures_palette_when_requested_and_reports_palette_flag() {
        let bytes = simple_wsa(false, true);
        let options = WsaOpenOptions { capture_palette: true, ..WsaOpenOptions::default() };
        let animation = WsaAnimation::open(bytes, options).unwrap();
        assert!(animation.get_animation_palette());
        assert_eq!(animation.palette().unwrap()[0], 0x3f);
    }

    #[test]
    fn close_invalidates_handle_and_getters_remain_metadata_only() {
        let bytes = simple_wsa(false, false);
        let mut animation = WsaAnimation::open(bytes, WsaOpenOptions::default()).unwrap();
        animation.close_animation();
        assert_eq!(animation.animate_frame(0, 0, 0, &mut FakeView::default()), Err(WsaError::Closed));
    }

    #[test]
    fn rejects_invalid_frames_and_undersized_user_buffers() {
        let bytes = simple_wsa(false, false);
        let error = WsaAnimation::open(
            bytes.clone(),
            WsaOpenOptions { user_buffer_capacity: Some(1), ..WsaOpenOptions::default() },
        ).unwrap_err();
        assert!(matches!(error, WsaError::BufferTooSmall { .. }));
        let mut animation = WsaAnimation::open(bytes, WsaOpenOptions::default()).unwrap();
        assert!(!animation.animate_frame(1, 0, 0, &mut FakeView::default()).unwrap());
    }
}
