# Map editor implementation tranche: spatial index event bridge

Upstream baseline: `electronicarts/CnC_Remastered_Collection` at `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`. Source contract: `docs/original-source-reference/MAP_EDITOR/MapModel-Contracts.md`.

## Implemented

- `map_spatial_index.rs` owns distinct Buildings and Technos occupancy indexes plus the Overlappers rectangle index.
- Building registration writes the full mask (including any bib footprint) to Buildings and the base mask to Technos.
- Building overlap bounds are indexed independently; overlap is not treated as occupancy.
- Wall registration writes its mask to Buildings and mirrors the registration into Technos, modeling the Buildings-to-Technos event bridge.
- Building removal removes its two occupancy registrations and overlap rectangle; wall removal removes its two occupancy registrations without touching overlap rectangles.
- Multi-index building registration rolls back prior occupancy changes if overlap-bound registration fails.
- `map_editor_state.rs` composes MapDocument with MapSpatialIndex and exposes placed-building/wall mutation methods.
- Building additions/removals request overlapper update work through the document's batching coordinator.
- Public exports added to `map-editor-model/src/lib.rs`.

## Tests added (not executed)

Tests cover full-vs-base building masks, wall registration in both occupancy sets, independent overlap indexing, rollback on invalid overlap bounds, building removal, and batch-coalesced overlapper rebuild requests.

## Remaining parity work

This is the first spatial event bridge, not the complete C# object model. It does not yet model typed Building/Overlay definitions, object references/events/delegates, TerrainType or InfantryGroup overlapper registration, dynamic mask changes, map cell overlay-change synchronization, Building bib Smudge ownership, or resource/wall icon calculation. Those remain separate source-specific tasks.

The Rust aggregate is named `MapEditorState` and wraps `MapDocument`; the existing serialized document remains separate from editor/runtime spatial indexes.

## Verification

Unit tests were added but not executed. The environment has no `cargo`, `rustc`, or `rustfmt`; compilation, formatting, tests, and Clippy remain unverified. No retail TD/RA validation is included.
