# Editor type catalog contracts

> **Audit status: cross-file synthesis of ten body-reviewed definition classes and eighteen body-reviewed TD/RA static registries (nine classes per variant).** TD/RA TemplateTypes remain Partially reviewed; declaration slices IDs0–127 are catalogued (128 entries each), while TD IDs0–215 are fully enumerated; RA IDs0–287 are enumerated. RA IDs288–400 and the complete consumer/initializer/serializer review remain open. Upstream baseline: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.

## Definition versus placed instance

The classes below describe catalog entries and renderer/placement metadata. They are not the placed map records. For example, BuildingType is referenced by Building, UnitType by Unit, InfantryType by Infantry/InfantryGroup, OverlayType by Overlay, SmudgeType by Smudge, and TemplateType by Template. Keep immutable game rules/catalog data separate from per-scenario mutable state.

## Definition matrix

| Definition | ID type | Implemented interfaces | Important contract |
|---|---|---|---|
| BuildingType | sbyte | ICellOverlapper, ICellOccupier, ITechnoType, IBrowsableType | Separate base/full masks; optional bib expands the full mask; fake name and factory/turret metadata |
| DirectionType | byte | none | ID, name and FacingType; cross-type equality also accepts FacingType |
| HouseType | sbyte | none | Unit/building team colors plus per-type override dictionary |
| InfantryType | sbyte | ITechnoType, IBrowsableType | Owner and localized display name; tile index 4 and center-slot preview |
| OverlayType | sbyte | ICellOccupier, IBrowsableType | Bit flags for resources, walls, crates and flags; one-cell mask |
| SmudgeType | sbyte | IBrowsableType | Logical Size, RenderSize and Bib flag are separate |
| TemplateType | ushort | IBrowsableType | Multi-icon dimensions, x/y IconMask, theatre list and template flags |
| TerrainType | sbyte | ICellOverlapper, ICellOccupier, IBrowsableType | Occupancy mask differs from render-derived overlap bounds |
| TheaterType | sbyte | none | ID/name plus caller-owned IEnumerable<string> tileset sequence |
| UnitType | sbyte | ICellOverlapper, ICellOccupier, ITechnoType, IBrowsableType | Aircraft/Vessel ID bits; 1×1 occupancy and fixed 3×3 overlap rectangle |

## Shared interface contracts

- [Type catalog interface contracts](TypeCatalog-Interfaces.md) records the exact declarations for IBrowsableType, ICellOccupier, ICellOverlapper and ITechnoType.
- IBrowsableType crosses into System.Drawing through Image Thumbnail; this is a UI/rendering concern.
- ICellOccupier exposes a two-dimensional bool mask. The interface itself does not define which dimension is x versus y; OccupierSet and each producer's array construction determine that contract.
- ICellOverlapper exposes a Rectangle independently from occupancy. TerrainType derives it from RenderSize; UnitType uses a fixed rectangle; BuildingType derives it from its full OccupyMask.
- ITechnoType standardizes sbyte ID and Name, but not display name, owner, render size, flags, or catalog identity.

## Identity and validation behavior

Most definition classes implement Equals with the same pattern: same-type comparison uses `this == obj` (reference equality unless an operator is supplied elsewhere), boxed ID values compare numerically, and strings compare Name using OrdinalIgnoreCase. GetHashCode generally hashes ID. DirectionType additionally compares to FacingType. TemplateType accepts both byte and ushort IDs. This is not a uniform stable-ID equality model; a Rust port should choose explicit typed IDs and document compatibility rather than inheriting C# reference semantics accidentally.

The constructors generally retain arrays/masks by reference and do not validate duplicate IDs, null strings, empty dimensions, mask consistency, or theatre membership. HouseType materializes OverrideTeamColors with ToDictionary using the default case-sensitive string comparer; duplicate keys throw during construction. Catalog validation should make these assumptions explicit.

## Flags and geometry that must not be normalized away

