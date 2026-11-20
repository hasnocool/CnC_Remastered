# Red Alert SmudgeTypes.cs — static catalog body audit

**Status:** Body reviewed. All 15 entries, static constructor and `GetTypes()` inspected at the pinned upstream commit. Rust mapping/tests remain open.

- Upstream path: `CnCTDRAMapEditor/RedAlert/SmudgeTypes.cs`
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source blob SHA: `ed2e73fc5313b89d5d9a042677a3c9593f18dbd9`
- [Exact source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/SmudgeTypes.cs)
- Source length: 57 lines.

## Catalog

`public static class SmudgeTypes` declares 15 public `static readonly SmudgeType` fields. Crater/scorch entries use default 1×1, None; bib entries explicitly set Size and Flag.

| Field | ID | Name | Logical size (W×H) | Flag |
| --- | --- | --- | --- | --- |
| `Crater1` | 0 | `cr1` | 1x1 | None |
| `Crater2` | 1 | `cr2` | 1x1 | None |
| `Crater3` | 2 | `cr3` | 1x1 | None |
| `Crater4` | 3 | `cr4` | 1x1 | None |
| `Crater5` | 4 | `cr5` | 1x1 | None |
| `Crater6` | 5 | `cr6` | 1x1 | None |
| `Scorch1` | 6 | `sc1` | 1x1 | None |
| `Scorch2` | 7 | `sc2` | 1x1 | None |
| `Scorch3` | 8 | `sc3` | 1x1 | None |
| `Scorch4` | 9 | `sc4` | 1x1 | None |
| `Scorch5` | 10 | `sc5` | 1x1 | None |
| `Scorch6` | 11 | `sc6` | 1x1 | None |
| `Bib1` | 12 | `bib1` | 4x2 | Bib1 |
| `Bib2` | 13 | `bib2` | 3x2 | Bib2 |
| `Bib3` | 14 | `bib3` | 2x2 | Bib3 |

## Static initialization and methods

1. Fields instantiate SmudgeType; DisplayName is Name, so the constructor does not call the text manager. Size and flag are retained.
2. The static constructor reflects public static fields, selects init-only SmudgeType values, and materializes an array. No sort, ID validation, null filter, or defensive copy.
3. `GetTypes()` returns the backing array through `IEnumerable<SmudgeType>`; callers may cast and mutate it. The private array is not readonly.

## Theatre initialization and consumers

`Map.InitTheater` initializes every smudge for the selected theatre. SmudgeType.Init requests tile 0; on success it computes RenderSize using image aspect ratio and global tile dimensions, then assigns Thumbnail. Failed lookup does not clear prior metadata.

MainForm's Smudge selector excludes entries with the Bib bit. Map's building-bib lifecycle selects Bib1/Bib2/Bib3 by exact equality; broad bib classification uses the shared Bib bit. TD and RA catalogs are identical.

## Dependencies and Rust mapping

Dependencies: SmudgeType/SmudgeTypeFlag, Size, tileset manager, Map.InitTheater, SmudgeTool and BuildingTool bib lifecycle.

**Rust mapping:** none. No typed Smudge catalog/bib model exists. Preserve logical Size separately from render metadata and flags None=0, Bib=1, Bib1=3, Bib2=5, Bib3=9. No Rust/render parity tests executed.
