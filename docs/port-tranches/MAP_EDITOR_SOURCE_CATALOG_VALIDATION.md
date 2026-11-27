# Map editor source catalog validation

The map-editor model now exposes `catalog_validation` with structural validation for the pinned TD/RA BuildingType and SmudgeType registries.

- Rejects duplicate BuildingType and SmudgeType IDs while preserving legitimate sparse Red Alert IDs.
- Requires non-empty building/smudge names, non-zero dimensions, and non-empty occupancy masks.
- Checks base and expanded occupancy masks for bounds and duplicate cells.
- Verifies non-bib occupancy is identical to the base mask.
- Verifies bib occupancy exactly replaces the final base row and occupies the final two rows of a mask whose height is base height + 1.
- Requires one each of Bib1, Bib2, and Bib3 definitions.
- `MapEditorState::register_variant_building_and_smudge_catalogs` validates the complete selected-variant catalogs before mutating either registry.

Synthetic regression tests cover both pinned catalog sets, duplicate IDs, bib-mask corruption, and missing bib definitions.

## Deliberate boundary

The current `MapDocument` does not carry the editor's selected theater. `GameVariant` alone is insufficient to decide whether a theater-restricted civilian BuildingType can be placed. Theater enforcement is therefore not guessed or inferred here; it remains blocked on porting the source editor's theater-selection state and exact theater identifiers.

Mutable `document_mut` also remains a known escape hatch that can bypass spatial synchronization. It must be replaced with aggregate-owned mutation APIs after all current callers are migrated.

Cargo formatting, compilation, tests, and Clippy were not run because the environment has no Rust toolchain. This is not retail TD/RA validation.
