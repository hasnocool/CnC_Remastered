# Building.cs — editor model source guide

> **Audit status: source inspected; explicit declarations and methods documented. Rust parity remains pending.**

- Upstream: [CnCTDRAMapEditor/Model/Building.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Building.cs)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 3831 characters.
- Namespace: MobiusEditor.Model

## Purpose
Placed building record implementing ICellOverlapper, ICellOccupier, INotifyPropertyChanged, and ICloneable.

## Complete explicit members
PropertyChanged event. Type: BuildingType; OverlapBounds delegates to Type.OverlapBounds; OccupyMask delegates to Type.OccupyMask. House: HouseType. Strength:int. Direction:DirectionType. Trigger:string default Trigger.None. BasePriority:int default -1. IsPrebuilt:bool default true. Sellable:bool false. Rebuild:bool false. BibCells:ISet<int>, private setter, initialized HashSet. Tint uses private Color backing field default White; getter applies 75% alpha when not prebuilt, setter stores raw tint.

## Methods
Clone() copies Type, House, Strength, Direction, Trigger, BasePriority, IsPrebuilt, Sellable, Rebuild, Tint. It does not copy BibCells. SetField<T> equality-checks then notifies. OnPropertyChanged emits event. Explicit ICloneable.Clone delegates to Clone.
Dependencies: BuildingType geometry, Rectangle/Color, collections, occupancy interfaces. Null Type breaks geometry access. Clone is shallow for Type/House and omits BibCells.

## Port status
No Rust equivalent is asserted until the current Rust workspace is inspected and behavior is mapped to implementation paths and tests.
