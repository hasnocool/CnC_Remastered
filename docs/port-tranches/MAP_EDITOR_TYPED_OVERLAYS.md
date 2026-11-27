# Map editor tranche: typed overlays and wall cell-change synchronization

Pinned source: `electronicarts/CnC_Remastered_Collection` at `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.

Reviewed source contracts:
- `docs/original-source-reference/MAP_EDITOR/Overlay.cs.md`
- `docs/original-source-reference/MAP_EDITOR/Smudge.cs.md`
- `docs/original-source-reference/MAP_EDITOR/Map.cs.md`
- `docs/original-source-reference/MAP_EDITOR/TypeCatalog-Contracts.md`

## Implemented

### Typed placed records

Added `placed_objects.rs`:
- `PlacedOverlay` retains Type ID, icon index, and tint.
- `PlacedSmudge` retains Type ID, icon index, independent Data value, and tint.
- Both preserve the source default white tint.
- Overlay type flags retain exact source bit values: TiberiumOrGold=1, Gems=2, Wall=4, Crate=8, Flag=16.
- Placeability follows the source bit rule: no bits other than Crate may be set.
- Smudge bib flags retain None=0, Bib=1, Bib1=3, Bib2=5, Bib3=9; bib membership is a bit test.
- Overlay definition occupancy is one cell.

### Overlay mutation and wall synchronization

`MapEditorState` now owns a typed overlay-type registry and placed-overlay cells. `set_overlay` validates the coordinate and type, stages spatial changes on a clone, removes prior wall registration, and registers a new wall in both Buildings and Technos before publishing the replacement. Failed type lookup or spatial registration leaves the prior overlay/index state unchanged.

Resource and wall changes route their 3×3 invalidation requests through MapDocument's update coordinator. Nested batches coalesce the work; outside a batch the work is returned immediately.

Internal wall occupier IDs are allocated from a descending high-ID range and checked against currently occupied IDs. They are runtime/editor identities, not serialized overlay type IDs.

## Deliberate boundaries

- The existing `MapDocument::CellLayers.overlay` numeric compatibility field is not yet replaced by the typed placed-overlay store. The typed overlay store is currently authoritative only through `MapEditorState::set_overlay`.
- This does not yet implement resource neighbor icon selection or wall connectivity icon bits; it only produces invalidation work.
- It does not yet implement bib placement/removal or complete Building/BuildingType definitions.
- Smudge records and SmudgeType metadata are introduced, but Map's bib ownership relationships remain outstanding.
- The full generic C# CellGrid event surface (generic cell-change args, clear event, intersections, adjacency and copy semantics) is not yet reproduced.

## Tests authored, not executed

Added coverage for exact flag values, placeability, bib-bit semantics, independent placed Smudge Data, wall registration in both indexes, wall-to-non-wall replacement, and unknown-type failure preserving the existing overlay. Rust compilation, formatting, tests and Clippy remain unverified because the Rust toolchain is unavailable in the current environment.
