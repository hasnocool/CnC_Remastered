# Unit.cs — editor model source guide

> **Audit status: source inspected; explicit declarations and methods documented. Rust parity remains pending.**

- Upstream: [CnCTDRAMapEditor/Model/Unit.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Unit.cs)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 3022 characters.
- Namespace: MobiusEditor.Model

## Purpose and members
Placed mobile-unit record implementing ICellOverlapper, ICellOccupier, INotifyPropertyChanged, ICloneable.
PropertyChanged event. Type:UnitType; OverlapBounds and OccupyMask delegate to Type. House:HouseType. Strength:int. Direction:DirectionType. Mission:string. Trigger:string default Trigger.None. Tint:Color default White.
All six editable fields except Tint use private backing fields and SetField notification.

## Methods
Clone() copies Type, House, Strength, Direction, Mission, Trigger; it does not copy Tint, so clone defaults to White. SetField<T> equality-checks and raises notification. OnPropertyChanged invokes subscribers. Explicit ICloneable.Clone delegates to Clone.
Dependencies: UnitType footprint and occupancy, Rectangle/Color. Null Type invalidates geometry access.

## Port status
No Rust equivalent is asserted until the current Rust workspace is inspected and behavior is mapped to implementation paths and tests.
