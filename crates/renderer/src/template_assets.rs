//! Theater-aware TD/RA template ICON-set lookup from user-supplied MIX archives.
//!
//! The catalog supplies each template's upstream basename. Theater suffixes
//! follow the TD/RA theater naming convention; no game art is bundled here.

use std::collections::BTreeSet;
use crate::{TerrainAtlas, TerrainError, TerrainMap, TextureId};
use rts_core::{GameVariant, template_catalog::{TemplateTheater, template_asset_for_theater, template_asset_metadata}};
use rts_formats::{iconset::{IconSet, IconSetError}, mix::MixArchive, palette_rgba::Palette256, ra_map_pack::RaTemplateMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateAssetError {
    UnknownTemplateType(u16),
    UnavailableInTheater { template: u16, variant: GameVariant, theater: TemplateTheater },
    MissingAsset { template: u16, filename: String, filename_crc: i32 },
    DuplicateTemplateType(u16),
    WrongVariant { expected: GameVariant, actual: GameVariant },
    IconSet(IconSetError),
}

impl std::fmt::Display for TemplateAssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownTemplateType(id) => write!(f, "template type {id} is absent from the source-derived catalog"),
            Self::UnavailableInTheater { template, variant, theater } => write!(f, "template type {template} is unavailable in {variant:?} theater {theater:?}"),
            Self::MissingAsset { template, filename, filename_crc } => write!(f, "template type {template} asset {filename} (MIX CRC {filename_crc:#010x}) was not found"),
            Self::DuplicateTemplateType(id) => write!(f, "template type {id} was requested more than once"),
            Self::WrongVariant { expected, actual } => write!(f, "MapPack variant {actual:?} cannot be composed by this provider; expected {expected:?}"),
            Self::IconSet(error) => write!(f, "template ICON set could not be parsed: {error}"),
        }
    }
}
impl std::error::Error for TemplateAssetError {}
impl From<IconSetError> for TemplateAssetError { fn from(value: IconSetError) -> Self { Self::IconSet(value) } }

#[derive(Debug)]
pub enum TemplateTerrainBuildError {
    Asset(TemplateAssetError),
    Terrain(TerrainError),
}
impl std::fmt::Display for TemplateTerrainBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asset(error) => write!(f, "{error}"),
            Self::Terrain(error) => write!(f, "{error}"),
        }
    }
}
impl std::error::Error for TemplateTerrainBuildError {}
impl From<TemplateAssetError> for TemplateTerrainBuildError {
    fn from(value: TemplateAssetError) -> Self { Self::Asset(value) }
}
impl From<TerrainError> for TemplateTerrainBuildError {
    fn from(value: TerrainError) -> Self { Self::Terrain(value) }
}

/// Resolves source-derived template names against one or more caller-owned MIX archives.
/// Archive precedence is the slice order, matching upstream's registered MIX search order.
pub struct TemplateAssetProvider<'a> {
    variant: GameVariant,
    theater: TemplateTheater,
    archives: &'a [&'a MixArchive<'a>],
}

impl<'a> TemplateAssetProvider<'a> {
    pub fn new(variant: GameVariant, theater: TemplateTheater, archives: &'a [&'a MixArchive<'a>]) -> Self {
        Self { variant, theater, archives }
    }

    pub fn filename_for_template(&self, template_type: u16) -> Result<String, TemplateAssetError> {
        let metadata = template_asset_metadata(self.variant, template_type)
            .ok_or(TemplateAssetError::UnknownTemplateType(template_type))?;
        if template_asset_for_theater(self.variant, template_type, self.theater).is_none() {
            return Err(TemplateAssetError::UnavailableInTheater {
                template: template_type, variant: self.variant, theater: self.theater,
            });
        }
        Ok(format!("{}{}", metadata.asset_stem, theater_suffix(self.variant, self.theater)
            .ok_or(TemplateAssetError::UnavailableInTheater {
                template: template_type, variant: self.variant, theater: self.theater,
            })?))
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

    /// Decode and compose an authentic Red Alert MapPack using the matching
    /// theater ICON sets from the caller-supplied MIX archives.
    pub fn build_ra_terrain_atlas(
        &self,
        texture_id: TextureId,
        map: &RaTemplateMap,
        palette: &Palette256,
    ) -> Result<(TerrainMap, TerrainAtlas), TemplateTerrainBuildError> {
        if self.variant != GameVariant::RedAlert {
            return Err(TemplateAssetError::WrongVariant {
                expected: GameVariant::RedAlert, actual: self.variant,
            }.into());
        }
        let types: BTreeSet<u16> = map.cells().iter().map(|cell| cell.template_type).collect();
        let loaded = self.load_icon_sets(types)?;
        let icon_sets: Vec<(u16, &IconSet)> = loaded.iter().map(|(id, set)| (*id, set)).collect();
        let templates = map.cells().iter().map(|cell| Some((cell.template_type, cell.icon)));
        Ok(TerrainAtlas::build_from_template_catalog(
            texture_id, self.variant, self.theater, map.width(), map.height(),
            templates, &icon_sets, palette,
        )?)
    }

    /// Load a deterministic set of unique catalog entries for renderer composition.
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
    fn rejects_cross_variant_theaters_before_asset_lookup() {
        let empty: [&MixArchive<'_>; 0] = [];
        let provider = TemplateAssetProvider::new(GameVariant::RedAlert, TemplateTheater::Desert, &empty);
        assert!(matches!(provider.filename_for_template(3), Err(TemplateAssetError::UnavailableInTheater { .. })));
    }
}
