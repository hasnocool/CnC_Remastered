# Red Alert TheaterTypes.cs — static catalog body audit

> **Body reviewed.** Full 47-line registry, three definitions, static constructor and GetTypes inspected. Rust mapping/tests open.

Upstream `CnCTDRAMapEditor/RedAlert/TheaterTypes.cs`; pinned `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`; blob `19748321124d5d37e9923f4d84746376e65e2668`. [Source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TheaterTypes.cs)

Private commonTilesets ordered array: `RA_Units`, `RA_Structures`, `RA_VFX`, `Common_VFX`. Each theatre's terrain key is prepended.

| Field | ID | Name | Terrain key | Common tilesets |
|---|---:|---|---|---|
| `Temperate` | 0 | `temperate` | `RA_Terrain_Temperate` | `RA_Units` → `RA_Structures` → `RA_VFX` → `Common_VFX` |
| `Snow` | 1 | `snow` | `RA_Terrain_Snow` | `RA_Units` → `RA_Structures` → `RA_VFX` → `Common_VFX` |
| `Interior` | 2 | `interior` | `RA_Terrain_Interior` | `RA_Units` → `RA_Structures` → `RA_VFX` → `Common_VFX` |

Static constructor reflects public static readonly TheaterType fields and ToArray()s without sort/validation. GetTypes returns backing array; no registry Init method.

RA IDs are 0/1/2 for Temperate/Snow/Interior; no duplicate ID in this registry.

GamePlugin passes catalog to Map; New(name) resolves by name/falls back to Temperate. MapSection converter resolves names or falls back to first catalog entry (order-dependent). Theater property changes call Map.InitTheater, which initializes template, smudge, overlay, terrain, infantry, unit and building catalogs.

Rust has no typed theatre catalog or ordered tileset sequence. Not Rust-mapped/Tested. Preserve IDs/names/order/fallback; no tests executed.