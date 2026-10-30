//! Keyframe animation decoder and uncompressed-shape cache from
//! REDALERT/2KEYFRAM.CPP and TIBERIANDAWN/KEYFRAME.CPP.
//!
//! The frame table uses a 14-byte packed header followed by eight-byte frame
//! records and a 24-byte offset look-ahead/reserved area. Frame payloads are
//! LCW-compressed; interframes apply the Westwood XOR-delta command stream.
//! Cached shape records use the legacy 12-byte ShapeHeaderType prefix, one
//! line-header byte per row, four-byte-aligned pixel data, and buffer-relative
//! offsets so Vec relocation cannot invalidate stored pointers.

use crate::lcw::{self, LcwError};

pub const KEYFRAME_HEADER_SIZE: usize = 14;
pub const KEYFRAME_RECORD_SIZE: usize = 8;
pub const KEYFRAME_LOOKAHEAD_SIZE: usize = 16;
pub const KEYFRAME_PALETTE_SIZE: usize = 768;
pub const KEYFRAME_SUBFRAME_OFFSETS: usize = 7;
pub const KEYFRAME_CACHE_MAGIC: u16 = 56_789;
pub const KEYFRAME_MAX_SLOTS: usize = 1_500;
pub const KEYFRAME_THEATER_SLOT_START: usize = 1_000;

pub const KF_DELTA: u8 = 0x20;
pub const KF_KEYFRAME: u8 = 0x80;

