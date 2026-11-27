# Typed cell instances and catalog-driven Building lifecycle

Pinned upstream: `electronicarts/CnC_Remastered_Collection` at `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.

Source contracts used:
- `MAP_EDITOR/Overlay.cs.md`: placed Overlay owns Type, Icon and Tint; Map stores Overlay instances in the Overlay CellGrid.
- `MAP_EDITOR/Smudge.cs.md`: placed Smudge owns Type, Icon, Data and Tint.
- `MAP_EDITOR/Building.cs.md`: placed Building fields, defaults, tint alpha behavior, and BibCells ownership.
- `MAP_EDITOR/BuildingType.cs.md`: base/full occupancy masks, bib expansion, width/height, HasBib, fake-name transformation and metadata.
- `MAP_EDITOR/SmudgeType.cs.md`: Bib/Bib1/Bib2/Bib3 flags and logical Size.
- `MAP_EDITOR/Map.cs.md`: Buildings uses full mask; Technos uses base mask; bib placement/removal and row-major icon assignment.

## Typed cell ownership

- `CellLayers` now owns `Option<PlacedOverlay>` and `Option<PlacedSmudge>` directly.
- Overlay and Smudge retain independent type ID, icon, tint; Smudge additionally retains Data.
- `MapDocument` provides checked typed accessors and replacement methods.
- `MapEditorState` no longer maintains a second Overlay/Smudge instance map. Its derived icon calculations and wall lifecycle read/write the document's cell instances.
- The old numeric `CellLayers.overlay` and `CellLayers.smudge` fields remain as legacy format adapter slots; they are deliberately not treated as type IDs or silently synchronized because the serialization contract uses variant-specific catalog slots.

## Building catalog and lifecycle

- Added `BuildingTypeDefinition` with ID/name/display/tile name, dimensions, HasBib, owner house, optional theater metadata, fake/turret/factory metadata, and distinct base/full occupancy masks.
- `from_base_mask` reproduces the audited bib expansion: for HasBib, allocate a mask one row taller than the base mask, copy all base rows except the last, then fill the final two rows (replacing the base's last row and adding one new row). Base mask remains unchanged.
- Added `PlacedBuilding` with source defaults: strength 256, direction supplied by placement, Trigger=None, BasePriority=-1, IsPrebuilt=true, Sellable=false, Rebuild=false, white Tint, and owned BibCells.
- Non-prebuilt effective tint applies 75% alpha while retaining the raw Tint.
- `clone_without_bib_cells` explicitly mirrors `Building.Clone` property-copy behavior; ordinary aggregate snapshots retain BibCells so the aggregate's ownership invariant remains intact.
- `register_building_type` and `add_typed_building` form the new catalog-driven path. Callers supply a type ID and placement context; masks, HasBib, dimensions, bib type selection, defaults, tint and bib ownership are derived from registered definitions. The typed PlacedBuilding owns its BibCells directly; the separate ownership map is retained only for the older explicit-property compatibility API.
- Building width 2/3/4 selects Bib3/Bib2/Bib1. Bib smudges are placed row-major at `anchor.y + building.height + row - 1`, with Data=0 and the building tint. Out-of-bounds cells are skipped.
- Buildings occupancy uses the full mask; Technos occupancy uses the base mask. Removal removes only currently Bib-flagged smudges from owned cells.

## Compatibility and boundaries

- Numeric overlay/smudge slots remain for existing scenario format adapters. Their meaning is format/catalog-specific and is not interchangeable with a typed definition ID.
- The older explicit-property `add_building_with_bibs` API remains for existing callers/tests; new code should use `add_typed_building`.
- `add_typed_building` currently accepts caller-supplied `OverlapRect`, because the editor pixel/cell metrics adapter is not yet connected to this model. BuildingType-derived occupancy and bib properties no longer need to be passed individually.
- Direct mutation through `MapEditorState::document_mut` can bypass aggregate spatial synchronization. Prefer aggregate placement/removal methods for typed objects; a later API-hardening tranche should narrow mutable document access.
- BuildingType thumbnail rendering and complete TD/RA static catalogs are imported from the pinned variant registries. The register_variant_building_and_smudge_catalogs method loads the matching catalog for the map variant.

## Tests and verification

Added regression tests for typed cell instance retention, source bib mask expansion, non-prebuilt tint alpha, automatic catalog-driven bib placement/removal, default Building values, distinct Buildings/Technos masks, and unknown type rejection. Tests are authored only. Rust formatting, compilation, tests and Clippy have not been run because the current environment has no Rust toolchain. Retail TD/RA validation remains a separate milestone requiring user-supplied retail assets.

## Variant catalog import

- Imported all 60 Tiberian Dawn BuildingType definitions and all 63 Red Alert BuildingType definitions from their pinned static registries, preserving sparse Red Alert IDs, row-major masks, owners, theater restrictions, fake/turret flags, factory overlays, and text IDs.
- Imported all 15 SmudgeType definitions for each variant (12 crater/scorch entries and Bib1/Bib2/Bib3 with source sizes and exact flag values).
- MapEditorState::register_variant_building_and_smudge_catalogs selects the matching variant catalog; it does not merge TD and RA IDs.
- Source BuildingType.OverlapBounds is the rectangle at the placement anchor with width/height equal to the final OccupyMask array dimensions. Bib buildings therefore add exactly one to overlap height.
- Upstream references: CnCTDRAMapEditor/TiberianDawn/BuildingTypes.cs, RedAlert/BuildingTypes.cs, both SmudgeTypes.cs, and Model/BuildingType.cs at f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