- OverlayTypeFlag values are None=0, TiberiumOrGold=1, Gems=2, Wall=4, Crate=8, Flag=16. IsPlaceable is true only when no bits other than Crate are set; None and Crate-only therefore pass.
- SmudgeTypeFlag values are None=0, Bib=1, Bib1=3, Bib2=5, Bib3=9. Bib is a shared bit; use a bit test.
- TemplateTypeFlag values are None=0, Clear=2, Water=4, OreMine=8; bit zero is unused.
- UnitTypeIDMask uses Aircraft=32 and Vessel=64 in the signed sbyte ID. TD's sampled UnitTypes catalog uses aircraft IDs; RA's sampled catalog uses both aircraft and vessel IDs.
- BuildingType's bib mask adds one row and makes the bottom two rows full-width, while BaseOccupyMask remains the original array.
- TerrainType's OccupyMask controls occupancy, while OverlapBounds is computed from rendered pixel dimensions with ceiling division.
- UnitType has a 1×1 OccupyMask but a fixed (-1,-1,3,3) OverlapBounds.
- TemplateType.IconMask is allocated as [IconWidth, IconHeight] and written as [x,y] while iteration is y-outer/x-inner. This differs from the common row-first convention and must be tested explicitly.

## Initialization and image pipeline

BuildingType, InfantryType and UnitType create mock placed objects and call MapRenderer for thumbnails. OverlayType, SmudgeType, TemplateType and TerrainType query Globals.TheTilesetManager with theatre tilesets and type-specific tile indexes. TheaterType supplies the tileset sequence. Constructors for several types resolve DisplayName immediately through Globals.TheGameTextManager, so static catalog initialization has an ordering dependency on global text-manager availability.

Missing tile lookups do not have uniform reset behavior: BuildingType/OverlayType/SmudgeType/TerrainType can retain an earlier thumbnail or render size; TemplateType explicitly sets Thumbnail to null when no icon was found; InfantryType and UnitType still create a preview bitmap after their optional RenderSize lookup. The Rust model should keep deterministic catalog data separate from disposable renderer resources and define explicit missing-asset results.

## Variant registry provenance sampled

The pinned source tree contains parallel TD and RA registry files: BuildingTypes, HouseTypes, InfantryTypes, OverlayTypes, SmudgeTypes, TemplateTypes, TerrainTypes, TheaterTypes, UnitTypes and DirectionTypes. The inspected BuildingTypes/HouseTypes/InfantryTypes/UnitTypes declarations demonstrate that IDs, names, owners, masks, theatre lists and available unit classes differ by variant. In particular, TD and RA use different building footprints and house identifiers; RA UnitTypes includes vessel IDs while TD's sampled UnitTypes list includes aircraft IDs.

This is evidence that registry data must remain variant-specific. Tranche 6R body-reviewed DirectionTypes, HouseTypes, InfantryTypes and UnitTypes for both variants; BuildingTypes, OverlayTypes, SmudgeTypes, TemplateTypes, TerrainTypes and TheaterTypes remain open in both variants. Exact catalog entries and initialization behavior for the reviewed files are summarized in Variant-Registry-Catalogs.md.

## Variant-specific static registries — Tranche 6R

Eight registry bodies are now reviewed: DirectionTypes, HouseTypes, InfantryTypes and UnitTypes for TD and RA. See [Variant-Registry-Catalogs.md](Variant-Registry-Catalogs.md) for comparison and [per-file tables](README.md).

- Both direction catalogs have 16 byte IDs in 16-step increments, with FacingType only for canonical directions.
- TD/RA catalog sizes: houses 10/20, infantry 20/26, units 27/36.
- TD BadGuy has BAD_UNIT/BAD_STRUCTURE plus harv/mcv overrides; its base-player helper maps only BadGuy to GoodGuy. RA maps USSR, Ukraine and BadGuy to GoodGuy.
- TD infantry field E7 is ID 5 with Name e6; RA E8/E9 use spy/thf tokens.
- Aircraft/Vessel masks are 32/64. TD has no vessel records.
- Catalog arrays are reflection-collected without explicit sorting; GetTypes returns backing arrays. InfantryType/UnitType constructors eagerly resolve text through Globals.TheGameTextManager.

