// Filename: crates/renderer/src/lib.rs
//! Portable 2D presentation layer for CnC_Remastered.
//!
//! The renderer owns GPU resources and presentation only. It does not own or
//! mutate authoritative simulation state. Coordinates supplied to draw calls
//! are physical pixels in the current surface viewport.

use std::{sync::Arc, time::{Duration, Instant}};
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use winit::window::Window;

pub type TextureId = u64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color { pub r: f64, pub g: f64, pub b: f64, pub a: f64 }
impl Color {
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);
    pub const DARK_MENU: Self = Self::rgba(0.025, 0.035, 0.052, 1.0);
    pub const PANEL: Self = Self::rgba(0.055, 0.075, 0.095, 0.96);
    pub const ACCENT: Self = Self::rgba(0.78, 0.58, 0.22, 1.0);
    pub const fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self { Self { r, g, b, a } }
    fn as_wgpu(self) -> wgpu::Color { wgpu::Color { r: self.r, g: self.g, b: self.b, a: self.a } }
    fn as_f32(self) -> [f32; 4] { [self.r as f32, self.g as f32, self.b as f32, self.a as f32] }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport { pub width: u32, pub height: u32 }
impl Viewport {
    pub const fn new(width: u32, height: u32) -> Self { Self { width, height } }
    pub const fn is_visible(self) -> bool { self.width > 0 && self.height > 0 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera2d { pub center_x: f32, pub center_y: f32, pub zoom: f32 }
impl Default for Camera2d {
    fn default() -> Self { Self { center_x: 0.0, center_y: 0.0, zoom: 1.0 } }
}
impl Camera2d {
    pub fn set_zoom(&mut self, zoom: f32) {
        if zoom.is_finite() { self.zoom = zoom.clamp(0.05, 64.0); }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RenderLayer {
    Background = 0, Terrain = 10, Smudge = 20, Overlay = 30, Building = 40,
    Vehicle = 50, Infantry = 60, Projectile = 70, Effect = 80, Interface = 100,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpriteInstance {
    pub texture: TextureId,
    pub layer: RenderLayer,
    /// Top-left position in physical pixels.
    pub x: f32, pub y: f32, pub width: f32, pub height: f32,
    pub rotation_radians: f32,
    /// Normalized atlas UV rectangle: [u_min, v_min, u_max, v_max].
    pub uv: [f32; 4],
    pub tint: Color,
}
impl SpriteInstance {
    pub fn new(texture: TextureId, layer: RenderLayer, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { texture, layer, x, y, width, height, rotation_radians: 0.0,
            uv: [0.0, 0.0, 1.0, 1.0], tint: Color::rgba(1.0, 1.0, 1.0, 1.0) }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SpriteBatch { sprites: Vec<SpriteInstance> }
impl SpriteBatch {
    pub fn with_capacity(capacity: usize) -> Self { Self { sprites: Vec::with_capacity(capacity) } }
    pub fn push(&mut self, sprite: SpriteInstance) { self.sprites.push(sprite); }
    pub fn clear(&mut self) { self.sprites.clear(); }
    pub fn len(&self) -> usize { self.sprites.len() }
    pub fn is_empty(&self) -> bool { self.sprites.is_empty() }
    pub fn ordered(&mut self) -> &[SpriteInstance] {
        self.sprites.sort_by_key(|sprite| sprite.layer);
        &self.sprites
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameStats { pub frame_index: u64, pub frame_time: Duration, pub viewport: Viewport }

/// A small immediate-mode 2D UI draw list. Text uses a bundled 5x7 bitmap
/// alphabet, avoiding platform font dependencies in the launcher bootstrap.
#[derive(Debug, Default, Clone)]
pub struct UiFrame { vertices: Vec<Vertex> }
impl UiFrame {
    pub fn new() -> Self { Self::default() }
    pub fn clear(&mut self) { self.vertices.clear(); }
    pub fn is_empty(&self) -> bool { self.vertices.is_empty() }
    pub fn vertex_count(&self) -> usize { self.vertices.len() }
    pub fn rect(&mut self, viewport: Viewport, x: f32, y: f32, w: f32, h: f32, color: Color) {
        if w <= 0.0 || h <= 0.0 || !viewport.is_visible() { return; }
        let x0 = (x / viewport.width as f32) * 2.0 - 1.0;
        let x1 = ((x + w) / viewport.width as f32) * 2.0 - 1.0;
        let y0 = 1.0 - (y / viewport.height as f32) * 2.0;
        let y1 = 1.0 - ((y + h) / viewport.height as f32) * 2.0;
        let c = color.as_f32();
        self.vertices.extend_from_slice(&[
            Vertex::new([x0,y0],[0.0,0.0],c), Vertex::new([x1,y0],[1.0,0.0],c), Vertex::new([x1,y1],[1.0,1.0],c),
            Vertex::new([x0,y0],[0.0,0.0],c), Vertex::new([x1,y1],[1.0,1.0],c), Vertex::new([x0,y1],[0.0,1.0],c),
        ]);
    }
    pub fn outline(&mut self, viewport: Viewport, x: f32, y: f32, w: f32, h: f32, thickness: f32, color: Color) {
        self.rect(viewport,x,y,w,thickness,color);
        self.rect(viewport,x,y+h-thickness,w,thickness,color);
        self.rect(viewport,x,y,thickness,h,color);
        self.rect(viewport,x+w-thickness,y,thickness,h,color);
    }
    pub fn text(&mut self, viewport: Viewport, x: f32, y: f32, scale: f32, color: Color, value: &str) {
        if !scale.is_finite() || scale <= 0.0 { return; }
        let mut cursor = x;
        for ch in value.chars() {
            if ch == '\n' { cursor = x; continue; }
            let rows = glyph(ch.to_ascii_uppercase());
            for (row, bits) in rows.iter().enumerate() {
                for col in 0..5 {
                    if bits & (1 << (4-col)) != 0 {
                        self.rect(viewport, cursor + col as f32 * scale, y + row as f32 * scale, scale, scale, color);
                    }
                }
            }
            cursor += 6.0 * scale;
        }
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct Vertex { position: [f32; 2], uv: [f32; 2], color: [f32; 4] }
impl Vertex { const fn new(position: [f32;2], uv: [f32;2], color: [f32;4]) -> Self { Self { position, uv, color } } }
const ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4];

pub struct GpuTexture {
    pub id: TextureId,
    _texture: wgpu::Texture,
    bind_group: wgpu::BindGroup,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum RendererError {
    CreateSurface(String), RequestAdapter(String), RequestDevice(String),
    NoSurfaceFormats, NoPresentModes, NoAlphaModes, Surface(String),
    InvalidTextureData { width: u32, height: u32, bytes: usize },
}
impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateSurface(e) => write!(f, "could not create render surface: {e}"),
            Self::RequestAdapter(e) => write!(f, "could not select graphics adapter: {e}"),
            Self::RequestDevice(e) => write!(f, "could not create graphics device: {e}"),
            Self::NoSurfaceFormats => f.write_str("surface exposes no supported formats"),
            Self::NoPresentModes => f.write_str("surface exposes no presentation modes"),
            Self::NoAlphaModes => f.write_str("surface exposes no alpha modes"),
            Self::Surface(e) => write!(f, "surface presentation failed: {e}"),
            Self::InvalidTextureData { width, height, bytes } => write!(f, "RGBA texture {width}x{height} requires {} bytes, got {bytes}", width as usize * height as usize * 4),
        }
    }
}
impl std::error::Error for RendererError {}

/// Backend-independent presentation contract. Implementations consume
/// presentation data only; simulation remains outside this crate.
pub trait PresentationBackend: Send {
    fn viewport(&self) -> Viewport;
    fn resize(&mut self, viewport: Viewport);
    fn render_clear(&mut self, color: Color) -> Result<FrameStats, RendererError>;
}

pub struct WgpuRenderer {
    _instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    texture_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    sprite_pipeline: wgpu::RenderPipeline,
    ui_pipeline: wgpu::RenderPipeline,
    viewport: Viewport,
    frame_index: u64,
    last_frame: Instant,
}

impl WgpuRenderer {
    pub async fn new(window: Arc<Window>, vsync: bool, low_power: bool) -> Result<Self, RendererError> {
        let size = window.inner_size();
        let viewport = Viewport::new(size.width.max(1), size.height.max(1));
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).map_err(|e| RendererError::CreateSurface(e.to_string()))?;
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: if low_power { wgpu::PowerPreference::LowPower } else { wgpu::PowerPreference::HighPerformance },
            compatible_surface: Some(&surface), force_fallback_adapter: false,
        }).await.map_err(|e| RendererError::RequestAdapter(e.to_string()))?;
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("cnc-remastered-render-device"), required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: if low_power { wgpu::MemoryHints::MemoryUsage } else { wgpu::MemoryHints::Performance },
            trace: wgpu::Trace::Off,
        }).await.map_err(|e| RendererError::RequestDevice(e.to_string()))?;

        let capabilities = surface.get_capabilities(&adapter);
        let format = capabilities.formats.iter().copied().find(wgpu::TextureFormat::is_srgb)
            .or_else(|| capabilities.formats.first().copied()).ok_or(RendererError::NoSurfaceFormats)?;
        let requested = if vsync { wgpu::PresentMode::Fifo } else { wgpu::PresentMode::Immediate };
        let present_mode = capabilities.present_modes.iter().copied().find(|m| *m == requested)
            .or_else(|| capabilities.present_modes.first().copied()).ok_or(RendererError::NoPresentModes)?;
        let alpha_mode = capabilities.alpha_modes.first().copied().ok_or(RendererError::NoAlphaModes)?;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, format, width: viewport.width, height: viewport.height,
            present_mode, desired_maximum_frame_latency: 2, alpha_mode, view_formats: vec![],
        };
        surface.configure(&device, &config);

        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cnc-sprite-texture-layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            ],
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("cnc-nearest-sprite-sampler"), mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest, mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let sprite_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cnc-sprite-shader"), source: wgpu::ShaderSource::Wgsl(SPRITE_SHADER.into()),
        });
        let ui_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cnc-ui-shader"), source: wgpu::ShaderSource::Wgsl(UI_SHADER.into()),
        });
        let sprite_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cnc-sprite-pipeline-layout"), bind_group_layouts: &[&texture_layout], push_constant_ranges: &[],
        });
        let ui_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cnc-ui-pipeline-layout"), bind_group_layouts: &[], push_constant_ranges: &[],
        });
        let sprite_pipeline = create_pipeline(&device, &sprite_layout, &sprite_shader, format, "cnc-sprite-pipeline");
        let ui_pipeline = create_pipeline(&device, &ui_layout, &ui_shader, format, "cnc-ui-pipeline");
        Ok(Self { _instance: instance, surface, device, queue, config, texture_layout, sampler,
            sprite_pipeline, ui_pipeline, viewport, frame_index: 0, last_frame: Instant::now() })
    }

    pub fn viewport(&self) -> Viewport { self.viewport }
    pub fn resize(&mut self, viewport: Viewport) {
        if !viewport.is_visible() { return; }
        self.viewport = viewport;
        self.config.width = viewport.width; self.config.height = viewport.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn create_texture_rgba(&self, id: TextureId, width: u32, height: u32, rgba: &[u8]) -> Result<GpuTexture, RendererError> {
        let required = width as usize * height as usize * 4;
        if width == 0 || height == 0 || rgba.len() != required {
            return Err(RendererError::InvalidTextureData { width, height, bytes: rgba.len() });
        }
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("cnc-sprite-atlas"), size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            rgba,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(height) },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cnc-sprite-atlas-bind-group"), layout: &self.texture_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        Ok(GpuTexture { id, _texture: texture, bind_group, width, height })
    }

    pub fn render(&mut self, clear: Color, sprites: Option<(&SpriteBatch, &GpuTexture)>, ui: &UiFrame) -> Result<FrameStats, RendererError> {
        let frame = self.surface.get_current_texture().map_err(|e| RendererError::Surface(e.to_string()))?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("cnc-frame-encoder") });

        let sprite_vertices = sprites.map(|(batch, texture)| build_sprite_vertices(batch, texture.id, self.viewport));
        let sprite_buffer = sprite_vertices.as_ref().filter(|v| !v.is_empty()).map(|vertices| self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cnc-sprite-vertices"), contents: bytemuck::cast_slice(vertices), usage: wgpu::BufferUsages::VERTEX,
        }));
        let ui_buffer = (!ui.vertices.is_empty()).then(|| self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cnc-ui-vertices"), contents: bytemuck::cast_slice(&ui.vertices), usage: wgpu::BufferUsages::VERTEX,
        }));

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cnc-main-render-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(clear.as_wgpu()), store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None,
            });
            if let (Some(buffer), Some((_, texture)), Some(vertices)) = (&sprite_buffer, sprites, &sprite_vertices) {
                if !vertices.is_empty() {
                    pass.set_pipeline(&self.sprite_pipeline);
                    pass.set_bind_group(0, &texture.bind_group, &[]);
                    pass.set_vertex_buffer(0, buffer.slice(..));
                    pass.draw(0..vertices.len() as u32, 0..1);
                }
            }
            if let (Some(buffer), Some(vertices)) = (&ui_buffer, (!ui.vertices.is_empty()).then_some(&ui.vertices)) {
                pass.set_pipeline(&self.ui_pipeline);
                pass.set_vertex_buffer(0, buffer.slice(..));
                pass.draw(0..vertices.len() as u32, 0..1);
            }
        }
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        self.frame_index = self.frame_index.saturating_add(1);
        let now = Instant::now();
        let frame_time = now.saturating_duration_since(self.last_frame);
        self.last_frame = now;
        Ok(FrameStats { frame_index: self.frame_index, frame_time, viewport: self.viewport })
    }

    pub fn render_clear(&mut self, color: Color) -> Result<FrameStats, RendererError> {
        self.render(color, None, &UiFrame::default())
    }
}

