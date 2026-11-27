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

## Tranche 6S-C-G — TemplateTypes IDs192–215 TD / 192–223 RA (completed)

- Run-start target main `24a37c2e21d31591e530175ccddb6a43b3f75486`; target recursive tree 496 entries, complete. Pinned upstream `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`; recursive upstream tree 1375 entries, complete.
- Re-fetched inventory, roadmap, cross-reference, MAP_EDITOR README, TypeCatalog-Contracts and both full TemplateTypes sources. Source blobs matched the pinned upstream tree: TD `97fdfd2109d241da8b51139d974028ed1198b140`, RA `027bb5658e2297cf2ade84f9a01567acc8238db8`.
- Parsed exactly 24 TD declarations IDs192–215 (lines216–239) and 32 RA declarations IDs192–223 (lines216–247); field, explicit ID, key, dimensions, theatre whitelist and source-line anchor documented.
- TD final slice Shore40–63, keys sh40–sh63, Desert-only. TD declaration enumeration complete: 216/216.
- RA slice Road20–43 (IDs192–215), then Rough01–08 (IDs216–223), keys d20–d43/rf01–rf08, all Temperate/Snow. RA coverage 224/401.
- All 56 use the five-argument constructor, verified to delegate with TemplateTypeFlag.None. No zero-sized dimensions occur in this slice.
- Re-read TemplateType constructors/properties/equality/hash/string/Init and Map.InitTheater. Init allocates a max-footprint square bitmap and width×height mask, iterates y-major/x-minor, and sets mask cells only for successful theatre tile lookups. Map initializes every template before smudge/overlay/terrain/object registries. No positive-dimension guard exists; RA IDs100–102 and 111 remain an untested source-confirmed hazard.
- Registry static constructors reflect public static readonly TemplateType fields and materialize with ToArray without explicit sorting; GetTypes returns the backing array as IEnumerable.
- Rust editor model stores CellLayers.template as Option<u16>; runtime Red Alert TemplateClass is a distinct engine-object layer. No typed editor catalogue mapping, Rust code change or test execution is claimed.
- Added three guides/comparison; updated inventory, Rust cross-reference, MAP_EDITOR README, TypeCatalog-Contracts and roadmap. Both rows remain Partially reviewed pending dependencies, consumers and GamePlugin serializer audit.
- Inventory recomputed from 1,331 path rows: 17 Reviewed, 121 Partially reviewed, 20 Header reviewed, 67 Body reviewed, 12 Resource reviewed, 1,094 Inventory only. Rust-mapped/Tested/Complete remain 0.

## Next batch — Tranche 6S-C-H (RA TemplateTypes IDs224–255)

1. Refresh main/ref/tree, both recursive trees, inventory, roadmap, cross-reference, README and exact pinned RA source.
2. Parse exactly IDs224–255 from Red Alert TemplateTypes.cs; TD has no further declarations because its registry ends at ID215.
3. Document each field, ID, tile key, dimension, theatre whitelist, flag and source-line anchor; add an RA-focused guide and comparison.
4. Keep RA TemplateTypes Partially reviewed until complete body, consumers, initialization and GamePlugin serialization are audited. TD declaration enumeration is complete, but its source row remains Partially reviewed pending dependency/consumer/serializer review.
5. Continue RA IDs256–287, then IDs288–400; afterwards audit both GamePlugin bodies and serialization call chains.
6. Continue editor map models, designer/resource XML, Steamworks/native bindings, shared libraries, engine systems and second-pass audits. Catalogue enumeration is not Rust mapping or parity testing.

## Persistent verification rules

- The 1,331 individual inventory path rows remain the source of truth; recompute status counts from those rows every run.
- Body reviewed, Rust-mapped, Tested and Complete are separate evidence states. Catalog tables alone do not advance a registry to Body reviewed.
- Commit through a base-tree update and expected-SHA ref lease with force=false; fetch the new ref, commit/tree, full recursive tree and each changed blob after update.