TemplateTypes remains Partially reviewed in both variants. The full registries contain 216 TD and 401 RA entries; declaration slices IDs0–159 are documented, but remaining declarations and full initializer/consumer/serializer parity are open. No typed Rust editor catalog or parity tests exist.

## Rust implementation comparison

The current `crates/map-editor-model` tree contains `MapDocument`, `CellLayers`, `CellMetrics` and `object_selection`. CellLayers stores optional numeric IDs for template, terrain, overlay and smudge; object_selection defines generic EditorObjectKind values. There is no typed definition catalog for the ten classes above, no house/theatre/direction catalog, no thumbnail/tileset adapter, and no mask-backed type registry. Consequently none of these ten C# files is marked Rust-mapped.

## Required Rust decomposition

1. Add variant-keyed immutable catalogs with validated stable IDs and names.
2. Represent occupancy masks, overlap rectangles and logical/render sizes as separate types.
3. Keep theatre availability and ordered tileset references explicit.
4. Use typed bitflags with the exact upstream numeric values.
5. Model template icon masks with an explicit coordinate convention and row-major icon ID mapping.
6. Represent house color overrides with an explicit case policy and duplicate-key errors.
7. Separate asset lookup outcomes from definition construction; make missing assets deterministic.
8. Keep image/bitmap handles in renderer-facing caches, not serialized map documents.
9. Add TD/RA catalog fixtures for IDs, masks, text keys, theatre lists, and unit classification.

## Verification boundary

The ten definition source bodies and four small interface declarations were inspected at the pinned commit. Eighteen TD/RA registry bodies (DirectionTypes, HouseTypes, InfantryTypes, UnitTypes, BuildingTypes, OverlayTypes, SmudgeTypes, TerrainTypes and TheaterTypes for both variants) were inspected. TemplateTypes remains Partially reviewed for both variants; declaration slices IDs0–127 are documented, while remaining declarations and complete consumer/initializer/serializer review are open. No Rust code changed and no Rust test, C# runtime test, image comparison, or serializer parity test was executed.


## Static registry review update — Tranche 6S-A

TD and RA BuildingTypes, OverlayTypes and SmudgeTypes source bodies are fully documented. Counts: buildings 60/63, overlays 27/25, smudges 15/15. See [Variant-Static-Catalogs-6S-A.md](Variant-Static-Catalogs-6S-A.md) and six per-variant guides.

- Building catalog entries include IDs, field names, tokens/effective fake names, text keys, masks, bib, owner, theatre, fake/turret and factory overlay metadata.
- Overlay catalogs include IDs, default/explicit display keys, flags and theatre restrictions.
- Smudge catalogs are identical and include logical sizes and bib flags.
- Registry initialization, Map.InitTheater and MainForm consumer filters were traced.
- No typed Rust catalog, tests, or parity results exist. TerrainTypes and TheaterTypes were Body reviewed; TemplateTypes are Partially reviewed for IDs0–31, full catalogs remain open.


## Static terrain and theatre registries — Tranche 6S-B

Four registry bodies per variant are Body reviewed: TerrainTypes and TheaterTypes. See [Variant-Terrain-Theater-Catalogs-6S-B.md](Variant-Terrain-Theater-Catalogs-6S-B.md) and four per-file guides.

- TD/RA terrain catalogs contain 32/35 entries with exact masks, theatres and flags; TD has two transformable split types; RA has five Water ice entries, nine Interior boxes and OreMine Mine.
- Theatre sequences put terrain key first, then common tilesets. TD Temperate/Winter share ID2; source anomaly retained.
- Map.InitTheater initializes every TerrainType before UI filtering. MainForm/TerrainTool filter by theatre; terrain selector doesn't require thumbnails. False tile lookup leaves metadata unchanged.
- Body-reviewed only; not Rust-mapped/Tested/Complete