impl PresentationBackend for WgpuRenderer {
    fn viewport(&self) -> Viewport { WgpuRenderer::viewport(self) }
    fn resize(&mut self, viewport: Viewport) { WgpuRenderer::resize(self, viewport); }
    fn render_clear(&mut self, color: Color) -> Result<FrameStats, RendererError> { WgpuRenderer::render_clear(self, color) }
}

fn create_pipeline(device: &wgpu::Device, layout: &wgpu::PipelineLayout, shader: &wgpu::ShaderModule, format: wgpu::TextureFormat, label: &str) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label), layout: Some(layout),
        vertex: wgpu::VertexState { module: shader, entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout { array_stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex, attributes: &ATTRIBUTES }] },
        fragment: Some(wgpu::FragmentState { module: shader, entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })] }),
        primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, ..Default::default() },
        depth_stencil: None, multisample: wgpu::MultisampleState::default(), multiview: None, cache: None,
    })
}

fn build_sprite_vertices(batch: &SpriteBatch, texture_id: TextureId, viewport: Viewport) -> Vec<Vertex> {
    let mut ordered = batch.clone();
    let mut vertices = Vec::with_capacity(ordered.len() * 6);
    for sprite in ordered.ordered().iter().filter(|sprite| sprite.texture == texture_id) {
        if sprite.width <= 0.0 || sprite.height <= 0.0 { continue; }
        let cx = sprite.x + sprite.width * 0.5;
        let cy = sprite.y + sprite.height * 0.5;
        let (sin, cos) = sprite.rotation_radians.sin_cos();
        let corners = [
            (-0.5,-0.5,sprite.uv[0],sprite.uv[1]), (0.5,-0.5,sprite.uv[2],sprite.uv[1]),
            (0.5,0.5,sprite.uv[2],sprite.uv[3]), (-0.5,0.5,sprite.uv[0],sprite.uv[3]),
        ];
        let mut quad = [Vertex::zeroed(); 4];
        for (i,(dx,dy,u,v)) in corners.into_iter().enumerate() {
            let px = cx + (dx * sprite.width) * cos - (dy * sprite.height) * sin;
            let py = cy + (dx * sprite.width) * sin + (dy * sprite.height) * cos;
            let position = [(px / viewport.width as f32) * 2.0 - 1.0, 1.0 - (py / viewport.height as f32) * 2.0];
            quad[i] = Vertex::new(position,[u,v],sprite.tint.as_f32());
        }
        vertices.extend_from_slice(&[quad[0],quad[1],quad[2],quad[0],quad[2],quad[3]]);
    }
    vertices
}

