// Filename: crates/renderer/src/terrain.rs
//! Camera-aware terrain atlas construction and visible-chunk composition.

use std::collections::{BTreeSet, HashMap};
use rts_formats::{
    iconset::{IconSet, IconSetError},
    palette_rgba::{Palette256, RgbaImage},
    tmp::{TmpArchive, TmpError},
};
use crate::{
    Camera2d, Color, FrameStats, RenderLayer, RendererError, SpriteBatch, SpriteInstance,
    TextureId, UiFrame, Viewport, WgpuRenderer,
};

pub const TERRAIN_CHUNK_CELLS: u32 = 16;
const MAX_ATLAS_DIMENSION: u32 = 8192;
const MAX_ATLAS_BYTES: usize = 128 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerrainError {
    InvalidMapDimensions { width: u32, height: u32, cells: usize },
    InvalidTileIndex(usize),
    UnresolvedTemplateCell { template: u16, icon: u8 },
    UnknownTemplateType(u16),
    DuplicateTemplateType(u16),
    InconsistentTemplateIconSize {
        template: u16,
        expected: (u32, u32),
        actual: (u32, u32),
    },
    InvalidTileImage(usize),
    MapAllocationFailed { cells: usize },
    AtlasTooLarge { width: u32, height: u32, bytes: usize },
    InvalidCamera,
    Renderer(String),
    Tmp(TmpError),
    IconSet(IconSetError),
}
impl std::fmt::Display for TerrainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMapDimensions { width, height, cells } =>
                write!(f, "terrain map {width}x{height} does not match {cells} cell entries"),
            Self::InvalidTileIndex(index) => write!(f, "terrain map references missing TMP tile {index}"),
            Self::UnresolvedTemplateCell { template, icon } => write!(
                f,
                "map template {template} icon {icon} has no resolved terrain image"
            ),
            Self::UnknownTemplateType(template) => {
                write!(f, "map template type {template} is absent from the theater catalog")
            }
            Self::DuplicateTemplateType(template) => {
                write!(f, "map template type {template} appears more than once in the theater catalog")
            }
            Self::InconsistentTemplateIconSize { template, expected, actual } => write!(
                f,
                "map template type {template} has {actual:?} icon dimensions; expected {expected:?}"
            ),
            Self::InvalidTileImage(index) => {
                write!(f, "terrain tile image {index} has invalid dimensions or pixel storage")
            }
            Self::MapAllocationFailed { cells } => write!(f, "could not reserve terrain storage for {cells} cells"),
            Self::AtlasTooLarge { width, height, bytes } =>
                write!(f, "terrain atlas {width}x{height} ({bytes} bytes) exceeds supported limits"),
            Self::InvalidCamera => f.write_str("terrain camera, zoom, or viewport is invalid"),
            Self::Renderer(error) => write!(f, "terrain texture upload failed: {error}"),
            Self::Tmp(error) => write!(f, "terrain TMP decode failed: {error}"),
            Self::IconSet(error) => write!(f, "terrain ICON-set decode failed: {error}"),
        }
    }
}
impl std::error::Error for TerrainError {}
impl From<RendererError> for TerrainError {
    fn from(value: RendererError) -> Self { Self::Renderer(value.to_string()) }
}
impl From<TmpError> for TerrainError {
    fn from(value: TmpError) -> Self { Self::Tmp(value) }
}
impl From<IconSetError> for TerrainError {
    fn from(value: IconSetError) -> Self { Self::IconSet(value) }
}

/// Row-major map terrain references. None means no base terrain sprite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerrainMap {
    width: u32,
    height: u32,
    cells: Vec<Option<usize>>,
}
impl TerrainMap {
    pub fn new(width: u32, height: u32, cells: Vec<Option<usize>>) -> Result<Self, TerrainError> {
        let expected = (width as usize).checked_mul(height as usize);
        if width == 0 || height == 0 || expected != Some(cells.len()) {
            return Err(TerrainError::InvalidMapDimensions { width, height, cells: cells.len() });
        }
        Ok(Self { width, height, cells })
    }

