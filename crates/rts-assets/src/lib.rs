//! Backend-independent Westwood theater asset resolution and decoded terrain images.
//!
//! This crate owns no GPU or UI resources. It resolves source-derived TD/RA
//! template names from caller-provided MIX archives, decodes ICONSET logical
//! icons through their MAP indirection, and returns reusable RGBA tile images.

use std::collections::{BTreeSet, HashMap};
use rts_core::{GameVariant, template_catalog::{TemplateTheater, template_asset_for_theater, template_asset_metadata}};
use rts_formats::{
    iconset::{IconSet, IconSetError},
    mix::MixArchive,
    palette_rgba::{Palette256, RgbaImage},
};

const MAX_TERRAIN_IMAGE_BYTES: usize = 128 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateAssetError {
    UnknownTemplateType(u16),
    UnavailableInTheater { template: u16, variant: GameVariant, theater: TemplateTheater },
    MissingAsset { template: u16, filename: String, filename_crc: i32 },
    DuplicateTemplateType(u16),
    WrongVariant { expected: GameVariant, actual: GameVariant },
    InvalidMapDimensions { width: u32, height: u32, cells: usize },
    UnresolvedTemplateCell { template: u16, icon: u8 },
    InconsistentIconSize { template: u16, expected: (u32, u32), actual: (u32, u32) },
    ImageBudgetExceeded { requested: usize, limit: usize },
    AllocationFailed,
    IconSet(IconSetError),
}
impl std::fmt::Display for TemplateAssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownTemplateType(id) => write!(f, "template type {id} is absent from the source-derived catalog"),
            Self::UnavailableInTheater { template, variant, theater } => write!(f, "template type {template} is unavailable in {variant:?} theater {theater:?}"),
            Self::MissingAsset { template, filename, filename_crc } => write!(f, "template type {template} asset {filename} (MIX CRC {filename_crc:#010x}) was not found"),
            Self::DuplicateTemplateType(id) => write!(f, "template type {id} was requested more than once"),
            Self::WrongVariant { expected, actual } => write!(f, "map variant {actual:?} cannot be loaded by this provider; expected {expected:?}"),
            Self::InvalidMapDimensions { width, height, cells } => write!(f, "terrain map {width}x{height} does not match {cells} cells"),
            Self::UnresolvedTemplateCell { template, icon } => write!(f, "template {template} logical icon {icon} has no image in this theater"),
            Self::InconsistentIconSize { template, expected, actual } => write!(f, "template {template} icon size {actual:?} differs from expected {expected:?}"),
            Self::ImageBudgetExceeded { requested, limit } => write!(f, "decoded terrain images require {requested} bytes, exceeding {limit}"),
            Self::AllocationFailed => f.write_str("could not reserve terrain image storage"),
            Self::IconSet(error) => write!(f, "template ICON set could not be parsed: {error}"),
        }
    }
}
impl std::error::Error for TemplateAssetError {}
impl From<IconSetError> for TemplateAssetError {
    fn from(value: IconSetError) -> Self { Self::IconSet(value) }
}

/// Decoded, renderer-independent terrain map. Cell entries reference entries
/// in tiles; None represents a cell without a template image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerrainAssetSet {
    width: u32,
    height: u32,
    tile_width: u32,
    tile_height: u32,
    cells: Vec<Option<usize>>,
    tiles: Vec<RgbaImage>,
}
impl TerrainAssetSet {
    pub const fn dimensions(&self) -> (u32, u32) { (self.width, self.height) }
    pub const fn tile_size(&self) -> (u32, u32) { (self.tile_width, self.tile_height) }
    pub fn tiles(&self) -> &[RgbaImage] { &self.tiles }
    pub fn tile_for_cell(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height { return None; }
        self.cells.get((y as usize) * self.width as usize + x as usize).copied().flatten()
    }
    pub fn cells(&self) -> &[Option<usize>] { &self.cells }
}

/// Resolves source-derived template names against caller-owned MIX archives.
/// Archive precedence follows slice order, matching the upstream registered
/// MIX search order. Archive and file bytes remain owned by the caller.
pub struct TemplateAssetProvider<'a> {
    variant: GameVariant,
    theater: TemplateTheater,
    archives: &'a [&'a MixArchive<'a>],
}
impl<'a> TemplateAssetProvider<'a> {
    pub fn new(variant: GameVariant, theater: TemplateTheater, archives: &'a [&'a MixArchive<'a>]) -> Self {
        Self { variant, theater, archives }
    }

