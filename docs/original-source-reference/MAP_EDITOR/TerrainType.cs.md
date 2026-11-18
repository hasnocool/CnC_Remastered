# TerrainType.cs — body-reviewed editor definition

> **Audit status: Body reviewed.** Complete 118-line source body inspected. TD/RA registry data and runtime renderer parity remain open.

- Upstream: [CnCTDRAMapEditor/Model/TerrainType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/TerrainType.cs)
- Pinned source blob: `a07540aec42445303da88092dd8798b9dfbdbed0`
- Namespace/type: `MobiusEditor.Model.TerrainType`
- Interfaces: `ICellOverlapper`, `ICellOccupier`, `IBrowsableType`
- Imports: `MobiusEditor.Interface`, `MobiusEditor.Utility`, `System`, `System.Drawing`

## Exact properties and flags

- `sbyte ID`, `string Name`, `TheaterType[] Theaters`, `bool IsTransformable`, `TemplateTypeFlag TemplateType`: public get/private set.
- `string DisplayName`: computed getter returning Name.
- `Rectangle OverlapBounds`: origin Point.Empty; width is ceil(RenderSize.Width / TileWidth), height is ceil(RenderSize.Height / TileHeight), implemented with integer `(value + tile - 1) / tile`.
- `bool[,] OccupyMask`: public get/private set.
- `Size Size`: computed from OccupyMask columns/rows.
- `Size RenderSize`, `Image Thumbnail`: public get/set.

Primary constructor:
`TerrainType(sbyte id, string name, TheaterType[] theaters, bool[,] occupyMask, bool isTransformable, TemplateTypeFlag templateType)`.
Three forwarding overloads default either TemplateTypeFlag.None, IsTransformable=false, or both. The mask and theatre array are stored by reference; no defensive copy or dimension validation is made.

## Method bodies

- `Equals(object obj)`: same-type comparison is reference equality; boxed sbyte compares ID; string compares Name using OrdinalIgnoreCase; otherwise base.
- `GetHashCode()`: hashes ID.
- `ToString()`: returns Name.
- `Init(TheaterType theater)`:
  1. Starts with tileName=Name.
  2. If the TemplateType OreMine bit is set, uses the literal tile name `OREMINE`.
  3. Requests tile index 22 for transformable terrain; otherwise index 0.
  4. On success, sets RenderSize to source image dimensions divided by Globals.TileScale and creates Thumbnail at half source width/height.
  5. On failure, leaves RenderSize/Thumbnail unchanged.

OverlapBounds depends on RenderSize, while placement occupancy depends on OccupyMask. These are deliberately separate footprints. A missing tile on a repeated Init can retain an old preview/size.

## Consumers and dependencies

TD and RA `TerrainTypes` catalogs provide masks, theatre eligibility, transformable flags and TemplateTypeFlag values. `TerrainTool` uses OccupyMask for collision/placement and the tile-22 convention for transformable terrain. `Map` indexes Terrain in Technos and uses overlap geometry for redraw/overlap queries. Ore-mine terrain reuses the OREMINE tile set.

## Rust port requirements

No typed TerrainType catalog exists in `map-editor-model`; the current layer is only `Option<u16>`. Preserve the distinction among logical occupancy mask, render pixel size, and overlap-cell rectangle. Use checked ceiling division and reject zero tile dimensions explicitly. Keep tile lookup and thumbnail generation in a renderer/catalog adapter.

## Verification

- [x] All properties, four constructor overloads and every method body inspected.
- [x] OreMine name substitution, transformable tile index and footprint split recorded.
- [ ] Complete TD/RA terrain catalog and tool/map call-site audit.
- [ ] Golden occupancy/overlap and renderer fixture tests.
- [ ] Rust catalog mapping and missing-tile tests.

No Rust code was changed and no test was executed.
