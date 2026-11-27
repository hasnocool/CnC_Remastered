//! wgpu-renderer adapter for the backend-independent theater asset provider.

use std::collections::BTreeSet;
use rts_assets::{TemplateAssetError, TemplateAssetProvider as SharedTemplateAssetProvider};
use rts_core::GameVariant;
use rts_formats::{
    iconset::IconSet,
    mix::MixArchive,
    palette_rgba::Palette256,
    ra_map_pack::RaTemplateMap,
    td_map::TdTemplateMap,
};
use crate::{TerrainAtlas, TerrainError, TerrainMap, TextureId};

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

/// Compatibility facade preserving the renderer's previous public API.
/// All MIX lookup and ICONSET decoding now live in rts-assets.
pub struct TemplateAssetProvider<'a> {
    inner: SharedTemplateAssetProvider<'a>,
}
impl<'a> TemplateAssetProvider<'a> {
    pub fn new(variant: GameVariant, theater: rts_core::template_catalog::TemplateTheater,
        archives: &'a [&'a MixArchive<'a>]) -> Self {
        Self { inner: SharedTemplateAssetProvider::new(variant, theater, archives) }
    }

    pub fn filename_for_template(&self, template_type: u16) -> Result<String, TemplateAssetError> {
        self.inner.filename_for_template(template_type)
    }

    pub fn load_icon_set(&self, template_type: u16) -> Result<IconSet, TemplateAssetError> {
        self.inner.load_icon_set(template_type)
    }

    pub fn load_icon_sets(&self, template_types: impl IntoIterator<Item = u16>)
        -> Result<Vec<(u16, IconSet)>, TemplateAssetError> {
        self.inner.load_icon_sets(template_types)
    }

    pub fn build_ra_terrain_atlas(
        &self,
        texture_id: TextureId,
        map: &RaTemplateMap,
        palette: &Palette256,
    ) -> Result<(TerrainMap, TerrainAtlas), TemplateTerrainBuildError> {
        if self.inner.variant() != GameVariant::RedAlert {
            return Err(TemplateAssetError::WrongVariant {
                expected: GameVariant::RedAlert, actual: self.inner.variant(),
            }.into());
        }
        let types = map.cells().iter().map(|cell| cell.template_type).collect::<BTreeSet<_>>();
        let loaded = self.inner.load_icon_sets(types)?;
        let icon_sets: Vec<(u16, &IconSet)> = loaded.iter().map(|(id, set)| (*id, set)).collect();
        let templates = map.cells().iter().map(|cell| Some((cell.template_type, cell.icon)));
        Ok(TerrainAtlas::build_from_template_catalog(
            texture_id, self.inner.variant(), self.inner.theater(), map.width(), map.height(),
            templates, &icon_sets, palette,
        )?)
    }

    pub fn build_td_terrain_atlas(
        &self,
        texture_id: TextureId,
        map: &TdTemplateMap,
        palette: &Palette256,
    ) -> Result<(TerrainMap, TerrainAtlas), TemplateTerrainBuildError> {
        if self.inner.variant() != GameVariant::TiberianDawn {
            return Err(TemplateAssetError::WrongVariant {
                expected: GameVariant::TiberianDawn, actual: self.inner.variant(),
            }.into());
        }
        let types = map.cells().iter().map(|cell| u16::from(cell.template_type)).collect::<BTreeSet<_>>();
        let loaded = self.inner.load_icon_sets(types)?;
        let icon_sets: Vec<(u16, &IconSet)> = loaded.iter().map(|(id, set)| (*id, set)).collect();
        let templates = map.cells().iter()
            .map(|cell| Some((u16::from(cell.template_type), cell.icon)));
        Ok(TerrainAtlas::build_from_template_catalog(
            texture_id, self.inner.variant(), self.inner.theater(), map.width(), map.height(),
            templates, &icon_sets, palette,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rts_core::template_catalog::TemplateTheater;

    #[test]
    fn resolves_source_catalog_names_with_variant_specific_suffixes() {
        let empty: [&MixArchive<'_>; 0] = [];
        let td = TemplateAssetProvider::new(GameVariant::TiberianDawn, TemplateTheater::Desert, &empty);
        let ra = TemplateAssetProvider::new(GameVariant::RedAlert, TemplateTheater::Snow, &empty);
        assert_eq!(td.filename_for_template(0).unwrap(), "CLEAR1.DES");
        assert_eq!(ra.filename_for_template(3).unwrap(), "SH01.SNO");
    }

    #[test]
    fn td_atlas_builder_rejects_red_alert_provider() {
        let empty: [&MixArchive<'_>; 0] = [];
        let provider = TemplateAssetProvider::new(GameVariant::RedAlert, TemplateTheater::Temperate, &empty);
        let map = TdTemplateMap::decode_binary(&[0, 0], 1, 1).unwrap();
        let palette = Palette256::parse(&vec![0; 768], rts_formats::palette_rgba::PaletteEncoding::Rgb8).unwrap();
        assert!(matches!(
            provider.build_td_terrain_atlas(77, &map, &palette),
            Err(TemplateTerrainBuildError::Asset(TemplateAssetError::WrongVariant { .. }))
        ));
    }

    #[test]
    fn rejects_cross_variant_theaters_before_asset_lookup() {
        let empty: [&MixArchive<'_>; 0] = [];
        let provider = TemplateAssetProvider::new(GameVariant::RedAlert, TemplateTheater::Desert, &empty);
        assert!(matches!(provider.filename_for_template(3), Err(TemplateAssetError::UnavailableInTheater { .. })));
    }
}
