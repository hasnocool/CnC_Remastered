# TIBERIANDAWN/FOOT.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [FOOT.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/FOOT.CPP)

## Purpose

Implements behavior shared by mobile ground units. `FootClass` sits between the general techno/object systems and specific mobile classes such as vehicles and infantry. It connects player orders to navigation, movement, cell entry, missions, and the game's radio/message system.

## Main class: `FootClass`

A mobile world object with a navigation computer/driver state. It knows how to approach targets, accept destinations, run common movement missions, enter cells, face directions, and update its map footprint while moving.

## Function inventory from the source index

| Function | Plain-English job |
|---|---|
| `Active_Click_With` | Turns a player click into an attack, move, or other mobile-unit action. |
| `Approach_Target` | Sets navigation to move toward a target object. |
| `Assign_Destination` | Gives the navigation system a destination. |
| `Assign_Mission` | Assigns the mobile object's mission/order. |
| `Basic_Path` | Finds a basic path for a ground object. |
| `Body_Facing` | Changes the object's facing direction. |
| `Can_Enter_Cell` | Checks whether the object may enter a particular cell. |
| `Death_Announcement` | Announces that the unit has died. |
| `Detach` / `Detach_All` | Removes stale tracking/target references. |
| `Enters_Building` | Handles a unit entering a building. |
| `Greatest_Threat` | Finds the most significant threat to this unit. |
| `Likely_Coord` | Predicts the coordinate the object is likely to occupy shortly. |
| `Limbo` / `Unlimbo` | Removes or restores the unit to active map participation with movement-specific fixups. |
| `Mark` | Updates the map/render marking for the mobile object's footprint. |
| `Mission_Attack` | Handles moving toward and attacking a target. |
| `Mission_Capture` | Handles a capture mission. |
| `Mission_Enter` | Handles cooperative entry into a building/target. |
| `Mission_Guard_Area` | Guards an area around a location. |
| `Mission_Hunt` | Runs the default hunt behavior. |
| `Mission_Move` | Moves the unit toward its assigned destination. |
| `Offload_Tiberium_Bail` | Handles unloading a portion of harvested Tiberium. |
| `Override_Mission` / `Restore_Mission` | Temporarily replaces an order and later restores the previous one. |
| `Per_Cell_Process` | Runs logic when movement reaches the next cell or cell boundary. |
| `Receive_Message` | Handles movement/navigation-related radio messages. |
| `Rescue_Mission` | Redirects the unit to assist/rescue. |
| `Sell_Back` | Performs the mobile object's sell-back behavior. |
| `Set_Speed` | Starts or adjusts movement speed/physics. |
| `Sort_Y` | Provides the draw-order coordinate for the object. |
| `Start_Driver` / `Stop_Driver` | Starts or stops the movement driver and its reservations. |
| `Stun` | Stops/stuns the object in preparation for removal or another state. |
| `Take_Damage` | Applies damage with mobile-object-specific handling. |

## Dependencies

Builds on `TECHNO.CPP` and `OBJECT.CPP`; uses `MAP.CPP`/`CELL.CPP` for movement legality and occupancy; uses pathfinding/navigation helpers; derived `UNIT.CPP` and `INFANTRY.CPP` specialize movement and missions.

## Rust port mapping

- Related current modules: `crates/rts-core/src/map.rs`, `crates/rts-core/src/coordinate.rs`, `crates/rts-core/src/red_alert_mpath.rs`, `crates/rts-core/src/facing.rs`, `crates/rts-core/src/tracker.rs`.
- Status: movement-related building blocks exist, but no complete TD mobile-unit navigation/mission/driver implementation is established.
- Porting requirement: movement updates must be deterministic and nonblocking; cell reservation changes must be atomic within the simulation tick to avoid two units claiming the same destination.

## Evidence limits

Function list is taken from the original source index. Exact mission state transitions, navigation-computer fields, and movement timing need line-by-line source review.
