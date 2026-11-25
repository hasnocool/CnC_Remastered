## Tranche 6S-C-E — TemplateTypes IDs128–159 (completed)

- Refreshed target `main` at `ab1100306d26e43ae6961730cd925facf94000af`; target commit tree `bbc6e3547254fd44958bdf5f6e47402bba533144`; recursive tree: 490 entries, complete (`truncated=false`). Pinned upstream `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`; recursive upstream tree was fetched and verified complete (1,375 entries).
- Re-fetched inventory, roadmap, Rust cross-reference, MAP_EDITOR README and TypeCatalog-Contracts, plus current Rust editor model/runtime TemplateClass. Exact TD/RA TemplateTypes, shared TemplateType model, Map and both GamePlugin files were fetched at the pinned upstream commit.
- TD/RA TemplateTypes blob SHAs matched the pinned recursive tree: TD `97fdfd2109d241da8b51139d974028ed1198b140`; RA `027bb5658e2297cf2ade84f9a01567acc8238db8`.
- Parsed exactly 32 contiguous declarations IDs128–159 in each variant.
- TD: Road36–Road43 (2×2, all three theatres); River1–River13 (Winter/Temperate); River14–River24 (Desert-only; River20 6×8, River21 5×8).
- RA: Falls2a, Ford1/2, Bridge1/1d/2/2d and Slope01–Slope25; all Temperate/Snow. Slope21 is 1×2, Slope22 2×1; Slope02/06/16/20 are 2×3 and Slope09/13/23 are 3×2.
- All 64 entries use the five-argument TemplateType constructor; the pinned overload delegates with TemplateTypeFlag.None. No 0×0 declarations occur in this slice; previously documented RA IDs100–102 and 111 remain unchanged.
- Reconfirmed theatre path: both plugin theatre-change handlers call Map.InitTheater(GameType); Map.InitTheater iterates every TemplateType and calls Init(Theater). TemplateType.Init sizes the thumbnail from max(IconWidth, IconHeight), allocates/clears the mask and sets mask cells only when theatre-specific tile lookup succeeds.
- Added three guides/comparison and updated inventory, Rust cross-reference, MAP_EDITOR README, TypeCatalog-Contracts and this roadmap. Both registry paths remain Partially reviewed; cumulative declaration coverage is IDs0–159 (160/216 TD, 160/401 RA).
- Inventory counts recomputed from all 1,331 path rows: 17 legacy Reviewed, 121 Partially reviewed, 20 Header reviewed, 67 Body reviewed, 12 Resource reviewed, 1,094 Inventory only. Rust-mapped/Tested/Complete remain 0.
- Rust editor model still stores `CellLayers.template: Option<u16>`; no typed catalog mapping. The Rust Red Alert TemplateClass module is runtime object behavior, not the editor registry. No Rust code or tests changed/executed.

## Next batch — Tranche 6S-C-F (TemplateTypes IDs160–191)

1. Refresh target main/ref/tree, inventory, roadmap, cross-reference, README and exact pinned TD/RA sources before editing.
2. Parse exactly IDs160–191 from both registries, preserving field/token, dimensions, theatre arrays, flags, declaration order and source-line anchors.
3. Add per-variant guides and comparison; update inventory, cross-reference, TypeCatalog-Contracts, MAP_EDITOR README and roadmap.
4. TD has 56 declarations remaining after ID159; RA has 241. Keep both inventory rows Partially reviewed until entire source bodies and dependencies are audited.
5. Complete GamePlugin INI/MapPack serialization body review and theatre/catalog consumer tracing after declaration coverage.
6. Continue map models, designer/resource XML, shared libraries, runtime systems and second-pass audits. Catalog documentation does not establish Rust mapping or tests.

## Persistent verification rules

- The 1,331 individual inventory path rows remain the source of truth; recompute status counts from those rows every run.
- Body reviewed, Rust-mapped, Tested and Complete are separate evidence states. Catalog tables alone do not advance a registry to Body reviewed.
- Commit through a base-tree update and expected-SHA ref lease with force=false; fetch the new ref, commit/tree, full recursive tree and each changed blob after update.