const SHAPE_HEADER_SIZE: usize = 12;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyFrameVariant {
    RedAlert,
    TiberianDawn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShapeBufferKind {
    Big,
    Theater,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CachedShapeHandle {
    pub buffer: ShapeBufferKind,
    pub header_offset: usize,
    pub raw_length: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BuildFrameResult {
    /// Uncached decoded bytes when UseBigShapeBuffer is disabled or allocation
    /// of the legacy uncompressed-shape buffers failed.
    Decoded(Vec<u8>),
    /// A cached legacy ShapeHeaderType record and its raw pixel payload.
    Cached(CachedShapeHandle),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyFrameHeader {
    pub frames: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub largest_frame_size: u16,
    pub flags: i16,
}

impl KeyFrameHeader {
    pub fn buffer_size(self) -> usize {
        self.width.wrapping_mul(self.height) as usize
    }

    pub fn has_palette(self) -> bool { self.flags & 1 != 0 }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KeyFrameError {
    TruncatedHeader,
    TruncatedFrameTable { frame: usize, bytes_needed: usize },
    InvalidFrame { requested: usize, frame_count: usize },
    InvalidAnimationSlot(usize),
    AnimationSlotExhausted,
    InvalidPayloadOffset(usize),
    InvalidDeltaOffset(usize),
    InvalidDeltaStream(&'static str),
    Lcw(LcwError),
    BaseFrameTooLarge { actual: usize, maximum: usize },
    ShapeBufferFull { kind: ShapeBufferKind, requested_end: usize, capacity_limit: usize },
    ShapeBufferAllocationFailed(ShapeBufferKind),
    InvalidCachedShape(CachedShapeHandle),
    InvalidPaletteOffset(usize),
}

impl From<LcwError> for KeyFrameError {
    fn from(value: LcwError) -> Self { Self::Lcw(value) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FrameOffsets([u32; KEYFRAME_SUBFRAME_OFFSETS]);

#[derive(Clone, Debug)]
struct ShapeBuffer {
    bytes: Vec<u8>,
    capacity_limit: usize,
}

impl ShapeBuffer {
    fn new() -> Self { Self { bytes: Vec::new(), capacity_limit: 0 } }

    fn initialize(&mut self, initial_capacity: usize, kind: ShapeBufferKind) -> Result<(), KeyFrameError> {
        if self.capacity_limit != 0 { return Ok(()); }
        self.bytes.try_reserve_exact(initial_capacity)
            .map_err(|_| KeyFrameError::ShapeBufferAllocationFailed(kind))?;
        self.capacity_limit = initial_capacity;
        Ok(())
    }

    fn grow(&mut self, increment: usize, kind: ShapeBufferKind) -> Result<(), KeyFrameError> {
        let new_limit = self.capacity_limit.checked_add(increment)
            .ok_or(KeyFrameError::ShapeBufferAllocationFailed(kind))?;
        let additional = new_limit.saturating_sub(self.bytes.len());
        self.bytes.try_reserve_exact(additional)
            .map_err(|_| KeyFrameError::ShapeBufferAllocationFailed(kind))?;
        self.capacity_limit = new_limit;
        Ok(())
    }

    fn remaining(&self) -> usize { self.capacity_limit.saturating_sub(self.bytes.len()) }

    fn append_shape(
        &mut self,
        kind: ShapeBufferKind,
        height: usize,
        raw: &[u8],
    ) -> Result<CachedShapeHandle, KeyFrameError> {
        let header_offset = self.bytes.len();
        let raw_offset = align4(header_offset.checked_add(SHAPE_HEADER_SIZE)
            .and_then(|v| v.checked_add(height))
            .ok_or(KeyFrameError::ShapeBufferFull {
                kind,
                requested_end: usize::MAX,
                capacity_limit: self.capacity_limit,
            })?);
        let raw_end = raw_offset.checked_add(raw.len()).ok_or(KeyFrameError::ShapeBufferFull {
            kind,
            requested_end: usize::MAX,
            capacity_limit: self.capacity_limit,
        })?;
        let next = align4(raw_end);
        if next > self.capacity_limit {
            return Err(KeyFrameError::ShapeBufferFull {
                kind,
                requested_end: next,
                capacity_limit: self.capacity_limit,
            });
        }
        let raw_offset_u32 = u32::try_from(raw_offset).map_err(|_| KeyFrameError::ShapeBufferFull {
            kind,
            requested_end: raw_offset,
            capacity_limit: u32::MAX as usize,
        })?;

        self.bytes.resize(next, 0);
        // ShapeHeaderType { draw_flags, shape_data offset, shape_buffer }.
        self.bytes[header_offset..header_offset + 4].copy_from_slice(&u32::MAX.to_le_bytes());
        self.bytes[header_offset + 4..header_offset + 8].copy_from_slice(&raw_offset_u32.to_le_bytes());
        let buffer_flag: i32 = if kind == ShapeBufferKind::Theater { 1 } else { 0 };
        self.bytes[header_offset + 8..header_offset + 12].copy_from_slice(&buffer_flag.to_le_bytes());
        self.bytes[raw_offset..raw_end].copy_from_slice(raw);

        Ok(CachedShapeHandle { buffer: kind, header_offset, raw_length: raw.len() })
    }

    fn raw_data(&self, handle: CachedShapeHandle) -> Result<&[u8], KeyFrameError> {
        let header_end = handle.header_offset.checked_add(SHAPE_HEADER_SIZE)
            .ok_or(KeyFrameError::InvalidCachedShape(handle))?;
        let header = self.bytes.get(handle.header_offset..header_end)
            .ok_or(KeyFrameError::InvalidCachedShape(handle))?;
        let raw_offset = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        let buffer_flag = i32::from_le_bytes(header[8..12].try_into().unwrap());
        if (handle.buffer == ShapeBufferKind::Theater && buffer_flag != 1)
            || (handle.buffer == ShapeBufferKind::Big && buffer_flag != 0)
        {
            return Err(KeyFrameError::InvalidCachedShape(handle));
        }
        let raw_end = raw_offset.checked_add(handle.raw_length)
            .ok_or(KeyFrameError::InvalidCachedShape(handle))?;
        self.bytes.get(raw_offset..raw_end).ok_or(KeyFrameError::InvalidCachedShape(handle))
    }

    fn clear(&mut self) { self.bytes.clear(); }
}

fn align4(value: usize) -> usize { value.saturating_add(3) & !3 }

/// Decode the Westwood XOR-delta stream used by Build_Frame. A zero command
/// is a short run (count byte followed by the repeated XOR byte); 0x80 plus
/// a zero word is the stream terminator. Destination skips leave pixels intact.
pub fn apply_xor_delta(target: &mut [u8], delta: &[u8]) -> Result<usize, KeyFrameError> {
    let mut source = 0usize;
    let mut destination = 0usize;

    loop {
        let command = *delta.get(source)
            .ok_or(KeyFrameError::InvalidDeltaStream("missing command or terminator"))?;
        source += 1;

        if command == 0 {
            let count = usize::from(*delta.get(source)
                .ok_or(KeyFrameError::InvalidDeltaStream("truncated short-run count"))?);
            source += 1;
            let value = *delta.get(source)
                .ok_or(KeyFrameError::InvalidDeltaStream("truncated short-run value"))?;
            source += 1;
            if count == 0 { return Err(KeyFrameError::InvalidDeltaStream("zero-length short run")); }
            let end = destination.checked_add(count)
                .ok_or(KeyFrameError::InvalidDeltaStream("destination overflow"))?;
            if end > target.len() { return Err(KeyFrameError::InvalidDeltaStream("short run exceeds frame buffer")); }
            for pixel in &mut target[destination..end] { *pixel ^= value; }
            destination = end;
            continue;
        }

        if command < 0x80 {
            let count = command as usize;
            let end_source = source.checked_add(count)
                .ok_or(KeyFrameError::InvalidDeltaStream("source overflow"))?;
            let end_destination = destination.checked_add(count)
                .ok_or(KeyFrameError::InvalidDeltaStream("destination overflow"))?;
            let literals = delta.get(source..end_source)
                .ok_or(KeyFrameError::InvalidDeltaStream("truncated short dump"))?;
            if end_destination > target.len() {
                return Err(KeyFrameError::InvalidDeltaStream("short dump exceeds frame buffer"));
            }
            for (pixel, value) in target[destination..end_destination].iter_mut().zip(literals.iter()) {
                *pixel ^= *value;
            }
            source = end_source;
            destination = end_destination;
            continue;
        }

        if command != 0x80 {
            let count = usize::from(command - 0x80);
            destination = destination.checked_add(count)
                .ok_or(KeyFrameError::InvalidDeltaStream("short skip overflow"))?;
            if destination > target.len() {
                return Err(KeyFrameError::InvalidDeltaStream("short skip exceeds frame buffer"));
            }
            continue;
        }

        let lo = *delta.get(source)
            .ok_or(KeyFrameError::InvalidDeltaStream("truncated long command"))?;
        let hi = *delta.get(source + 1)
            .ok_or(KeyFrameError::InvalidDeltaStream("truncated long command"))?;
        source += 2;
        let code = u16::from_le_bytes([lo, hi]);

        if code == 0 { return Ok(destination); }

        if code < 0x8000 {
            // The assembly uses a positive signed word as a long skip.
            destination = destination.checked_add(code as usize)
                .ok_or(KeyFrameError::InvalidDeltaStream("long skip overflow"))?;
            if destination > target.len() {
                return Err(KeyFrameError::InvalidDeltaStream("long skip exceeds frame buffer"));
            }
            continue;
        }

        if code >= 0xc000 {
            // LONGRUN: count = code - 0xc000, followed by one XOR byte.
            let count = (code - 0xc000) as usize;
            let value = *delta.get(source)
                .ok_or(KeyFrameError::InvalidDeltaStream("truncated long-run value"))?;
            source += 1;
            if count == 0 { return Err(KeyFrameError::InvalidDeltaStream("zero-length long run")); }
            let end = destination.checked_add(count)
                .ok_or(KeyFrameError::InvalidDeltaStream("long-run destination overflow"))?;
            if end > target.len() { return Err(KeyFrameError::InvalidDeltaStream("long run exceeds frame buffer")); }
            for pixel in &mut target[destination..end] { *pixel ^= value; }
            destination = end;
        } else {
            // LONGDUMP: count = code - 0x8000, followed by count XOR bytes.
            let count = (code - 0x8000) as usize;
            let end_source = source.checked_add(count)
                .ok_or(KeyFrameError::InvalidDeltaStream("long-dump source overflow"))?;
            let end_destination = destination.checked_add(count)
                .ok_or(KeyFrameError::InvalidDeltaStream("long-dump destination overflow"))?;
            let literals = delta.get(source..end_source)
                .ok_or(KeyFrameError::InvalidDeltaStream("truncated long dump"))?;
            if end_destination > target.len() {
                return Err(KeyFrameError::InvalidDeltaStream("long dump exceeds frame buffer"));
            }
            for (pixel, value) in target[destination..end_destination].iter_mut().zip(literals.iter()) {
                *pixel ^= *value;
            }
            source = end_source;
            destination = end_destination;
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeyFrameCache {
    variant: KeyFrameVariant,
    use_big_shape_buffer: bool,
    original_use_big_shape_buffer: bool,
    realloc_shape_buffer_flag: bool,
    big_shape_buffer_length: usize,
    theater_shape_buffer_length: usize,
    big_shape_buffer: ShapeBuffer,
    theater_shape_buffer: ShapeBuffer,
    slots: Vec<Option<Vec<Option<CachedShapeHandle>>>>,
    total_slots_used: usize,
    theater_slots_used: usize,
    total_big_shapes: usize,
    total_theater_shapes: usize,
    last_frame_length: usize,
    buffers_initialized: bool,
}

impl KeyFrameCache {
    pub fn new(variant: KeyFrameVariant) -> Self {
        let (big_length, theater_length) = match variant {
            KeyFrameVariant::RedAlert => (8_000_000, 4_000_000),
            KeyFrameVariant::TiberianDawn => (12_000 * 1024, 1000 * 1024),
        };
        Self {
            variant,
            use_big_shape_buffer: false,
            original_use_big_shape_buffer: false,
            realloc_shape_buffer_flag: false,
            big_shape_buffer_length: big_length,
            theater_shape_buffer_length: theater_length,
            big_shape_buffer: ShapeBuffer::new(),
            theater_shape_buffer: ShapeBuffer::new(),
            slots: vec![None; KEYFRAME_MAX_SLOTS],
            total_slots_used: 0,
            theater_slots_used: KEYFRAME_THEATER_SLOT_START,
            total_big_shapes: 0,
            total_theater_shapes: 0,
            last_frame_length: 0,
            buffers_initialized: false,
        }
    }

    pub fn variant(&self) -> KeyFrameVariant { self.variant }
    pub fn use_big_shape_buffer(&self) -> bool { self.use_big_shape_buffer }
    pub fn original_use_big_shape_buffer(&self) -> bool { self.original_use_big_shape_buffer }
    pub fn realloc_shape_buffer_pending(&self) -> bool { self.realloc_shape_buffer_flag }
    pub fn last_frame_length(&self) -> usize { self.last_frame_length }
    pub fn total_big_shapes(&self) -> usize { self.total_big_shapes }
    pub fn total_theater_shapes(&self) -> usize { self.total_theater_shapes }
    pub fn big_buffer_used(&self) -> usize { self.big_shape_buffer.bytes.len() }
    pub fn theater_buffer_used(&self) -> usize { self.theater_shape_buffer.bytes.len() }

    /// Check_Use_Compressed_Shapes: Red Alert enables the cache unconditionally
    /// in the pinned source; Tiberian Dawn uses GlobalMemoryStatus > 16 MiB.
    pub fn check_use_compressed_shapes(&mut self, total_physical_memory: u64) -> bool {
        let enabled = match self.variant {
            KeyFrameVariant::RedAlert => true,
            KeyFrameVariant::TiberianDawn => total_physical_memory > 16 * 1024 * 1024,
        };
        self.use_big_shape_buffer = enabled;
        self.original_use_big_shape_buffer = enabled;
        enabled
    }

    pub fn disable_uncompressed_shapes(&mut self) { self.use_big_shape_buffer = false; }
    pub fn enable_uncompressed_shapes(&mut self) {
        self.use_big_shape_buffer = self.original_use_big_shape_buffer;
    }

    fn initialize_buffers(&mut self) -> Result<(), KeyFrameError> {
        if self.buffers_initialized { return Ok(()); }
        self.big_shape_buffer.initialize(self.big_shape_buffer_length, ShapeBufferKind::Big)?;
        self.theater_shape_buffer.initialize(self.theater_shape_buffer_length, ShapeBufferKind::Theater)?;
        self.buffers_initialized = true;
        Ok(())
    }

    /// Reallocate_Big_Shape_Buffer. The original increments differ: Red Alert
    /// adds 2,000,000 bytes, while Tiberian Dawn's expression is 200*1024.
    pub fn reallocate_big_shape_buffer(&mut self) -> Result<bool, KeyFrameError> {
        if !self.realloc_shape_buffer_flag { return Ok(false); }
        let increment = match self.variant {
            KeyFrameVariant::RedAlert => 2_000_000,
            KeyFrameVariant::TiberianDawn => 200 * 1024,
        };
        if let Err(error) = self.big_shape_buffer.grow(increment, ShapeBufferKind::Big) {
            self.use_big_shape_buffer = false;
            return Err(error);
        }
        self.big_shape_buffer_length = self.big_shape_buffer.capacity_limit;
        self.realloc_shape_buffer_flag = false;
        Ok(true)
    }

    /// Reset_Theater_Shapes deletes theater frame-slot arrays and rewinds
    /// the theater-only cache without disturbing ordinary cached shapes.
    pub fn reset_theater_shapes(&mut self) {
        for index in KEYFRAME_THEATER_SLOT_START..self.theater_slots_used {
            self.slots[index] = None;
        }
        self.theater_shape_buffer.clear();
        self.total_theater_shapes = 0;
        self.theater_slots_used = KEYFRAME_THEATER_SLOT_START;
    }

    pub fn get_build_frame_count(data: &[u8]) -> u16 {
        read_u16(data, 0).unwrap_or(0)
    }
    pub fn get_build_frame_x(data: &[u8]) -> u16 {
        read_u16(data, 2).unwrap_or(0)
    }
    pub fn get_build_frame_y(data: &[u8]) -> u16 {
        read_u16(data, 4).unwrap_or(0)
    }
    pub fn get_build_frame_width(data: &[u8]) -> u16 {
        read_u16(data, 6).unwrap_or(0)
    }
    pub fn get_build_frame_height(data: &[u8]) -> u16 {
        read_u16(data, 8).unwrap_or(0)
    }

    pub fn get_build_frame_palette(data: &[u8]) -> Result<Option<[u8; KEYFRAME_PALETTE_SIZE]>, KeyFrameError> {
        let header = parse_header(data)?;
        if !header.has_palette() { return Ok(None); }
        let offset = KEYFRAME_HEADER_SIZE
            .checked_add(header.frames as usize * KEYFRAME_RECORD_SIZE)
            .and_then(|v| v.checked_add(KEYFRAME_LOOKAHEAD_SIZE))
            .ok_or(KeyFrameError::InvalidPaletteOffset(usize::MAX))?;
        let end = offset.checked_add(KEYFRAME_PALETTE_SIZE)
            .ok_or(KeyFrameError::InvalidPaletteOffset(offset))?;
        let source = data.get(offset..end).ok_or(KeyFrameError::InvalidPaletteOffset(offset))?;
        let mut palette = [0u8; KEYFRAME_PALETTE_SIZE];
        palette.copy_from_slice(source);
        Ok(Some(palette))
    }

    /// Build_Frame port. The mutable source is intentional: the original
    /// writes UNCOMPRESS_MAGIC_NUMBER and a slot index into header.x/header.y
    /// after creating per-animation cache bookkeeping.
    pub fn build_frame(
        &mut self,
        data: &mut [u8],
        frame_number: u16,
        is_theater_shape: bool,
    ) -> Result<BuildFrameResult, KeyFrameError> {
        self.last_frame_length = 0;
        let header = parse_header(data)?;
        let frame = frame_number as usize;
        if frame >= header.frames as usize {
            return Err(KeyFrameError::InvalidFrame {
                requested: frame,
                frame_count: header.frames as usize,
            });
        }

        let mut cache_kind = if is_theater_shape { ShapeBufferKind::Theater } else { ShapeBufferKind::Big };
        let mut slot_index = None;

        if self.use_big_shape_buffer {
            if self.initialize_buffers().is_err() {
                self.use_big_shape_buffer = false;
            }
        }

        if self.use_big_shape_buffer {
            let remaining = self.big_shape_buffer.remaining();
            let threshold = match self.variant {
                KeyFrameVariant::RedAlert => 128_000,
                KeyFrameVariant::TiberianDawn => 128 * 1024,
            };
            if remaining < threshold { self.realloc_shape_buffer_flag = true; }

            let x = read_u16(data, 2).ok_or(KeyFrameError::TruncatedHeader)?;
            let y = read_u16(data, 4).ok_or(KeyFrameError::TruncatedHeader)?;
            let slot = if x != KEYFRAME_CACHE_MAGIC {
                let slot = if is_theater_shape {
                    let slot = self.theater_slots_used;
                    self.theater_slots_used += 1;
                    slot
                } else {
                    let slot = self.total_slots_used;
                    self.total_slots_used += 1;
                    slot
                };
                if slot >= KEYFRAME_MAX_SLOTS { return Err(KeyFrameError::AnimationSlotExhausted); }
                write_u16(data, 2, KEYFRAME_CACHE_MAGIC)?;
                write_u16(data, 4, slot as u16)?;
                self.slots[slot] = Some(vec![None; header.frames as usize]);
                slot
            } else {
                y as usize
            };

            if slot >= KEYFRAME_MAX_SLOTS { return Err(KeyFrameError::InvalidAnimationSlot(slot)); }
            if self.slots[slot].is_none() {
                self.slots[slot] = Some(vec![None; header.frames as usize]);
            }
            let cached = self.slots[slot].as_ref()
                .and_then(|frames| frames.get(frame))
                .copied()
                .flatten();
            if let Some(handle) = cached {
                self.last_frame_length = 0;
                return Ok(BuildFrameResult::Cached(handle));
            }
            slot_index = Some(slot);
        }

        let decoded = decode_frame(data, frame, header)?;
        self.last_frame_length = decoded.len();

        if self.use_big_shape_buffer {
            let buffer = match cache_kind {
                ShapeBufferKind::Big => &mut self.big_shape_buffer,
                ShapeBufferKind::Theater => &mut self.theater_shape_buffer,
            };
            let handle = buffer.append_shape(cache_kind, header.height as usize, &decoded)?;
            if let Some(slot) = slot_index {
                if let Some(frames) = self.slots[slot].as_mut() {
                    frames[frame] = Some(handle);
                }
            }
            match cache_kind {
                ShapeBufferKind::Big => self.total_big_shapes += 1,
                ShapeBufferKind::Theater => self.total_theater_shapes += 1,
            }
            Ok(BuildFrameResult::Cached(handle))
        } else {
            cache_kind = ShapeBufferKind::Big;
            let _ = cache_kind;
            Ok(BuildFrameResult::Decoded(decoded))
        }
    }

    /// Resolve Get_Shape_Header_Data(handle) to the shape's raw pixel bytes.
    pub fn get_shape_header_data(&self, handle: CachedShapeHandle) -> Result<&[u8], KeyFrameError> {
        match handle.buffer {
            ShapeBufferKind::Big => self.big_shape_buffer.raw_data(handle),
            ShapeBufferKind::Theater => self.theater_shape_buffer.raw_data(handle),
        }
    }
}

fn parse_header(data: &[u8]) -> Result<KeyFrameHeader, KeyFrameError> {
    if data.len() < KEYFRAME_HEADER_SIZE { return Err(KeyFrameError::TruncatedHeader); }
    Ok(KeyFrameHeader {
        frames: read_u16(data, 0).ok_or(KeyFrameError::TruncatedHeader)?,
        x: read_u16(data, 2).ok_or(KeyFrameError::TruncatedHeader)?,
        y: read_u16(data, 4).ok_or(KeyFrameError::TruncatedHeader)?,
        width: read_u16(data, 6).ok_or(KeyFrameError::TruncatedHeader)?,
        height: read_u16(data, 8).ok_or(KeyFrameError::TruncatedHeader)?,
        largest_frame_size: read_u16(data, 10).ok_or(KeyFrameError::TruncatedHeader)?,
        flags: i16::from_le_bytes([
            *data.get(12).ok_or(KeyFrameError::TruncatedHeader)?,
            *data.get(13).ok_or(KeyFrameError::TruncatedHeader)?,
        ]),
    })
}

fn read_u16(data: &[u8], offset: usize) -> Option<u16> {
    let value = data.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_le_bytes([value[0], value[1]]))
}
fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    let value = data.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}
fn write_u16(data: &mut [u8], offset: usize, value: u16) -> Result<(), KeyFrameError> {
    let end = offset.checked_add(2).ok_or(KeyFrameError::TruncatedHeader)?;
    let target = data.get_mut(offset..end).ok_or(KeyFrameError::TruncatedHeader)?;
    target.copy_from_slice(&value.to_le_bytes());
    Ok(())
}

fn read_frame_offsets(data: &[u8], frame: usize) -> Result<FrameOffsets, KeyFrameError> {
    let start = KEYFRAME_HEADER_SIZE
        .checked_add(frame.checked_mul(KEYFRAME_RECORD_SIZE)
            .ok_or(KeyFrameError::TruncatedFrameTable { frame, bytes_needed: usize::MAX })?)
        .ok_or(KeyFrameError::TruncatedFrameTable { frame, bytes_needed: usize::MAX })?;
    let end = start.checked_add(KEYFRAME_SUBFRAME_OFFSETS * 4)
        .ok_or(KeyFrameError::TruncatedFrameTable { frame, bytes_needed: usize::MAX })?;
    let bytes = data.get(start..end).ok_or(KeyFrameError::TruncatedFrameTable {
        frame,
        bytes_needed: end,
    })?;
    let mut values = [0u32; KEYFRAME_SUBFRAME_OFFSETS];
    for (index, value) in values.iter_mut().enumerate() {
        *value = read_u32(bytes, index * 4).ok_or(KeyFrameError::TruncatedFrameTable {
            frame,
            bytes_needed: end,
        })?;
    }
    Ok(FrameOffsets(values))
}

fn payload_offset(data: &[u8], base: usize, offset: u32) -> Result<usize, KeyFrameError> {
    let value = base.checked_add(offset as usize)
        .ok_or(KeyFrameError::InvalidPayloadOffset(usize::MAX))?;
    if value > data.len() { return Err(KeyFrameError::InvalidPayloadOffset(value)); }
    Ok(value)
}

fn decode_lcw_payload(data: &[u8], offset: usize, palette_prefix: usize, maximum: usize) -> Result<Vec<u8>, KeyFrameError> {
    let start = offset.checked_add(palette_prefix)
        .ok_or(KeyFrameError::InvalidPayloadOffset(usize::MAX))?;
    let source = data.get(start..).ok_or(KeyFrameError::InvalidPayloadOffset(start))?;
    let decoded = lcw::decompress(source, maximum)?;
    if decoded.len() > maximum {
        return Err(KeyFrameError::BaseFrameTooLarge { actual: decoded.len(), maximum });
    }
    Ok(decoded)
}

fn decode_frame(data: &[u8], frame: usize, header: KeyFrameHeader) -> Result<Vec<u8>, KeyFrameError> {
    let mut offsets = read_frame_offsets(data, frame)?.0;
    let frame_flags = (offsets[0] >> 24) as u8;
    let palette_prefix = if header.has_palette() { KEYFRAME_PALETTE_SIZE } else { 0 };
    let frame_size = header.buffer_size();

    if frame_flags & KF_KEYFRAME != 0 {
        let offset = payload_offset(data, 0, offsets[0] & 0x00ff_ffff)?;
        return decode_lcw_payload(data, offset, palette_prefix, frame_size);
    }

    let mut current_frame = 0usize;
    if frame_flags & KF_DELTA != 0 {
        current_frame = (offsets[1] & 0xffff) as usize;
        offsets = read_frame_offsets(data, current_frame)?.0;
    }

    let current_offset = offsets[1] & 0x00ff_ffff;
    let difference = (offsets[0] & 0x00ff_ffff).wrapping_sub(current_offset);
    let base_offset = payload_offset(data, 0, current_offset)?;
    let mut output = decode_lcw_payload(data, base_offset, palette_prefix, frame_size)?;
    if output.len() > frame_size {
        return Err(KeyFrameError::BaseFrameTooLarge { actual: output.len(), maximum: frame_size });
    }
    output.resize(frame_size, 0);

    let first_delta_offset = base_offset.checked_add(palette_prefix)
        .and_then(|v| v.checked_add(difference as usize))
        .ok_or(KeyFrameError::InvalidDeltaOffset(usize::MAX))?;
    let delta = data.get(first_delta_offset..)
        .ok_or(KeyFrameError::InvalidDeltaOffset(first_delta_offset))?;
    apply_xor_delta(&mut output, delta)?;

    if frame_flags & KF_DELTA != 0 {
        current_frame = current_frame.wrapping_add(1);
        let mut subframe = 2usize;

        while current_frame <= frame {
            let offset_value = *offsets.get(subframe)
                .ok_or(KeyFrameError::TruncatedFrameTable { frame: current_frame, bytes_needed: subframe + 1 })?;
            let difference = (offset_value & 0x00ff_ffff).wrapping_sub(current_offset);
            let delta_offset = base_offset.checked_add(palette_prefix)
                .and_then(|v| v.checked_add(difference as usize))
                .ok_or(KeyFrameError::InvalidDeltaOffset(usize::MAX))?;
            let delta = data.get(delta_offset..)
                .ok_or(KeyFrameError::InvalidDeltaOffset(delta_offset))?;
            apply_xor_delta(&mut output, delta)?;

            current_frame = current_frame.wrapping_add(1);
            subframe += 2;

            if subframe >= KEYFRAME_SUBFRAME_OFFSETS - 1 && current_frame <= frame {
                offsets = read_frame_offsets(data, current_frame)?.0;
                subframe = 0;
            }
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keyframe_animation() -> Vec<u8> {
        let mut data = vec![0u8; 14 + 16 + 5];
        data[0..2].copy_from_slice(&1u16.to_le_bytes());
        data[2..4].copy_from_slice(&0u16.to_le_bytes());
        data[4..6].copy_from_slice(&0u16.to_le_bytes());
        data[6..8].copy_from_slice(&2u16.to_le_bytes());
        data[8..10].copy_from_slice(&2u16.to_le_bytes());
        data[10..12].copy_from_slice(&4u16.to_le_bytes());
        data[12..14].copy_from_slice(&0i16.to_le_bytes());

        let payload_offset = 14 + 16;
        let flags_and_offset = ((KF_KEYFRAME as u32) << 24) | payload_offset as u32;
        data[14..18].copy_from_slice(&flags_and_offset.to_le_bytes());
        data[18..22].copy_from_slice(&0u32.to_le_bytes());
        data[22..26].copy_from_slice(&0u32.to_le_bytes());
        data[payload_offset..].copy_from_slice(&[0x84, 1, 2, 3, 4]);
        data.push(0x80);
        data
    }

    #[test]
    fn xor_delta_supports_dump_run_skip_and_long_commands() {
        let mut pixels = [0u8; 16];
        let delta = [
            3, 1, 2, 3,       // short dump
            0, 2, 0x0f,       // short run
            0x82,             // short skip
            0x80, 1, 0,       // long skip
            0x80, 1, 0x80, 9, // long dump
            0x80, 1, 0xc0, 7, // long run
            0x80, 0, 0,       // terminator
        ];
        let end = apply_xor_delta(&mut pixels, &delta).unwrap();
        assert_eq!(&pixels[..3], &[1, 2, 3]);
        assert_eq!(&pixels[3..5], &[0x0f, 0x0f]);
        assert_eq!(pixels[7], 0);
        assert_eq!(pixels[8], 9);
        assert_eq!(pixels[9], 7);
        assert_eq!(end, 10);
    }

    #[test]
    fn metadata_getters_and_palette_position_follow_legacy_header_layout() {
        let mut data = keyframe_animation();
        assert_eq!(KeyFrameCache::get_build_frame_count(&data), 1);
        assert_eq!(KeyFrameCache::get_build_frame_width(&data), 2);
        assert_eq!(KeyFrameCache::get_build_frame_height(&data), 2);
        assert_eq!(KeyFrameCache::get_build_frame_palette(&data).unwrap(), None);

        data[12..14].copy_from_slice(&1i16.to_le_bytes());
        let palette_offset = 14 + 8 + 16;
        data.resize(palette_offset + KEYFRAME_PALETTE_SIZE, 0x3c);
        assert_eq!(KeyFrameCache::get_build_frame_palette(&data).unwrap().unwrap()[0], 0x3c);
    }

    #[test]
    fn build_frame_decodes_lcw_keyframe_and_tracks_last_length() {
        let mut data = keyframe_animation();
        let mut cache = KeyFrameCache::new(KeyFrameVariant::RedAlert);
        cache.check_use_compressed_shapes(0);
        let decoded = cache.build_frame(&mut data, 0, false).unwrap();
        assert_eq!(decoded, BuildFrameResult::Decoded(vec![1, 2, 3, 4]));
        assert_eq!(cache.last_frame_length(), 4);
    }

    #[test]
    fn big_shape_cache_marks_animation_header_and_reuses_cached_frame() {
        let mut data = keyframe_animation();
        let mut cache = KeyFrameCache::new(KeyFrameVariant::RedAlert);
        cache.check_use_compressed_shapes(0);
        let first = cache.build_frame(&mut data, 0, false).unwrap();
        let BuildFrameResult::Cached(handle) = first else { panic!("expected cached frame") };
        assert_eq!(cache.get_shape_header_data(handle).unwrap(), &[1, 2, 3, 4]);
        assert_eq!(KeyFrameCache::get_build_frame_x(&data), KEYFRAME_CACHE_MAGIC);
        assert_eq!(KeyFrameCache::get_build_frame_y(&data), 0);

        let second = cache.build_frame(&mut data, 0, false).unwrap();
        assert_eq!(second, BuildFrameResult::Cached(handle));
        assert_eq!(cache.last_frame_length(), 0);
    }

    #[test]
    fn theater_cache_reset_rewinds_only_theater_buffer() {
        let mut data = keyframe_animation();
        let mut cache = KeyFrameCache::new(KeyFrameVariant::RedAlert);
        cache.check_use_compressed_shapes(0);
        let theater = cache.build_frame(&mut data, 0, true).unwrap();
        let BuildFrameResult::Cached(handle) = theater else { panic!("expected theater cached frame") };
        assert_eq!(handle.buffer, ShapeBufferKind::Theater);
        assert_eq!(cache.get_shape_header_data(handle).unwrap(), &[1, 2, 3, 4]);
        cache.reset_theater_shapes();
        assert_eq!(cache.theater_buffer_used(), 0);
        assert_eq!(cache.total_theater_shapes(), 0);
        assert!(cache.get_shape_header_data(handle).is_err());
    }

    #[test]
    fn uncompressed_shape_policy_differs_by_variant() {
        let mut red_alert = KeyFrameCache::new(KeyFrameVariant::RedAlert);
        let mut tiberian_dawn = KeyFrameCache::new(KeyFrameVariant::TiberianDawn);
        assert!(red_alert.check_use_compressed_shapes(0));
        assert!(!tiberian_dawn.check_use_compressed_shapes(16 * 1024 * 1024));
        assert!(tiberian_dawn.check_use_compressed_shapes(16 * 1024 * 1024 + 1));
        tiberian_dawn.disable_uncompressed_shapes();
        assert!(tiberian_dawn.original_use_big_shape_buffer());
        tiberian_dawn.enable_uncompressed_shapes();
        assert!(tiberian_dawn.use_big_shape_buffer());
    }

    #[test]
    fn malformed_delta_commands_are_rejected_without_out_of_bounds_writes() {
        assert!(apply_xor_delta(&mut [0u8; 2], &[3, 1, 2, 3, 0x80, 0, 0]).is_err());
        assert!(apply_xor_delta(&mut [0u8; 2], &[0x80, 0]).is_err());
    }
}
