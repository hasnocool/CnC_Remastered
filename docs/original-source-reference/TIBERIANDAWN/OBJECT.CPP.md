# TIBERIANDAWN/OBJECT.CPP

**Review status:** Partially reviewed — source function index inspected; full implementation/call-graph pass remains.
**Upstream:** [OBJECT.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/OBJECT.CPP)

## Purpose

This is the common foundation for objects that exist in the Tiberian Dawn world. It provides shared behavior for being created, entering or leaving the active map, drawing, selection, damage, ownership/target queries, and communication with other game objects.

Do not confuse an object instance with an object type. An instance is the actual tank, building, projectile, or other thing in a match. The type describes reusable properties such as cost, dimensions, construction time, cameo art, and occupancy footprint.

## Main classes

### `ObjectClass`
The base world-object behavior. It handles common lifecycle and interaction operations so every derived object does not have to implement them independently.

### `ObjectTypeClass`
The shared catalogue/type descriptor. It exposes the common information needed to build, display, place, and price a particular kind of object.

## Function inventory from the source index

| Function | Plain-English job |
|---|---|
| `Debug_Dump` | Prints diagnostic state for debugging. |
| `Detach_All` | Removes the object from all systems that currently track it. |
| `Detach_This_From_All` | Removes references to this object from other tracked objects. |
| `Fire_Out` | Notifies the object that an attached animation/effect has completed. |
| `Get_Mission` | Returns its current mission/order. |
| `Init` | Initializes the base object system. |
| `Limbo` | Takes the object out of active world participation. |
| `Mark` / `Mark_For_Redraw` | Updates the map/render bookkeeping for the object's visual footprint. |
| `Move` | Forces a position change through the base object movement path. |
| `Passive_Click_With` | Handles the passive/right-click interaction path. |
| `Receive_Message` | Receives a radio/message-system event from another object. |
| `Render` | Draws the object in the world. |
| `Repair` / `Can_Repair` | Performs or checks repair behavior. |
| `Revealed` | Marks the object as revealed to a specified house/player. |
| `Select` / `Unselect` | Changes selection state. |
| `Sell_Back` / `Can_Demolish` | Sells an object back when allowed. |
| `Take_Damage` | Applies damage through the shared object damage path. |
| `Unlimbo` | Brings an object into active world participation. |
| `Value` | Returns the object's target/value representation. |
| `What_Action` | Determines the action appropriate for a clicked target. |
| `What_Am_I` / `Is_Techno` | Identifies the runtime object category. |
| `In_Which_Layer` | Reports the render/world layer used by the object. |
| `Get_Ownable` | Returns ownership/house legality information. |
| `Can_Player_Fire` / `Can_Player_Move` | Checks whether player-issued attack or movement orders are permitted. |
| `Target_Coord` | Returns the coordinate used when treating the object as a target. |
| `ObjectTypeClass::Cost_Of` | Gets the purchase/build cost. |
| `Dimensions` | Gets the object's pixel dimensions. |
| `Get_Cameo_Data` | Gets the sidebar cameo graphic data. |
| `Max_Pips` | Gets the maximum status/cargo pips shown for this type. |
| `Occupy_List` / `Overlap_List` | Describes cells physically occupied or visually overlapped by the object's footprint. |
| `Time_To_Build` | Gets the production time. |
| `Who_Can_Build_Me` | Finds the factory/building type capable of producing it. |
| `One_Time` | Performs one-time initialization for object types. |

## Important concepts and state

- **Active versus limbo:** an object may exist in memory without currently participating in the map. Limbo/unlimbo transitions coordinate map occupancy and other tracking.
- **Occupancy versus overlap:** occupancy blocks or reserves cells; overlap describes visual/footprint spill into neighboring cells. These are not interchangeable.
- **Instance versus type:** runtime state belongs to an object instance; static catalogue data belongs to its type.
- **Target identity:** the engine uses target abstractions so orders and messages can refer to different object categories consistently.

## Dependencies and call relationships

Derived classes such as `TechnoClass`, `FootClass`, buildings, units, infantry, aircraft, and projectiles build on these common contracts. The map/cell layer tracks placement; the display layer consumes marking/render information; radio/event systems deliver interactions.

## Rust port mapping

- Related current modules: `crates/rts-core/src/abstract_object.rs`, `crates/rts-core/src/map.rs`, `crates/rts-core/src/radio.rs`, `crates/rts-core/src/layer.rs`, `crates/rts-core/src/tracker.rs`.
- Status: partial conceptual foundations. There is not yet a complete runtime object hierarchy with matching lifecycle, target, ownership, occupancy, selection, damage, and render contracts.
- Porting rule: keep object simulation state independent from Bevy entities; use stable typed handles and explicit map occupancy/tracking systems.

## Evidence limits

The table is based on the original source's function index and its own short descriptions. Each implementation body, branch, and call site still needs a symbol-by-symbol review before this page can be marked fully reviewed.
