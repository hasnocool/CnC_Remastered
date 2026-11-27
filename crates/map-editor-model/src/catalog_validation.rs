//! Structural validation for source-derived BuildingType and SmudgeType catalogs.
//! This deliberately validates catalog integrity without inventing map-theater state.

use std::collections::BTreeSet;

use crate::{BuildingTypeDefinition, SmudgeTypeDefinition, SmudgeTypeFlag};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogValidationError {
    DuplicateBuildingId(i8),
    DuplicateSmudgeId(i8),
    EmptyBuildingName(i8),
    EmptyBuildingOwner(i8),
    ZeroBuildingDimensions(i8),
    EmptyBuildingMask(i8),
    BuildingMaskOutOfBounds(i8),
    DuplicateBuildingMaskCell(i8),
    BibMaskMismatch(i8),
    NonBibMaskMismatch(i8),
    EmptySmudgeName(i8),
    ZeroSmudgeDimensions(i8),
    DuplicateSmudgeMask(i8),
    MissingBibSmudge(SmudgeTypeFlag),
}

pub fn validate_building_catalog(
    buildings: &[BuildingTypeDefinition],
) -> Result<(), CatalogValidationError> {
    let mut ids = BTreeSet::new();
    for building in buildings {
        if !ids.insert(building.id) {
            return Err(CatalogValidationError::DuplicateBuildingId(building.id));
        }
        if building.name.trim().is_empty() {
            return Err(CatalogValidationError::EmptyBuildingName(building.id));
        }
        if building.owner_house.trim().is_empty() {
            return Err(CatalogValidationError::EmptyBuildingOwner(building.id));
        }
        if building.width == 0 || building.height == 0 {
            return Err(CatalogValidationError::ZeroBuildingDimensions(building.id));
        }
        if building.base_occupy_mask.is_empty() || building.occupy_mask.is_empty() {
            return Err(CatalogValidationError::EmptyBuildingMask(building.id));
        }

        let mut base_cells = BTreeSet::new();
        for &(x, y) in &building.base_occupy_mask {
            if x >= building.width || y >= building.height {
                return Err(CatalogValidationError::BuildingMaskOutOfBounds(building.id));
            }
            if !base_cells.insert((x, y)) {
                return Err(CatalogValidationError::DuplicateBuildingMaskCell(building.id));
            }
        }

        let mut full_cells = BTreeSet::new();
        for &(x, y) in &building.occupy_mask {
            let max_y = building.height + if building.has_bib { 1 } else { 0 };
            if x >= building.width || y >= max_y {
                return Err(CatalogValidationError::BuildingMaskOutOfBounds(building.id));
            }
            if !full_cells.insert((x, y)) {
                return Err(CatalogValidationError::DuplicateBuildingMaskCell(building.id));
            }
        }

        if building.has_bib {
            let mut expected = building.base_occupy_mask.iter().copied()
                .filter(|(_, y)| *y < building.height - 1)
                .collect::<BTreeSet<_>>();
            for y in (building.height - 1)..=building.height {
                for x in 0..building.width {
                    expected.insert((x, y));
                }
            }
            if full_cells != expected {
                return Err(CatalogValidationError::BibMaskMismatch(building.id));
            }
        } else if full_cells != base_cells {
            return Err(CatalogValidationError::NonBibMaskMismatch(building.id));
        }
    }
    Ok(())
}

pub fn validate_smudge_catalog(
    smudges: &[SmudgeTypeDefinition],
) -> Result<(), CatalogValidationError> {
    let mut ids = BTreeSet::new();
    let mut bib_kinds = BTreeSet::new();
    for smudge in smudges {
        if !ids.insert(smudge.id) {
            return Err(CatalogValidationError::DuplicateSmudgeId(smudge.id));
        }
        if smudge.name.trim().is_empty() {
            return Err(CatalogValidationError::EmptySmudgeName(smudge.id));
        }
        if smudge.width == 0 || smudge.height == 0 {
            return Err(CatalogValidationError::ZeroSmudgeDimensions(smudge.id));
        }
        if smudge.is_bib() {
            let kind = match smudge.flags {
                flags if flags == SmudgeTypeFlag::Bib1.bits() => Some(SmudgeTypeFlag::Bib1),
                flags if flags == SmudgeTypeFlag::Bib2.bits() => Some(SmudgeTypeFlag::Bib2),
                flags if flags == SmudgeTypeFlag::Bib3.bits() => Some(SmudgeTypeFlag::Bib3),
                _ => None,
            };
            if let Some(kind) = kind {
                if !bib_kinds.insert(kind as u8) {
                    return Err(CatalogValidationError::DuplicateSmudgeMask(smudge.id));
                }
            }
        }
    }
    for required in [SmudgeTypeFlag::Bib1, SmudgeTypeFlag::Bib2, SmudgeTypeFlag::Bib3] {
        if !bib_kinds.contains(&(required as u8)) {
            return Err(CatalogValidationError::MissingBibSmudge(required));
        }
    }
    Ok(())
}

pub fn validate_variant_catalogs(
    buildings: &[BuildingTypeDefinition],
    smudges: &[SmudgeTypeDefinition],
) -> Result<(), CatalogValidationError> {
    validate_building_catalog(buildings)?;
    validate_smudge_catalog(smudges)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_catalogs::{
        red_alert_building_types, red_alert_smudge_types,
        tiberian_dawn_building_types, tiberian_dawn_smudge_types,
    };

    #[test]
    fn both_pinned_variant_catalogs_pass_structural_validation() {
        validate_variant_catalogs(&tiberian_dawn_building_types(), &tiberian_dawn_smudge_types()).unwrap();
        validate_variant_catalogs(&red_alert_building_types(), &red_alert_smudge_types()).unwrap();
    }

    #[test]
    fn duplicate_building_ids_are_rejected() {
        let mut catalog = tiberian_dawn_building_types();
        catalog.push(catalog[0].clone());
        assert_eq!(
            validate_building_catalog(&catalog),
            Err(CatalogValidationError::DuplicateBuildingId(0)),
        );
    }

    #[test]
    fn bib_mask_must_replace_the_last_base_row_and_add_one_row() {
        let mut catalog = tiberian_dawn_building_types();
        let building = catalog.iter_mut().find(|building| building.has_bib).unwrap();
        let building_id = building.id;
        building.occupy_mask.pop();
        assert_eq!(
            validate_building_catalog(&catalog),
            Err(CatalogValidationError::BibMaskMismatch(building_id)),
        );
    }

    #[test]
    fn missing_bib_variant_is_rejected() {
        let mut smudges = red_alert_smudge_types();
        smudges.retain(|smudge| smudge.flags != SmudgeTypeFlag::Bib2.bits());
        assert_eq!(
            validate_smudge_catalog(&smudges),
            Err(CatalogValidationError::MissingBibSmudge(SmudgeTypeFlag::Bib2)),
        );
    }
}
