# Tiberian Dawn TheaterTypes.cs — static catalog body audit

> **Body reviewed.** Full 47-line registry, three definitions, static constructor and GetTypes inspected. Rust mapping/tests open.

Upstream `CnCTDRAMapEditor/TiberianDawn/TheaterTypes.cs`; pinned `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`; blob `db23c9ae37df8f3e94f86ab01135e21c990d4123`. [Source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/TheaterTypes.cs)

Private commonTilesets ordered array: `TD_Units`, `TD_Structures`, `TD_VFX`, `Common_VFX`. Each theatre's terrain key is prepended.

| Field | ID | Name | Terrain key | Common tilesets |
|---|---:|---|---|---|
| `Desert` | 0 | `desert` | `TD_Terrain_Desert` | `TD_Units` → `TD_Structures` → `TD_VFX` → `Common_VFX` |
| `Temperate` | 2 | `temperate` | `TD_Terrain_Temperate` | `TD_Units` → `TD_Structures` → `TD_VFX` → `Common_VFX` |
| `Winter` | 2 | `winter` | `TD_Terrain_Winter` | `TD_Units` → `TD_Structures` → `TD_VFX` → `Common_VFX` |

Static constructor reflects public static readonly TheaterType fields and ToArray()s without sort/validation. GetTypes returns backing array; no registry Init method.

**Duplicate ID:** Temperate and Winter both ID2 (Desert ID0). TheaterType hashes by ID but same-type equality is reference-based. ID-keyed consequences untested. New falls back Temperate; unknown MapSection string falls back first catalog (declared Desert; reflection unsorted).

GamePlugin passes catalog to Map; New(name) resolves by name/falls back to Temperate. MapSection converter resolves names or falls back to first catalog entry (order-dependent). Theater property changes call Map.InitTheater, which initializes template, smudge, overlay, terrain, infantry, unit and building catalogs.

Rust has no typed theatre catalog or ordered tileset sequence. Not Rust-mapped/Tested. Preserve IDs/names/order/fallback; no tests executed.