    /// Resolves complete row-major template references to generic TMP tile indices.
    ///
    /// Each reference contains (template_type, logical_icon). The resolver
    /// must interpret both values; a template type is not itself a tile index.
    /// This adapter is for callers that explicitly have a TMP-backed mapping.
    /// Authentic TD/RA template rendering instead resolves each type to its
    /// theater-specific IFF ICON set and follows that set's MAP indirection.
    pub fn from_template_cells(
        width: u32,
        height: u32,
        templates: impl IntoIterator<Item = Option<(u16, u8)>>,
        mut resolve: impl FnMut(u16, u8) -> Option<usize>,
    ) -> Result<Self, TerrainError> {
        // Validate dimensions before consuming an arbitrary iterator or asking
        // the resolver to perform any work.
        let expected = (width as usize).checked_mul(height as usize);
        if width == 0 || height == 0 || expected.is_none() {
            return Err(TerrainError::InvalidMapDimensions {
                width,
                height,
                cells: 0,
            });
        }
        let expected = expected.expect("nonzero dimensions and checked product");

        let mut cells = Vec::new();
        cells.try_reserve_exact(expected).map_err(|_| {
            TerrainError::MapAllocationFailed { cells: expected }
        })?;

        for template in templates {
            if cells.len() == expected {
                return Err(TerrainError::InvalidMapDimensions {
                    width,
                    height,
                    cells: expected.saturating_add(1),
                });
            }
            let tile = match template {
                Some((template, icon)) => Some(
                    resolve(template, icon).ok_or(TerrainError::UnresolvedTemplateCell {
                        template,
                        icon,
                    })?,
                ),
                None => None,
            };
            cells.push(tile);
        }

        if cells.len() != expected {
            return Err(TerrainError::InvalidMapDimensions {
                width,
                height,
                cells: cells.len(),
            });
        }

        Ok(Self { width, height, cells })
    }
    pub const fn width(&self) -> u32 { self.width }
    pub const fn height(&self) -> u32 { self.height }
    pub fn tile_at(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height { return None; }
        self.cells.get((y as usize) * self.width as usize + x as usize).copied().flatten()
    }
}

/// Packed RGBA atlas containing only nonblank TMP tiles referenced by a map.
/// The texture ID must be unique for the map, theater, and palette identity.
#[derive(Debug, Clone)]
pub struct TerrainAtlas {
    texture_id: TextureId,
    tile_width: u32,
    tile_height: u32,
    atlas_columns: u32,
    tile_slots: HashMap<usize, u32>,
    image: RgbaImage,
}
impl TerrainAtlas {
    pub fn build(
        texture_id: TextureId,
        map: &TerrainMap,
        archive: &TmpArchive<'_>,
        palette: &Palette256,
    ) -> Result<Self, TerrainError> {
        let mut referenced = BTreeSet::new();
        for index in map.cells.iter().flatten().copied() {
            if archive.tile(index)?.is_some() {
                referenced.insert(index);
            }
        }

        let tile_width = u32::from(archive.width);
        let tile_height = u32::from(archive.height);
        let mut tiles = Vec::with_capacity(referenced.len());
        for tile_index in referenced {
            let tile = archive.tile_rgba(tile_index, palette, None)?
                .ok_or(TerrainError::InvalidTileIndex(tile_index))?;
            tiles.push((tile_index, tile));
        }

        Self::from_indexed_images(texture_id, tile_width, tile_height, tiles)
    }

