# Red Alert TerrainTypes.cs — static catalog body audit

> **Body reviewed; Rust catalog mapped.** Full 77-line registry and all 35 entries, static constructor and GetTypes inspected. Catalog test coverage is added but has not been executed.

Upstream `CnCTDRAMapEditor/RedAlert/TerrainTypes.cs`; pinned `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`; blob `4cff6d34bb347fad76cf7ccbf435e61a7a9213fa`. [Source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TerrainTypes.cs)

`public static class TerrainTypes` declares 35 public static readonly TerrainType fields and private static `TerrainType[] Types`; IDs are sbyte. No text keys.

| Field | ID | Name | Theatres | Mask [rows×cols] | Rows (1=true) | Transformable | Flag |
|---|---:|---|---|---|---|---|---|
| `Tree1Class` | 0 | `t01` | Temperate, Snow | 2×2 | `00/10` | no | None |
| `Tree2Class` | 1 | `t02` | Temperate, Snow | 2×2 | `00/10` | no | None |
| `Tree3Class` | 2 | `t03` | Temperate, Snow | 2×2 | `00/10` | no | None |
| `Tree5Class` | 3 | `t05` | Temperate, Snow | 2×2 | `00/10` | no | None |
| `Tree6Class` | 4 | `t06` | Temperate, Snow | 2×2 | `00/10` | no | None |
| `Tree7Class` | 5 | `t07` | Temperate, Snow | 2×2 | `00/10` | no | None |
| `Tree8Class` | 6 | `t08` | Temperate, Snow | 1×2 | `10` | no | None |
| `Tree10Class` | 7 | `t10` | Temperate, Snow | 2×2 | `00/11` | no | None |
| `Tree11Class` | 8 | `t11` | Temperate, Snow | 2×2 | `00/11` | no | None |
| `Tree12Class` | 9 | `t12` | Temperate, Snow | 2×2 | `00/10` | no | None |
| `Tree13Class` | 10 | `t13` | Temperate, Snow | 2×2 | `00/10` | no | None |
| `Tree14Class` | 11 | `t14` | Temperate, Snow | 2×2 | `00/11` | no | None |
| `Tree15Class` | 12 | `t15` | Temperate, Snow | 2×2 | `00/11` | no | None |
| `Tree16Class` | 13 | `t16` | Temperate, Snow | 2×2 | `00/10` | no | None |
| `Tree17Class` | 14 | `t17` | Temperate, Snow | 2×2 | `00/10` | no | None |
| `Clump1Class` | 15 | `tc01` | Temperate, Snow | 2×3 | `000/110` | no | None |
| `Clump2Class` | 16 | `tc02` | Temperate, Snow | 2×3 | `010/110` | no | None |
| `Clump3Class` | 17 | `tc03` | Temperate, Snow | 2×3 | `110/110` | no | None |
| `Clump4Class` | 18 | `tc04` | Temperate, Snow | 3×4 | `0000/1110/1000` | no | None |
| `Clump5Class` | 19 | `tc05` | Temperate, Snow | 3×4 | `0010/1110/0110` | no | None |
| `Ice01` | 20 | `ice01` | Snow | 2×2 | `11/11` | no | TemplateTypeFlag.Water |
| `Ice02` | 21 | `ice02` | Snow | 2×2 | `10/10` | no | TemplateTypeFlag.Water |
| `Ice03` | 22 | `ice03` | Snow | 1×2 | `11` | no | TemplateTypeFlag.Water |
| `Ice04` | 23 | `ice04` | Snow | 1×2 | `10` | no | TemplateTypeFlag.Water |
| `Ice05` | 24 | `ice05` | Snow | 1×2 | `10` | no | TemplateTypeFlag.Water |
| `Boxes01` | 25 | `boxes01` | Interior | 1×2 | `10` | no | None |
| `Boxes02` | 26 | `boxes02` | Interior | 1×2 | `10` | no | None |
| `Boxes03` | 27 | `boxes03` | Interior | 1×2 | `10` | no | None |
| `Boxes04` | 28 | `boxes04` | Interior | 1×2 | `10` | no | None |
| `Boxes05` | 29 | `boxes05` | Interior | 1×2 | `10` | no | None |
| `Boxes06` | 30 | `boxes06` | Interior | 1×2 | `10` | no | None |
| `Boxes07` | 31 | `boxes07` | Interior | 1×2 | `10` | no | None |
| `Boxes08` | 32 | `boxes08` | Interior | 1×2 | `10` | no | None |
| `Boxes09` | 33 | `boxes09` | Interior | 1×2 | `10` | no | None |
| `Mine` | 34 | `mine` | Temperate, Snow | 1×2 | `10` | no | TemplateTypeFlag.OreMine |

Mask rows preserve `bool[rows,columns]` initializer order (1=true, 0=false); Size uses columns as width, rows as height. OverlapBounds is render-derived.

Static constructor reflects public static readonly TerrainType fields and ToArray()s without explicit sort, validation or defensive copy. GetTypes returns backing array enumerable.

- t04/t09 absent. Clumps15–19; Ice20–24 Snow/Water; Boxes25–33 Interior; Mine34/mine Temperate/Snow/OreMine. No transformable entries.

GamePlugin passes registry to Map, which copies it. Map.InitTheater initializes every TerrainType before theatre filtering. MainForm/TerrainTool filter by theatre and sort Name; terrain selector does not require Thumbnail. Plugins resolve terrain names case-insensitively.

TerrainType.Init uses OREMINE when OreMine, shape22 transformable/shape0 otherwise. Success sets RenderSize and half thumbnail; false lookup leaves existing metadata (possible stale shared state, runtime untested).

Rust has numeric CellLayers.terrain only, no typed catalog/masks/theatres/flags/render cache/tool. Not Rust-mapped or Tested. Preserve data/orientation, separate occupancy/render bounds and deterministic missing-asset handling.

## Rust mapping update — catalog imported

All 35 source declarations are represented in `crates/map-editor-model/src/terrain_catalog.rs`, including original IDs/names, theatre lists, masks, Water/OreMine flags and non-transformable metadata. The mask rows are preserved top-to-bottom and converted to width=columns/height=rows. Automated catalog and mask tests are present but not yet run.
