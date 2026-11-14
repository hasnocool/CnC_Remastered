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
