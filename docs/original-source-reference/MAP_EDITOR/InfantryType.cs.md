# InfantryType.cs — body-reviewed editor definition

> **Audit status: Body reviewed.** Complete 101-line source body inspected. Variant registry, renderer behavior and Rust mapping remain open.

- Upstream: [CnCTDRAMapEditor/Model/InfantryType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/InfantryType.cs)
- Pinned source blob: `c647e4764d7bac97f56a1b75d9e8b8b9e7830a51`
- Namespace/type: `MobiusEditor.Model.InfantryType`
- Interfaces: `ITechnoType`, `IBrowsableType`
- Imports: `MobiusEditor.Interface`, `MobiusEditor.Render`, `MobiusEditor.Utility`, `System`, `System.Drawing`

## Exact members

| Property | Type | Access |
|---|---|---|
| ID | `sbyte` | public get, private set |
| Name | `string` | public get, private set |
| DisplayName | `string` | public get, private set |
| OwnerHouse | `string` | public get, private set |
| RenderSize | `Size` | public get/set |
| Thumbnail | `Image` | public get/set |

Constructors:

- `InfantryType(sbyte id, string name, string textId, string ownerHouse)` stores ID/name/owner and resolves DisplayName immediately from `Globals.TheGameTextManager[textId]`.
- `InfantryType(sbyte id, string name, string textId)` delegates with null OwnerHouse.

## Method bodies

- `Equals(object obj)`: same-type comparison is reference equality via `this == obj`; boxed sbyte compares ID; string compares Name with OrdinalIgnoreCase; other values use base.
- `GetHashCode()`: hashes ID.
- `ToString()`: returns Name.
- `Init(GameType gameType, TheaterType theater, HouseType house, DirectionType direction)`:
  1. Calls `Globals.TheTilesetManager.GetTileData(theater.Tilesets, Name, 4, out Tile tile)`.
  2. Only when lookup succeeds, sets RenderSize to tile image width/height divided by Globals.TileScale.
  3. Constructs `new Infantry(null)` and assigns Type=this, House, Strength=256, and Direction.
  4. Allocates a bitmap exactly Globals.TileWidth × Globals.TileHeight.
  5. Creates Graphics in a using block and invokes `MapRenderer.Render(theater, Point.Empty, Globals.TileSize, mockInfantry, InfantryStoppingType.Center).Item2(g)`.
  6. Assigns Thumbnail unconditionally after rendering.

The `gameType` parameter is not read by this method. If tile lookup fails, RenderSize is left at its previous/default value, but the thumbnail rendering path still runs. The lookup uses tile index 4; this is a specific infantry-art convention, not index zero.

## Dependencies and callers

- TD and RA `InfantryTypes` catalogs construct variant-specific definitions.
- `Infantry` is the placed object; the temporary instance supplies type/house/strength/direction to MapRenderer.
- `TheaterType.Tilesets`, the tileset manager, global tile dimensions/scale, game text manager, and MapRenderer are direct dependencies.
- ObjectProperties and InfantryTool consume the definition for ownership, display, placement and preview. Complete serializer and registry call-site coverage remains pending.

## Rust port requirements

No typed Rust InfantryType or thumbnail catalog exists in `map-editor-model`; `CellLayers` stores only numeric IDs. Separate variant-specific definition data from placed Infantry/InfantryGroup state. Preserve the index-4 tile lookup and center stopping-slot render path in the renderer adapter, but keep image objects out of the deterministic map model. Treat the currently unused GameType parameter as an observed source fact, not as permission to remove it without checking interface/call compatibility.

## Verification

- [x] Both constructors and every method body inspected.
- [x] Tile index, render size and thumbnail behavior recorded.
- [ ] Full TD/RA InfantryTypes catalog and text-key coverage.
- [ ] Renderer fixture comparison and Rust catalog mapping.
- [ ] Missing-tile and repeated-Init runtime tests.

No Rust code was changed and no test was executed.
