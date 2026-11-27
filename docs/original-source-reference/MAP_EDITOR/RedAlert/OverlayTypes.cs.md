# Red Alert OverlayTypes.cs — static catalog body audit

**Status:** Body reviewed. All 25 catalog fields, static constructor and `GetTypes()` inspected at the pinned upstream commit. Rust mapping and tests remain open.

- Upstream path: `CnCTDRAMapEditor/RedAlert/OverlayTypes.cs`
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source blob SHA: `77eeb621a64a6605a26470717b8b0dbe9116e40b`
- [Exact source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/OverlayTypes.cs)
- Source length: 66 lines.

## Catalog

`public static class OverlayTypes` declares 25 public `static readonly OverlayType` entries. Shorthand constructors use the name token as the display-text key; entries with explicit text IDs use those keys.

| Field | ID | Name token | Display key | Flag | Theatres |
| --- | --- | --- | --- | --- | --- |
| `Sandbag` | 0 | `sbag` | `TEXT_STRUCTURE_RA_SBAG` | Wall | all |
| `Cyclone` | 1 | `cycl` | `TEXT_STRUCTURE_RA_CYCL` | Wall | all |
| `Brick` | 2 | `brik` | `TEXT_STRUCTURE_RA_BRIK` | Wall | all |
| `Barbwire` | 3 | `barb` | `TEXT_STRUCTURE_RA_BARB` | Wall | all |
| `Wood` | 4 | `wood` | `TEXT_STRUCTURE_TD_WOOD` | Wall | all |
| `Gold1` | 5 | `gold01` | `gold01` | TiberiumOrGold | all |
| `Gold2` | 6 | `gold02` | `gold02` | TiberiumOrGold | all |
| `Gold3` | 7 | `gold03` | `gold03` | TiberiumOrGold | all |
| `Gold4` | 8 | `gold04` | `gold04` | TiberiumOrGold | all |
| `Gems1` | 9 | `gem01` | `gem01` | Gems | all |
| `Gems2` | 10 | `gem02` | `gem02` | Gems | all |
| `Gems3` | 11 | `gem03` | `gem03` | Gems | all |
| `Gems4` | 12 | `gem04` | `gem04` | Gems | all |
| `V12` | 13 | `v12` | `TEXT_STRUCTURE_RA_CIVILIAN` | None | Temperate+Snow |
| `V13` | 14 | `v13` | `TEXT_STRUCTURE_RA_CIVILIAN` | None | Temperate+Snow |
| `V14` | 15 | `v14` | `TEXT_STRUCTURE_TITLE_CIV13` | None | Temperate+Snow |
| `V15` | 16 | `v15` | `TEXT_STRUCTURE_TITLE_CIV14` | None | Temperate+Snow |
| `V16` | 17 | `v16` | `TEXT_STRUCTURE_TITLE_CIV15` | None | Temperate+Snow |
| `V17` | 18 | `v17` | `TEXT_STRUCTURE_TITLE_CIV16` | None | Temperate+Snow |
| `V18` | 19 | `v18` | `TEXT_STRUCTURE_TITLE_CIV17` | None | Temperate+Snow |
| `FlagSpot` | 20 | `fpls` | `fpls` | Flag | all |
| `WoodCrate` | 21 | `wcrate` | `wcrate` | Crate | all |
| `SteelCrate` | 22 | `scrate` | `scrate` | Crate | all |
| `Fence` | 23 | `fenc` | `TEXT_STRUCTURE_RA_FENC` | Wall | all |
| `WaterCrate` | 24 | `wwcrate` | `wwcrate` | Crate | all |

## ID coverage

- Entries: **25**.
- IDs: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24.
- Missing IDs within 0–24: .
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


## Rust mapping update — partial

The 25 source declarations are now represented in `map-editor-model::OverlayCatalog` for Red Alert, including original IDs, name/display keys, category flags and theatre restrictions. The source registry remains body-reviewed; runtime tile lookup, localization and map-derived resource/wall behavior are not yet mapped. Rust tests were added but have not been executed in this environment.
