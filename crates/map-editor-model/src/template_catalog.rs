//! Game- and theater-aware template palette access for the map editor.
//!
//! This module deliberately wraps the source-derived catalog in rts-core;
//! it does not duplicate template IDs, names, theater masks, or asset stems.
//! Asset presence is a separate concern: a catalog entry identifies the
//! expected asset family but does not claim that a retail archive is mounted.

use rts_core::{
    template_asset_metadata, template_asset_metadata_iter, GameVariant,
    TemplateAssetMetadata, TemplateTheater,
};

/// A zero-allocation view of the templates available to a map editor palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemplateCatalog {
    variant: GameVariant,
    theater: TemplateTheater,
}

impl TemplateCatalog {
    pub const fn new(variant: GameVariant, theater: TemplateTheater) -> Self {
        Self { variant, theater }
    }

    pub const fn variant(self) -> GameVariant {
        self.variant
    }

    pub const fn theater(self) -> TemplateTheater {
        self.theater
    }

    /// Looks up a template only when it is valid for this game and theater.
    pub fn get(self, type_id: u16) -> Option<&'static TemplateAssetMetadata> {
        template_asset_metadata(self.variant, type_id)
            .filter(|entry| entry.supports(self.variant, self.theater))
    }

    /// Iterates eligible templates in stable, ascending source type-ID order.
    pub fn iter(
        self,
    ) -> impl Iterator<Item = &'static TemplateAssetMetadata> {
        template_asset_metadata_iter(self.variant)
            .filter(move |entry| entry.supports(self.variant, self.theater))
    }

    pub fn contains(self, type_id: u16) -> bool {
        self.get(type_id).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_tiberian_dawn_templates_by_theater() {
        let jungle = TemplateCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Jungle);
        let desert = TemplateCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Desert);

        assert_eq!(jungle.get(0).unwrap().symbolic_name, "TEMPLATE_CLEAR1");
        assert!(jungle.contains(0));
        assert!(!jungle.contains(2));
        assert!(desert.contains(53));
        assert!(!jungle.contains(53));
    }

    #[test]
    fn filters_red_alert_templates_by_theater() {
        let temperate = TemplateCatalog::new(GameVariant::RedAlert, TemplateTheater::Temperate);
        let snow = TemplateCatalog::new(GameVariant::RedAlert, TemplateTheater::Snow);
        let interior = TemplateCatalog::new(GameVariant::RedAlert, TemplateTheater::Interior);

        assert!(temperate.contains(3));
        assert!(snow.contains(3));
        assert!(!interior.contains(3));
        assert!(interior.contains(253));
        assert!(!temperate.contains(253));
    }

    #[test]
    fn iterator_is_sorted_and_matches_lookup() {
        for catalog in [
            TemplateCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Temperate),
            TemplateCatalog::new(GameVariant::RedAlert, TemplateTheater::Snow),
        ] {
            let entries: Vec<_> = catalog.iter().collect();
            assert!(entries.windows(2).all(|pair| pair[0].type_id < pair[1].type_id));
            for entry in entries {
                assert_eq!(catalog.get(entry.type_id), Some(entry));
            }
        }
    }

    #[test]
    fn unknown_ids_and_cross_game_theaters_are_rejected() {
        let td_interior = TemplateCatalog::new(GameVariant::TiberianDawn, TemplateTheater::Interior);
        let ra_jungle = TemplateCatalog::new(GameVariant::RedAlert, TemplateTheater::Jungle);

        assert!(td_interior.iter().next().is_none());
        assert!(ra_jungle.iter().next().is_none());
        assert!(td_interior.get(u16::MAX).is_none());
    }

    #[test]
    fn asset_stem_is_metadata_not_a_claim_of_asset_availability() {
        let catalog = TemplateCatalog::new(GameVariant::RedAlert, TemplateTheater::Temperate);
        let shore = catalog.get(3).unwrap();

        assert_eq!(shore.asset_stem, "SH01");
        assert_eq!(shore.type_id, 3);
    }
}
