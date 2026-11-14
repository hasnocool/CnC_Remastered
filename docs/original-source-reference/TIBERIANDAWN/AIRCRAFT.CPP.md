# TIBERIANDAWN/AIRCRAFT.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [AIRCRAFT.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/AIRCRAFT.CPP)

## Purpose
Implements aircraft behavior: flight missions, attack runs, landing and rearming, passenger loading/unloading, target selection, firing, and air-specific map/render handling.

## Main class
`AircraftClass` is a mobile airborne techno object. It shares strategic/combat behavior with other techno objects but has flight and landing rules that are not ordinary ground-cell movement.

## Function inventory highlights
`AI`, click handlers for cell/object targets, constructors, `As_Target`, `Can_Enter_Cell`, `Cell_Seems_Ok`, `Desired_Load_Dir`, `Draw_It`, `Enter_Idle_Mode`, `Exit_Object`, `Fire_At`, `Fire_Coord`, `Fire_Direction`, `Good_Fire_Location`, `Good_LZ`, `In_Which_Layer`, `Init`, `Is_LZ_Clear`, `Mark`, and mission handlers for attack/enter/guard/hunt/move.

## Dependencies
Techno combat/ownership, map/cell landing checks, building helipads/repair centers, weapon/warhead, animation, display layers, and scenario loading.

## Rust mapping
Related foundations: `crates/rts-core/src/red_alert_aircraft_helpers.rs`, `red_alert_aircraft_types.rs`, `map.rs`, `combat.rs`, `weapon.rs`. Status: these named modules are primarily RA-oriented support; full TD aircraft behavior parity is not established.
