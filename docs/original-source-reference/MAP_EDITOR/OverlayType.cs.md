# OverlayType.cs — body-reviewed editor definition

> **Audit status: Body reviewed.** Complete 129-line source body inspected. Catalog data and runtime rendering parity remain open.

- Upstream: [CnCTDRAMapEditor/Model/OverlayType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OverlayType.cs)
- Pinned source blob: `d83c6f8e8f3c2203bc7bdb5c999015a280f44029`
- Namespace/type: `MobiusEditor.Model.OverlayTypeFlag`, `OverlayType`
- Interfaces: `ICellOccupier`, `IBrowsableType`
- Imports: `MobiusEditor.Interface`, `MobiusEditor.Utility`, `System`, `System.Drawing`

## Exact flag values

`[Flags] OverlayTypeFlag`:

| Flag | Value |
|---|---:|
| None | 0 |
| TiberiumOrGold | 1 |
| Gems | 2 |
| Wall | 4 |
| Crate | 8 |
| Flag | 16 |

## Exact properties

- `sbyte ID`, `string Name`, `string DisplayName`, `TheaterType[] Theaters`, `OverlayTypeFlag Flag`: public get/private set.
- `Image Thumbnail`: public get/set.
- `bool[,] OccupyMask`: computed; each access allocates a new 1×1 array containing true.
- Computed predicates: IsResource (TiberiumOrGold or Gems bit), IsTiberiumOrGold, IsGem, IsWall, IsCrate, IsFlag.
- `IsPlaceable`: `(Flag & ~OverlayTypeFlag.Crate) == OverlayTypeFlag.None`. Thus None and Crate-only are placeable; any additional category bit makes the value non-placeable.

## Constructors

Primary: `OverlayType(sbyte id, string name, string textId, TheaterType[] theaters, OverlayTypeFlag flag)`. It stores all fields and resolves DisplayName through `Globals.TheGameTextManager[textId]`.

Forwarders:
- `(sbyte id, string name, string textId, OverlayTypeFlag flag)` supplies null Theaters.
- `(sbyte id, string name, string textId, TheaterType[] theaters)` supplies None.
- `(sbyte id, string name, OverlayTypeFlag flag)` uses name as textId and null Theaters.
- `(sbyte id, string name, string textId)` supplies null Theaters and None.

## Method bodies

- `Equals(object obj)`: same-type branch is reference equality; boxed sbyte compares ID; string compares Name with OrdinalIgnoreCase; otherwise base.
- `GetHashCode()`: hashes ID.
- `ToString()`: returns Name.
- `Init(TheaterType theater)`: looks up tile index 0 from `theater.Tilesets` using Name. On success, assigns Thumbnail to a new Bitmap at the source image's exact dimensions. On failure it leaves Thumbnail unchanged (including a previously initialized image).

## Dependencies and consumers

TD and RA `OverlayTypes` catalogs construct the definitions and supply category flags/theatre availability. `Overlay` is the placed instance; `Map` uses overlay flags for resource totals, wall registration/connectivity, crate and placement filtering. `OverlaysTool`, `ResourcesTool`, `WallsTool`, and `SmudgeTool` consume these metadata.

## Rust port requirements

No typed OverlayType catalog exists in `map-editor-model`. Represent the bit flags with a Rust bitflag/newtype or a validated enum set; preserve the exact bit values and the unusual IsPlaceable rule. Avoid allocating an occupancy array per query: a static 1×1 mask or a direct single-cell occupancy operation is semantically equivalent if tested. Keep theatre eligibility and thumbnail loading outside the core map state.

## Verification

- [x] Enum values, computed predicates, constructors and all method bodies inspected.
- [x] Per-access mask allocation and failed-lookup thumbnail behavior recorded.
- [ ] Full TD/RA overlay catalog and caller audit.
- [ ] Resource adjacency/wall/crate golden fixtures.
- [ ] Rust mapping and tests.

No Rust code was changed and no test was executed.
