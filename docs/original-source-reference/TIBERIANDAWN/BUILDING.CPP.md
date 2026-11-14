# TIBERIANDAWN/BUILDING.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [BUILDING.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/BUILDING.CPP)

## Purpose
Implements buildings as fixed-footprint strategic objects. Buildings can be constructed, captured, selected, attacked, repaired, sold, animated, and used as factories or defensive structures.

## Main class
`BuildingClass` is the runtime building instance. It specializes techno/object behavior for a multi-cell stationary structure.

## Function inventory highlights
The source index includes `AI`, `Active_Click_With`, `As_Target`, `Assign_Target`, `Begin_Mode`, constructors, `Can_Demolish`, `Can_Enter_Cell`, `Can_Fire`, `Captured`, `Center_Coord`, `Click_With`, `Crew_Type`, `Death_Announcement`, `Detach/Detach_All`, `Draw_It`, `Drop_Debris`, `Enter_Idle_Mode`, `Exit_Object`, `Fire_At`, `Fire_Coord`, `Fire_Direction`, `Fire_Out`, and placement/production-related routines.

## Key concepts
A building has a footprint described by its type, not merely one occupied tile. Placement legality, occupancy, visual overlap, production exit cells, and draw order must agree. Factory buildings also coordinate with production state owned by the house/factory systems.

## Dependencies
`OBJECT.CPP`/`TECHNO.CPP`, `MAP.CPP`/`CELL.CPP`, `CDATA.CPP` type definitions, house/production systems, weapons/warheads, animation, display, and scenario INI.

## Rust mapping
Related foundations: `crates/rts-core/src/house.rs`, `map.rs`, `weapon.rs`, `warhead.rs`, `combat.rs`. No complete TD BuildingClass behavior is established. Port fixed footprint and production exit logic as explicit data/state, not as assumptions in the renderer.
