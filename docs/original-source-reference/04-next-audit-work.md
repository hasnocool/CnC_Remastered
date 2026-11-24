## Tranche 6S-C-D — TemplateTypes IDs96–127 (completed)

- Refreshed target main at `480eb457421e7ac00c6092b30e9d8565e3a98b08`; target recursive tree 487 entries, complete/untruncated. Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`; upstream recursive tree 1,375 entries, complete/untruncated.
- Fetched current inventory, this roadmap, Rust cross-reference, MAP_EDITOR README and TypeCatalog-Contracts before editing. Exact TD/RA TemplateTypes sources were fetched at the pinned upstream commit and their blob SHAs matched the pinned recursive tree.
- Parsed complete catalogs: TD 216 declarations IDs0–215; RA 401 declarations IDs0–400. Verified exactly 32 contiguous declarations IDs96–127 per variant.
- TD IDs96–127: Road4–Road35, tokens d04–d35, all Winter/Desert/Temperate; exact dimensions, theatres, flags and source lines recorded.
- RA IDs96–127: ShoreCliff38, Boulder1–6, Patch01–04/07/08/13/14/15, River01–13, Falls1/Falls1a/Falls2; all Temperate/Snow. Boulder4–6 (IDs100–102) and Patch15 (ID111) have literal 0×0 dimensions.
- Followed initialization call path: Red Alert GamePlugin `case "Theater"` calls Map.InitTheater; Map.InitTheater iterates all TemplateTypes and calls TemplateType.Init; TemplateType.Init constructs Bitmap using max(width,height) before loops and has no positive-dimension guard. The zero-sized Bitmap request is source-confirmed; exact runtime exception was not executed.
- Added two per-variant guides and one comparison; updated inventory, Rust cross-reference, MAP_EDITOR README, TypeCatalog-Contracts and this roadmap. Both TemplateTypes inventory rows remain Partially reviewed. Cumulative declaration coverage IDs0–127; full dependency/consumer/serializer review and remaining IDs open.
- Recomputed counts from all 1331 inventory path rows: 17 legacy Reviewed, 121 Partially reviewed, 20 Header reviewed, 67 Body reviewed, 12 Resource reviewed, 1094 Inventory only. Rust-mapped/Tested/Complete remain 0. Existing reference documents 257; three new guides bring the total to 260.
- Rust comparison: `CellLayers.template` is `Option<u16>`; `crates/rts-core/src/red_alert_template.rs` contains a partial runtime Red Alert TemplateClass implementation, not the editor TemplateType registry. No Rust files changed and no tests executed.

## Next batch — Tranche 6S-C-E (TemplateTypes IDs128–159)

1. Refresh main/ref/tree and exact pinned TD/RA TemplateTypes sources again.
2. Parse and validate IDs128–159 for both variants; preserve field/token, dimensions, theatre arrays, flags, declaration order and source-line anchors.
3. Add two per-variant guides and a comparison; update inventory, cross-reference, TypeCatalog-Contracts, MAP_EDITOR README and this roadmap.
4. Keep both registry paths Partially reviewed until all declarations, static initialization, consumers, plugin serialization and resource dependencies have been audited.
5. Prioritize a C# runtime reproduction for RA 0×0 TemplateType initialization before treating it as a confirmed crash. Do not silently normalize source dimensions.
6. Continue later 32-entry slices; then complete GamePlugin bodies/INI serialization, map models, resources/designers, shared libraries, remaining runtime systems and second-pass reviews.

## Persistent verification rules

- The 1,331 individual inventory path rows remain the source of truth; recompute status counts from those rows every run.
- Body reviewed, Rust-mapped, Tested and Complete are separate evidence states. Catalog tables alone do not advance a registry to Body reviewed.
- Commit through a base-tree update and expected-SHA ref lease with force=false; fetch the new ref, commit/tree, full recursive tree and each changed blob after update.
