// Filename: crates/renderer/src/lib.rs
//! Portable presentation layer for CnC_Remastered.
//!
//! This crate owns GPU resources and presentation only. It must never own or
//! mutate authoritative game simulation state.

use std::sync::Arc;
use std::time::{Duration, Instant};

use winit::window::Window;

pub type TextureId = u64;

/// A normalized RGBA clear or tint color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);
    pub const DARK_MENU: Self = Self::rgba(0.035, 0.045, 0.065, 1.0);

    pub const fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    fn as_wgpu(self) -> wgpu::Color {
        wgpu::Color { r: self.r, g: self.g, b: self.b, a: self.a }
    }
}

/// Pixel dimensions of the presentation surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub const fn is_visible(self) -> bool {
        self.width > 0 && self.height > 0
    }
}

/// Camera values are presentation-only; simulation coordinates remain integral.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera2d {
    pub center_x: f32,
    pub center_y: f32,
    pub zoom: f32,
}

impl Default for Camera2d {
    fn default() -> Self {
        Self { center_x: 0.0, center_y: 0.0, zoom: 1.0 }
    }
}

impl Camera2d {
    pub fn set_zoom(&mut self, zoom: f32) {
        if zoom.is_finite() {
            self.zoom = zoom.clamp(0.05, 64.0);
        }
    }
}

/// Stable layer ordering shared by game and editor presentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RenderLayer {
    Background = 0,
    Terrain = 10,
    Smudge = 20,
    Overlay = 30,
    Building = 40,
    Vehicle = 50,
    Infantry = 60,
    Projectile = 70,
    Effect = 80,
    Interface = 100,
}

/// A renderer-facing sprite command. Asset resolution is deliberately external.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpriteInstance {
    pub texture: TextureId,
    pub layer: RenderLayer,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation_radians: f32,
    pub tint: Color,
}

#[derive(Debug, Default, Clone)]
pub struct SpriteBatch {
    sprites: Vec<SpriteInstance>,
}

impl SpriteBatch {
    pub fn with_capacity(capacity: usize) -> Self {
        Self { sprites: Vec::with_capacity(capacity) }
    }

    pub fn push(&mut self, sprite: SpriteInstance) {
        self.sprites.push(sprite);
    }

    pub fn clear(&mut self) {
        self.sprites.clear();
    }

    pub fn len(&self) -> usize {
        self.sprites.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sprites.is_empty()
    }

    /// Stable sort preserves submission order for sprites sharing a layer.
    pub fn ordered(&mut self) -> &[SpriteInstance] {
        self.sprites.sort_by_key(|sprite| sprite.layer);
        &self.sprites
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameStats {
    pub frame_index: u64,
    pub frame_time: Duration,
    pub viewport: Viewport,
}

#[derive(Debug)]
pub enum RendererError {
    CreateSurface(String),
    RequestAdapter(String),
    RequestDevice(String),
    NoSurfaceFormats,
    NoPresentModes,
    NoAlphaModes,
    Surface(String),
}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateSurface(error) => write!(f, "could not create render surface: {error}"),
            Self::RequestAdapter(error) => write!(f, "could not select graphics adapter: {error}"),
            Self::RequestDevice(error) => write!(f, "could not create graphics device: {error}"),
            Self::NoSurfaceFormats => write!(f, "surface exposes no supported formats"),
            Self::NoPresentModes => write!(f, "surface exposes no presentation modes"),
            Self::NoAlphaModes => write!(f, "surface exposes no alpha modes"),
            Self::Surface(error) => write!(f, "surface presentation failed: {error}"),
        }
    }
}

impl std::error::Error for RendererError {}

/// wgpu-backed window presenter. The Arc-owned window makes the surface lifetime
/// independent of the caller's temporary borrow.
pub struct WgpuRenderer {
    _instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    viewport: Viewport,
    frame_index: u64,
    last_frame: Instant,
}

