# TIBERIANDAWN/UNIT.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [UNIT.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/UNIT.CPP)

## Purpose

Implements vehicle and other mobile-unit-specific behavior on top of `FootClass` and `TechnoClass`. It handles vehicle movement, combat, harvesting, transports, deployment, crew, doors, and vehicle-specific missions.

## Main class: `UnitClass`

Represents a concrete mobile vehicle unit. It inherits common object behavior and specializes it for vehicle rules and vehicle artwork/animation.

## Function inventory from the source index

- **Core and lifecycle:** `AI`, `Init`, `Limbo`, `Read_INI`, `Unlimbo`, `Validate`, `As_Target`.
- **Movement and placement:** `Can_Player_Move`, `Click_With`, `Can_Enter_Building`, `Blocking_Object`, `Desired_Load_Dir`, `Find_LZ`, `Goto_Clear_Spot`, `Set_Speed`, `Start_Driver`, `Stop_Driver`, `Scatter`, `Sort_Y`.
- **Combat and AI:** `Can_Fire`, `Mission_Attack`, `Mission_Guard`, `Mission_Hunt`, `Greatest_Threat`, `Look`, `Take_Damage`.
- **Harvesting:** `Goto_Tiberium`, `Harvesting`, `Mission_Harvest`.
- **Transport/deployment:** `APC_Open_Door`, `APC_Close_Door`, `Mission_Unload`, `Crew_Type`, `Goto_Clear_Spot`.
- **Player feedback and rendering:** `Draw_It`, `Enter_Idle_Mode`, `Pip_Count`, `Random_Animate`, `Remap_Table`, `Response_Attack`, `Response_Move`, `Response_Select`, `Flag_Attach`, `Flag_Remove`.
- **Map integration:** `Overlap_List`, `Per_Cell_Process`, `Start_Driver`, `Stop_Driver`.

The original index also lists helper routines `Recoil_Adjust` and `Turret_Adjust` for vehicle weapon/turret presentation.

## Dependencies

Inherits `FootClass` movement and `TechnoClass` combat/ownership. Uses map/cell reservations, pathfinding, weapons, turret/recoil helpers, animation, house flags, and scenario INI.

## Rust port mapping

- Related current modules: `crates/rts-core/src/turret.rs`, `crates/rts-core/src/weapon.rs`, `crates/rts-core/src/combat.rs`, `crates/rts-core/src/map.rs`, `crates/rts-core/src/house.rs`.
- Status: some reusable helpers exist; complete vehicle simulation, harvesting, APC transport, deployment, movement, and INI compatibility are not established.
