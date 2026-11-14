# TIBERIANDAWN/TECHNO.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [TECHNO.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/TECHNO.CPP)

## Purpose

Implements the common behavior for technological/combat-capable game objects: units and buildings that have owners, health, weapons, missions, targets, cloak state, cargo, and interaction with houses. It sits above the generic object layer and below specific object types.

## Main class: `TechnoClass`

A techno is a game object that participates in the strategic RTS systems. It can belong to a house, receive missions, acquire targets, fire weapons, be repaired/captured, reveal map cells, carry passengers, cloak, and communicate with other objects.

## Function inventory from the source index

The source index includes the following major responsibilities:

- **Orders and AI:** `AI`, `Assign_Destination`, `Assign_Target`, `Player_Assign_Mission`, `Enter_Idle_Mode`, `Receive_Message`.
- **Combat:** `Can_Fire`, `Fire_At`, `Fire_Direction`, `In_Range` (object/coordinate overloads), `Is_Weapon_Equipped`, `Greatest_Threat`, `Rearm_Delay`, `Record_The_Kill`.
- **Ownership and capture:** `Captured`, `Owner`, `Get_Ownable`, `Base_Is_Attacked`.
- **Repair and loading:** `Can_Repair`, `Desired_Load_Dir`, `Exit_Object`, `Find_Docking_Bay`, `Find_Exit_Cell`, `Kill_Cargo`.
- **Cloaking and visibility:** `Do_Cloak`, `Do_Shimmer`, `Do_Uncloak`, `Hidden`, `Evaluate_Cell`, `Evaluate_Object`.
- **Rendering and status:** `Draw_It`, `Draw_Pips`, `Mark`, `Pip_Count`, `Random_Animate`, `Remap_Table`.
- **Map and movement hooks:** `Nearby_Location`, `Per_Cell_Process`, `Detach`.
- **Player feedback:** `Response_Attack`, `Response_Move`.

The list above groups methods by role; it is not a replacement for the complete method-by-method signature inventory.

## Dependencies

Builds on `OBJECT.CPP` and its target/type abstractions. Uses house ownership, map/cell queries, radio messages, weapon/warhead definitions, animation, display, and derived unit/building classes.

## Rust port mapping

- Related current modules: `crates/rts-core/src/abstract_object.rs`, `crates/rts-core/src/house.rs`, `crates/rts-core/src/weapon.rs`, `crates/rts-core/src/warhead.rs`, `crates/rts-core/src/combat.rs`, `crates/rts-core/src/radio.rs`, `crates/rts-core/src/turret.rs`.
- Status: foundational types and combat helpers exist, but a complete `TechnoClass` lifecycle and behavior contract is not established.
- Porting requirement: keep damage/combat calculations deterministic, and make ownership/target references typed and generation-safe.

## Evidence limits

The source function index was inspected. Exact member layout, virtual dispatch, state transitions, and the full call graph remain to be extracted.
