# Editor type catalog contracts

> **Audit status: cross-file synthesis of ten body-reviewed definition classes.** This page records the model-type layer only. The TD/RA static registry files are sampled for construction provenance but are not marked body-reviewed here. Upstream baseline: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.

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

This is evidence that the registry data must remain variant-specific. It is not an exhaustive review of the static catalogs: ordering, every entry, every text key, theatre filtering, Init invocation order and serializer call sites remain open. Those registry files are the next audit batch.

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

The ten definition source bodies and four small interface declarations were inspected at the pinned commit. Four TD/RA registry files were sampled for constructor examples, not exhaustively audited. No Rust code changed and no Rust test, C# runtime test, image comparison, or serializer parity test was executed.
