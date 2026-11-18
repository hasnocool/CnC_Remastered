# SmudgeType.cs — body-reviewed editor definition

> **Audit status: Body reviewed.** Complete 117-line source body inspected. Theatre registry data and Rust mapping remain open.

- Upstream: [CnCTDRAMapEditor/Model/SmudgeType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/SmudgeType.cs)
- Pinned source blob: `66e33734ac5614b487ca224165cb9bba464e83ef`
- Namespace/types: `MobiusEditor.Model.SmudgeTypeFlag`, `SmudgeType`
- Interfaces: `IBrowsableType`
- Imports: `MobiusEditor.Interface`, `MobiusEditor.Utility`, `System`, `System.Drawing`

## Exact flag values

`[Flags] SmudgeTypeFlag` declares:

| Flag | Value |
|---|---:|
| None | 0 |
| Bib | 1 |
| Bib1 | 3 |
| Bib2 | 5 |
| Bib3 | 9 |

Bib1/Bib2/Bib3 each include the low Bib bit, so a bit test against Bib identifies all three bib variants. The values are not mutually exclusive enum ordinals.

## Exact properties and constructors

| Property | Type | Access |
|---|---|---|
| ID | `sbyte` | public get/private set |
| Name | `string` | public get/private set |
| DisplayName | `string` | expression-bodied getter returning Name |
| Size | `Size` | public get/set; logical footprint |
| Flag | `SmudgeTypeFlag` | public get/private set |
| RenderSize | `Size` | public get/set; image footprint in map units |
| Thumbnail | `Image` | public get/set |

Primary constructor: `SmudgeType(sbyte id, string name, Size size, SmudgeTypeFlag flag)`. Forwarders supply None or default Size(1,1).

## Method bodies

- `Equals(object obj)`: same-type comparison is reference equality; boxed sbyte compares ID; string compares Name with OrdinalIgnoreCase; otherwise base.
- `GetHashCode()`: hashes ID.
- `ToString()`: returns Name.
- `Init(TheaterType theater)`: requests tile index 0 for Name. If found:
  - If `image.Width * Globals.TileHeight > image.Height * Globals.TileWidth`, fixes RenderSize.Width to TileWidth and scales height by `image.Height * TileWidth / image.Width`.
  - Otherwise fixes RenderSize.Height to TileHeight and scales width by `image.Width * TileHeight / image.Height`.
  - All calculations use integer division.
  - Thumbnail is a new bitmap at the original image dimensions, not the scaled RenderSize.
  If lookup fails, RenderSize and Thumbnail are not reset.

The logical Size is independent from RenderSize and is not changed by Init. No guard exists for zero image dimensions or zero tile dimensions; valid non-empty image assets are assumed.

## Dependencies and consumers

TD and RA `SmudgeTypes` catalogs construct definitions. `Smudge` is the placed per-cell record; `Map.AddBibs/RemoveBibs` uses the Bib flag and width-derived Bib1/Bib2/Bib3 types. `SmudgeTool` filters bib entries for ordinary placement, while the Map owns building bib-cell relationships.

## Rust port requirements

No typed SmudgeType catalog exists in `map-editor-model`. Preserve the distinct logical footprint, render footprint, source image dimensions, and flag bits. Bib membership must be a bit test, not equality with Bib1/Bib2/Bib3. Keep the renderer's aspect-fit math deterministic and test integer-rounding boundaries.

## Verification

- [x] Enum values, constructors, all method bodies and image scaling inspected.
- [x] Bib bit-mask semantics and independent Size/RenderSize recorded.
- [ ] TD/RA smudge catalogs and every Bib consumer.
- [ ] Golden aspect-ratio fixtures and Rust mapping.
- [ ] Runtime tests for missing/zero-size tile data.

No Rust code was changed and no test was executed.
