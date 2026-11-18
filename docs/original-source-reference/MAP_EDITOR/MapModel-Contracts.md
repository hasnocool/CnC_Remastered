# Map model contracts — editor spatial graph and derived state

> This is a cross-file synthesis of pinned source bodies, not a substitute for each per-file guide. Upstream baseline: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.

## Model graph

- Map owns MapSection, BriefingSection, SteamSection, catalogs, typed cell grids, occupancy sets, overlapper index, waypoints, triggers, team types, and houses.
- CellMetrics defines the shared row-major coordinate space.
- CellGrid<T> stores one nullable value per cell and emits synchronous CellChanged/Cleared events.
- OccupierSet<T> stores object identity plus a per-cell occupancy mask.
- OverlapperSet<T> stores each overlapper's absolute rectangle and computes transitive overlap closure.
- ICellOccupier supplies bool[,] OccupyMask; ICellOverlapper supplies Rectangle OverlapBounds.
- Building, TerrainType, and InfantryGroup implement both interfaces. Template and OverlayType implement ICellOccupier. The exact model type determines which index is used.

## Coordinate and indexing rules

Flattened IDs are row-major: cell = y * width + x. CellMetrics maps all eight compass directions; FacingType.None leaves the coordinate unchanged. CellGrid's flattened indexer performs modulo/division directly and relies on valid width/index. CellMetrics constructors accept raw integer dimensions without validation. The Rust CellMetrics delegates to MapDimensions validation and returns Option for conversions.

CellGrid.CopyTo checks only total Length, not width and height separately. Same-area grids with different shapes therefore copy by flattened order. This is observable source behavior and should either be preserved intentionally or replaced with a documented stricter Rust invariant.

## Occupancy rules

OccupierSet builds mask points by adding non-negative (x,y) offsets to an anchor. CanAdd and Add reject null/duplicate objects, out-of-bounds mask points, and occupied cells. Add commits dictionary and cell-array state before raising OccupierAdded. Remove clears the object's mask cells and then raises OccupierRemoved. Clear empties both structures and emits one Cleared event, not one removal per object.

The Buildings set and Technos set are not redundant. Map registers a Building in Buildings using its full OccupyMask, then registers it in Technos using BaseOccupyMask and generates bib smudges separately. Wall Overlay objects are also inserted into Buildings; the Buildings event bridge inserts those walls into Technos. A Rust port that collapses these indexes into one generic cell layer will lose collision and rendering semantics.

OverlapperSet is separate from occupancy. It does not reject overlapping objects and does not clip rectangles to map bounds. Its query repeatedly adds every stored rectangle intersecting the current rectangle set until no new rectangle appears, then returns every point in the closure. This is a transitive expansion, not a simple query against the original rectangle only.

## Map update batching and invalidation

BeginUpdate increments a nesting counter. EndUpdate decrements it and calls Update only when the count reaches zero. No underflow guard exists. Overlay cell changes synchronize wall occupancy immediately, then—unless Update is already running—mark Resources or Walls dirty over the edited cell's 3x3 neighborhood. When not inside a batch, Update runs immediately.

Technos add/remove events update Overlappers immediately outside a batch; during a batch they set one rebuild flag. Update recomputes resource icons, wall connection icons, and optionally rebuilds the overlapper index. It then clears invalidations. Update sets an updating flag without try/finally, so an exception can leave the aggregate in a permanently suppressed state.

## Derived icons and bib ownership

Resource icons use eight-neighbor resource counts. Tiberium/gold map neighbor counts 0..8 to [0,1,3,4,6,7,8,10,11]; gems map them to [0,0,0,1,1,1,2,2,2]. Wall icons use four orthogonal neighbors of the same overlay type and bit values N=1, E=2, S=4, W=8.

Building bibs are represented as Smudge cells and tracked by Building.BibCells. Building widths 2, 3, and 4 select Bib3, Bib2, and Bib1 respectively. Removing bibs deletes only Smudge entries whose type flag includes Bib, then clears the ownership set.

