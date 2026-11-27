# Derived overlay icons and Building bib ownership

Upstream baseline: `electronicarts/CnC_Remastered_Collection` at `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.

Source contracts inspected:
- `docs/original-source-reference/MAP_EDITOR/Map.cs.md` — `UpdateResourceOverlays`, `UpdateWallOverlays`, `AddBibs`, `RemoveBibs`.
- `Building.cs.md` — `Building.BibCells` ownership.
- `SmudgeType.cs.md` and `Smudge.cs.md` — bib flags and placed-smudge fields.

## Implemented

- `derived_icons.rs` contains the exact source lookup tables:
  - Tiberium/gold: `[0,1,3,4,6,7,8,10,11]`.
  - Gems: `[0,0,0,1,1,1,2,2,2]`.
- Resource counts inspect all eight adjacent cells and count every resource overlay, including both gem and Tiberium/gold types. The center cell is not counted.
- Wall icons inspect only N/E/S/W and set N=1, E=2, S=4, W=8 only when the adjacent overlay has the exact same type ID.
- `MapEditorState::set_overlay` applies derived icons immediately when invalidation is not batched. `MapEditorState::end_update` applies coalesced icon work when the outermost batch completes.
- Overlay records use `Option<OccupierId>` for wall ownership. Non-wall overlays no longer use a sentinel ID that could collide with a real wall ID.
- `add_building_with_bibs` selects Bib3/Bib2/Bib1 for widths 2/3/4, loops the selected smudge definition's rows and columns, uses sequential row-major icons, places at `anchor.y + building_height + row - 1`, copies the building tint, and records every in-bounds cell in the building's bib ownership set.
- Bib removal checks the current smudge definition's Bib bit before deleting a cell, then clears the ownership set. A replacement non-bib smudge is preserved.
- Bib placements are staged before spatial registration and committed only after building registration succeeds.

## API notes

Register catalog definitions before use with `register_overlay_type` and `register_smudge_type`. Use `begin_update` / `end_update` on `MapEditorState` for batched overlay edits so derived icons are applied at the outermost completion. `add_building_with_bibs` takes the source BuildingType-derived properties explicitly; it does not invent a BuildingType catalog or infer HasBib/geometry from a numeric ID.

## Remaining work

- The typed overlay/smudge stores are not yet synchronized to `MapDocument::CellLayers.overlay` / `smudge`; serialization integration remains separate.
- The aggregate still does not own a full source-derived BuildingType/SmudgeType catalog or a general typed Building record. Bib creation currently accepts audited BuildingType properties as arguments.
- The legacy Buildings event bridge and all Building lifecycle call sites still need consolidation through this API.
- Source parity for the rest of Map's update/clone/event behavior remains incomplete.
- These are synthetic model tests only; no retail TD/RA validation is claimed.

## Verification

Regression tests were added for both resource lookup tables, eight-neighbor resource updates, cardinal same-type wall connectivity, batched icon application, bib type selection, row-major bib icons, ownership cleanup, and preservation of replacement non-bib smudges. The environment has no Rust toolchain, so formatting, compilation, tests, and Clippy have not been run.
