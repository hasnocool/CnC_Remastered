# TheaterType.cs — body-reviewed editor definition

> **Audit status: Body reviewed.** Complete 64-line source body inspected. Variant theater catalog ordering and resource mapping remain open.

- Upstream: [CnCTDRAMapEditor/Model/TheaterType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/TheaterType.cs)
- Pinned source blob: `7929dfd7e2a9d0900dda08c78b5054a15061b3d2`
- Namespace/type: `MobiusEditor.Model.TheaterType`
- Imports: `System`, `System.Collections.Generic`

## Exact members

| Property | Type | Access |
|---|---|---|
| ID | `sbyte` | public get/private set |
| Name | `string` | public get/private set |
| Tilesets | `IEnumerable<string>` | public get/private set |

Constructor: `TheaterType(sbyte id, string name, IEnumerable<string> tilesets)`. It assigns all three arguments directly. It does not materialize, copy, sort, or validate the tileset sequence; caller-owned mutable or lazy enumerables remain observable through the property.

## Method bodies

- `Equals(object obj)`: same-type branch uses `this == obj` (reference equality); boxed sbyte compares ID; string compares Name using OrdinalIgnoreCase; otherwise base.
- `GetHashCode()`: hashes ID.
- `ToString()`: returns Name.

No renderer, image, event, or serialization method exists in this class. The class is a lightweight identity plus ordered tileset-name source.

## Consumers and dependencies

TD and RA `TheaterTypes` catalogs supply different theater IDs/names/tileset sequences. BuildingType, OverlayType, SmudgeType, TemplateType and TerrainType carry TheaterType references or arrays. Their Init methods pass `theater.Tilesets` to the global tile manager. The MapSection converter uses theater names/IDs for the selected map theatre.

The registry's actual sequence order and every tileset lookup are not exhaustively audited in this tranche.

## Rust port requirements

No Rust TheaterType exists in `map-editor-model`. Represent the theater ID/name and an owned, deterministic tileset-name sequence. Decide whether duplicate names/order matter before normalizing. Avoid keeping borrowed/lazy iterators in persistent model state; use an owned catalog representation with explicit validation.

## Verification

- [x] Constructor and every method body inspected.
- [x] Reference-retained IEnumerable behavior and cross-type equality recorded.
- [ ] TD/RA theater IDs and exact ordered tileset lists.
- [ ] MapSection conversion and all catalog filter call sites.
- [ ] Rust mapping and tests.

No Rust code was changed and no test was executed.