impl WgpuRenderer {
    pub async fn new(window: Arc<Window>) -> Result<Self, RendererError> {
        let size = window.inner_size();
        let viewport = Viewport::new(size.width.max(1), size.height.max(1));
        let instance = wgpu::Instance::default();
        let surface = instance
            .create_surface(window)
            .map_err(|error| RendererError::CreateSurface(error.to_string()))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|error| RendererError::RequestAdapter(error.to_string()))?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("cnc-remastered-render-device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|error| RendererError::RequestDevice(error.to_string()))?;

        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .or_else(|| capabilities.formats.first().copied())
            .ok_or(RendererError::NoSurfaceFormats)?;
        let present_mode = capabilities
            .present_modes
            .iter()
            .copied()
            .find(|mode| *mode == wgpu::PresentMode::Fifo)
            .or_else(|| capabilities.present_modes.first().copied())
            .ok_or(RendererError::NoPresentModes)?;
        let alpha_mode = capabilities
            .alpha_modes
            .first()
            .copied()
            .ok_or(RendererError::NoAlphaModes)?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: viewport.width,
            height: viewport.height,
            present_mode,
            desired_maximum_frame_latency: 2,
            alpha_mode,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok(Self {
            _instance: instance,
            surface,
            device,
            queue,
            config,
            viewport,
            frame_index: 0,
            last_frame: Instant::now(),
        })
    }

    pub fn viewport(&self) -> Viewport {
        self.viewport
    }

    pub fn resize(&mut self, viewport: Viewport) {
        if !viewport.is_visible() {
            return;
        }
        self.viewport = viewport;
        self.config.width = viewport.width;
        self.config.height = viewport.height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Clears and presents one frame. Draw-list execution is the next renderer
    /// tranche; this method establishes the real GPU/window presentation path.
    pub fn render_clear(&mut self, color: Color) -> Result<FrameStats, RendererError> {
        let frame = self
            .surface
            .get_current_texture()
            .map_err(|error| RendererError::Surface(error.to_string()))?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some("cnc-remastered-frame-encoder") },
        );
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cnc-remastered-clear-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color.as_wgpu()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }
        self.queue.submit(Some(encoder.finish()));
        frame.present();

        self.frame_index = self.frame_index.saturating_add(1);
        let now = Instant::now();
        let frame_time = now.saturating_duration_since(self.last_frame);
        self.last_frame = now;
        Ok(FrameStats {
            frame_index: self.frame_index,
            frame_time,
            viewport: self.viewport,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_rejects_non_finite_zoom_and_clamps_valid_zoom() {
        let mut camera = Camera2d::default();
        camera.set_zoom(f32::NAN);
        assert_eq!(camera.zoom, 1.0);
        camera.set_zoom(100.0);
        assert_eq!(camera.zoom, 64.0);
        camera.set_zoom(0.0);
        assert_eq!(camera.zoom, 0.05);
    }

    #[test]
    fn sprite_batch_orders_layers_stably() {
        let mut batch = SpriteBatch::default();
        let sprite = |layer, x| SpriteInstance {
            texture: 1, layer, x, y: 0.0, width: 1.0, height: 1.0,
            rotation_radians: 0.0, tint: Color::BLACK,
        };
        batch.push(sprite(RenderLayer::Vehicle, 1.0));
        batch.push(sprite(RenderLayer::Terrain, 2.0));
        batch.push(sprite(RenderLayer::Vehicle, 3.0));
        let ordered = batch.ordered();
        assert_eq!(ordered[0].layer, RenderLayer::Terrain);
        assert_eq!(ordered[1].x, 1.0);
        assert_eq!(ordered[2].x, 3.0);
    }

    #[test]
    fn zero_sized_viewports_are_not_visible() {
        assert!(!Viewport::new(0, 600).is_visible());
        assert!(Viewport::new(800, 600).is_visible());
    }
}