## TemplateTypes — Tranche 6S-C-A partial slice

- TD/RA IDs0–31 documented in batch guides; static constructors/GetTypes inspected. Totals 216/401; remaining entries are not body-reviewed.
- TD Water/Water2 lack Water flag; RA Water/Water2 carry it. RA shore IDs3–31 are Temperate/Snow; TD shore IDs3–12 Winter/Temperate and slopes13–31 all theatres.
- TemplateType.Init generates thumbnail/mask from successful tile lookup; TemplateTool filters thumbnails, current theatre and Clear; whole placement respects mask, removal clears full footprint.
- TD binary templates use byte type+icon pairs; RA MapPack stores ushort type array then icon bytes. This is call-site tracing, not full plugin body audit.
- Both registries remain Partially reviewed, not Rust-mapped/Tested/Complete.

## TemplateTypes — Tranche 6S-C-C

- IDs64–95 documented in [comparison](Variant-TemplateTypes-6S-C-C.md) and per-variant guides; cumulative declarations IDs0–95 (96 each), total TD216/RA401.
- TD brush/patch/shore/boulder/road families have varying theatre whitelists. RA continues ShoreCliff06–37 with `wc` tokens. All entries default to `TemplateTypeFlag.None`.
- Catalog declarations only. Full behavior/dependency review, Rust mapping and tests remain open.


## TemplateTypes — Tranche 6S-C-D (partial declaration slice)

TD and RA IDs96–127 are documented in [Variant-TemplateTypes-6S-C-D.md](Variant-TemplateTypes-6S-C-D.md) and the two per-variant guides. Cumulative declaration coverage is IDs0–127 (128/216 TD and 128/401 RA); both inventory rows remain Partially reviewed.

- TD IDs96–127 are Road4–Road35 with tokens d04–d35, all available in Winter/Desert/Temperate.
- RA IDs96–127 cover ShoreCliff38, Boulder1–6, selected patches, River01–13 and Falls1/1a/2, all Temperate/Snow.
- RA Boulder4–6 (IDs100–102) and Patch15 (ID111) declare 0×0 footprints. The pinned source call path iterates all template types in Map.InitTheater and calls TemplateType.Init; Init constructs a Bitmap from max(width,height) without a positive-dimension guard. The zero-sized Bitmap request is source-confirmed; runtime exception/reachability testing remains outstanding.
- The Rust Red Alert TemplateClass lifecycle module is a runtime object implementation, not a mapping of the C# editor's static TemplateType catalog. CellLayers.template remains Option<u16>; no typed editor catalog parity exists.
- No Rust code or tests changed/executed. Do not advance either inventory row beyond Partially reviewed.


## TemplateTypes — Tranche 6S-C-E (partial declaration slice)

TD and RA IDs128–159 are documented in [Variant-TemplateTypes-6S-C-E.md](Variant-TemplateTypes-6S-C-E.md) and the per-variant guides. Cumulative declaration coverage is IDs0–159 (160/216 TD and 160/401 RA); both inventory rows remain Partially reviewed.

- TD IDs128–135 are Road36–Road43, all three theatres; IDs136–148 are River1–13, Winter/Temperate; IDs149–159 are River14–24, Desert-only.
- RA IDs128–134 cover Falls2a, fords and bridges; IDs135–159 cover Slope01–Slope25, all Temperate/Snow.
- Every declaration uses the five-argument constructor, verified to delegate with TemplateTypeFlag.None. This slice has no zero-sized dimensions.
- The shared theatre initialization call path was re-fetched; this remains declaration coverage, not a complete GamePlugin body or serializer review.
- Rust's numeric CellLayers.template: Option<u16> is not a typed editor catalog mapping; the runtime Red Alert TemplateClass module is a different responsibility.
- No Rust code or tests changed/executed. Next declaration range: IDs160–191.


## TemplateTypes — Tranche 6S-C-F

