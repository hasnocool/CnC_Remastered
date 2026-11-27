//! Reusable CPU-side composition for camera-visible terrain chunks.
//!
//! Cache entries retain map-space cell/atlas-slot pairs, never screen-space
//! coordinates. Camera movement therefore reuses composition without stale
//! transforms. Chunk contents are compared exactly, so edits rebuild only the
//! affected chunks without relying on caller-maintained revision counters.

use std::collections::HashMap;

use crate::{
    Camera2d, RenderLayer, SpriteBatch, SpriteInstance, TerrainAtlas, TerrainError,
    TerrainMap, Viewport, TERRAIN_CHUNK_CELLS,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TerrainChunkCacheStats {
    pub hits: u64,
    pub misses: u64,
    pub rebuilds: u64,
}

#[derive(Debug, Clone)]
struct CachedChunk {
    /// Exact row-major map content for this (possibly edge-clipped) chunk.
    width: u32,
    height: u32,
    cells: Vec<Option<usize>>,
    /// (map_x, map_y, atlas_slot), in row-major order.
    sprites: Vec<(u32, u32, u32)>,
}

/// CPU composition cache for terrain. Reuse one cache per active map/atlas.
/// A changed chunk is detected by exact cell comparison; unchanged chunks are
/// retained even when edits elsewhere change the map.
#[derive(Debug, Default)]
pub struct TerrainChunkCache {
    chunks: HashMap<(u32, u32), CachedChunk>,
    stats: TerrainChunkCacheStats,
    atlas_identity: Option<(u64, (u32, u32), (u32, u32))>,
}

impl TerrainChunkCache {
    pub fn new() -> Self { Self::default() }

    pub const fn stats(&self) -> TerrainChunkCacheStats { self.stats }

    /// Explicitly invalidate one chunk after a known edit.
    pub fn invalidate_chunk(&mut self, chunk_x: u32, chunk_y: u32) {
        self.chunks.remove(&(chunk_x, chunk_y));
    }

    /// Clear all cached composition and counters.
    pub fn clear(&mut self) {
        self.chunks.clear();
        self.stats = TerrainChunkCacheStats::default();
        self.atlas_identity = None;
    }

    fn prepare_atlas(&mut self, atlas: &TerrainAtlas) {
        let identity = (atlas.texture_id(), atlas.tile_size(), atlas.dimensions());
        if self.atlas_identity != Some(identity) {
            self.chunks.clear();
            self.stats = TerrainChunkCacheStats::default();
            self.atlas_identity = Some(identity);
        }
    }
}

impl TerrainAtlas {
    /// Emit visible sprites while reusing unchanged 16x16 map-space chunks.
    ///
    /// The cache stores only map coordinates and atlas slots; screen-space
    /// transforms are recalculated for every camera/viewport. Cell contents
    /// are compared exactly, so callers do not need a global map revision.
    pub fn visible_batch_cached(
        &self,
        map: &TerrainMap,
        camera: Camera2d,
        viewport: Viewport,
        cache: &mut TerrainChunkCache,
    ) -> Result<SpriteBatch, TerrainError> {
        if !viewport.is_visible()
            || !camera.center_x.is_finite()
            || !camera.center_y.is_finite()
            || !camera.zoom.is_finite()
            || camera.zoom <= 0.0
        {
            return Err(TerrainError::InvalidCamera);
        }

        cache.prepare_atlas(self);

        let (tile_width, tile_height) = self.tile_size();
        let (atlas_width, atlas_height) = self.dimensions();
        let atlas_columns = atlas_width / tile_width;
        let zoom = camera.zoom.clamp(0.05, 64.0);
        let left = camera.center_x - viewport.width as f32 / (2.0 * zoom);
        let top = camera.center_y - viewport.height as f32 / (2.0 * zoom);
        let right = camera.center_x + viewport.width as f32 / (2.0 * zoom);
        let bottom = camera.center_y + viewport.height as f32 / (2.0 * zoom);
        let min_x = ((left / tile_width as f32).floor() as i64)
            .clamp(0, map.width() as i64) as u32;
        let min_y = ((top / tile_height as f32).floor() as i64)
            .clamp(0, map.height() as i64) as u32;
        let max_x = ((right / tile_width as f32).ceil() as i64)
            .clamp(0, map.width() as i64) as u32;
        let max_y = ((bottom / tile_height as f32).ceil() as i64)
            .clamp(0, map.height() as i64) as u32;

        let capacity = ((max_x - min_x) as usize)
            .saturating_mul((max_y - min_y) as usize);
        let mut batch = SpriteBatch::with_capacity(capacity);
        if min_x >= max_x || min_y >= max_y {
            return Ok(batch);
        }

        let first_chunk_x = min_x / TERRAIN_CHUNK_CELLS;
        let first_chunk_y = min_y / TERRAIN_CHUNK_CELLS;
        let last_chunk_x = (max_x - 1) / TERRAIN_CHUNK_CELLS;
        let last_chunk_y = (max_y - 1) / TERRAIN_CHUNK_CELLS;

        for chunk_y in first_chunk_y..=last_chunk_y {
            for chunk_x in first_chunk_x..=last_chunk_x {
                let chunk_x0 = chunk_x * TERRAIN_CHUNK_CELLS;
                let chunk_y0 = chunk_y * TERRAIN_CHUNK_CELLS;
                let chunk_x1 = (chunk_x0 + TERRAIN_CHUNK_CELLS).min(map.width());
                let chunk_y1 = (chunk_y0 + TERRAIN_CHUNK_CELLS).min(map.height());

                let chunk_width = chunk_x1 - chunk_x0;
                let chunk_height = chunk_y1 - chunk_y0;
                let mut cells = Vec::with_capacity(
                    (chunk_width as usize).saturating_mul(chunk_height as usize),
                );
                for y in chunk_y0..chunk_y1 {
                    for x in chunk_x0..chunk_x1 {
                        cells.push(map.tile_at(x, y));
                    }
                }

                let key = (chunk_x, chunk_y);
                let cached = match cache.chunks.get_mut(&key) {
                    Some(existing)
                        if existing.width == chunk_width
                            && existing.height == chunk_height
                            && existing.cells == cells => {
                        cache.stats.hits = cache.stats.hits.saturating_add(1);
                        existing
                    }
                    Some(existing) => {
                        cache.stats.misses = cache.stats.misses.saturating_add(1);
                        cache.stats.rebuilds = cache.stats.rebuilds.saturating_add(1);
                        let mut composed = Vec::new();
                        for (index, tile) in cells.iter().copied().enumerate() {
                            if let Some(slot) = tile.and_then(|value| self.slot_for_tile(value)) {
                                let x = chunk_x0 + (index % chunk_width as usize) as u32;
                                let y = chunk_y0 + (index / chunk_width as usize) as u32;
                                composed.push((x, y, slot));
                            }
                        }
                        existing.width = chunk_width;
                        existing.height = chunk_height;
                        existing.cells = cells;
                        existing.sprites = composed;
                        existing
                    }
                    None => {
                        cache.stats.misses = cache.stats.misses.saturating_add(1);
                        cache.stats.rebuilds = cache.stats.rebuilds.saturating_add(1);
                        let mut composed = Vec::new();
                        for (index, tile) in cells.iter().copied().enumerate() {
                            if let Some(slot) = tile.and_then(|value| self.slot_for_tile(value)) {
                                let x = chunk_x0 + (index % chunk_width as usize) as u32;
                                let y = chunk_y0 + (index / chunk_width as usize) as u32;
                                composed.push((x, y, slot));
                            }
                        }
                        cache.chunks.insert(key, CachedChunk {
                            width: chunk_width,
                            height: chunk_height,
                            cells,
                            sprites: composed,
                        });
                        cache.chunks.get_mut(&key).expect("chunk inserted")
                    }
                };

                for &(x, y, slot) in &cached.sprites {
                    if x < min_x || x >= max_x || y < min_y || y >= max_y {
                        continue;
                    }
                    let atlas_x = slot % atlas_columns;
                    let atlas_y = slot / atlas_columns;
                    let u0 = atlas_x as f32 * tile_width as f32 / atlas_width as f32;
                    let v0 = atlas_y as f32 * tile_height as f32 / atlas_height as f32;
                    let u1 = (atlas_x + 1) as f32 * tile_width as f32 / atlas_width as f32;
                    let v1 = (atlas_y + 1) as f32 * tile_height as f32 / atlas_height as f32;
                    let sx = (x as f32 * tile_width as f32 - camera.center_x) * zoom
                        + viewport.width as f32 * 0.5;
                    let sy = (y as f32 * tile_height as f32 - camera.center_y) * zoom
                        + viewport.height as f32 * 0.5;
                    let mut sprite = SpriteInstance::new(
                        self.texture_id(),
                        RenderLayer::Terrain,
                        sx,
                        sy,
                        tile_width as f32 * zoom,
                        tile_height as f32 * zoom,
                    );
                    sprite.uv = [u0, v0, u1, v1];
                    batch.push(sprite);
                }
            }
        }

        Ok(batch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rts_formats::{
        iconset::IconSet,
        palette_rgba::{Palette256, PaletteEncoding},
    };

    fn fixture_atlas() -> TerrainAtlas {
        let mut body = b"ICON".to_vec();
        for (id, data) in [
            (*b"SINF", &[1, 1, 0, 8][..]),
            (*b"SSET", &[1u8; 128][..]),
            (*b"MAP ", &[0, 1][..]),
        ] {
            body.extend_from_slice(&id);
            body.extend_from_slice(&(data.len() as u32).to_be_bytes());
            body.extend_from_slice(data);
            if data.len() & 1 != 0 { body.push(0); }
        }
        let mut bytes = b"FORM".to_vec();
        bytes.extend_from_slice(&(body.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&body);
        let icon_set = IconSet::load(&bytes, None).unwrap();
        let palette = Palette256::parse(&vec![0; 768], PaletteEncoding::Rgb8).unwrap();
        let (_, atlas) = TerrainAtlas::build_from_template_icon_sets(
            99, 2, 1, [Some((10, 0)), Some((10, 1))],
            &[(10, &icon_set)], &palette,
        ).unwrap();
        atlas
    }

    #[test]
    fn caches_unchanged_chunks_and_rebuilds_only_edited_chunk() {
        let atlas = fixture_atlas();
        let map = TerrainMap::new(33, 1, vec![Some(0); 33]).unwrap();
        let mut cache = TerrainChunkCache::new();
        let camera = Camera2d { center_x: 396.0, center_y: 12.0, zoom: 1.0 };
        let viewport = Viewport::new(792, 24);

        atlas.visible_batch_cached(&map, camera, viewport, &mut cache).unwrap();
        assert_eq!(cache.stats().misses, 3);
        assert_eq!(cache.stats().rebuilds, 3);

        atlas.visible_batch_cached(&map, camera, viewport, &mut cache).unwrap();
        assert_eq!(cache.stats().hits, 3);
        assert_eq!(cache.stats().rebuilds, 3);

        let mut changed_cells = vec![Some(0); 33];
        changed_cells[1] = Some(1);
        let changed_map = TerrainMap::new(33, 1, changed_cells).unwrap();
        atlas.visible_batch_cached(&changed_map, camera, viewport, &mut cache).unwrap();
        assert_eq!(cache.stats().hits, 5);
        assert_eq!(cache.stats().misses, 4);
        assert_eq!(cache.stats().rebuilds, 4);
    }

    #[test]
    fn recalculates_screen_positions_when_camera_moves() {
        let atlas = fixture_atlas();
        let map = TerrainMap::new(1, 1, vec![Some(0)]).unwrap();
        let mut cache = TerrainChunkCache::new();
        let mut first = atlas.visible_batch_cached(
            &map, Camera2d { center_x: 12.0, center_y: 12.0, zoom: 1.0 },
            Viewport::new(24, 24), &mut cache,
        ).unwrap();
        let mut moved = atlas.visible_batch_cached(
            &map, Camera2d { center_x: 0.0, center_y: 0.0, zoom: 2.0 },
            Viewport::new(24, 24), &mut cache,
        ).unwrap();
        assert_eq!(cache.stats().hits, 1);
        assert_ne!(first.ordered()[0].x, moved.ordered()[0].x);
    }
}
