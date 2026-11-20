# Tiberian Dawn BuildingTypes.cs — static catalog body audit

**Status:** Body reviewed. All 60 catalog entries, static constructor and `GetTypes()` were inspected at the pinned upstream commit. Rust mapping and tests remain open.

- Upstream path: `CnCTDRAMapEditor/TiberianDawn/BuildingTypes.cs`
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source blob SHA: `53222da65ce5fa68bc712c3d620c63c432badf9a`
- [Exact source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/BuildingTypes.cs)
- Source length: 102 lines.

## Declaration and catalog

`public static class BuildingTypes` contains 60 public `static readonly BuildingType` entries and a private `static readonly BuildingType[] Types`. Numeric IDs are passed to the model's `sbyte` constructor.

Mask notation: `H×W:row0/row1…`; `#` occupied, `.` false. First array dimension is height/row; second is width/column. `BuildingType.Size` is width×height. `Theatres=all` means null. Fake entries derive effective `Name` by replacing the input token's last character with `f`; `Tilename` retains the input token.

| Field | ID | Tile token | Effective Name | Text key | Base mask | Bib | Owner | Theatres | Fake | Turret | Factory overlay |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `Weapon` | 0 | `weap` | `weap` | `TEXT_STRUCTURE_TITLE_GDI_WEAPONS_FACTORY` | `3x3:.../###/###` | yes | `Goodguy` | all | no | no | `weap2` |
| `GTower` | 1 | `gtwr` | `gtwr` | `TEXT_STRUCTURE_TITLE_GDI_GUARD_TOWER` | `1x1:#` | no | `Goodguy` | all | no | no | — |
| `ATower` | 2 | `atwr` | `atwr` | `TEXT_STRUCTURE_TITLE_GDI_ADV_GUARD_TOWER` | `2x1:./#` | no | `Goodguy` | all | no | no | — |
| `Obelisk` | 3 | `obli` | `obli` | `TEXT_STRUCTURE_TITLE_NOD_OBELISK` | `2x1:./#` | no | `Badguy` | all | no | no | — |
| `Command` | 4 | `hq` | `hq` | `TEXT_STRUCTURE_TITLE_GDI_COMM_CENTER` | `2x2:#./##` | yes | `Goodguy` | all | no | no | — |
| `Turret` | 5 | `gun` | `gun` | `TEXT_STRUCTURE_TITLE_NOD_TURRET` | `1x1:#` | no | `Badguy` | all | no | yes | — |
| `Const` | 6 | `fact` | `fact` | `TEXT_STRUCTURE_RA_FACT` | `2x3:###/###` | yes | `Goodguy` | all | no | no | — |
| `Refinery` | 7 | `proc` | `proc` | `TEXT_STRUCTURE_TITLE_GDI_REFINERY` | `3x3:.#./###/...` | yes | `Goodguy` | all | no | no | — |
| `Storage` | 8 | `silo` | `silo` | `TEXT_STRUCTURE_TITLE_GDI_SILO` | `1x2:##` | yes | `Goodguy` | all | no | no | — |
| `Helipad` | 9 | `hpad` | `hpad` | `TEXT_STRUCTURE_TITLE_GDI_HELIPAD` | `2x2:##/##` | yes | `Goodguy` | all | no | no | — |
| `SAM` | 10 | `sam` | `sam` | `TEXT_STRUCTURE_TITLE_NOD_SAM_SITE` | `1x2:##` | no | `Badguy` | all | no | no | — |
| `AirStrip` | 11 | `afld` | `afld` | `TEXT_STRUCTURE_TITLE_NOD_AIRFIELD` | `2x4:####/####` | yes | `Badguy` | all | no | no | — |
| `Power` | 12 | `nuke` | `nuke` | `TEXT_STRUCTURE_TITLE_GDI_POWER_PLANT` | `2x2:#./##` | yes | `Goodguy` | all | no | no | — |
| `AdvancedPower` | 13 | `nuk2` | `nuk2` | `TEXT_STRUCTURE_TITLE_GDI_ADV_POWER_PLANT` | `2x2:#./##` | yes | `Goodguy` | all | no | no | — |
| `Hospital` | 14 | `hosp` | `hosp` | `TEXT_UNIT_TITLE_HOSP` | `2x2:##/##` | yes | `Goodguy` | all | no | no | — |
| `Barracks` | 15 | `pyle` | `pyle` | `TEXT_STRUCTURE_TITLE_GDI_BARRACKS` | `2x2:##/..` | yes | `Goodguy` | all | no | no | — |
| `Tanker` | 16 | `arco` | `arco` | `TEXT_UNIT_TITLE_ARCO` | `1x2:##` | no | `Goodguy` | all | no | no | — |
| `Repair` | 17 | `fix` | `fix` | `TEXT_STRUCTURE_TITLE_GDI_REPAIR_FACILITY` | `3x3:.#./###/.#.` | yes | `Goodguy` | all | no | no | — |
| `BioLab` | 18 | `bio` | `bio` | `TEXT_UNIT_TITLE_BIO` | `2x2:##/##` | yes | `Goodguy` | all | no | no | — |
| `Hand` | 19 | `hand` | `hand` | `TEXT_STRUCTURE_TITLE_NOD_HAND_OF_NOD` | `3x2:../##/.#` | yes | `Badguy` | all | no | no | — |
| `Temple` | 20 | `tmpl` | `tmpl` | `TEXT_STRUCTURE_TITLE_NOD_TEMPLE_OF_NOD` | `3x3:.../###/###` | yes | `Badguy` | all | no | no | — |
| `Eye` | 21 | `eye` | `eye` | `TEXT_STRUCTURE_TITLE_GDI_ADV_COMM_CENTER` | `2x2:#./##` | yes | `Goodguy` | all | no | no | — |
| `Mission` | 22 | `miss` | `miss` | `TEXT_STRUCTURE_TITLE_CIV35` | `2x3:###/###` | yes | `Goodguy` | all | no | no | — |
| `V01` | 23 | `v01` | `v01` | `TEXT_STRUCTURE_TITLE_CIV1` | `2x2:../##` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V02` | 24 | `v02` | `v02` | `TEXT_STRUCTURE_TITLE_CIV2` | `2x2:../##` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V03` | 25 | `v03` | `v03` | `TEXT_STRUCTURE_TITLE_CIV3` | `2x2:.#/##` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V04` | 26 | `v04` | `v04` | `TEXT_STRUCTURE_TITLE_CIV4` | `2x2:../##` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V05` | 27 | `v05` | `v05` | `TEXT_STRUCTURE_TITLE_CIV5` | `1x2:##` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V06` | 28 | `v06` | `v06` | `TEXT_STRUCTURE_TITLE_CIV6` | `1x2:##` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V07` | 29 | `v07` | `v07` | `TEXT_STRUCTURE_TITLE_CIV7` | `1x2:##` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V08` | 30 | `v08` | `v08` | `TEXT_STRUCTURE_TITLE_CIV8` | `1x1:#` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V09` | 31 | `v09` | `v09` | `TEXT_STRUCTURE_TITLE_CIV9` | `1x1:#` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V10` | 32 | `v10` | `v10` | `TEXT_STRUCTURE_TITLE_CIV10` | `1x1:#` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V11` | 33 | `v11` | `v11` | `TEXT_STRUCTURE_TITLE_CIV11` | `1x1:#` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V12` | 34 | `v12` | `v12` | `TEXT_STRUCTURE_TITLE_CIV12` | `1x1:#` | no | `Neutral` | Temperate | no | no | — |
| `V13` | 35 | `v13` | `v13` | `TEXT_STRUCTURE_TITLE_CIV12` | `1x1:#` | no | `Neutral` | Temperate | no | no | — |
| `V14` | 36 | `v14` | `v14` | `TEXT_STRUCTURE_TITLE_CIV13` | `1x1:#` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V15` | 37 | `v15` | `v15` | `TEXT_STRUCTURE_TITLE_CIV14` | `1x1:#` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V16` | 38 | `v16` | `v16` | `TEXT_STRUCTURE_TITLE_CIV15` | `1x1:#` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V17` | 39 | `v17` | `v17` | `TEXT_STRUCTURE_TITLE_CIV16` | `1x1:#` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V18` | 40 | `v18` | `v18` | `TEXT_STRUCTURE_TITLE_CIV17` | `1x1:#` | no | `Neutral` | Temperate+Winter | no | no | — |
| `V19` | 41 | `v19` | `v19` | `TEXT_STRUCTURE_CIVILIAN_TITLE` | `1x1:#` | no | `Neutral` | all | no | no | — |
| `V20` | 42 | `v20` | `v20` | `TEXT_STRUCTURE_TITLE_CIV18` | `2x2:../##` | no | `Neutral` | Desert | no | no | — |
| `V21` | 43 | `v21` | `v21` | `TEXT_STRUCTURE_TITLE_CIV19` | `2x2:##/.#` | no | `Neutral` | Desert | no | no | — |
| `V22` | 44 | `v22` | `v22` | `TEXT_STRUCTURE_TITLE_CIV20` | `1x2:##` | no | `Neutral` | Desert | no | no | — |
| `V23` | 45 | `v23` | `v23` | `TEXT_STRUCTURE_TITLE_CIV21` | `1x1:#` | no | `Neutral` | Desert | no | no | — |
| `V24` | 46 | `v24` | `v24` | `TEXT_STRUCTURE_TITLE_CIV22` | `2x2:../##` | no | `Neutral` | Desert | no | no | — |
| `V25` | 47 | `v25` | `v25` | `TEXT_STRUCTURE_TITLE_CIV1` | `2x2:.#/##` | no | `Neutral` | Desert | no | no | — |
| `V26` | 48 | `v26` | `v26` | `TEXT_STRUCTURE_TITLE_CIV23` | `1x2:##` | no | `Neutral` | Desert | no | no | — |
| `V27` | 49 | `v27` | `v27` | `TEXT_STRUCTURE_TITLE_CIV24` | `1x1:#` | no | `Neutral` | Desert | no | no | — |
| `V28` | 50 | `v28` | `v28` | `TEXT_STRUCTURE_TITLE_CIV25` | `1x1:#` | no | `Neutral` | Desert | no | no | — |
| `V29` | 51 | `v29` | `v29` | `TEXT_STRUCTURE_TITLE_CIV26` | `1x1:#` | no | `Neutral` | Desert | no | no | — |
| `V30` | 52 | `v30` | `v30` | `TEXT_STRUCTURE_TITLE_CIV27` | `1x2:##` | no | `Neutral` | Desert | no | no | — |
| `V31` | 53 | `v31` | `v31` | `TEXT_STRUCTURE_TITLE_CIV28` | `1x2:##` | no | `Neutral` | Desert | no | no | — |
| `V32` | 54 | `v32` | `v32` | `TEXT_STRUCTURE_TITLE_CIV29` | `1x2:##` | no | `Neutral` | Desert | no | no | — |
| `V33` | 55 | `v33` | `v33` | `TEXT_STRUCTURE_TITLE_CIV30` | `1x2:##` | no | `Neutral` | Desert | no | no | — |
| `V34` | 56 | `v34` | `v34` | `TEXT_STRUCTURE_TITLE_CIV31` | `1x1:#` | no | `Neutral` | Desert | no | no | — |
| `V35` | 57 | `v35` | `v35` | `TEXT_STRUCTURE_TITLE_CIV32` | `1x1:#` | no | `Neutral` | Desert | no | no | — |
| `V36` | 58 | `v36` | `v36` | `TEXT_STRUCTURE_TITLE_CIV33` | `1x1:#` | no | `Neutral` | Desert | no | no | — |
| `V37` | 59 | `v37` | `v37` | `TEXT_STRUCTURE_TITLE_CIV34` | `2x4:.###/.###` | no | `Neutral` | Desert | no | no | — |

## ID coverage

- Entries: **60**.
- IDs in source declaration order: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59.
- Missing IDs within 0–59: none.
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
