# Tranche 6S-C-J — Red Alert TemplateTypes IDs 288–319

Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.

- Source: [CnCTDRAMapEditor/RedAlert/TemplateTypes.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs)
- Pinned source blob: `027bb5658e2297cf2ade84f9a01567acc8238db8`
- This batch: 32 contiguous declarations at source lines 312–343.
- Registry total: 401 declarations, IDs 0–400. After this batch, declaration enumeration is TD 216/216 and RA 320/401.

This is a declaration tranche plus a bounded consumer/Rust cross-check. It does not complete the full `TemplateTypes.cs` body/dependency/serializer audit and does not make the source path Rust-mapped, Tested or Complete.

## Exact declarations

The five-argument constructor is used for every row. The shared overload delegates to the six-argument constructor with `TemplateTypeFlag.None`. Every row explicitly lists only `TheaterTypes.Interior`.

| Field | ID | Tile key | Footprint | Theatre whitelist | Flag | Source |
|---|---:|---|---:|---|---|---:|
| GSTR0009 | 288 | `gstr0009` | 1×1 | Interior only | None | [312](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L426) |
| GSTR0010 | 289 | `gstr0010` | 1×1 | Interior only | None | [313](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L313) |
| GSTR0011 | 290 | `gstr0011` | 1×1 | Interior only | None | [314](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L314) |
| LWAL0001 | 291 | `lwal0001` | 1×1 | Interior only | None | [315](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L315) |
| LWAL0002 | 292 | `lwal0002` | 1×1 | Interior only | None | [316](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L316) |
| LWAL0003 | 293 | `lwal0003` | 1×1 | Interior only | None | [317](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L317) |
| LWAL0004 | 294 | `lwal0004` | 1×1 | Interior only | None | [318](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L318) |
| LWAL0005 | 295 | `lwal0005` | 1×1 | Interior only | None | [319](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L319) |
| LWAL0006 | 296 | `lwal0006` | 1×1 | Interior only | None | [320](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L320) |
| LWAL0007 | 297 | `lwal0007` | 1×1 | Interior only | None | [321](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L321) |
| LWAL0008 | 298 | `lwal0008` | 1×1 | Interior only | None | [322](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L322) |
| LWAL0009 | 299 | `lwal0009` | 1×1 | Interior only | None | [323](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L323) |
| LWAL0010 | 300 | `lwal0010` | 1×1 | Interior only | None | [324](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L324) |
| LWAL0011 | 301 | `lwal0011` | 1×1 | Interior only | None | [325](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L325) |
| LWAL0012 | 302 | `lwal0012` | 1×1 | Interior only | None | [326](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L326) |
| LWAL0013 | 303 | `lwal0013` | 1×1 | Interior only | None | [327](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L327) |
| LWAL0014 | 304 | `lwal0014` | 1×1 | Interior only | None | [328](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L328) |
| LWAL0015 | 305 | `lwal0015` | 1×1 | Interior only | None | [329](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L329) |
| LWAL0016 | 306 | `lwal0016` | 1×1 | Interior only | None | [330](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L330) |
| LWAL0017 | 307 | `lwal0017` | 1×1 | Interior only | None | [331](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L331) |
| LWAL0018 | 308 | `lwal0018` | 1×1 | Interior only | None | [332](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L332) |
| LWAL0019 | 309 | `lwal0019` | 1×1 | Interior only | None | [333](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L333) |
| LWAL0020 | 310 | `lwal0020` | 1×1 | Interior only | None | [334](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L334) |
| LWAL0021 | 311 | `lwal0021` | 1×1 | Interior only | None | [335](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L335) |
| LWAL0022 | 312 | `lwal0022` | 1×1 | Interior only | None | [336](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L336) |
| LWAL0023 | 313 | `lwal0023` | 1×1 | Interior only | None | [337](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L337) |
| LWAL0024 | 314 | `lwal0024` | 1×1 | Interior only | None | [338](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L338) |
| LWAL0025 | 315 | `lwal0025` | 1×1 | Interior only | None | [339](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L339) |
| LWAL0026 | 316 | `lwal0026` | 1×1 | Interior only | None | [340](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L340) |
| LWAL0027 | 317 | `lwal0027` | 1×1 | Interior only | None | [341](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L341) |
| STRP0001 | 318 | `strp0001` | 1×1 | Interior only | None | [342](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L342) |
| STRP0002 | 319 | `strp0002` | 1×1 | Interior only | None | [343](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L343) |

### Family ranges

- IDs 288–290: GSTR0009–GSTR0011.
- IDs 291–317: LWAL0001–LWAL0027.
- IDs 318–319: STRP0001–STRP0002.

