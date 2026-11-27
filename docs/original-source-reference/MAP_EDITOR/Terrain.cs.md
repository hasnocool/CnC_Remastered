# Terrain.cs — editor model source guide

> **Audit status: source inspected; explicit declarations and methods documented. Rust parity remains pending.**

- Upstream: [CnCTDRAMapEditor/Model/Terrain.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Terrain.cs)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 2528 characters.
- Namespace: MobiusEditor.Model

## Purpose and members
Placed terrain object implementing ICellOverlapper, ICellOccupier, INotifyPropertyChanged, ICloneable.
PropertyChanged event. Type:TerrainType; Icon:int; OverlapBounds delegates to Type.OverlapBounds; OccupyMask delegates to Type.OccupyMask; Trigger:string default Trigger.None; Tint:Color default White.
Type, Icon, Trigger use backing fields and SetField notifications; Tint is an auto-property.

## Methods
Clone() copies Type, Icon, Trigger; Tint is not copied and resets to White. SetField<T> equality-checks then notifies. OnPropertyChanged invokes subscribers. Explicit ICloneable.Clone delegates to Clone.
Dependencies: TerrainType geometry and occupancy. Null Type invalidates geometry access.

## Port status
No Rust equivalent is asserted until the current Rust workspace is inspected and behavior is mapped to implementation paths and tests.


## Rust mapping update — map-owned terrain objects

Added `crates/map-editor-model/src/terrain_objects.rs` and integrated it into `MapDocument`. Each placed terrain is represented once as `TerrainObject` with stable `TerrainObjectId`, anchor/origin `CellCoord`, source terrain type ID, icon index and optional trigger name. The document owns the object collection and a derived per-cell owner index built from the source occupancy mask.

Object insertion validates the complete footprint before mutation; overlapping/out-of-bounds placements are rejected atomically. Removal checks that every occupied cell is still owned by the same object. Movement validates the full destination before releasing the old footprint and preserves identity and trigger. Existing `CellLayers.terrain` / `terrain_icon` are maintained as compatibility mirrors, not used as the object identity or occupancy index.

The source C# `Terrain` also has Tint, PropertyChanged notifications and Clone behavior (which resets Tint to white). These are not yet ported. Object-level history commands and editor selection/drag integration remain open. Render overlap is not calculated from occupancy.


## Rust history integration update

`MapEditHistory` now records terrain object placement, removal and movement as distinct atomic history commands. Each command carries the before/after `TerrainObject` and its source occupancy definition. Undo/redo validates the current expected object and complete footprint before changing the object collection, owner index and legacy cell mirror. A conflict retains the command and leaves the map unchanged. The editor currently exposes placement, object picking by occupied cell, and undoable removal; object movement is model/history-supported but not yet exposed in the UI.
