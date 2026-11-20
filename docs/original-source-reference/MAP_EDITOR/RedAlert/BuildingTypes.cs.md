# Red Alert BuildingTypes.cs — static catalog body audit

**Status:** Body reviewed. All 63 catalog entries, static constructor and `GetTypes()` were inspected at the pinned upstream commit. Rust mapping and tests remain open.

- Upstream path: `CnCTDRAMapEditor/RedAlert/BuildingTypes.cs`
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source blob SHA: `75fccb7dc7447f010989a48cbf9e04c3da922e3b`
- [Exact source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/BuildingTypes.cs)
- Source length: 105 lines.

## Declaration and catalog

`public static class BuildingTypes` contains 63 public `static readonly BuildingType` entries and a private `static readonly BuildingType[] Types`. Numeric IDs are passed to the model's `sbyte` constructor.

Mask notation: `H×W:row0/row1…`; `#` occupied, `.` false. First array dimension is height/row; second is width/column. `BuildingType.Size` is width×height. `Theatres=all` means null. Fake entries derive effective `Name` by replacing the input token's last character with `f`; `Tilename` retains the input token.

| Field | ID | Tile token | Effective Name | Text key | Base mask | Bib | Owner | Theatres | Fake | Turret | Factory overlay |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `AdvancedTech` | 0 | `atek` | `atek` | `TEXT_STRUCTURE_RA_ATEK` | `2x2:##/##` | yes | `Greece` | all | no | no | — |
| `IronCurtain` | 1 | `iron` | `iron` | `TEXT_STRUCTURE_RA_IRON` | `2x2:../##` | no | `USSR` | all | no | no | — |
| `Weapon` | 2 | `weap` | `weap` | `TEXT_STRUCTURE_RA_WEAP` | `2x3:###/###` | yes | `Greece` | all | no | no | `weap2` |
| `Chronosphere` | 3 | `pdox` | `pdox` | `TEXT_STRUCTURE_RA_PDOX` | `2x2:##/##` | no | `Greece` | all | no | no | — |
| `Pillbox` | 4 | `pbox` | `pbox` | `TEXT_STRUCTURE_RA_PBOX` | `1x1:#` | no | `Greece` | all | no | no | — |
| `CamoPillbox` | 5 | `hbox` | `hbox` | `TEXT_STRUCTURE_RA_HBOX` | `1x1:#` | no | `Greece` | all | no | no | — |
| `Command` | 6 | `dome` | `dome` | `TEXT_STRUCTURE_RA_DOME` | `2x2:##/##` | yes | `Greece` | all | no | no | — |
| `GapGenerator` | 7 | `gap` | `gap` | `TEXT_STRUCTURE_RA_GAP` | `2x1:./#` | no | `Greece` | all | no | no | — |
| `Turret` | 8 | `gun` | `gun` | `TEXT_STRUCTURE_RA_GUN` | `1x1:#` | no | `Greece` | all | no | yes | — |
| `AAGun` | 9 | `agun` | `agun` | `TEXT_STRUCTURE_RA_AGUN` | `2x1:./#` | no | `Greece` | all | no | yes | — |
| `FlameTurret` | 10 | `ftur` | `ftur` | `TEXT_STRUCTURE_RA_FTUR` | `1x1:#` | no | `USSR` | all | no | no | — |
| `Const` | 11 | `fact` | `fact` | `TEXT_STRUCTURE_RA_FACT` | `3x3:###/###/###` | yes | `Greece` | all | no | no | — |
| `Refinery` | 12 | `proc` | `proc` | `TEXT_STRUCTURE_RA_PROC` | `3x3:.#./###/#..` | yes | `Greece` | all | no | no | — |
| `Storage` | 13 | `silo` | `silo` | `TEXT_STRUCTURE_RA_SILO` | `1x1:#` | no | `Greece` | all | no | no | — |
| `Helipad` | 14 | `hpad` | `hpad` | `TEXT_STRUCTURE_RA_HPAD` | `2x2:##/##` | yes | `Greece` | all | no | no | — |
| `SAM` | 15 | `sam` | `sam` | `TEXT_STRUCTURE_RA_SAM` | `1x2:##` | no | `USSR` | all | no | no | — |
| `AirStrip` | 16 | `afld` | `afld` | `TEXT_STRUCTURE_RA_AFLD` | `2x3:###/###` | no | `USSR` | all | no | no | — |
| `Power` | 17 | `powr` | `powr` | `TEXT_STRUCTURE_RA_POWR` | `2x2:##/##` | yes | `Greece` | all | no | no | — |
| `AdvancedPower` | 18 | `apwr` | `apwr` | `TEXT_STRUCTURE_RA_APWR` | `3x3:.../###/###` | yes | `Greece` | all | no | no | — |
| `SovietTech` | 19 | `stek` | `stek` | `TEXT_STRUCTURE_RA_STEK` | `3x3:.../###/###` | yes | `USSR` | all | no | no | — |
| `Hospital` | 20 | `hosp` | `hosp` | `TEXT_STRUCTURE_RA_HOSP` | `2x2:##/##` | yes | `Greece` | all | no | no | — |
| `Barracks` | 21 | `barr` | `barr` | `TEXT_STRUCTURE_RA_BARR` | `2x2:##/##` | yes | `USSR` | all | no | no | — |
| `Tent` | 22 | `tent` | `tent` | `TEXT_STRUCTURE_RA_TENT` | `2x2:##/##` | yes | `Greece` | all | no | no | — |
| `Kennel` | 23 | `kenn` | `kenn` | `TEXT_STRUCTURE_RA_KENN` | `1x1:#` | no | `USSR` | all | no | no | — |
| `Repair` | 24 | `fix` | `fix` | `TEXT_STRUCTURE_RA_FIX` | `3x3:.#./###/.#.` | no | `Greece` | all | no | no | — |
| `BioLab` | 25 | `bio` | `bio` | `TEXT_STRUCTURE_RA_BIO` | `2x2:##/##` | yes | `Greece` | all | no | no | — |
| `Mission` | 26 | `miss` | `miss` | `TEXT_STRUCTURE_RA_MISS` | `2x3:###/###` | yes | `Greece` | all | no | no | — |
| `ShipYard` | 27 | `syrd` | `syrd` | `TEXT_STRUCTURE_RA_SYRD` | `3x3:###/###/###` | no | `Greece` | all | no | no | — |
| `SubPen` | 28 | `spen` | `spen` | `TEXT_STRUCTURE_RA_SPEN` | `3x3:###/###/###` | no | `USSR` | all | no | no | — |
| `MissileSilo` | 29 | `mslo` | `mslo` | `TEXT_STRUCTURE_RA_MSLO` | `1x2:##` | no | `Greece` | Temperate+Snow | no | no | — |
| `ForwardCom` | 30 | `fcom` | `fcom` | `TEXT_STRUCTURE_RA_FCOM` | `2x2:../##` | yes | `USSR` | all | no | no | — |
| `Tesla` | 31 | `tsla` | `tsla` | `TEXT_STRUCTURE_RA_TSLA` | `2x1:./#` | no | `USSR` | all | no | no | — |
| `FakeWeapon` | 32 | `weap` | `weaf` | `TEXT_STRUCTURE_RA_WEAF` | `2x3:###/###` | yes | `Greece` | all | yes | no | `weap2` |
| `FakeConst` | 33 | `fact` | `facf` | `TEXT_STRUCTURE_RA_FACF` | `3x3:###/###/###` | yes | `Greece` | all | yes | no | — |
| `FakeShipYard` | 34 | `syrd` | `syrf` | `TEXT_STRUCTURE_RA_SYRF` | `3x3:###/###/###` | no | `Greece` | all | yes | no | — |
| `FakeSubPen` | 35 | `spen` | `spef` | `TEXT_STRUCTURE_RA_SPEF` | `3x3:###/###/###` | no | `USSR` | all | yes | no | — |
| `FakeCommand` | 36 | `dome` | `domf` | `TEXT_STRUCTURE_RA_DOMF` | `2x2:##/##` | yes | `Greece` | all | yes | no | — |
| `AVMine` | 43 | `minv` | `minv` | `TEXT_STRUCTURE_RA_MINV` | `1x1:#` | no | `Greece` | all | no | no | — |
| `APMine` | 44 | `minp` | `minp` | `TEXT_STRUCTURE_RA_MINP` | `1x1:#` | no | `Greece` | all | no | no | — |
| `V01` | 45 | `v01` | `v01` | `TEXT_STRUCTURE_TITLE_CIV1` | `2x2:../##` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V02` | 46 | `v02` | `v02` | `TEXT_STRUCTURE_TITLE_CIV2` | `2x2:../##` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V03` | 47 | `v03` | `v03` | `TEXT_STRUCTURE_TITLE_CIV3` | `2x2:.#/##` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V04` | 48 | `v04` | `v04` | `TEXT_STRUCTURE_TITLE_CIV4` | `2x2:../##` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V05` | 49 | `v05` | `v05` | `TEXT_STRUCTURE_TITLE_CIV5` | `1x2:##` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V06` | 50 | `v06` | `v06` | `TEXT_STRUCTURE_TITLE_CIV6` | `1x2:##` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V07` | 51 | `v07` | `v07` | `TEXT_STRUCTURE_TITLE_CIV7` | `1x2:##` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V08` | 52 | `v08` | `v08` | `TEXT_STRUCTURE_TITLE_CIV8` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V09` | 53 | `v09` | `v09` | `TEXT_STRUCTURE_TITLE_CIV9` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V10` | 54 | `v10` | `v10` | `TEXT_STRUCTURE_TITLE_CIV10` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V11` | 55 | `v11` | `v11` | `TEXT_STRUCTURE_TITLE_CIV11` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V12` | 56 | `v12` | `v12` | `TEXT_STRUCTURE_RA_CIVILIAN` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V13` | 57 | `v13` | `v13` | `TEXT_STRUCTURE_RA_CIVILIAN` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V14` | 58 | `v14` | `v14` | `TEXT_STRUCTURE_TITLE_CIV13` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V15` | 59 | `v15` | `v15` | `TEXT_STRUCTURE_TITLE_CIV14` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V16` | 60 | `v16` | `v16` | `TEXT_STRUCTURE_TITLE_CIV15` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V17` | 61 | `v17` | `v17` | `TEXT_STRUCTURE_TITLE_CIV16` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V18` | 62 | `v18` | `v18` | `TEXT_STRUCTURE_TITLE_CIV17` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `V19` | 63 | `v19` | `v19` | `TEXT_STRUCTURE_RA_CIVILIAN` | `1x1:#` | no | `Neutral` | Temperate+Snow | no | no | — |
| `Barrel` | 82 | `barl` | `barl` | `TEXT_STRUCTURE_RA_BARL` | `1x1:#` | no | `Neutral` | all | no | no | — |
| `Barrel3` | 83 | `brl3` | `brl3` | `TEXT_STRUCTURE_RA_BRL3` | `1x1:#` | no | `Neutral` | all | no | no | — |
| `Queen` | 84 | `quee` | `quee` | `TEXT_STRUCTURE_RA_QUEE` | `1x2:##` | no | `Special` | all | no | no | — |
| `Larva1` | 85 | `lar1` | `lar1` | `TEXT_STRUCTURE_RA_LAR1` | `1x1:#` | no | `Special` | all | no | no | — |
| `Larva2` | 86 | `lar2` | `lar2` | `TEXT_STRUCTURE_RA_LAR2` | `1x1:#` | no | `Special` | all | no | no | — |