const SPRITE_SHADER: &str = r#"
struct VertexInput { @location(0) position: vec2<f32>, @location(1) uv: vec2<f32>, @location(2) color: vec4<f32> };
struct VertexOutput { @builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32>, @location(1) color: vec4<f32> };
@vertex fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    output.uv = input.uv; output.color = input.color; return output;
}
@group(0) @binding(0) var sprite_texture: texture_2d<f32>;
@group(0) @binding(1) var sprite_sampler: sampler;
@fragment fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(sprite_texture, sprite_sampler, input.uv) * input.color;
}
"#;

const UI_SHADER: &str = r#"
struct VertexInput { @location(0) position: vec2<f32>, @location(1) uv: vec2<f32>, @location(2) color: vec4<f32> };
struct VertexOutput { @builtin(position) position: vec4<f32>, @location(0) color: vec4<f32> };
@vertex fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput; output.position = vec4<f32>(input.position, 0.0, 1.0); output.color = input.color; return output;
}
@fragment fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> { return input.color; }
"#;

fn glyph(ch: char) -> [u8; 7] {
    match ch {
        'A'=>[14,17,17,31,17,17,17], 'B'=>[30,17,17,30,17,17,30], 'C'=>[14,17,16,16,16,17,14],
        'D'=>[30,17,17,17,17,17,30], 'E'=>[31,16,16,30,16,16,31], 'F'=>[31,16,16,30,16,16,16],
        'G'=>[14,17,16,23,17,17,15], 'H'=>[17,17,17,31,17,17,17], 'I'=>[14,4,4,4,4,4,14],
        'J'=>[7,2,2,2,18,18,12], 'K'=>[17,18,20,24,20,18,17], 'L'=>[16,16,16,16,16,16,31],
        'M'=>[17,27,21,21,17,17,17], 'N'=>[17,25,21,19,17,17,17], 'O'=>[14,17,17,17,17,17,14],
        'P'=>[30,17,17,30,16,16,16], 'Q'=>[14,17,17,17,21,18,13], 'R'=>[30,17,17,30,20,18,17],
        'S'=>[15,16,16,14,1,1,30], 'T'=>[31,4,4,4,4,4,4], 'U'=>[17,17,17,17,17,17,14],
        'V'=>[17,17,17,17,17,10,4], 'W'=>[17,17,17,21,21,21,10], 'X'=>[17,17,10,4,10,17,17],
        'Y'=>[17,17,10,4,4,4,4], 'Z'=>[31,1,2,4,8,16,31],
        '0'=>[14,17,19,21,25,17,14], '1'=>[4,12,4,4,4,4,14], '2'=>[14,17,1,2,4,8,31],
        '3'=>[30,1,1,14,1,1,30], '4'=>[2,6,10,18,31,2,2], '5'=>[31,16,16,30,1,1,30],
        '6'=>[14,16,16,30,17,17,14], '7'=>[31,1,2,4,8,8,8], '8'=>[14,17,17,14,17,17,14],
        '9'=>[14,17,17,15,1,1,14], '-'=>[0,0,0,31,0,0,0], '_'=>[0,0,0,0,0,0,31],
        ':'=>[0,4,4,0,4,4,0], '.'=>[0,0,0,0,0,12,12], '/'=>[1,2,2,4,8,8,16],
        '['=>[14,8,8,8,8,8,14], ']'=>[14,2,2,2,2,2,14], '+'=>[0,4,4,31,4,4,0],
        '='=>[0,31,0,31,0,0,0], '?'=>[14,17,1,2,4,0,4], '!'=>[4,4,4,4,4,0,4],
        ' '=>[0;7], _=>[31,17,1,2,4,0,4],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn camera_clamps_zoom_and_ignores_nan() {
        let mut camera=Camera2d::default(); camera.set_zoom(f32::NAN); assert_eq!(camera.zoom,1.0);
        camera.set_zoom(100.0); assert_eq!(camera.zoom,64.0); camera.set_zoom(0.0); assert_eq!(camera.zoom,0.05);
    }
    #[test] fn sprite_batch_orders_layers_stably() {
        let mut batch=SpriteBatch::default();
        let mut a=SpriteInstance::new(1,RenderLayer::Vehicle,1.0,0.0,1.0,1.0);
        let b=SpriteInstance::new(1,RenderLayer::Terrain,2.0,0.0,1.0,1.0);
        let c=SpriteInstance::new(1,RenderLayer::Vehicle,3.0,0.0,1.0,1.0);
        a.tint=Color::BLACK; batch.push(a); batch.push(b); batch.push(c);
        let ordered=batch.ordered(); assert_eq!(ordered[0].layer,RenderLayer::Terrain);
        assert_eq!(ordered[1].x,1.0); assert_eq!(ordered[2].x,3.0);
    }
    #[test] fn ui_text_generates_geometry_and_empty_text_does_not() {
        let mut ui=UiFrame::new(); let viewport=Viewport::new(800,600);
        ui.text(viewport,10.0,10.0,2.0,Color::ACCENT,"TD 95");
        assert!(ui.vertex_count()>0); ui.clear(); assert!(ui.is_empty());
    }
    #[test] fn rejects_invalid_texture_upload_size() {
        let required=4*4*4; assert_ne!(required, 3);
    }
    #[test] fn zero_sized_viewports_are_not_visible() {
        assert!(!Viewport::new(0,600).is_visible()); assert!(Viewport::new(800,600).is_visible());
    }
}
