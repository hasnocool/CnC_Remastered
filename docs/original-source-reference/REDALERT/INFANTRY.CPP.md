# REDALERT/INFANTRY.CPP

**Review status:** Partially reviewed — function index inspected.
**Upstream:** [INFANTRY.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/INFANTRY.CPP)

## Purpose
Implements Red Alert soldier behavior: infantry movement and cell occupancy, action/animation states, firing, fear, idle behavior, and combat AI.

## Main class
`InfantryClass` represents an individual infantry object. It specializes movement and combat for infantry-specific sub-cell placement and animation.

## Function index highlights
- **Simulation:** `AI`, `Doing_AI`, `Firing_AI`, `Fear_AI`, `Edge_Of_World_AI`.
- **Orders/combat:** `Active_Click_With`, `Assign_Destination`, `Assign_Target`, `Can_Fire`, `Fire_At`, `Greatest_Threat`, `Take_Damage`.
- **Cell movement:** `Can_Enter_Cell`, `Clear_Occupy_Bit`, `Set_Occupy_Bit`, `Start_Driver`, `Stop_Driver`, `Scatter`.
- **Animation/rendering:** `Do_Action`, `Draw_It`, `Enter_Idle_Mode`, `Get_Image_Data`, `Random_Animate`, facing and readiness helpers.
- **Other:** constructors, `Init`, `Limbo`, `Unlimbo`, `Full_Name`, radio responses, validation and target conversion.

## Difference from TD
Red Alert's index includes explicit animation AI, firing AI, fear processing, edge-of-world handling, and image-data lookup. Keep these separate from Tiberian Dawn's soldier state machine.

## Rust mapping
Related foundations: `crates/rts-core/src/map.rs`, `coordinate.rs`, `facing.rs`, `combat.rs`, `weapon.rs`, `warhead.rs`, `red_alert_animation_types.rs`. Status: incomplete; no complete RA infantry state machine or sub-cell occupancy parity.
