# Tiberian Dawn OverlayTypes.cs — static catalog body audit

**Status:** Body reviewed. All 27 catalog fields, static constructor and `GetTypes()` inspected at the pinned upstream commit. Rust mapping and tests remain open.

- Upstream path: `CnCTDRAMapEditor/TiberianDawn/OverlayTypes.cs`
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source blob SHA: `6440e70fab029e6d4f8a2554f05543e905011ec9`
- [Exact source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/OverlayTypes.cs)
- Source length: 68 lines.

## Catalog

`public static class OverlayTypes` declares 27 public `static readonly OverlayType` entries. Shorthand constructors use the name token as the display-text key; entries with explicit text IDs use those keys.

| Field | ID | Name token | Display key | Flag | Theatres |
| --- | --- | --- | --- | --- | --- |
| `Sandbag` | 1 | `sbag` | `TEXT_STRUCTURE_TITLE_GDI_SANDBAGS` | Wall | all |
| `Cyclone` | 2 | `cycl` | `TEXT_STRUCTURE_TITLE_GDI_CHAIN_LINK` | Wall | all |
| `Brick` | 3 | `brik` | `TEXT_STRUCTURE_TITLE_GDI_CONCRETE` | Wall | all |
| `Barbwire` | 4 | `barb` | `TEXT_STRUCTURE_RA_BARB` | Wall | all |
| `Wood` | 5 | `wood` | `TEXT_STRUCTURE_TD_WOOD` | Wall | all |
| `Tiberium1` | 6 | `ti1` | `ti1` | TiberiumOrGold | all |
| `Tiberium2` | 7 | `ti2` | `ti2` | TiberiumOrGold | all |
| `Tiberium3` | 8 | `ti3` | `ti3` | TiberiumOrGold | all |
| `Tiberium4` | 9 | `ti4` | `ti4` | TiberiumOrGold | all |
| `Tiberium5` | 10 | `ti5` | `ti5` | TiberiumOrGold | all |
| `Tiberium6` | 11 | `ti6` | `ti6` | TiberiumOrGold | all |
| `Tiberium7` | 12 | `ti7` | `ti7` | TiberiumOrGold | all |
| `Tiberium8` | 13 | `ti8` | `ti8` | TiberiumOrGold | all |
| `Tiberium9` | 14 | `ti9` | `ti9` | TiberiumOrGold | all |
| `Tiberium10` | 15 | `ti10` | `ti10` | TiberiumOrGold | all |
| `Tiberium11` | 16 | `ti11` | `ti11` | TiberiumOrGold | all |
| `Tiberium12` | 17 | `ti12` | `ti12` | TiberiumOrGold | all |
| `V12` | 20 | `v12` | `TEXT_STRUCTURE_TITLE_CIV12` | None | Temperate |
| `V13` | 21 | `v13` | `TEXT_STRUCTURE_TITLE_CIV12` | None | Temperate |
| `V14` | 22 | `v14` | `TEXT_STRUCTURE_TITLE_CIV13` | None | Temperate+Winter |
| `V15` | 23 | `v15` | `TEXT_STRUCTURE_TITLE_CIV14` | None | Temperate+Winter |
| `V16` | 24 | `v16` | `TEXT_STRUCTURE_TITLE_CIV15` | None | Temperate+Winter |
| `V17` | 25 | `v17` | `TEXT_STRUCTURE_TITLE_CIV16` | None | Temperate+Winter |
| `V18` | 26 | `v18` | `TEXT_STRUCTURE_TITLE_CIV17` | None | Temperate+Winter |
| `FlagSpot` | 27 | `fpls` | `fpls` | Flag | all |
| `WoodCrate` | 28 | `wcrate` | `wcrate` | Crate | all |
| `SteelCrate` | 29 | `scrate` | `scrate` | Crate | all |

## ID coverage

- Entries: **27**.
- IDs: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29.
- Missing IDs within 0–29: 0, 18, 19.
- No duplicate-ID/name/key, flag or theatre validation.

## Static initialization and methods

1. Static fields construct OverlayType. Constructors resolve display-text keys through the global text manager; shorthand uses Name. Flags and theatre arrays are retained.
2. The static constructor reflects public static fields, filters init-only OverlayType fields, and materializes the array. No sort or validation.
3. `GetTypes()` exposes the backing array via `IEnumerable<OverlayType>`, not a copy. Unlike BuildingTypes, its private array is not readonly.

## Category flags and consumers

OverlayTypeFlag values: None=0, TiberiumOrGold=1, Gems=2, Wall=4, Crate=8, Flag=16. `IsPlaceable` permits only None or Crate-only flags.

- MainForm Overlay availability/list uses IsPlaceable and null-or-current-theatre, then sorts by Name.
- Resources uses IsResource; painting distinguishes IsGem from IsTiberiumOrGold.
- Wall uses IsWall; selector sorts by Name and currently does not theatre-filter.
- `Map.InitTheater` initializes all catalog entries; OverlayType.Init requests tile 0 and replaces Thumbnail only on success.

## Dependencies and Rust mapping

Dependencies: OverlayType/OverlayTypeFlag, theatre definitions, text/tileset managers, Map.InitTheater and MainForm filters.

**Rust mapping:** none; `CellLayers.overlay` is numeric-only. Preserve IDs, keys, category flags and theatre restrictions. No catalog/render/Rust tests executed.
