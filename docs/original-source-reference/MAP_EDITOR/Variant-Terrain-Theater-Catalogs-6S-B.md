# Tranche 6S-B — TD/RA terrain and theatre catalogs

Body review complete for four registries. Exact declarations, masks, theatre restrictions, flags, reflection and GetTypes inspected. Rust mapping/tests not claimed.

- [TD TerrainTypes](TiberianDawn/TerrainTypes.cs.md) · [RA TerrainTypes](RedAlert/TerrainTypes.cs.md)

- [TD TheaterTypes](TiberianDawn/TheaterTypes.cs.md) · [RA TheaterTypes](RedAlert/TheaterTypes.cs.md)

- Models: [TerrainType](TerrainType.cs.md) · [TheaterType](TheaterType.cs.md)

| Catalog | TD | RA |

|---|---:|---:|

| Terrain | 32 (0–31) | 35 (0–34) |

| Theatre | 3 | 3 |

| Transformable | 2 split | 0 |

| Water | 0 | 5 ice |

| OreMine | 0 | 1 Mine |

TD has desert-only t04/t09/t18 and rocks; Tree8 all theatres; Split2/Split3 transformable shape22. RA omits t04/t09; has clumps, Snow ice/Water, Interior boxes and Mine/OreMine. Exact masks are per-file.

| Variant | Field | ID | Name | Terrain key |

|---|---|---:|---|---|

| TD | Desert | 0 | desert | TD_Terrain_Desert |

| TD | Temperate | 2 | temperate | TD_Terrain_Temperate |

| TD | Winter | 2 | winter | TD_Terrain_Winter |

| RA | Temperate | 0 | temperate | RA_Terrain_Temperate |

| RA | Snow | 1 | snow | RA_Terrain_Snow |

| RA | Interior | 2 | interior | RA_Terrain_Interior |

Common sequence appended: TD_Units, TD_Structures, TD_VFX, Common_VFX; RA_Units, RA_Structures, RA_VFX, Common_VFX. TD Temperate/Winter share ID2; source anomaly preserved, impact untested.

GamePlugin passes catalogs to Map; Theater change calls Map.InitTheater. InitTheater initializes Template, Smudge, Overlay, Terrain, Infantry, Unit, Building in order. TerrainType.Init uses OREMINE/shape22-or-0; success sets RenderSize/thumbnail. MainForm/TerrainTool theatre-filter; terrain selector doesn't require thumbnail. False lookup leaves metadata unchanged.

Rust only has numeric terrain layers; no typed catalog, masks, flags, theatre/render metadata. Not Rust-mapped/Tested. Add catalog, duplicate-ID/fallback, shape/OreMine, miss-reset, selector and render fixtures. No Rust code/tests changed.