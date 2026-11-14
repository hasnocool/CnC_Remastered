# REDALERT/UNIT.CPP

**Review status:** Partially reviewed — function index inspected.
**Upstream:** [UNIT.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/UNIT.CPP)

## Purpose
Implements Red Alert vehicle behavior on top of FootClass and TechnoClass: movement, vehicle weapons/turrets, harvesting, transports, deployment, and vehicle-specific AI.

## Main class
`UnitClass` represents a vehicle or mobile mechanical unit. Its behavior combines shared navigation/combat with vehicle-specific state.

## Function index highlights
The source index covers AI and lifecycle, click/order handling, approach/destination assignment, cell-entry legality, firing direction/AI, cargo credit value, crew type, edge-of-world behavior, idle animation, house flags, deployment to clear cells, Tiberium seeking/harvesting, transport loading/unloading, movement, damage, and drawing. Helper `Recoil_Adjust` updates pixel offsets for recoil.

## Dependencies
Uses RA FootClass navigation, TechnoClass combat/ownership, map/cell occupancy, pathfinding, weapon/turret data, animation, house flags, and scenario serialization.

## Difference from TD
RA has explicit `Approach_Target`, `Credit_Load`, `Edge_Of_World_AI`, `Firing_AI`, and `Fire_Direction` entries in the function index. Audit those paths separately rather than assuming TD's UnitClass behavior.

## Rust mapping
Related foundations: `crates/rts-core/src/turret.rs`, `weapon.rs`, `combat.rs`, `map.rs`, `house.rs`, `red_alert_aircraft_helpers.rs`. Status: incomplete; no complete RA vehicle simulation.
