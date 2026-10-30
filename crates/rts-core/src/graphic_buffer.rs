//! Portable GraphicBufferClass / GraphicViewPortClass state from both
//! variants' WIN32LIB/GBUFFER.CPP. DirectDraw calls are injected through
//! SurfaceBackend; system-memory buffers retain the source's byte geometry.

use std::fmt;

pub const GBC_NONE: u32 = 0;
pub const GBC_VIDEOMEM: u32 = 1;
pub const GBC_VISIBLE: u32 = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BufferOffset {
    SystemMemory(usize),
    DirectMemory(usize),
    NotLocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectDrawSurfaceConfig {
    pub primary_surface: bool,
    pub offscreen: bool,
    pub width: i32,
    pub height: i32,
    pub mode_x: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SurfaceLock {
    pub memory_offset: usize,
    pub pitch_bytes: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceError { SurfaceLost, Other }

pub trait SurfaceBackend {
    fn lock_surface(&mut self) -> Result<SurfaceLock, SurfaceError>;
    fn unlock_surface(&mut self) -> Result<(), SurfaceError>;
    fn restore_surfaces(&mut self);
    fn on_focus_loss(&mut self) {}
    fn release_surface(&mut self) {}
    fn attach_surface(&mut self, _other_surface: u64) {}
    fn blit(&mut self, _request: DirectDrawBlit) -> Result<(), SurfaceError> { Err(SurfaceError::Other) }
}

pub trait MouseBlocker {
    fn block_mouse(&mut self);
    fn unblock_mouse(&mut self);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectDrawBlit {
    pub source_x: i32,
    pub source_y: i32,
    pub source_right: i32,
    pub source_bottom: i32,
    pub dest_x: i32,
    pub dest_y: i32,
    pub dest_right: i32,
    pub dest_bottom: i32,
    pub source_key: bool,
    pub wait: bool,
    pub asynchronous: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphicBufferError {
    InvalidDimensions,
    SizeOverflow,
    BufferTooSmall { required: usize, supplied: usize },
    NotDirectDraw,
    MissingSurface,
}
impl fmt::Display for GraphicBufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for GraphicBufferError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphicBuffer {
    pub width: i32,
    pub height: i32,
    pub size: usize,
    pub data: Vec<u8>,
    pub allocated: bool,
    pub is_direct_draw: bool,
    pub offset: BufferOffset,
    pub pitch: i32,
    pub x_add: i32,
    pub x_pos: i32,
    pub y_pos: i32,
    pub lock_count: i32,
    pub total_locks: u64,
    pub surface_config: Option<DirectDrawSurfaceConfig>,
    pub surface_available: bool,
}

impl GraphicBuffer {
    pub fn new(width: i32, height: i32, supplied: Option<&[u8]>, supplied_size: usize, flags: u32) -> Result<Self, GraphicBufferError> {
        if width < 0 || height < 0 { return Err(GraphicBufferError::InvalidDimensions); }
        let pixel_size = (width as usize).checked_mul(height as usize).ok_or(GraphicBufferError::SizeOverflow)?;
        let direct_draw = flags & (GBC_VIDEOMEM | GBC_VISIBLE) != 0;
        if direct_draw {
            let config = DirectDrawSurfaceConfig {
                primary_surface: flags & GBC_VISIBLE != 0,
                offscreen: flags & GBC_VISIBLE == 0,
                width,
                height,
                mode_x: width == 320,
            };
            return Ok(Self {
                width, height, size: supplied_size, data: Vec::new(), allocated: false,
                is_direct_draw: true, offset: BufferOffset::NotLocked, pitch: 0,
                x_add: 0, x_pos: 0, y_pos: 0, lock_count: 0, total_locks: 0,
                surface_config: Some(config), surface_available: false,
            });
        }

        let size = if supplied_size == 0 { pixel_size } else { supplied_size };
        let (data, allocated) = if let Some(bytes) = supplied {
            if bytes.len() < size {
                return Err(GraphicBufferError::BufferTooSmall { required: size, supplied: bytes.len() });
            }
            (bytes[..size].to_vec(), false)
        } else {
            (vec![0; size], true)
        };
        Ok(Self {
            width, height, size, data, allocated, is_direct_draw: false,
            offset: BufferOffset::SystemMemory(0), pitch: 0, x_add: 0,
            x_pos: 0, y_pos: 0, lock_count: 0, total_locks: 0,
            surface_config: None, surface_available: false,
        })
    }

    pub fn default_uninitialized() -> Self {
        Self {
            width: 0, height: 0, size: 0, data: Vec::new(), allocated: false,
            is_direct_draw: false, offset: BufferOffset::SystemMemory(0), pitch: 0,
            x_add: 0, x_pos: 0, y_pos: 0, lock_count: 0, total_locks: 0,
            surface_config: None, surface_available: false,
        }
    }

    /// DD_Init sets a primary surface for GBC_VISIBLE and an offscreen surface
    /// otherwise; the host reports when surface creation succeeds.
    pub fn mark_surface_created(&mut self, success: bool) {
        self.surface_available = success;
        if success {
            self.allocated = false;
            self.is_direct_draw = true;
            self.offset = BufferOffset::NotLocked;
            self.lock_count = 0;
        }
    }

    pub fn attach_dd_surface(&mut self, backend: &mut dyn SurfaceBackend, other_surface: u64) {
        backend.attach_surface(other_surface);
    }

    pub fn lock(&mut self, backend: Option<&mut dyn SurfaceBackend>, game_in_focus: bool, mouse: Option<&mut dyn MouseBlocker>) -> bool {
        if !self.is_direct_draw { return true; }
        if !self.surface_available || !game_in_focus { return false; }

        let mut mouse = mouse;
        if let Some(m) = mouse.as_deref_mut() { m.block_mouse(); }
        if self.lock_count != 0 {
            self.lock_count = self.lock_count.wrapping_add(1);
            if let Some(m) = mouse.as_deref_mut() { m.unblock_mouse(); }
            return true;
        }

        let Some(backend) = backend else {
            if let Some(m) = mouse.as_deref_mut() { m.unblock_mouse(); }
            return false;
        };
        let mut attempts = 0;
        while self.lock_count == 0 && attempts < 2 {
            match backend.lock_surface() {
                Ok(lock) => {
                    self.offset = BufferOffset::DirectMemory(lock.memory_offset);
                    self.pitch = lock.pitch_bytes.wrapping_sub(self.width);
                    self.lock_count += 1;
                    self.total_locks = self.total_locks.wrapping_add(1);
                    if let Some(m) = mouse.as_deref_mut() { m.unblock_mouse(); }
                    return true;
                }
                Err(SurfaceError::SurfaceLost) => {
                    backend.on_focus_loss();
                    backend.restore_surfaces();
                    attempts += 1;
                }
                Err(SurfaceError::Other) => {
                    if let Some(m) = mouse.as_deref_mut() { m.unblock_mouse(); }
                    return false;
                }
            }
        }
        if let Some(m) = mouse.as_deref_mut() { m.unblock_mouse(); }
        false
    }

    pub fn unlock(&mut self, backend: Option<&mut dyn SurfaceBackend>, mouse: Option<&mut dyn MouseBlocker>) -> bool {
        if self.lock_count == 0 || !self.is_direct_draw { return true; }
        let mut mouse = mouse;
        if self.lock_count == 1 {
            if let Some(backend) = backend {
                if let Some(m) = mouse.as_deref_mut() { m.block_mouse(); }
                if backend.unlock_surface().is_err() {
                    if let Some(m) = mouse.as_deref_mut() { m.unblock_mouse(); }
                    return false;
                }
                self.offset = BufferOffset::NotLocked;
                self.lock_count -= 1;
                if let Some(m) = mouse.as_deref_mut() { m.unblock_mouse(); }
                return true;
            }
        }
        self.lock_count -= 1;
        true
    }

    /// Release the DirectDraw surface and unwind any outstanding lock nesting.
    pub fn un_init(&mut self, mut backend: Option<&mut dyn SurfaceBackend>) {
        if self.is_direct_draw {
            if let Some(surface) = backend.as_deref_mut() {
                while self.lock_count > 0 {
                    match surface.unlock_surface() {
                        Ok(()) => self.lock_count -= 1,
                        Err(SurfaceError::SurfaceLost) => surface.restore_surfaces(),
                        Err(SurfaceError::Other) => { self.lock_count -= 1; }
                    }
                }
                surface.release_surface();
            }
            self.surface_available = false;
        }
    }

    pub fn row_stride(&self) -> i32 { self.width.wrapping_add(self.pitch) }

    pub fn get_offset(&self) -> BufferOffset { self.offset }
    pub fn get_width(&self) -> i32 { self.width }
    pub fn get_height(&self) -> i32 { self.height }
    pub fn get_pitch(&self) -> i32 { self.pitch }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphicViewPort {
    pub offset: BufferOffset,
    pub width: i32,
    pub height: i32,
    pub x_add: i32,
    pub x_pos: i32,
    pub y_pos: i32,
    pub pitch: i32,
    pub is_direct_draw: bool,
    pub lock_count: i32,
    pub is_physical_buffer: bool,
}

impl Default for GraphicViewPort {
    fn default() -> Self {
        Self { offset: BufferOffset::SystemMemory(0), width: 0, height: 0, x_add: 0,
            x_pos: 0, y_pos: 0, pitch: 0, is_direct_draw: false, lock_count: 0,
            is_physical_buffer: false }
    }
}

impl GraphicViewPort {
    pub fn attach(&mut self, buffer: &GraphicBuffer, mut x: i32, mut y: i32, mut width: i32, mut height: i32) -> bool {
        if self.is_physical_buffer { return false; }
        if x < 0 { x = 0; }
        if x >= buffer.width { x = buffer.width - 1; }
        if y < 0 { y = 0; }
        if y >= buffer.height { y = buffer.height - 1; }
        if x.wrapping_add(width) > buffer.width { width = buffer.width - x; }
        if y.wrapping_add(height) > buffer.height { height = buffer.height - y; }

        let stride = buffer.width.wrapping_add(buffer.pitch);
        let row_offset = (stride as i64).wrapping_mul(y as i64);
        let offset = match buffer.offset {
            BufferOffset::SystemMemory(base) | BufferOffset::DirectMemory(base) => {
                let value = (base as i64).wrapping_add(row_offset).wrapping_add(x as i64);
                if value < 0 { BufferOffset::SystemMemory(0) } else if buffer.is_direct_draw {
                    BufferOffset::DirectMemory(value as usize)
                } else { BufferOffset::SystemMemory(value as usize) }
            }
            BufferOffset::NotLocked => BufferOffset::NotLocked,
        };
        self.offset = offset;
        self.x_pos = x;
        self.y_pos = y;
        self.x_add = buffer.width.wrapping_sub(width);
        self.width = width;
        self.height = height;
        self.pitch = buffer.pitch;
        self.is_direct_draw = buffer.is_direct_draw;
        true
    }

    pub fn change(&mut self, buffer: &GraphicBuffer, x: i32, y: i32, width: i32, height: i32) -> bool {
        if self.is_physical_buffer { return false; }
        self.attach(buffer, x, y, width, height)
    }

    pub fn lock(&mut self, buffer: &mut GraphicBuffer, backend: Option<&mut dyn SurfaceBackend>, focus: bool, mouse: Option<&mut dyn MouseBlocker>) -> bool {
        if !buffer.lock(backend, focus, mouse) { return false; }
        if !self.is_physical_buffer {
            let x = self.x_pos;
            let y = self.y_pos;
            let w = self.width;
            let h = self.height;
            self.attach(buffer, x, y, w, h);
        }
        true
    }

    pub fn unlock(&mut self, buffer: &mut GraphicBuffer, backend: Option<&mut dyn SurfaceBackend>, mouse: Option<&mut dyn MouseBlocker>) -> bool {
        if !buffer.unlock(backend, mouse) { return false; }
        if !self.is_physical_buffer && self.is_direct_draw && buffer.lock_count == 0 {
            self.offset = BufferOffset::SystemMemory(0);
        }
        true
    }

    pub fn size_of_region(&self, width: i32, height: i32) -> i32 {
        width.wrapping_mul(height)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LogicPageState { pub current: Option<u64> }
impl LogicPageState {
    pub fn set(&mut self, next: Option<u64>) -> Option<u64> {
        let old = self.current;
        self.current = next;
        old
    }
}

pub fn direct_draw_blit(
    source_x: i32, source_y: i32, dest_x: i32, dest_y: i32,
    width: i32, height: i32, mask: bool,
) -> DirectDrawBlit {
    DirectDrawBlit {
        source_x, source_y, source_right: source_x.wrapping_add(width),
        source_bottom: source_y.wrapping_add(height), dest_x, dest_y,
        dest_right: dest_x.wrapping_add(width), dest_bottom: dest_y.wrapping_add(height),
        source_key: mask, wait: true, asynchronous: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockSurface { locks: usize, unlocks: usize, lost_once: bool, restores: usize, released: bool }
    impl SurfaceBackend for MockSurface {
        fn lock_surface(&mut self) -> Result<SurfaceLock, SurfaceError> {
            self.locks += 1;
            if self.lost_once { self.lost_once = false; return Err(SurfaceError::SurfaceLost); }
            Ok(SurfaceLock { memory_offset: 0x1000, pitch_bytes: 400 })
        }
        fn unlock_surface(&mut self) -> Result<(), SurfaceError> { self.unlocks += 1; Ok(()) }
        fn restore_surfaces(&mut self) { self.restores += 1; }
        fn release_surface(&mut self) { self.released = true; }
    }
    #[derive(Default)]
    struct MockMouse { blocks: usize, unblocks: usize }
    impl MouseBlocker for MockMouse {
        fn block_mouse(&mut self) { self.blocks += 1; }
        fn unblock_mouse(&mut self) { self.unblocks += 1; }
    }

    #[test]
    fn system_buffer_and_viewport_match_source_geometry() {
        let buffer = GraphicBuffer::new(100, 50, None, 0, GBC_NONE).unwrap();
        let mut view = GraphicViewPort::default();
        assert!(view.attach(&buffer, 10, 5, 40, 20));
        assert_eq!(view.offset, BufferOffset::SystemMemory(510));
        assert_eq!((view.width, view.height, view.x_add), (40, 20, 60));
        assert_eq!(view.size_of_region(4, 5), 20);
    }

    #[test]
    fn viewport_clamps_to_physical_buffer_edges() {
        let buffer = GraphicBuffer::new(100, 50, None, 0, GBC_NONE).unwrap();
        let mut view = GraphicViewPort::default();
        assert!(view.attach(&buffer, 90, 45, 30, 20));
        assert_eq!((view.x_pos, view.y_pos, view.width, view.height), (90, 45, 10, 5));
    }

    #[test]
    fn direct_draw_lock_restores_once_and_tracks_nested_locks() {
        let mut buffer = GraphicBuffer::new(320, 200, None, 0, GBC_VISIBLE).unwrap();
        assert_eq!(buffer.surface_config.unwrap().primary_surface, true);
        buffer.mark_surface_created(true);
        let mut surface = MockSurface { lost_once: true, ..MockSurface::default() };
        let mut mouse = MockMouse::default();
        assert!(buffer.lock(Some(&mut surface), true, Some(&mut mouse)));
        assert_eq!(surface.restores, 1);
        assert_eq!(buffer.pitch, 80);
        assert_eq!(buffer.lock_count, 1);
        assert!(buffer.lock(Some(&mut surface), true, Some(&mut mouse)));
        assert_eq!(buffer.lock_count, 2);
        assert!(buffer.unlock(Some(&mut surface), Some(&mut mouse)));
        assert_eq!(buffer.lock_count, 1);
        assert!(buffer.unlock(Some(&mut surface), Some(&mut mouse)));
        assert_eq!(buffer.lock_count, 0);
        assert_eq!(surface.unlocks, 1);
        assert_eq!(mouse.blocks, mouse.unblocks);
    }

    #[test]
    fn direct_draw_viewport_tracks_lock_and_unlock_offsets() {
        let mut buffer = GraphicBuffer::new(100, 50, None, 0, GBC_VIDEOMEM).unwrap();
        buffer.mark_surface_created(true);
        let mut view = GraphicViewPort::default();
        view.attach(&buffer, 5, 6, 20, 10);
        let mut surface = MockSurface::default();
        assert!(view.lock(&mut buffer, Some(&mut surface), true, None));
        assert_eq!(view.offset, BufferOffset::DirectMemory(6501));
        assert!(view.unlock(&mut buffer, Some(&mut surface), None));
        assert_eq!(view.offset, BufferOffset::SystemMemory(0));
    }

    #[test]
    fn non_direct_draw_lock_is_always_successful() {
        let mut buffer = GraphicBuffer::new(8, 8, None, 0, GBC_NONE).unwrap();
        assert!(buffer.lock(None, false, None));
        assert!(buffer.unlock(None, None));
    }

    #[test]
    fn direct_draw_blit_uses_half_open_rectangles_and_source_key() {
        let blit = direct_draw_blit(2, 3, 4, 5, 6, 7, true);
        assert_eq!((blit.source_right, blit.source_bottom), (8, 10));
        assert_eq!((blit.dest_right, blit.dest_bottom), (10, 12));
        assert!(blit.source_key && blit.wait && blit.asynchronous);
    }
}
