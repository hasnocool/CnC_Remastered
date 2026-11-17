# Map.cs — editor map aggregate, mutation batching, and derived spatial state

> **Source status: Body reviewed. Rust mapping: not implemented as an aggregate.** The full 23,856-byte source body was read at the pinned upstream commit. This page documents all local type and method bodies; exhaustive external call-site and serializer parity work remains open.

- Upstream source: [CnCTDRAMapEditor/Model/Map.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Map.cs)
- Upstream blob: 1b478aebee005a8db2b4b7bfb4e044356829bd5c
- Namespace: MobiusEditor.Model
- Imports: MobiusEditor.Interface, MobiusEditor.Render, MobiusEditor.Utility, System, generic/observable collections, component model, drawing and drawing2D, LINQ, TGASharpLib.

## Local types

### MapLayerFlag

[Flags] enum with explicit bit values:
- None=0
- Basic=1<<0; Map=1<<1; Template=1<<2; Terrain=1<<3; Resources=1<<4; Walls=1<<5; Overlay=1<<6; Smudge=1<<7
- Waypoints=1<<8; CellTriggers=1<<9; Houses=1<<10; Infantry=1<<11; Units=1<<12; Buildings=1<<13; Boundaries=1<<14; TechnoTriggers=1<<15
- OverlayAll = Resources | Walls | Overlay
- Technos = Terrain | Walls | Infantry | Units | Buildings
- All = int.MaxValue (not merely the OR of currently named flags).

### MapContext

Implements ITypeDescriptorContext. Public properties Container:IContainer, Instance:object, PropertyDescriptor:PropertyDescriptor have private setters and are not assigned in this file's constructor. Map returns Instance as Map. FractionalPercentages is public readonly bool. Constructor stores Instance=map and the fractional flag. GetService returns null; OnComponentChanged is empty; OnComponentChanging returns true.

## Map state inventory

Map implements ICloneable. Private fields:
- updateCount:int = 0
- updating:bool = false
- invalidateLayers:IDictionary<MapLayerFlag,ISet<Point>>, initialized to a Dictionary
- invalidateOverlappers:bool (default false)

Public readonly section/catalog/model members:
- BasicSection:BasicSection
- MapSection:MapSection, BriefingSection:BriefingSection, SteamSection:SteamSection
- HouseType:Type; HouseTypes:HouseType[]
- TheaterTypes:List<TheaterType>; TemplateTypes:List<TemplateType>; TerrainTypes:List<TerrainType>; OverlayTypes:List<OverlayType>; SmudgeTypes:List<SmudgeType>
- EventTypes:string[]; ActionTypes:string[]; MissionTypes:string[]; TeamMissionTypes:string[]
- DirectionTypes:List<DirectionType>; InfantryTypes:List<InfantryType>; UnitTypes:List<UnitType>; BuildingTypes:List<BuildingType>; MovieTypes:List<string>
- Metrics:CellMetrics
- Templates:CellGrid<Template>; Overlay:CellGrid<Overlay>; Smudge:CellGrid<Smudge>
- Technos:OccupierSet<ICellOccupier>; Buildings:OccupierSet<ICellOccupier>; Overlappers:OverlapperSet<ICellOverlapper>
- Waypoints:Waypoint[]; CellTriggers:CellGrid<CellTrigger>; Triggers:ObservableCollection<Trigger>; TeamTypes:List<TeamType>
- Houses:House[] is mutable (not readonly).

Computed/delegating properties:
- Theater reads/writes MapSection.Theater.
- TopLeft maps MapSection.X/Y.
- Size maps MapSection.Width/Height.
- Bounds builds Rectangle(TopLeft,Size); setter writes left/top/width/height.
- TiberiumOrGoldValue and GemValue are mutable ints.
- TotalResources iterates non-null Overlay cells; for resource overlays adds (Icon+1) times GemValue for gems, otherwise TiberiumOrGoldValue.

## Constructor

