# Map editor implementation tranche: typed cell grid

Upstream baseline: `electronicarts/CnC_Remastered_Collection` at `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`. Contract summarized in `docs/original-source-reference/MAP_EDITOR/MapModel-Contracts.md`.

## Implemented

- `crates/map-editor-model/src/cell_grid.rs`: generic row-major `CellGrid<T>` storing one optional value per cell.
- Coordinate and flattened-index accessors with explicit bounds errors.
- Changed-value events carry index, coordinate, previous value and current value; assigning an equal value emits no event.
- Clear returns one aggregate `Cleared` event, not one event per populated cell.
- `copy_to` checks total length only and copies flattened order, preserving the original same-area/different-shape behavior.
- Public exports added to `map-editor-model/src/lib.rs`.

## Tests added (not executed)

- Row-major set/get and event payload.
- Equal-value assignment suppression.
- Single clear event and idempotent clear.
- Same-area, different-shape copy behavior.
- Invalid coordinates/indexes and mismatched copy lengths.

## Remaining parity work

This grid exposes explicit event values to its caller; it does not yet implement the original C# event-subscription/delegate API. The Map aggregate must subscribe or route these events to synchronize wall occupancy and derived icons. Batch updates, invalidation scopes, object identity, typed layer grids, and map clone behavior remain separate tranches.

No cargo tests, rustfmt, cargo check, or Clippy run is claimed. This tranche is not retail TD/RA validation.
