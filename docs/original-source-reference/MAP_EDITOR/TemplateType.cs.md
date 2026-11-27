# TemplateType.cs — body-reviewed editor definition

> **Audit status: Body reviewed.** Complete 134-line source body inspected. The C# type body is Body reviewed; full registry/serializer parity remains open. Rust has partial engine-side template asset metadata, not a complete C# editor-type port.

- Upstream: [CnCTDRAMapEditor/Model/TemplateType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/TemplateType.cs)
- Pinned source blob: `f62e53665e9a45ddebba9e1f7d01ca61045fae64`
- Namespace/types: `MobiusEditor.Model.TemplateTypeFlag`, `TemplateType`
- Interface: `IBrowsableType`
- Imports: `MobiusEditor.Interface`, `MobiusEditor.Utility`, `System`, `System.Drawing`

## Exact flag values

`[Flags] TemplateTypeFlag`:

| Flag | Value |
|---|---:|
| None | 0 |
| Clear | 2 |
| Water | 4 |
| OreMine | 8 |

Bit zero is unused. Do not renumber these as sequential enum ordinals.

## Exact properties and constructor

- `ushort ID`, `string Name`, `int IconWidth`, `int IconHeight`, `TheaterType[] Theaters`, `TemplateTypeFlag Flag`: public get/private set.
- `Size IconSize`: computed from IconWidth × IconHeight.
- `int NumIcons`: IconWidth * IconHeight.
- `bool[,] IconMask`: public get/set.
- `Image Thumbnail`: public get/set.

Constructors are `TemplateType(ushort id, string name, int iconWidth, int iconHeight, TheaterType[] theaters, TemplateTypeFlag flag)` and a forwarding overload defaulting Flag to None. The constructor stores dimensions and metadata without validating positivity, overflow, or mask shape.

## Method bodies

- `Equals(object obj)`: same-type branch uses reference equality; boxed byte and boxed ushort compare ID; string compares Name with OrdinalIgnoreCase; otherwise base.
- `GetHashCode()`: hashes ID.
- `ToString()`: returns Name.
- `Init(TheaterType theater)`:
  1. Uses a square icon cell of `Globals.OriginalTileWidth / 4` pixels in each dimension.
  2. Chooses `iconSize = Math.Max(IconWidth, IconHeight)`; allocates a square-ish thumbnail of iconSize cells in both dimensions, leaving unused grid area transparent.
  3. Allocates `bool[IconWidth, IconHeight]` and clears it.
  4. Iterates y outer, x inner, incrementing icon from zero in row-major order.
  5. For each successful `GetTileData(theater.Tilesets, Name, icon, out tile)`, draws the tile into the fixed icon cell at (x,y), and writes `mask[x,y] = true`.
  6. Sets Thumbnail to the generated bitmap if any icon was found; otherwise sets it to null. Assigns IconMask in either case.

The array is indexed `[x,y]`, not the usual row-first `[y,x]`. Missing icons remain transparent and false in the mask. The temporary bitmap allocated when no icon is found is not disposed before Thumbnail is set to null. Zero/negative dimensions or a zero OriginalTileWidth/4 can fail at Bitmap construction; no validation is performed. The method does not itself enforce the Theaters array; callers/catalog filtering select eligible theatres.

## Consumers and dependencies

- TD and RA `TemplateTypes` catalogs supply IDs, icon dimensions, theatre sets, and flags.
- `Template` is the placed single-cell instance containing a TemplateType and icon index.
- `TemplateTool` iterates the icon grid, uses IconMask to decide which cells are valid, and draws the thumbnail/individual icons. `TerrainType` also consumes TemplateTypeFlag.OreMine when choosing its source tile name.

## Rust port requirements

`map-editor-model` still has no C#-equivalent typed `TemplateType`; `CellLayers` stores `template: Option<u16>` plus `template_icon: u8`. A partial engine-side mapping exists in `rts-core::template_catalog` (TD216 and RA400 ID/name/stem/theatre-mask records), and `renderer::template_assets` resolves ICON sets from caller-supplied MIX archives. It does not model C# icon dimensions, TemplateTypeFlag, `[x,y]` IconMask, Thumbnail initialization, reflection registry semantics or TemplateTool UI. RA metadata ID400 AntHill is missing even though the C# editor declares it; see the 6S-C-J guide. Port the remaining editor metadata separately, preserve the exact mask orientation and row-major logical icon mapping, and keep disposable thumbnail images outside the serializable map model.

## Verification

- [x] All properties, both constructors and every method body inspected.
- [x] Icon iteration order, mask orientation, transparent holes and no-result path recorded.
- [ ] Complete TD/RA template registry and all icon call sites.
- [ ] Tile fixture/mask parity and full C# editor catalog mapping (partial engine-side asset mapping exists).
- [ ] Zero-size and missing-icon runtime tests.

No Rust code was changed and no test was executed.


## Catalog slice dependency update — Tranche 6S-C-A

TD/RA TemplateTypes IDs0–31 are documented in [Variant-TemplateTypes-6S-C-A.md](Variant-TemplateTypes-6S-C-A.md). Catalog totals 216/401 and remain Partially reviewed. TD Water/Water2 lack Water flag; RA Water/Water2 carry it. Static construction/GetTypes inspected, catalog beyond ID31 open. TemplateTool/serialization traces are in comparison.

## Catalog and asset-pipeline reconciliation — Tranche 6S-C-J

- The RA `TemplateTypes` catalog has 401 IDs0–400; declaration slices through ID319 are now documented. IDs288–319 are 1×1, Interior-only, Flag=None.
- `Map.InitTheater` initializes every template; `TemplateType.Init` uses `TilesetManager.GetTileData` to set thumbnails and [x,y] masks. The `TemplateTool` selector excludes missing thumbnails, wrong-theatre entries and Clear.
- `Tileset.Load` keeps at most the first frame in its active preprocessor branch; the single-tile lookup discards FPS. `TilesetManager` uses `tilesets.Join(searchTilesets, ...)` with the dictionary as outer sequence, so searchTilesets is not explicitly the primary traversal order.
- RA serialization uses a UInt16 type plane plus icon plane; TD uses interleaved byte pairs. Rust raw decoders exist, but full writers/INI serializers do not. Both Rust atlas builders currently pass raw empty sentinels to catalog lookup; see [Serialization-Contract.md](Serialization-Contract.md).
- Rust asset metadata for RA IDs288–319 matches the C# keys and Interior mask=4. RA ID400 AntHill is not present in the Rust catalog; the pinned C++ header defines `FIXIT_ANTS`, but the enum and CData record are conditionally compiled. Build-macro parity remains unresolved.
- No Rust code or tests changed/executed. No retail tile assets or C# runtime initialization were validated.