    pub const fn variant(&self) -> GameVariant { self.variant }
    pub const fn theater(&self) -> TemplateTheater { self.theater }

    pub fn filename_for_template(&self, template_type: u16) -> Result<String, TemplateAssetError> {
        let metadata = template_asset_metadata(self.variant, template_type)
            .ok_or(TemplateAssetError::UnknownTemplateType(template_type))?;
        if template_asset_for_theater(self.variant, template_type, self.theater).is_none() {
            return Err(TemplateAssetError::UnavailableInTheater {
                template: template_type, variant: self.variant, theater: self.theater,
            });
        }
        let suffix = theater_suffix(self.variant, self.theater)
            .ok_or(TemplateAssetError::UnavailableInTheater {
                template: template_type, variant: self.variant, theater: self.theater,
            })?;
        Ok(format!("{}{}", metadata.asset_stem, suffix))
    }

    pub fn load_icon_set(&self, template_type: u16) -> Result<IconSet, TemplateAssetError> {
        let filename = self.filename_for_template(template_type)?;
        let filename_crc = rts_formats::mix::filename_crc(&filename);
        let bytes = self.archives.iter().find_map(|archive| archive.get_by_crc(filename_crc))
            .ok_or_else(|| TemplateAssetError::MissingAsset {
                template: template_type, filename, filename_crc,
            })?;
        Ok(IconSet::load(bytes, None)?)
    }

    pub fn load_icon_sets(&self, template_types: impl IntoIterator<Item = u16>) -> Result<Vec<(u16, IconSet)>, TemplateAssetError> {
        let mut seen = BTreeSet::new();
        let mut loaded = Vec::new();
        for template_type in template_types {
            if !seen.insert(template_type) {
                return Err(TemplateAssetError::DuplicateTemplateType(template_type));
            }
            loaded.push((template_type, self.load_icon_set(template_type)?));
        }
        Ok(loaded)
    }

    /// Decode populated cells to a shared set of unique RGBA images. Logical
    /// icons are resolved through ICONSET MAP indirection, not treated as
    /// physical icon indices or TMP tile indices.
    pub fn build_terrain(
        &self,
        width: u32,
        height: u32,
        templates: impl IntoIterator<Item = Option<(u16, u8)>>,
        palette: &Palette256,
    ) -> Result<TerrainAssetSet, TemplateAssetError> {
        let expected = (width as usize).checked_mul(height as usize);
        if width == 0 || height == 0 || expected.is_none() {
            return Err(TemplateAssetError::InvalidMapDimensions { width, height, cells: 0 });
        }
        let expected = expected.expect("validated non-zero dimensions");
        let mut template_cells = Vec::new();
        template_cells.try_reserve_exact(expected).map_err(|_| TemplateAssetError::AllocationFailed)?;
        for cell in templates {
            if template_cells.len() == expected {
                return Err(TemplateAssetError::InvalidMapDimensions {
                    width, height, cells: expected.saturating_add(1),
                });
            }
            template_cells.push(cell);
        }
        if template_cells.len() != expected {
            return Err(TemplateAssetError::InvalidMapDimensions {
                width, height, cells: template_cells.len(),
            });
        }

        let template_types = template_cells.iter().flatten().map(|(kind, _)| *kind).collect::<BTreeSet<_>>();
        let loaded = self.load_icon_sets(template_types.iter().copied())?;
        let icon_sets = loaded.into_iter().collect::<HashMap<_, _>>();
        let mut cells = Vec::new();
        cells.try_reserve_exact(expected).map_err(|_| TemplateAssetError::AllocationFailed)?;
        let mut tile_lookup = HashMap::<(u16, u8), usize>::new();
        let mut tiles = Vec::<RgbaImage>::new();
        let mut tile_size: Option<(u32, u32)> = None;
        let mut total_bytes = 0usize;

        for cell in template_cells {
            let Some((template_type, logical_icon)) = cell else {
                cells.push(None);
                continue;
            };
            if let Some(&slot) = tile_lookup.get(&(template_type, logical_icon)) {
                cells.push(Some(slot));
                continue;
            }
            let icon_set = icon_sets.get(&template_type)
                .ok_or(TemplateAssetError::UnknownTemplateType(template_type))?;
            let image = icon_set.logical_icon_rgba(usize::from(logical_icon), palette, None)?
                .ok_or(TemplateAssetError::UnresolvedTemplateCell { template: template_type, icon: logical_icon })?;
            let actual_size = (image.width, image.height);
            if let Some(expected_size) = tile_size {
                if actual_size != expected_size {
                    return Err(TemplateAssetError::InconsistentIconSize {
                        template: template_type, expected: expected_size, actual: actual_size,
                    });
                }
            } else {
                tile_size = Some(actual_size);
            }
            total_bytes = total_bytes.checked_add(image.pixels.len())
                .ok_or(TemplateAssetError::ImageBudgetExceeded { requested: usize::MAX, limit: MAX_TERRAIN_IMAGE_BYTES })?;
            if total_bytes > MAX_TERRAIN_IMAGE_BYTES {
                return Err(TemplateAssetError::ImageBudgetExceeded {
                    requested: total_bytes, limit: MAX_TERRAIN_IMAGE_BYTES,
                });
            }
            let slot = tiles.len();
            tiles.try_reserve(1).map_err(|_| TemplateAssetError::AllocationFailed)?;
            tiles.push(image);
            tile_lookup.insert((template_type, logical_icon), slot);
            cells.push(Some(slot));
        }
        let (tile_width, tile_height) = tile_size.unwrap_or((1, 1));
        Ok(TerrainAssetSet { width, height, tile_width, tile_height, cells, tiles })
    }
}

