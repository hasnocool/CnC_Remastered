# Tiberian Dawn TerrainTypes.cs — static catalog body audit

> **Body reviewed; Rust catalog mapped.** Full 73-line registry and all 32 entries, static constructor and GetTypes inspected. Catalog test coverage is added but has not been executed.

Upstream `CnCTDRAMapEditor/TiberianDawn/TerrainTypes.cs`; pinned `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`; blob `6b1b288d16cebdc39db99edada49795c890bf3f0`. [Source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/TerrainTypes.cs)

`public static class TerrainTypes` declares 32 public static readonly TerrainType fields and private static `TerrainType[] Types`; IDs are sbyte. No text keys.

| Field | ID | Name | Theatres | Mask [rows×cols] | Rows (1=true) | Transformable | Flag |
|---|---:|---|---|---|---|---|---|
| `Tree1` | 0 | `t01` | Winter, Temperate | 2×2 | `00/10` | no | None |
| `Tree2` | 1 | `t02` | Winter, Temperate | 2×2 | `00/10` | no | None |
| `Tree3` | 2 | `t03` | Winter, Temperate | 2×2 | `00/10` | no | None |
| `Tree4` | 3 | `t04` | Desert | 1×1 | `1` | no | None |
| `Tree5` | 4 | `t05` | Winter, Temperate | 2×2 | `00/10` | no | None |
| `Tree6` | 5 | `t06` | Winter, Temperate | 2×2 | `00/10` | no | None |
| `Tree7` | 6 | `t07` | Winter, Temperate | 2×2 | `00/10` | no | None |
| `Tree8` | 7 | `t08` | Winter, Temperate, Desert | 1×2 | `10` | no | None |
| `Tree9` | 8 | `t09` | Desert | 1×2 | `10` | no | None |
| `Tree10` | 9 | `t10` | Winter, Temperate | 2×2 | `00/11` | no | None |
| `Tree11` | 10 | `t11` | Winter, Temperate | 2×2 | `00/11` | no | None |
| `Tree12` | 11 | `t12` | Winter, Temperate | 2×2 | `00/10` | no | None |
| `Tree13` | 12 | `t13` | Winter, Temperate | 2×2 | `00/10` | no | None |
| `Tree14` | 13 | `t14` | Winter, Temperate | 2×2 | `00/11` | no | None |
| `Tree15` | 14 | `t15` | Winter, Temperate | 2×2 | `00/11` | no | None |
| `Tree16` | 15 | `t16` | Winter, Temperate | 2×2 | `00/10` | no | None |
| `Tree17` | 16 | `t17` | Winter, Temperate | 2×2 | `00/10` | no | None |
| `Tree18` | 17 | `t18` | Desert | 2×3 | `000/010` | no | None |
| `Split1` | 18 | `split2` | Temperate, Winter | 2×2 | `00/10` | yes | None |
| `Split2` | 19 | `split3` | Temperate, Winter, Desert | 2×2 | `00/10` | yes | None |
| `Clump1` | 20 | `tc01` | Winter, Temperate | 2×3 | `000/110` | no | None |
| `Clump2` | 21 | `tc02` | Winter, Temperate | 2×3 | `010/110` | no | None |
| `Clump3` | 22 | `tc03` | Winter, Temperate | 2×3 | `110/110` | no | None |
| `Clump4` | 23 | `tc04` | Winter, Temperate | 3×4 | `0000/1110/1000` | no | None |
| `Clump5` | 24 | `tc05` | Winter, Temperate | 3×4 | `0010/1110/0110` | no | None |
| `Rock1` | 25 | `rock1` | Desert | 2×2 | `00/11` | no | None |
| `Rock2` | 26 | `rock2` | Desert | 2×2 | `11/00` | no | None |
| `Rock3` | 27 | `rock3` | Desert | 2×3 | `000/110` | no | None |
| `Rock4` | 28 | `rock4` | Desert | 1×2 | `10` | no | None |
| `Rock5` | 29 | `rock5` | Desert | 1×2 | `10` | no | None |
| `Rock6` | 30 | `rock6` | Desert | 2×3 | `000/111` | no | None |
| `Rock7` | 31 | `rock7` | Desert | 1×5 | `11110` | no | None |

Mask rows preserve `bool[rows,columns]` initializer order (1=true, 0=false); Size uses columns as width, rows as height. OverlapBounds is render-derived.

Static constructor reflects public static readonly TerrainType fields and ToArray()s without explicit sort, validation or defensive copy. GetTypes returns backing array enumerable.

- Desert-only Tree4 ID3/t04, Tree9 ID8/t09, Tree18 ID17/t18 and Rocks25–31. Tree8 all theatres. Split1/2 IDs18/19 transformable; Clumps20–24 Winter/Temperate. No flags.

GamePlugin passes registry to Map, which copies it. Map.InitTheater initializes every TerrainType before theatre filtering. MainForm/TerrainTool filter by theatre and sort Name; terrain selector does not require Thumbnail. Plugins resolve terrain names case-insensitively.

TerrainType.Init uses OREMINE when OreMine, shape22 transformable/shape0 otherwise. Success sets RenderSize and half thumbnail; false lookup leaves existing metadata (possible stale shared state, runtime untested).

Rust has numeric CellLayers.terrain only, no typed catalog/masks/theatres/flags/render cache/tool. Not Rust-mapped or Tested. Preserve data/orientation, separate occupancy/render bounds and deterministic missing-asset handling.

## Rust mapping update — catalog imported

All 32 source declarations are represented in `crates/map-editor-model/src/terrain_catalog.rs`, including original IDs/names, theatre lists, masks, transformable metadata and TemplateTypeFlag.None. The mask rows are preserved top-to-bottom and converted to width=columns/height=rows. Automated catalog and mask tests are present but not yet run.
