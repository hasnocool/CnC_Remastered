# REDALERT/TECHNO.CPP

**Review status:** Partially reviewed — function index inspected.
**Upstream:** [TECHNO.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/TECHNO.CPP)

## Purpose
Implements shared strategic/combat behavior for Red Alert's techno objects: units and buildings that have an owner, health, weapons, missions, targets, visibility/cloak state, and interactions with houses.

## Main class
`TechnoClass` is the common strategic object layer between generic world objects and specific unit/building classes.

## Functions grouped by responsibility
- **AI/orders:** `AI`, `Assign_Destination`, `Assign_Target`, `Player_Assign_Mission`, `Enter_Idle_Mode`, `Receive_Message`.
- **Combat rating and damage:** `Anti_Air`, `Anti_Armor`, `Anti_Infantry`, `Combat_Damage`, `Can_Fire`, `Fire_At`, `In_Range`, `Greatest_Threat`, `Rearm_Delay`.
- **Movement/placement:** `Can_Teleport_Here`, `Nearby_Location`, `Find_Exit_Cell`, `Desired_Load_Dir`, `Per_Cell_Process`.
- **Cloak/visibility:** `Cloaking_AI`, `Do_Cloak`, `Do_Shimmer`, `Do_Uncloak`, `Hidden`.
- **Ownership and status:** `Captured`, `Owner`, `Get_Ownable`, `Base_Is_Attacked`, `Record_The_Kill`.
- **Presentation/cargo:** `Draw_It`, `Draw_Pips`, `Pip_Count`, `Remap_Table`, `Kill_Cargo`, `Exit_Object`, `Find_Docking_Bay`.

## RA-specific behavior
The function index includes anti-air/anti-armor/anti-infantry scoring, cell area-scan modifiers, teleport legality, cloak AI, and explicit combat-damage calculation. These are not safe to infer from TD's TechnoClass.

## Dependencies
Builds on RA `OBJECT.CPP`; calls into house, map/cell, weapon/warhead, radio, animation, and display systems; derived units and buildings provide specialized behavior.

## Rust mapping
Related foundations: `crates/rts-core/src/abstract_object.rs`, `house.rs`, `combat.rs`, `weapon.rs`, `warhead.rs`, `radio.rs`, `red_alert_mpath.rs`; RA game crate `crates/red-alert`. Status: incomplete behavior parity.

## Remaining audit
Enumerate every signature and member, then compare exact RA-vs-TD overrides and all damage/strength scoring formulas.