    /// Builds a terrain atlas from authentic TD/RA template ICON sets.
    ///
    /// The supplied catalog must already be filtered to the selected game
    /// variant and theater. Each map cell contains a template type and its
    /// logical icon index. The icon set's MAP chunk resolves that logical
    /// index to a physical SSET image; empty or invalid MAP entries are errors
    /// for occupied template cells, matching the engine's map validation.
    ///
    /// Returns the resolved generic terrain grid alongside the packed atlas.
    pub fn build_from_template_icon_sets(
        texture_id: TextureId,
        width: u32,
        height: u32,
        templates: impl IntoIterator<Item = Option<(u16, u8)>>,
        icon_sets: &[(u16, &IconSet)],
        palette: &Palette256,
    ) -> Result<(TerrainMap, Self), TerrainError> {
        let expected = (width as usize).checked_mul(height as usize);
        if width == 0 || height == 0 || expected.is_none() {
            return Err(TerrainError::InvalidMapDimensions {
                width,
                height,
                cells: 0,
            });
        }
        let expected = expected.expect("nonzero dimensions and checked product");

        let mut catalog = HashMap::with_capacity(icon_sets.len());
        let mut tile_size = None;
        for &(template_type, icon_set) in icon_sets {
            if catalog.insert(template_type, icon_set).is_some() {
                return Err(TerrainError::DuplicateTemplateType(template_type));
            }
            let dimensions = (
                u32::try_from(icon_set.width()).map_err(|_| IconSetError::InvalidIconDimensions)?,
                u32::try_from(icon_set.height()).map_err(|_| IconSetError::InvalidIconDimensions)?,
            );
            if dimensions.0 == 0 || dimensions.1 == 0 {
                return Err(TerrainError::InvalidTileImage(template_type as usize));
            }
            if let Some(expected_size) = tile_size {
                if dimensions != expected_size {
                    return Err(TerrainError::InconsistentTemplateIconSize {
                        template: template_type,
                        expected: expected_size,
                        actual: dimensions,
                    });
                }
            } else {
                tile_size = Some(dimensions);
            }
        }

        let (tile_width, tile_height) = tile_size.unwrap_or((1, 1));
        let mut cells = Vec::new();
        cells.try_reserve_exact(expected).map_err(|_| {
            TerrainError::MapAllocationFailed { cells: expected }
        })?;
        let mut image_slots = HashMap::new();
        let mut images = Vec::new();

        for template in templates {
            if cells.len() == expected {
                return Err(TerrainError::InvalidMapDimensions {
                    width,
                    height,
                    cells: expected.saturating_add(1),
                });
            }

            let tile = match template {
                None => None,
                Some((template_type, logical_icon)) => {
                    if let Some(&slot) = image_slots.get(&(template_type, logical_icon)) {
                        Some(slot)
                    } else {
                        let icon_set = catalog.get(&template_type)
                            .copied()
                            .ok_or(TerrainError::UnknownTemplateType(template_type))?;
                        let next_count = images.len().checked_add(1)
                            .ok_or(TerrainError::AtlasTooLarge {
                                width: u32::MAX,
                                height: u32::MAX,
                                bytes: usize::MAX,
                            })?;
                        validate_atlas_layout(next_count, tile_width, tile_height)?;

                        let image = icon_set
                            .logical_icon_rgba(usize::from(logical_icon), palette, None)?
                            .ok_or(TerrainError::UnresolvedTemplateCell {
                                template: template_type,
                                icon: logical_icon,
                            })?;
                        let slot = images.len();
                        images.push(image);
                        image_slots.insert((template_type, logical_icon), slot);
                        Some(slot)
                    }
                }
            };
            cells.push(tile);
        }

        if cells.len() != expected {
            return Err(TerrainError::InvalidMapDimensions {
                width,
                height,
                cells: cells.len(),
            });
        }

        let map = TerrainMap { width, height, cells };
        let indexed_images = images.into_iter().enumerate().collect();
        let atlas = Self::from_indexed_images(
            texture_id,
            tile_width,
            tile_height,
            indexed_images,
        )?;
        Ok((map, atlas))
    }

    fn from_indexed_images(
        texture_id: TextureId,
        tile_width: u32,
        tile_height: u32,
        tiles: Vec<(usize, RgbaImage)>,
    ) -> Result<Self, TerrainError> {
        let count = tiles.len().max(1);
        let (width, height, byte_count) =
            validate_atlas_layout(count, tile_width, tile_height)?;
        let mut pixels = vec![0u8; byte_count];
        let mut tile_slots = HashMap::with_capacity(tiles.len());

        for (slot, (tile_index, tile)) in tiles.into_iter().enumerate() {
            let expected_bytes = (tile_width as usize)
                .checked_mul(tile_height as usize)
                .and_then(|size| size.checked_mul(4));
            if tile.width != tile_width
                || tile.height != tile_height
                || expected_bytes != Some(tile.pixels.len())
            {
                return Err(TerrainError::InvalidTileImage(tile_index));
            }
            copy_tile_into_atlas(
                &mut pixels,
                width,
                tile_width,
                tile_height,
                slot as u32,
                &tile,
            );
            tile_slots.insert(tile_index, slot as u32);
        }

        Ok(Self {
            texture_id,
            tile_width,
            tile_height,
            atlas_columns: width / tile_width,
            tile_slots,
            image: RgbaImage { width, height, pixels },
        })
    }