All 32 are 1×1. Each therefore has exactly one logical icon slot (icon 0) when its tile payload resolves. No Temperate or Snow availability is declared.

## Registry body and consumer path

### Static catalog construction

`TemplateTypes` is a public static class. The 401 public static readonly `TemplateType` fields are initialized before its static constructor runs. The static constructor reflects `BindingFlags.Static | BindingFlags.Public`, retains fields where `IsInitOnly` is true and the field type is assignable to `TemplateType`, reads each value, and materializes an array with `ToArray()`. It does not explicitly sort the result. `GetTypes()` returns that backing array as `IEnumerable<TemplateType>`; it does not return a copy.

Source: [registry declaration and reflection initializer](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L426).

### Type initialization and asset lookup

The shared `TemplateType` has a `ushort ID`, name, icon width/height, theatre array, flag, mutable `IconMask` and `Thumbnail`. Its five-argument constructor forwards with `TemplateTypeFlag.None`. `Init(TheaterType)`:

1. Allocates a square icon cell using `Globals.OriginalTileWidth / 4` for both pixel dimensions.
2. Allocates a thumbnail using `Math.Max(IconWidth, IconHeight)` cells per side and a `bool[IconWidth, IconHeight]` mask.
3. Iterates y outer/x inner, incrementing the logical icon from zero.
4. Calls `Globals.TheTilesetManager.GetTileData(theater.Tilesets, Name, icon, out Tile)`.
5. Draws successful tiles at (x,y) and sets `mask[x,y]`; missing icons remain transparent/false.
6. Sets `Thumbnail` to the bitmap only if at least one icon was found; always assigns `IconMask`.