TD and RA IDs160–191 are documented in [Variant-TemplateTypes-6S-C-F.md](Variant-TemplateTypes-6S-C-F.md) and per-variant guides. Cumulative coverage is IDs0–191 (192/216 TD and 192/401 RA); both inventory rows remain Partially reviewed.

- TD: River25, Ford1/2, Falls1/2, theatre-specific bridge variants, Desert Shore24–31 and Shore36–39, Winter Patch16–20, Temperate/Winter Shore34/35.
- RA: Slope26–38 then Road01–19, all Temperate/Snow. Slope27 is 3×2; roads range 1×2 through 4×3.
- All 64 use the five-argument constructor delegating with TemplateTypeFlag.None. IDs are variant-local: TD ID160 River25 differs from RA ID160 Slope26.
- This slice has no 0×0 dimensions; earlier RA IDs100–102 and 111 remain an untested runtime concern because Init has no positive-dimension guard.
- CellLayers.template remains Option<u16>; runtime TemplateClass is not the editor catalog. No Rust code/tests changed or executed.
- Remaining declarations: TD IDs192–215 (24), RA IDs192–400 (209). Keep both paths Partially reviewed.


## TemplateTypes — Tranche 6S-C-G

TD IDs192–215 and RA IDs192–223 are documented in [Variant-TemplateTypes-6S-C-G.md](Variant-TemplateTypes-6S-C-G.md) and per-variant guides. TD declaration coverage is complete (216/216); RA coverage is 224/401. Both rows remain Partially reviewed.

- TD: Shore40–Shore63, keys sh40–sh63, Desert-only; footprints range from 1×2 to 6×1.
- RA: Road20–Road43 then Rough01–Rough08, keys d20–d43/rf01–rf08, Temperate/Snow; Road21 is 3×2, Roads26–33 and 36–43 are 2×2, Rough01–07 are 1×1 and Rough08 is 1×2.
- All 56 declarations use the five-argument constructor delegating to TemplateTypeFlag.None. No zero-sized dimensions occur in this slice.
- Shared Init creates a max-dimension square bitmap and width×height icon mask, setting cells only on successful theatre tile lookup. Map.InitTheater initializes every template. No positive-dimension guard exists; earlier RA IDs100–102 and 111 remain an untested source hazard.
- Reflection registry construction uses public static readonly TemplateType fields and ToArray without explicit sorting; GetTypes exposes the backing array as IEnumerable.
- Rust CellLayers.template remains Option<u16>; runtime TemplateClass is not the editor registry. No Rust mapping/tests.
- Next RA declaration slice: IDs224–255. TD has no remaining declarations. Full dependency/consumer/serializer audit remains open.

## Latest declaration tranche — 6S-C-H

- RA IDs224–255: Rough09–11, Road44–45, River14–15, RiverCliff01–04, Bridge1a–3f, F01–F06 and Interior-only ARRO0001–0003. All use the five-argument constructor and therefore `TemplateTypeFlag.None`.
- IDs253–255 are 1×1 and explicitly restricted to `TheaterTypes.Interior`; the preceding 29 declarations are Temperate/Snow.
- A declaration does not prove a key resolves to a tile. `TemplateType.Init` performs theatre-specific lookup and mask population.
- Coverage: TD216/216; RA256/401. Both registry paths remain Partially reviewed pending complete implementation/dependency/consumer/serialization review. Rust mapping and tests remain absent.


## Latest declaration tranche — Red Alert IDs256–287

The RA registry adds ARRO0004–0015, FLOR0001–0007, GFLR0001–0005 and GSTR0001–0008 at source lines280–311. All have 1×1 footprints, explicit Interior-only theatre lists and Flag=None. This is catalogue metadata, not evidence of tile payload availability or placement/rendering behavior. Cumulative RA declaration coverage is 288/401; the RA and TD TemplateTypes sources remain Partially reviewed pending full initializer, consumer and GamePlugin serialization audits. See [6S-C-I](RedAlert/TemplateTypes-6S-C-I.md).
