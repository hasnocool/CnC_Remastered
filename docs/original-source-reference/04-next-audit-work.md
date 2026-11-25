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

## Tranche 6S-C-F — TemplateTypes IDs160–191 (completed)

- Run-start main: 3fb4ea3965111066fc419cf93ef0ef061aaf856c; target tree 0efbd5dfc23b7f73fd3ca037672ff70473313494, recursive count 493, complete. Pinned upstream tree: 1,375 entries, complete.
- Re-fetched inventory, roadmap, cross-reference, MAP_EDITOR README and TypeCatalog-Contracts. Exact TD/RA TemplateTypes and shared TemplateType/Map implementations were inspected at the pinned commit.
- Source blobs matched the pinned tree: TD 97fdfd2109d241da8b51139d974028ed1198b140; RA 027bb5658e2297cf2ade84f9a01567acc8238db8. Shared TemplateType blob f62e53665e9a45ddebba9e1f7d01ca61045fae64; Map blob 1b478aebee005a8db2b4b7bfb4e044356829bd5d.
- Parsed exactly 32 contiguous IDs160–191 per variant, retaining field, ID, tile key, width, height, theatre array, implicit flag and line.
- TD: River25; fords/falls; bridge variants; Desert Shore24–31 and Shore36–39; Winter Patch16–20; Temperate/Winter Shore34/35. Shore32/33 are earlier IDs51/52. RA: Slope26–38 then Road01–19, all Temperate/Snow. Slope27 is 3×2; Road05 is 3×4; other road exceptions are recorded in the guide. All 64 flags are None.
- Re-read shared TemplateType members, constructors, equality/hash/string and Init, plus Map.InitTheater. Init creates a max-dimension square thumbnail and width×height mask, loops y-major/x-minor, and marks successful theatre-specific tile lookups. There is no positive-dimension guard before Bitmap allocation; this tranche has no zero-sized records. RA IDs100–102 and 111 remain an untested source hazard.
- Added three guides/comparison; updated inventory, cross-reference, README, TypeCatalog-Contracts and roadmap. Both TemplateTypes rows remain Partially reviewed; coverage is IDs0–191 (192/216 TD, 192/401 RA).
- Inventory counts recomputed from 1,331 path rows: 17 Reviewed, 121 Partially reviewed, 1094 Inventory only, 20 Header reviewed, 67 Body reviewed, 12 Resource reviewed. Rust-mapped/Tested/Complete remain 0.
- Rust CellLayers.template remains Option<u16>; runtime Red Alert TemplateClass is not the editor catalog. No Rust code/tests changed or executed.

## Next batch — Tranche 6S-C-G (TD IDs192–215; RA IDs192–223)

1. Refresh main/ref, complete recursive target/upstream trees, inventory, roadmap, cross-reference, README and exact pinned sources.
2. Parse the final 24 TD declarations (IDs192–215) and next 32 RA declarations (IDs192–223), retaining all fields and source-line anchors. Do not invent TD IDs216–223; TD ends at ID215.
3. Add per-variant guides/comparison and update inventory, cross-reference, TypeCatalog-Contracts, README and roadmap.
4. TD declaration coverage will reach 216/216 but remains Partially reviewed until full body, consumers and serializers are audited. RA will reach 224/401 and remain Partially reviewed.
5. Continue RA IDs224–255, then complete GamePlugin INI/MapPack body reviews and theatre/catalog consumer tracing.
6. Continue map models, designer/resource XML, shared libraries, runtime systems and second-pass audits. Catalog enumeration alone is not Rust mapping or testing.

## Persistent verification rules

- The 1,331 individual inventory path rows remain the source of truth; recompute status counts from those rows every run.
- Body reviewed, Rust-mapped, Tested and Complete are separate evidence states. Catalog tables alone do not advance a registry to Body reviewed.
- Commit through a base-tree update and expected-SHA ref lease with force=false; fetch the new ref, commit/tree, full recursive tree and each changed blob after update.