## Clone semantics

Map.Clone creates a new aggregate but shares BasicSection, catalog objects, triggers, team types, houses, ordinary occupiers, buildings, and infantry objects. It creates new InfantryGroup containers but shallow-copies their Infantry array. It copies cell grids and section properties, and re-adds Buildings so event handlers rebuild Technos and bibs. It does not copy TiberiumOrGoldValue or GemValue. This is a mixed shallow/deep clone and should not be assumed to provide isolation.

## Current Rust coverage

- crates/map-editor-model/src/lib.rs: MapDocument with GameVariant, MapDimensions, Vec<CellLayers>, and waypoint BTreeMap. It has four source-level unit tests for cell storage, bounds rejection, variant dimensions, and unique waypoint numbers.
- crates/map-editor-model/src/cell_metrics.rs: concrete partial geometry mapping with four source-level tests for row-major conversion, adjacency, edges, and clipping.
- crates/map-editor-model/src/object_selection.rs: object selection and popup-control plans; it is not a typed editor map aggregate or occupancy index.

These tests are present in source, but were not executed during this audit run. No Rust-mapped, Tested, or Complete status is claimed for the C# Map, CellGrid, OccupierSet, or OverlapperSet files.

## Required Rust decomposition

1. Keep a validated MapDocument and variant-specific serialized sections separate from UI state.
2. Add typed per-cell template/overlay/smudge/terrain records and theatre catalog IDs.
3. Add stable object IDs and a mask-based occupancy index with explicit add/remove/clear events or commands.
4. Add a separate overlap rectangle index and deterministic transitive closure queries.
5. Represent batch updates as a safe scope/transaction that cannot underflow and always restores derived-state flags on error.
6. Recompute resource/wall icons incrementally with deterministic golden fixtures.
7. Model Building base occupancy, bib occupancy, and wall registration as explicit relationships.
8. Define clone/snapshot semantics intentionally; do not inherit the mixed C# aliasing behavior accidentally.
9. Keep preview rendering and image sharpening outside the serialization/document core.

## Audit boundary

The listed source bodies were inspected at the pinned commit. Cross-file call-site and serializer audits are not exhaustive, and no runtime C# or Rust test was executed. Defects described here are source-derived risks; reproduction in the original editor remains a separate validation step.


## Type-definition and catalog layer

The placed map records depend on a separate set of definition objects. The ten source-body guides are linked from [TypeCatalog-Contracts.md](TypeCatalog-Contracts.md): BuildingType, DirectionType, HouseType, InfantryType, OverlayType, SmudgeType, TemplateType, TerrainType, TheaterType and UnitType.

Important separations:
- Definition IDs and names are variant catalog data; placed Building/Unit/Infantry/Overlay/Smudge/Template records hold per-scenario state.
- OccupyMask and OverlapBounds are independent contracts. BuildingType can expand its full mask for bibs while retaining BaseOccupyMask; TerrainType derives overlap bounds from RenderSize; UnitType uses a 1×1 occupancy mask and fixed 3×3 overlap bounds.
- TemplateType.IconMask is [IconWidth, IconHeight] and is written as [x,y]; do not transpose it during a Rust port.
- OverlayTypeFlag, SmudgeTypeFlag, TemplateTypeFlag and UnitTypeIDMask contain meaningful numeric bit values, including non-sequential bib flags and unused TemplateTypeFlag bit zero.
- TheaterType carries the ordered tileset-name source consumed by type initialization. Catalog and text-manager initialization order therefore matters.
- The four small interface contracts are recorded in [TypeCatalog-Interfaces.md](TypeCatalog-Interfaces.md).

The current Rust model has no typed equivalents for these definitions or catalogs. Its numeric CellLayers IDs and generic EditorObjectKind are not a Rust mapping of the original definition classes. TD/RA registry data, complete catalog ordering and Init call sites remain a separate follow-up audit.