    pub const fn texture_id(&self) -> TextureId { self.texture_id }
    pub const fn tile_size(&self) -> (u32, u32) { (self.tile_width, self.tile_height) }
    pub const fn dimensions(&self) -> (u32, u32) { (self.image.width, self.image.height) }
    pub fn image(&self) -> &RgbaImage { &self.image }
    pub fn slot_for_tile(&self, tile_index: usize) -> Option<u32> {
        self.tile_slots.get(&tile_index).copied()
    }
    pub fn upload(&self, renderer: &mut WgpuRenderer) -> Result<(), TerrainError> {
        renderer.cache_rgba_image(self.texture_id, &self.image)?;
        Ok(())
    }

    /// Build the visible chunk batch and submit it using this atlas's cached texture ID.
    pub fn render_visible(
        &self,
        renderer: &mut WgpuRenderer,
        map: &TerrainMap,
        camera: Camera2d,
        viewport: Viewport,
        clear: Color,
        ui: &UiFrame,
    ) -> Result<FrameStats, TerrainError> {
        let batch = self.visible_batch(map, camera, viewport)?;
        Ok(renderer.render_cached(clear, Some((&batch, self.texture_id)), ui)?)
    }

    /// Emit screen-space sprites for visible cells, traversing 16x16 chunks.
    /// Camera center is in world pixels; zoom scales position and tile size.
    pub fn visible_batch(
        &self, map: &TerrainMap, camera: Camera2d, viewport: Viewport,
    ) -> Result<SpriteBatch, TerrainError> {
        if !viewport.is_visible() || !camera.center_x.is_finite()
            || !camera.center_y.is_finite() || !camera.zoom.is_finite() || camera.zoom <= 0.0
        { return Err(TerrainError::InvalidCamera); }

        let zoom = camera.zoom.clamp(0.05, 64.0);
        let left = camera.center_x - viewport.width as f32 / (2.0 * zoom);
        let top = camera.center_y - viewport.height as f32 / (2.0 * zoom);
        let right = camera.center_x + viewport.width as f32 / (2.0 * zoom);
        let bottom = camera.center_y + viewport.height as f32 / (2.0 * zoom);
        let min_x = ((left / self.tile_width as f32).floor() as i64).clamp(0, map.width as i64) as u32;
        let min_y = ((top / self.tile_height as f32).floor() as i64).clamp(0, map.height as i64) as u32;
        let max_x = ((right / self.tile_width as f32).ceil() as i64).clamp(0, map.width as i64) as u32;
        let max_y = ((bottom / self.tile_height as f32).ceil() as i64).clamp(0, map.height as i64) as u32;
        let mut batch = SpriteBatch::with_capacity(
            ((max_x - min_x) as usize).saturating_mul((max_y - min_y) as usize),
        );
        if min_x >= max_x || min_y >= max_y { return Ok(batch); }

        for chunk_y in (min_y / TERRAIN_CHUNK_CELLS)..=((max_y - 1) / TERRAIN_CHUNK_CELLS) {
            for chunk_x in (min_x / TERRAIN_CHUNK_CELLS)..=((max_x - 1) / TERRAIN_CHUNK_CELLS) {
                let x0 = (chunk_x * TERRAIN_CHUNK_CELLS).max(min_x);
                let y0 = (chunk_y * TERRAIN_CHUNK_CELLS).max(min_y);
                let x1 = ((chunk_x + 1) * TERRAIN_CHUNK_CELLS).min(max_x);
                let y1 = ((chunk_y + 1) * TERRAIN_CHUNK_CELLS).min(max_y);
                for y in y0..y1 {
                    for x in x0..x1 {
                        let Some(tile_index) = map.tile_at(x, y) else { continue; };
                        let Some(slot) = self.slot_for_tile(tile_index) else { continue; };
                        let ax = slot % self.atlas_columns;
                        let ay = slot / self.atlas_columns;
                        let u0 = ax as f32 * self.tile_width as f32 / self.image.width as f32;
                        let v0 = ay as f32 * self.tile_height as f32 / self.image.height as f32;
                        let u1 = (ax + 1) as f32 * self.tile_width as f32 / self.image.width as f32;
                        let v1 = (ay + 1) as f32 * self.tile_height as f32 / self.image.height as f32;
                        let sx = (x as f32 * self.tile_width as f32 - camera.center_x) * zoom
                            + viewport.width as f32 * 0.5;
                        let sy = (y as f32 * self.tile_height as f32 - camera.center_y) * zoom
                            + viewport.height as f32 * 0.5;
                        let mut sprite = SpriteInstance::new(
                            self.texture_id, RenderLayer::Terrain, sx, sy,
                            self.tile_width as f32 * zoom, self.tile_height as f32 * zoom,
                        );
                        sprite.uv = [u0, v0, u1, v1];
                        sprite.tint = Color::rgba(1.0, 1.0, 1.0, 1.0);
                        batch.push(sprite);
                    }
                }
            }
        }
        Ok(batch)
    }
}

