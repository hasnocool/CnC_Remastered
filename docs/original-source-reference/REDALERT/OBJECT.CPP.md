# REDALERT/OBJECT.CPP

**Review status:** Partially reviewed — function index inspected; header/state/call-site audit pending.
**Upstream:** [OBJECT.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/OBJECT.CPP)

## Purpose
Red Alert's common world-object implementation. It is broader than the Tiberian Dawn base class and includes generic AI, player-click dispatch, trigger attachment, target/range helpers, docking/exit behavior, firing coordinates, cloak/hidden behavior, ownership checks, and object lifecycle/tracking.

## Main class
`ObjectClass` is the common runtime object contract. The exact inheritance tree and all member fields must be read from `OBJECT.H`; do not copy TD's class layout into RA.

## Function groups from the source index
- **Simulation and lifecycle:** `AI`, constructors, `Limbo`, `Unlimbo`, `Detach`, `Detach_All`, `Fire_Out`.
- **Player interaction:** `Active_Click_With` overloads for object and cell targets, `Clicked_As_Target`.
- **Targeting/combat geometry:** `Center_Coord`, `Docking_Coord`, `Exit_Coord`, `Fire_Coord`, `In_Range`.
- **Ownership and permissions:** `Get_Ownable`, `Can_Demolish`, `Can_Player_Fire`, `Can_Player_Move`, `Can_Repair`.
- **Effects and visibility:** `Catch_Fire`, `Do_Shimmer`, `Hidden`.
- **Tracking and presentation:** `Detach`, `Detach_All`, `In_Which_Layer`, `Mark`, `Render`, `Receive_Message`, `Get_Mission`, `Debug_Dump`.
- **Other object interactions:** `Attach_Trigger`, `Exit_Object`, `What_Action`, `What_Am_I`.

## Dependencies
The RA map/cell layer tracks object placement; trigger and event systems attach/dispatch behavior; techno and derived classes specialize combat, movement, and ownership; display and animation systems consume render/effect hooks.

## TD comparison
Do not treat this as a drop-in duplicate of TD `OBJECT.CPP`. Red Alert's index exposes additional generic AI, trigger, docking, exit, fire-coordinate, cloak, and target-range responsibilities. The RA implementation is the authority for RA behavior.

## Rust mapping
Related foundation: `crates/rts-core/src/abstract_object.rs`, `tracker.rs`, `radio.rs`, `layer.rs`, `map.rs`, and `crates/red-alert/src/lib.rs`. Status: partial; no complete RA object behavior tree is established.

## Remaining audit
Enumerate exact base classes, virtual methods, fields, enums, and all function bodies; record which methods are RA-only and the call sites that invoke them.