Signature:
~~~csharp
Map(BasicSection basicSection, TheaterType theater, Size cellSize, Type houseType,
    IEnumerable<HouseType> houseTypes, IEnumerable<TheaterType> theaterTypes,
    IEnumerable<TemplateType> templateTypes, IEnumerable<TerrainType> terrainTypes,
    IEnumerable<OverlayType> overlayTypes, IEnumerable<SmudgeType> smudgeTypes,
    IEnumerable<string> eventTypes, IEnumerable<string> actionTypes,
    IEnumerable<string> missionTypes, IEnumerable<DirectionType> directionTypes,
    IEnumerable<InfantryType> infantryTypes, IEnumerable<UnitType> unitTypes,
    IEnumerable<BuildingType> buildingTypes, IEnumerable<string> teamMissionTypes,
    IEnumerable<Waypoint> waypoints, IEnumerable<string> movieTypes)
~~~

The constructor retains BasicSection by reference; copies catalog enumerables into arrays/lists; creates CellMetrics and the three cell grids, Technos/Buildings occupancy sets, Overlappers, Triggers, TeamTypes, Houses, Waypoints, and CellTriggers. Houses are created with Activator.CreateInstance(HouseType, houseTypeEntry), then SetDefault() is called on each. It calls SetDefault on MapSection, BriefingSection, and SteamSection; clears all grids/sets; sets TopLeft=(1,1), Size=Metrics.Size-(2,2), and Theater; then subscribes Overlay.CellChanged, Technos add/remove, Buildings add/remove, and Triggers.CollectionChanged. The subscriptions are installed after initial clears.

## Public methods and complete local behavior

- BeginUpdate(): increments updateCount. It does not perform work on the first or nested begin.
- EndUpdate(): pre-decrements updateCount; calls Update only when the result equals zero. There is no underflow guard or pairing validation. Calling EndUpdate without a matching BeginUpdate makes the count negative and skips Update.
- InitTheater(GameType): calls Init(Theater) for every TemplateType, SmudgeType, OverlayType, and TerrainType. It then initializes each InfantryType using the matching owner HouseType (FirstOrDefault) and the first South DirectionType; UnitType and BuildingType use the first North DirectionType. Missing North/South entries cause First() to throw.
- Update(): sets updating=true; recomputes resource overlays for invalidated Resources points and wall overlays for invalidated Walls points; if invalidateOverlappers is true, clears Overlappers and rebuilds it from every Technos item implementing ICellOverlapper; then clears invalidation collections/flag and sets updating=false. No try/finally protects the updating flag or pending invalidations if a derived-state operation throws.
- UpdateResourceOverlays(ISet<Point>): uses lookup arrays tiberiumCounts={0,1,3,4,6,7,8,10,11} and gemCounts={0,0,0,1,1,1,2,2,2}. For each resource overlay at an intersecting location, counts resource overlays in all eight AdjacentFacings and selects the gem or Tiberium/gold icon by that count.
- UpdateWallOverlays(ISet<Point>): checks only North/East/South/West neighbors. It sets icon bits 1,2,4,8 respectively when the neighbor has exactly the same OverlayType.
- RemoveBibs(Building): intersects Smudge with building.BibCells, removes only cells whose SmudgeType.Flag contains Bib, then clears the Building.BibCells set.
- AddBibs(Point,Building): exits unless Building.Type.HasBib. Finds SmudgeTypes with flags exactly Bib1/Bib2/Bib3. Building widths 2/3/4 select Bib3/Bib2/Bib1 respectively. If a type exists, loops its rows/columns, assigns sequential icons, computes cells below the building at Y + building height + row - 1, and for in-bounds cells writes a new Smudge with type/icon/Data=0/Tint=building.Tint and records the cell in BibCells.
- Clone(): constructs a new Map with the same BasicSection, theatre, metrics size, type/catalog collections, waypoints, and movie types; copies TopLeft/Size; begins a batch; copies MapSection/BriefingSection/SteamSection via the shared reflection CopyTo extension; copies Templates/Overlay/Smudge/CellTriggers; shallow-copies Houses; adds the same Trigger objects; copies Technos with special handling for InfantryGroup (new group plus shallow copy of its Infantry array), skips Building objects in Technos, and adds other occupiers by reference; then copies Buildings by reference (their add event also populates Technos and bibs); shallow-copies TeamTypes; ends the batch and returns the new Map. BasicSection, catalog entries, trigger/team objects, houses, ordinary occupiers, buildings, and infantry members are shared references. TiberiumOrGoldValue and GemValue are not copied by this method.
- GeneratePreview(Size previewSize,bool sharpen): derives pixel mapBounds from Bounds and original tile dimensions; computes previewScale as the smaller width/height ratio and scaledSize as previewSize divided by that scale; allocates a full-map bitmap and preview bitmap; renders only Template|Resources layers over Bounds.Points(); creates a Graphics transform with scale and centered translation, clears preview black, draws the map-bounds source rectangle into the preview, and optionally calls Bitmap.Sharpen(1.0f) before TGA conversion. The transform Matrix is not disposed explicitly. Zero/degenerate mapBounds can make previewScale zero and the scaled-size division invalid.
- GenerateMapPreview(): calls GeneratePreview(Globals.MapPreviewSize,false).
- GenerateWorkshopPreview(): calls GeneratePreview(Globals.WorkshopPreviewSize,true).
- Explicit ICloneable.Clone() delegates to Clone().