fn validate_atlas_layout(
    tile_count: usize,
    tile_width: u32,
    tile_height: u32,
) -> Result<(u32, u32, usize), TerrainError> {
    let count = u32::try_from(tile_count.max(1)).map_err(|_| TerrainError::AtlasTooLarge {
        width: u32::MAX,
        height: u32::MAX,
        bytes: usize::MAX,
    })?;
    let columns = (count as f64).sqrt().ceil() as u32;
    let rows = count.div_ceil(columns.max(1));
    let width = columns.checked_mul(tile_width).unwrap_or(u32::MAX);
    let height = rows.checked_mul(tile_height).unwrap_or(u32::MAX);
    let byte_count = (width as usize)
        .checked_mul(height as usize)
        .and_then(|bytes| bytes.checked_mul(4));
    if tile_width == 0
        || tile_height == 0
        || width == 0
        || height == 0
        || width > MAX_ATLAS_DIMENSION
        || height > MAX_ATLAS_DIMENSION
        || byte_count.is_none_or(|bytes| bytes > MAX_ATLAS_BYTES)
    {
        return Err(TerrainError::AtlasTooLarge {
            width,
            height,
            bytes: byte_count.unwrap_or(usize::MAX),
        });
    }
    Ok((width, height, byte_count.expect("validated atlas byte count")))
}

