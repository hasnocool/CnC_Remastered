# UnitType.cs — body-reviewed editor definition

> **Audit status: Body reviewed.** Complete 138-line source body inspected. TD/RA registry tables and renderer parity remain open.

- Upstream: [CnCTDRAMapEditor/Model/UnitType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/UnitType.cs)
- Pinned source blob: `8096200fa282a796cfc69633cadcb759ff4a54b4`
- Namespace/types: `MobiusEditor.Model.UnitTypeIDMask`, `UnitType`
- Interfaces: `ICellOverlapper`, `ICellOccupier`, `ITechnoType`, `IBrowsableType`
- Imports: `MobiusEditor.Interface`, `MobiusEditor.Render`, `MobiusEditor.Utility`, `System`, `System.Drawing`

## Exact constants and properties

`UnitTypeIDMask` is static and declares public const sbyte masks:
- Aircraft = `1 << 5` = 32.
- Vessel = `1 << 6` = 64.

UnitType properties:

| Property | Type | Access / role |
|---|---|---|
| ID | `sbyte` | public get/private set |
| Name | `string` | public get/private set |
| DisplayName | `string` | public get/private set |
| OverlapBounds | `Rectangle` | computed fixed (-1,-1,3,3) |
| OccupyMask | `bool[,]` | computed new 1×1 true mask per access |
| OwnerHouse | `string` | public get/private set |
| HasTurret | `bool` | public get/private set |
| IsFixedWing | `bool` | public get/private set |
| IsUnit | `bool` | true when neither Aircraft nor Vessel bit is set |
| IsAircraft | `bool` | tests Aircraft bit |
| IsVessel | `bool` | tests Vessel bit |
| RenderSize | `Size` | public get/set |
| Thumbnail | `Image` | public get/set |

Constructors:
- Full: `UnitType(sbyte id, string name, string textId, string ownerHouse, bool hasTurret, bool isFixedWing)`.
- Forwarders omit IsFixedWing, owner/turret, owner only, or owner with turret.
All paths resolve DisplayName from `Globals.TheGameTextManager[textId]`.

## Method bodies

- `Equals(object obj)`: same-type branch uses reference equality; boxed sbyte compares ID; string compares Name with OrdinalIgnoreCase; otherwise base.
- `GetHashCode()`: hashes ID.
- `ToString()`: returns Name.
- `Init(GameType gameType, TheaterType theater, HouseType house, DirectionType direction)`:
  1. Looks up tile index 0 for Name in theater.Tilesets.
  2. On success, sets RenderSize to image dimensions divided by TileScale; on failure it leaves RenderSize unchanged.
  3. Creates a temporary Unit with this Type, supplied House/Direction, Strength=256.
  4. Allocates a bitmap three TileWidth by three TileHeight.
  5. Calls `MapRenderer.Render(gameType, theater, new Point(1, 1), Globals.TileSize, mockUnit)`, invokes the returned draw callback inside a Graphics using block, and assigns Thumbnail.

The Aircraft/Vessel bits classify the ID; IsFixedWing is separate metadata and is not derived from IsAircraft. The 1×1 occupancy mask and 3×3 overlap rectangle are intentionally different.

## Catalog and consumer relationships

TD and RA `UnitTypes` catalogs use the same ID mask constants. The sampled TD catalog assigns aircraft IDs with bit 5; the RA catalog assigns both aircraft bit-5 and vessel bit-6 IDs. Placed Unit instances refer to UnitType; UnitTool and ObjectProperties consume turret, owner, render and display metadata. Map uses ICellOccupier/ICellOverlapper independently of the rendered footprint.

## Rust port requirements

No typed Rust UnitType catalog exists in `map-editor-model`. Use a wider unsigned ID type or a validated newtype with explicit Aircraft/Vessel masks; do not let signed-byte conversions alter bit tests. Keep IsFixedWing independent. Preserve 1×1 occupancy versus fixed 3×3 overlap bounds. Thumbnail and RenderSize initialization belong to the rendering/catalog layer.

## Verification

- [x] Constants, all properties, constructor overloads and method bodies inspected.
- [x] ID masks, independent flags and occupancy/overlap distinction recorded.
- [ ] Full TD/RA UnitTypes tables and serializer ID parity.
- [ ] Renderer fixture comparison and Rust catalog mapping.
- [ ] Boundary tests for ID bit masks and missing tiles.

No Rust code was changed and no test was executed.
