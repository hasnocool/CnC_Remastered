# Infantry.cs — editor model source guide

> **Audit status: source inspected; explicit declarations and methods documented. Rust parity remains pending.**

- Upstream: [CnCTDRAMapEditor/Model/Infantry.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Infantry.cs)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 4966 characters.
- Namespace: MobiusEditor.Model

## Types
InfantryStoppingType: Center=0, UpperLeft=1, UpperRight=2, LowerLeft=3, LowerRight=4.
Infantry: individual slot record. InfantryGroup: cell-level occupancy/overlap implementation.

## Infantry members
PropertyChanged event; InfantryGroup auto-property; Type:InfantryType; House:HouseType; Strength:int; Direction:DirectionType; Mission:string; Trigger:string default Trigger.None; Tint:Color default White.
Constructor stores owning group. Clone creates a new Infantry with same group and copies Type, House, Strength, Direction, Trigger, Mission. SetField<T> suppresses equal assignments and raises PropertyChanged. OnPropertyChanged invokes subscribers. Explicit ICloneable.Clone delegates to Clone.

## InfantryGroup members
Static stoppingLocations Point array sized Globals.NumInfantryStops. OverlapBounds is Rectangle(-1,-1,3,3). OccupyMask returns a fresh 1x1 true array. Public readonly Infantry array sized NumInfantryStops.
Static constructor places center and four quarter/three-quarter tile corners using Globals.PixelWidth/PixelHeight.
ClosestStoppingTypes(Point) computes squared Euclidean distance to each stop, sorts ascending, returns stop types nearest-first.
Dependencies: Vector2 and LINQ. Clone retains original group reference. Test equal-distance ordering and global slot-count assumptions.

## Port status
No Rust equivalent is asserted until the current Rust workspace is inspected and behavior is mapped to implementation paths and tests.