## Tranche 6S-C-H — Red Alert TemplateTypes IDs224–255 (completed)

- Run-start target `main`: `a278d4d52c570309f164cca103d42218c4f3aa13`; commit tree `dffc982ad40eb98d61ff3fc19dd401db2889b8af`; recursive target tree 554 entries, `truncated=false`. Pinned upstream `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`; recursive upstream tree 1,375 entries, `truncated=false`.
- Re-fetched current inventory, roadmap, Rust cross-reference, MAP_EDITOR README and TypeCatalog-Contracts. Full RA `TemplateTypes.cs` fetched at the pinned commit; blob `027bb5658e2297cf2ade84f9a01567acc8238db8` matches the upstream recursive tree.
- Parsed exactly 32 contiguous declarations, IDs224–255, source lines248–279. Field, explicit ID, key, dimensions, theatre whitelist, default flag and source-line anchors are recorded.
- IDs224–226 Rough09–11: 1×2, 2×1, 2×1; Temperate/Snow. IDs227–228 Road44/45: 1×1; Temperate/Snow. IDs229–230 River14/15: 1×2 and 2×1; Temperate/Snow.
- IDs231–234 RiverCliff01–04: 2×2; Temperate/Snow. IDs235–237 Bridge1a–1c: 4×3; IDs238–240 Bridge2a–2c: 5×3; IDs241–246 Bridge3a–3f: 4×2; all Temperate/Snow.
- IDs247–252 F01–F06: 3×3; Temperate/Snow. IDs253–255 ARRO0001–0003: 1×1; Interior only.
- All 32 use the five-argument constructor, which delegates with `TemplateTypeFlag.None`. TD has 216 declarations (IDs0–215); RA has 401 (IDs0–400). Coverage: TD216/216 and RA256/401. Both inventory rows remain Partially reviewed pending complete body/dependency/consumer/serializer review.
- Inventory reconciliation: the upstream recursive tree contains 1,331 blobs. The inventory markdown has 1,341 `- `` bullets because ten are status-key definitions, not source paths. Comparing path names to the upstream tree yields 1,331/1,331 exact matches, no missing paths and no extra source paths. Status counts remain 17 Reviewed, 121 Partially reviewed, 20 Header reviewed, 67 Body reviewed, 12 Resource reviewed, 1,094 Inventory only; Rust-mapped/Tested/Complete=0.
- No Rust code changed; no tests or runtime bitmap/tile lookup were executed. Rust `CellLayers.template: Option<u16>` remains an untyped identifier, not a concrete RA registry mapping.

## Next tranche — 6S-C-I: Red Alert TemplateTypes IDs256–287

1. Refresh `main`, commit/tree and both recursive trees; fetch inventory, roadmap, cross-reference, MAP_EDITOR README, TypeCatalog-Contracts and exact pinned RA source.
2. Parse exactly IDs256–287 and record field, ID, tile key, dimensions, theatre whitelist, flag and source line.
3. Add an RA slice guide and TD/RA comparison; update inventory, cross-reference, README, catalogue synthesis and roadmap.
4. Keep RA TemplateTypes Partially reviewed until IDs288–400 and full body, initializer, consumers and GamePlugin serialization are audited. TD remains Partially reviewed pending dependency/consumer/serializer review despite complete declaration enumeration.
5. After RA IDs288–400, prioritize both GamePlugin full-body serialization/INI contracts, theatre initialization consumers, editor map models and designer/resource files, Steamworks/native bindings, shared libraries and remaining engine systems.

## Persistent verification rules

- Reconcile inventory path rows against the pinned upstream recursive tree; exclude the ten status-key definition bullets from the 1,331 source paths.
- Body reviewed, Rust-mapped, Tested and Complete remain separate evidence states. Catalogue declaration slices do not promote either TemplateTypes source beyond Partially reviewed.
- Commit using a base-tree update and expected-SHA lease with `force=false`; verify the updated ref, commit/tree, complete recursive target tree and every changed blob after the update.
