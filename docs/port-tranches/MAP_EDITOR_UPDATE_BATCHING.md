# Map editor implementation tranche: Map update batching and invalidation

Upstream baseline: `electronicarts/CnC_Remastered_Collection` at `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`. Source contract: `docs/original-source-reference/MAP_EDITOR/MapModel-Contracts.md`.

## Implemented

- `crates/map-editor-model/src/map_update.rs` adds a source-derived update coordinator.
- Nested `begin_update` / `end_update` calls defer work until the outermost batch ends.
- Unmatched `end_update` returns `BatchUnderflow` rather than corrupting the batch counter.
- Overlay invalidation records the clipped 3x3 neighborhood in either the resource or wall invalidation set.
- Duplicate dirty cells are coalesced within a batch.
- Techno add/remove callers can request one overlapper-index rebuild, coalesced during batches.
- An update guard restores the updating flag when the callback returns or unwinds.
- `MapDocument` owns the coordinator and exposes batch, invalidation, and rebuild-request methods.
- Tests cover edge clipping, nested batches, dirty-cell accumulation, immediate flushing, underflow, bounds errors, and MapDocument integration.

## Explicit boundary

This tranche ports update scheduling and invalidation state, not the complete C# Map aggregate. It does not synchronize Building/Techno occupancy, subscribe to C#-style delegates, calculate resource or wall icons, create Building bib smudges, rebuild actual overlapper entries, or implement typed catalogs/section ownership. Those behaviors require the audited source types and their own implementation tranches. The caller receives a `MapUpdateWork` plan and must apply the corresponding derived-state operations.

The C# source's unguarded underflow and non-finally updating flag are intentionally made safe in Rust; these are documented safety improvements, not claimed byte-for-byte behavior.

## Verification

Unit tests were added but not executed. The environment has no `cargo`, `rustc`, or `rustfmt`; compilation, formatting, tests, and Clippy remain unverified. No retail TD/RA validation is included.