fn copy_tile_into_atlas(
    atlas: &mut [u8], atlas_width: u32, tile_width: u32, tile_height: u32,
    slot: u32, tile: &RgbaImage,
) {
    let columns = atlas_width / tile_width;
    let ox = (slot % columns) * tile_width;
    let oy = (slot / columns) * tile_height;
    for row in 0..tile_height {
        let src = (row * tile_width * 4) as usize;
        let dst = (((oy + row) * atlas_width + ox) * 4) as usize;
        let bytes = (tile_width * 4) as usize;
        atlas[dst..dst + bytes].copy_from_slice(&tile.pixels[src..src + bytes]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rts_formats::{
        palette_rgba::{PaletteEncoding, PALETTE_BYTES},
        tmp::TmpVariant,
    };

    fn fixture() -> Vec<u8> {
        let mut bytes = vec![0u8; 42];
        bytes[0..2].copy_from_slice(&2u16.to_le_bytes());
        bytes[2..4].copy_from_slice(&2u16.to_le_bytes());
        bytes[12..16].copy_from_slice(&32u32.to_le_bytes());
        bytes[20..24].copy_from_slice(&0x0d1a_ffffu32.to_le_bytes());
        bytes[24..28].copy_from_slice(&42i32.to_le_bytes());
        bytes[28..32].copy_from_slice(&40i32.to_le_bytes());
        bytes[32..36].copy_from_slice(&[1, 1, 1, 1]);
        bytes[36..40].copy_from_slice(&[2, 2, 2, 2]);
        bytes[40..42].copy_from_slice(&[0, 1]);
        bytes
    }
    fn palette() -> Palette256 {
        let mut raw = vec![0u8; PALETTE_BYTES];
        raw[3..6].copy_from_slice(&[200, 10, 20]);
        raw[6..9].copy_from_slice(&[10, 200, 20]);
        raw[9..12].copy_from_slice(&[20, 30, 220]);
        Palette256::parse(&raw, PaletteEncoding::Rgb8).unwrap()
    }

    fn icon_set_fixture(second_pixel: u8, logical_map: &[u8]) -> Vec<u8> {
        fn chunk(id: [u8; 4], data: &[u8], output: &mut Vec<u8>) {
            output.extend_from_slice(&id);
            output.extend_from_slice(&(data.len() as u32).to_be_bytes());
            output.extend_from_slice(data);
            if data.len() & 1 != 0 {
                output.push(0);
            }
        }

        let mut body = b"ICON".to_vec();
        chunk(*b"SINF", &[1, 1, 0, 8], &mut body);
        let mut pixels = vec![1u8; 64];
        pixels.extend(std::iter::repeat_n(second_pixel, 64));
        chunk(*b"SSET", &pixels, &mut body);
        chunk(*b"MAP ", logical_map, &mut body);

        let mut file = b"FORM".to_vec();
        file.extend_from_slice(&(body.len() as u32).to_be_bytes());
        file.extend_from_slice(&body);
        file
    }

    #[test]
    fn resolves_map_document_template_ids_without_assuming_tile_identity() {
        let map = TerrainMap::from_template_cells(
            3,
            1,
            [Some((12, 3)), None, Some((4, 1))],
            |template, icon| match (template, icon) {
                (12, 3) => Some(7),
                (4, 1) => Some(2),
                _ => None,
            },
        ).unwrap();

        assert_eq!(map.tile_at(0, 0), Some(7));
        assert_eq!(map.tile_at(1, 0), None);
        assert_eq!(map.tile_at(2, 0), Some(2));
    }

    #[test]
    fn rejects_unresolved_map_template_type_and_icon_pairs() {
        assert_eq!(
            TerrainMap::from_template_cells(1, 1, [Some((99, 4))], |_, _| None).unwrap_err(),
            TerrainError::UnresolvedTemplateCell { template: 99, icon: 4 },
        );
    }

    #[test]
    fn rejects_template_iterators_with_too_few_or_too_many_cells() {
        let too_few = TerrainMap::from_template_cells(2, 1, [Some(1)], |_| Some(0));
        assert_eq!(
            too_few.unwrap_err(),
            TerrainError::InvalidMapDimensions { width: 2, height: 1, cells: 1 },
        );

        let too_many = TerrainMap::from_template_cells(1, 1, [Some(1), Some(2)], |_| Some(0));
        assert_eq!(
            too_many.unwrap_err(),
            TerrainError::InvalidMapDimensions { width: 1, height: 1, cells: 2 },
        );
    }

    #[test]
    fn validates_dimensions_before_consuming_templates_or_resolving_ids() {
        let mut consumed = false;
        let mut resolved = false;
        let result = TerrainMap::from_template_cells(
            0,
            1,
            std::iter::from_fn(|| {
                consumed = true;
                Some(Some(1))
            }),
            |_| {
                resolved = true;
                Some(0)
            },
        );

        assert_eq!(
            result.unwrap_err(),
            TerrainError::InvalidMapDimensions { width: 0, height: 1, cells: 0 },
        );
        assert!(!consumed);
        assert!(!resolved);
    }

    #[test]
    fn builds_atlas_from_multiple_template_icon_sets_and_map_indirection() {
        let first_bytes = icon_set_fixture(2, &[1]);
        let second_bytes = icon_set_fixture(3, &[1]);
        let first = IconSet::load(&first_bytes, None).unwrap();
        let second = IconSet::load(&second_bytes, None).unwrap();

        let (map, atlas) = TerrainAtlas::build_from_template_icon_sets(
            88,
            3,
            1,
            [Some((10, 0)), None, Some((20, 0))],
            &[(10, &first), (20, &second)],
            &palette(),
        ).unwrap();

        assert_eq!(map.tile_at(0, 0), Some(0));
        assert_eq!(map.tile_at(1, 0), None);
        assert_eq!(map.tile_at(2, 0), Some(1));
        assert_eq!(atlas.slot_for_tile(0), Some(0));
        assert_eq!(atlas.slot_for_tile(1), Some(1));
        assert_eq!(atlas.dimensions(), (16, 8));
        assert_eq!(&atlas.image().pixels[0..4], &[10, 200, 20, 255]);
        assert_eq!(&atlas.image().pixels[32..36], &[20, 30, 220, 255]);
    }

    #[test]
    fn rejects_template_cells_whose_icon_set_map_entry_is_empty() {
        let bytes = icon_set_fixture(2, &[u8::MAX]);
        let icon_set = IconSet::load(&bytes, None).unwrap();
        let result = TerrainAtlas::build_from_template_icon_sets(
            88,
            1,
            1,
            [Some((10, 0))],
            &[(10, &icon_set)],
            &palette(),
        );

        assert_eq!(
            result.unwrap_err(),
            TerrainError::UnresolvedTemplateCell { template: 10, icon: 0 },
        );
    }

    #[test]
    fn packs_only_referenced_tiles_in_deterministic_order() {
        let bytes = fixture();
        let archive = TmpArchive::parse_as(&bytes, TmpVariant::TiberianDawn).unwrap();
        let map = TerrainMap::new(3, 1, vec![Some(1), Some(0), Some(1)]).unwrap();
        let atlas = TerrainAtlas::build(77, &map, &archive, &palette()).unwrap();
        assert_eq!(atlas.dimensions(), (4, 2));
        assert_eq!(atlas.tile_size(), (2, 2));
        assert_eq!(atlas.slot_for_tile(0), Some(0));
        assert_eq!(atlas.slot_for_tile(1), Some(1));
        assert_eq!(&atlas.image().pixels[0..4], &[200, 10, 20, 255]);
        assert_eq!(&atlas.image().pixels[8..12], &[10, 200, 20, 255]);
    }

    #[test]
    fn visible_batch_culls_and_emits_atlas_uvs() {
        let bytes = fixture();
        let archive = TmpArchive::parse_as(&bytes, TmpVariant::TiberianDawn).unwrap();
        let map = TerrainMap::new(3, 1, vec![Some(0), Some(1), Some(0)]).unwrap();
        let atlas = TerrainAtlas::build(77, &map, &archive, &palette()).unwrap();
        let batch = atlas.visible_batch(
            &map, Camera2d { center_x: 3.0, center_y: 1.0, zoom: 1.0 }, Viewport::new(6, 2),
        ).unwrap();
        assert_eq!(batch.len(), 3);
        let mut ordered = batch.clone();
        assert_eq!(ordered.ordered()[0].texture, 77);
        assert_eq!(ordered.ordered()[0].layer, RenderLayer::Terrain);
        assert_eq!(ordered.ordered()[0].uv, [0.0, 0.0, 0.5, 1.0]);
    }

    #[test]
    fn empty_cells_produce_no_sprites_or_atlas_slots() {
        let bytes = fixture();
        let archive = TmpArchive::parse_as(&bytes, TmpVariant::TiberianDawn).unwrap();
        let map = TerrainMap::new(2, 1, vec![None, None]).unwrap();
        let atlas = TerrainAtlas::build(77, &map, &archive, &palette()).unwrap();
        assert!(atlas.slot_for_tile(0).is_none());
        let batch = atlas.visible_batch(
            &map, Camera2d { center_x: 2.0, center_y: 1.0, zoom: 1.0 }, Viewport::new(4, 2),
        ).unwrap();
        assert!(batch.is_empty());
    }
}