The array is indexed `[x,y]`, not `[y,x]`. The initializer does not itself enforce the `Theaters` array; consumers filter theatre availability. Source: [TemplateType.Init](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/TemplateType.cs#L102).

The RA `GamePlugin` constructor supplies `TemplateTypes.GetTypes()` to `Map`; the Map constructor copies the enumerable into a list. `Map.InitTheater` iterates every template type and calls `Init(Theater)` before initializing smudge, overlay, terrain and techno types. Sources: [RA plugin construction](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/GamePlugin.cs#L178), [Map catalog copy and theatre initialization](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Map.cs#L192).

`TilesetManager` loads configured tileset XML files and exposes tile lookup by the theatre's tileset names. The lookup loop is implemented as `tilesets.Join(searchTilesets, ...)`: the dictionary is the outer sequence, so the code does not explicitly traverse the supplied theatre sequence as the primary order. The first successful hit follows dictionary enumeration order; verify this before assuming the theatre list is a guaranteed precedence list. `Tileset.Load` records FPS but the active `#else` allocates `Math.Min(1, frameNodes.Count)` frame slots, so only the first frame is retained. The single-`Tile` lookup returns `tiles[0]`; `TemplateType.Init` does not use FPS. Sources: [TilesetManager lookup](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Utility/TilesetManager.cs#L59), [Tileset frame selection](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Utility/Tileset.cs#L90).

### Editor selection and placement

`TemplateTool` filters the map catalog to entries with a non-null thumbnail, a theatre list containing the active theatre, and no Clear flag. It groups by the leading lowercase name prefix, then creates ListView entries. A 1×1 type has no separate icon-cell selection; whole-type placement uses its single true mask cell. The tool writes a `Template` per valid cell and uses the mask for placement. Removal clears the full rectangular footprint. For IDs288–319, the footprint is one cell. If no matching tile was found during `Init`, `Thumbnail` is null and the type is omitted from the selector. Source: [TemplateTool catalog filter](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/TemplateTool.cs#L142).

No tileset payload/runtime lookup was executed for these 32 keys in this audit. Their declarations alone do not prove that every retail installation supplies the corresponding Interior tile.

## Serialization contract for these IDs

RA's editor MapPack uses a type plane followed by an icon plane:

- `LoadINI` reads one little-endian UInt16 type ID per cell, resolves it against `Map.TemplateTypes`, and nulls types unavailable in the current theatre.
- It then reads one icon byte per cell. For non-Clear types, an icon greater than or equal to `NumIcons` clears that cell.
- `SaveINI` writes UInt16 IDs (or `ushort.MaxValue` for a null template), then icon bytes (or `byte.MaxValue` for null).
- IDs288–319 are within the UInt16 type field and have `NumIcons = 1`.

Sources: [RA MapPack load](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/GamePlugin.cs#L478), [RA MapPack save](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/GamePlugin.cs#L1486).

TD differs: its editor BIN stream stores interleaved one-byte type/icon pairs per cell. Do not share the RA plane decoder with TD.

## Rust mapping reconciliation

There is a concrete but incomplete Rust asset mapping; earlier statements that there was no template-related Rust mapping were too broad.

- [rts-core/src/template_catalog.rs](https://github.com/hasnocool/CnC_Remastered/blob/4aeb2cb78795289240aadc6456bb9bb5a8fc4bdb/crates/rts-core/src/template_catalog.rs#L549) has RA metadata entries IDs288–319 at lines549–580. All 32 symbolic names and asset stems match these C# tile keys (case-normalized), and every entry has `theater_mask: 4`. `TemplateAssetMetadata::supports` maps RA Interior to bit 2 (value 4).
- [renderer/src/template_assets.rs](https://github.com/hasnocool/CnC_Remastered/blob/4aeb2cb78795289240aadc6456bb9bb5a8fc4bdb/crates/renderer/src/template_assets.rs#L70) resolves the stem plus the RA theatre suffix `.INT`, searches caller-supplied MIX archives by filename CRC, loads an `IconSet`, and can build a terrain atlas from `RaTemplateMap`.
- [map-editor-model/src/lib.rs](https://github.com/hasnocool/CnC_Remastered/blob/4aeb2cb78795289240aadc6456bb9bb5a8fc4bdb/crates/map-editor-model/src/lib.rs#L11) stores `CellLayers.template: Option<u16>` plus `template_icon: u8`; `template_cells()` exposes the pair. It still has no typed C#-equivalent `TemplateType` with dimensions, flags, theatre list, `IconMask`, thumbnail lifecycle or TemplateTool placement/undo behavior.

The RA Rust asset catalog currently contains 400 entries, IDs0–399, while the C# editor catalog contains 401, IDs0–400. Therefore the asset mapping for IDs288–319 is real and source-aligned, but the C# `TemplateTypes.cs` path is not fully Rust-mapped.

### ID400 AntHill discrepancy

The C# editor declares `AntHill = new TemplateType(400, "hill01", 4, 3, new TheaterType[] { TheaterTypes.Temperate })` at source line424. Rust's catalog comment says the RA `FIXIT_ANTS/TEMPLATE_HILL01` entry is omitted because the feature macro is non-default. However, the pinned upstream [REDALERT/DEFINES.H](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/DEFINES.H#L87) contains an unconditional `#define FIXIT_ANTS` at line87; the enum member is guarded at lines2145–2146, and [REDALERT/CDATA.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/CDATA.CPP#L2461) declares/registers AntHill under the same guard at lines2461–2467 and 2747–2749. The build/include configuration was not exhaustively traced here, so the exact compiled configuration remains to be confirmed; the pinned header does not support describing the macro as absent by default without further evidence.

As currently written, `template_asset_metadata(RedAlert, 400)` returns `None`, so `TemplateAssetProvider` reports `UnknownTemplateType(400)`. Reconcile the macro/build contract and the C# editor's unconditional ID400 before declaring RA catalog parity.

### Empty-cell sentinel integration gap

The C# serializers write null TD cells as `0xFF,0xFF` and null RA cells as `ushort.MaxValue` plus `byte.MaxValue`. The Rust raw decoders preserve these raw values: TD stores type `255`; RA stores type `65535`. But `TemplateAssetProvider::build_td_terrain_atlas` and `build_ra_terrain_atlas` collect every cell's type ID into a set and pass all of them to `load_icon_sets`; neither path filters the null sentinel first. Since those IDs are absent from the catalogs, a map containing an empty cell reaches `UnknownTemplateType(255)` for TD or `UnknownTemplateType(65535)` for RA before atlas construction. This is a source-level integration defect inferred from the concrete call path; it was not executed in this run.

Port requirement: preserve raw decoder values if needed for format fidelity, but normalize the game-specific null sentinel to `None` before catalog lookup and pass empty cells through as empty terrain cells. Add regression tests for both variants.

## Review and verification boundary

- This tranche adds declaration metadata and cross-file behavior evidence only.
- Rust static cross-check: 32/32 IDs, stems and RA Interior masks match; no Rust source was changed.
- Rust source contains catalog/decoder unit tests, but none were executed in this run. No retail MIX/ICON-set payloads or C# runtime image initialization were validated.
- `TemplateTypes.cs` remains **Partially reviewed** until IDs320–400 and the complete registry/initializer/consumer/serializer audit are closed. No Rust-mapped, Tested or Complete status is claimed.
- Next declaration slice: RA IDs320–351 (6S-C-K).