## Event handlers and derived-state wiring

- Overlay_CellChanged first removes an old wall from Buildings and adds a new wall to Buildings. Those collection events feed wall occupancy into Technos. If updating is true, it returns after this occupancy synchronization. Otherwise it classifies old/new overlays as Resources or Walls, creates a HashSet<Point> for each affected layer, unions the 3x3 neighborhood produced by Inflate(Rectangle(location,1x1),1,1).Points(), and calls Update immediately when updateCount==0.
- Technos_OccupierAdded: if the added object implements ICellOverlapper, adds it to Overlappers immediately outside a batch; during a batch sets invalidateOverlappers.
- Technos_OccupierRemoved mirrors that behavior with Overlappers.Remove or invalidation.
- Buildings_OccupierAdded: for Building, adds it to Technos using Type.BaseOccupyMask and calls AddBibs; for every other occupier (including wall Overlay objects) adds it using its default OccupyMask.
- Buildings_OccupierRemoved: removes Building bibs when applicable, then removes the object from Technos.
- Triggers_CollectionChanged scans every Building. If a building has a non-empty Trigger string that no current Trigger.Name matches, it assigns Trigger.None. The handler does not limit work to the changed item.

## Important source-derived risks

- BeginUpdate/EndUpdate is a manual counter with no underflow protection and no disposable scope.
- Update lacks try/finally; exceptions can leave updating=true and derived state partially refreshed.
- CopyTo/Clone semantics are mixed: some arrays/sections are copied, but most catalog and object references are shallow. Resource-value multipliers are omitted from Clone.
- Building occupancy deliberately differs between Buildings and Technos: the former receives BuildingType.OccupyMask (including bib footprint), the latter receives BaseOccupyMask; bibs are separate Smudge objects.
- Wall overlays are inserted into Buildings, then Buildings event handlers insert them into Technos. This is the bridge that makes walls participate in occupancy.
- Resource adjacency uses eight neighbors; wall connectivity uses four orthogonal neighbors and exact type equality.
- Overlay invalidation expands to a 3x3 neighborhood because neighboring resource/wall icons depend on the edited cell.
- Preview generation depends on MapRenderer, global tile constants, GDI+ transforms, TGA conversion, and the Sharpen extension; this is editor preview behavior, not map serialization.

## Rust mapping

The current crates/map-editor-model MapDocument has GameVariant, MapDimensions, Vec<CellLayers>, and a BTreeMap of waypoints. It does not represent MapSection/theatre, Basic/Briefing/Steam sections, catalogs, typed Template/Overlay/Smudge cells, Technos/Buildings/Overlappers, houses, triggers, team types, resource totals, batching/invalidation, bib lifecycle, cloning, or preview generation. The existing CellMetrics module maps only a subset of grid geometry. The current object_selection module plans selection/UI actions and is not this aggregate model.

No Rust file was changed in this batch. No Rust build or test was run. This source page is Body reviewed, not Rust-mapped, Tested, or Complete.