## ID coverage

- Entries: **63**.
- IDs in source declaration order: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 82, 83, 84, 85, 86.
- Missing IDs within 0–86: 37, 38, 39, 40, 41, 42, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81.
- Registry performs no duplicate-ID, theatre, mask-shape, or field-name validation.

## Static initialization and methods

1. Static fields construct definitions. The model constructor resolves display-text keys through `Globals.TheGameTextManager`, retains the base mask/metadata, and derives the full bib mask.
2. The static constructor reflects over `typeof(BuildingTypes).GetFields(BindingFlags.Static | BindingFlags.Public)`, filters `IsInitOnly` and BuildingType assignability, reads field values and materializes an array. No explicit sort, validation or null filter.
3. `GetTypes()` returns the same array through `IEnumerable<BuildingType>`; it is not a copy. Callers can cast and mutate it even though the field is readonly.

## Initialization and consumers

The registry does not render previews. Both game plugins pass `GetTypes()` to Map. `Map.InitTheater(gameType)` later calls each definition's `Init` with the selected theatre, matching owner-house lookup and north direction. BuildingType.Init renders a mock building at strength 256 through MapRenderer and only assigns Thumbnail when render bounds are non-empty.

MainForm's Structures selector theatre-filters and sorts by `IsFake`, then `Name`. Static catalog instances are shared per game variant, not per map.

## Dependencies and Rust mapping

Dependencies: BuildingType, TheaterTypes for restricted entries, global text manager, Map.InitTheater, MapRenderer and Structures filtering. `System.Drawing` is imported.

**Rust mapping:** none. The Rust map-editor-model stores numeric layers and has no typed building catalog. Preserve entries as typed IDs and validated masks/theatre sets; represent fake/effective Name and Tilename explicitly. **Tests:** none executed; catalog, mask, alias, theatre and preview fixtures remain outstanding.
