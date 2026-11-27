# Map editor implementation tranche: occupancy and overlap

Upstream baseline: `electronicarts/CnC_Remastered_Collection` at `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.

## Source behavior

The source-derived contract is summarized in [MapModel-Contracts.md](../original-source-reference/MAP_EDITOR/MapModel-Contracts.md):

- `OccupierSet<T>` maintains a per-cell occupancy index and an object-to-mask-cell index.
- Occupancy mask points are relative to an anchor; out-of-bounds and already occupied cells reject an add.
- `OverlapperSet<T>` is a separate index. It allows rectangles to overlap and expands a query transitively through intersecting rectangles.
- Map uses distinct occupancy registrations for Buildings and Technos; this tranche intentionally does not merge those sets.

## Rust implementation

- `crates/map-editor-model/src/occupier_set.rs`
  - Stable numeric adapter IDs.
  - Anchor-relative mask points preserving x/y orientation.
  - Atomic preflight before any mutation.
  - Duplicate ID, bounds, and collision errors.
  - Cell lookup, object mask lookup, remove, and clear.
- `crates/map-editor-model/src/overlapper_set.rs`
  - Independent rectangle index.
  - Half-open rectangle intersection with widened edge arithmetic.
  - Transitive closure query.
  - Deterministic sorted results, remove, and clear.
- `crates/map-editor-model/src/lib.rs`: public exports.

## Deliberate remaining gaps

This is a first behavioral tranche, not a complete port of the C# map model. It does not yet include the original object-reference identity model, event callbacks, the Map aggregate's registration bridge, Building base/full occupancy and bib ownership, wall/resource derived icons, batching/invalidation, or source clone semantics. Those remain explicit follow-up work.

## Tests and verification

Unit tests cover mask indexing, collision atomicity, duplicate IDs, out-of-bounds masks, removal/clear consistency, duplicate mask points, half-open rectangle intersection, transitive closure, overlap acceptance, and invalid rectangles.

The Rust toolchain is not installed in the execution environment. Tests, rustfmt, cargo check, and Clippy have **not** been run. No runtime parity or retail-asset validation is claimed.