fn theater_suffix(variant: GameVariant, theater: TemplateTheater) -> Option<&'static str> {
    match (variant, theater) {
        (GameVariant::TiberianDawn, TemplateTheater::Desert) => Some(".DES"),
        (GameVariant::TiberianDawn, TemplateTheater::Jungle) => Some(".JUN"),
        (GameVariant::TiberianDawn, TemplateTheater::Temperate) => Some(".TEM"),
        (GameVariant::TiberianDawn, TemplateTheater::Winter) => Some(".WIN"),
        (GameVariant::RedAlert, TemplateTheater::Temperate) => Some(".TEM"),
        (GameVariant::RedAlert, TemplateTheater::Snow) => Some(".SNO"),
        (GameVariant::RedAlert, TemplateTheater::Interior) => Some(".INT"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_source_catalog_names_with_variant_specific_suffixes() {
        let empty: [&MixArchive<'_>; 0] = [];
        let td = TemplateAssetProvider::new(GameVariant::TiberianDawn, TemplateTheater::Desert, &empty);
        let ra = TemplateAssetProvider::new(GameVariant::RedAlert, TemplateTheater::Snow, &empty);
        assert_eq!(td.filename_for_template(0).unwrap(), "CLEAR1.DES");
        assert_eq!(ra.filename_for_template(3).unwrap(), "SH01.SNO");
    }

    #[test]
    fn rejects_cross_variant_theaters_before_archive_lookup() {
        let empty: [&MixArchive<'_>; 0] = [];
        let provider = TemplateAssetProvider::new(GameVariant::RedAlert, TemplateTheater::Desert, &empty);
        assert!(matches!(provider.filename_for_template(3), Err(TemplateAssetError::UnavailableInTheater { .. })));
    }

    #[test]
    fn validates_cell_count_before_resolving_assets() {
        let empty: [&MixArchive<'_>; 0] = [];
        let provider = TemplateAssetProvider::new(GameVariant::TiberianDawn, TemplateTheater::Temperate, &empty);
        let palette = Palette256::parse(&vec![0; 768], rts_formats::palette_rgba::PaletteEncoding::Rgb8).unwrap();
        assert!(matches!(
            provider.build_terrain(2, 1, [None], &palette),
            Err(TemplateAssetError::InvalidMapDimensions { .. })
        ));
    }
    #[test]
    fn builds_empty_cell_grid_without_archive_lookup() {
        let empty: [&MixArchive<'_>; 0] = [];
        let provider = TemplateAssetProvider::new(GameVariant::TiberianDawn, TemplateTheater::Temperate, &empty);
        let palette = Palette256::parse(
            &vec![0; rts_formats::palette_rgba::PALETTE_BYTES],
            rts_formats::palette_rgba::PaletteEncoding::Rgb8,
        ).unwrap();
        let assets = provider.build_terrain(2, 1, [None, None], &palette).unwrap();
        assert_eq!(assets.dimensions(), (2, 1));
        assert_eq!(assets.tile_size(), (1, 1));
        assert!(assets.tiles().is_empty());
        assert_eq!(assets.cells(), &[None, None]);
        assert_eq!(assets.tile_for_cell(2, 0), None);
    }

}
