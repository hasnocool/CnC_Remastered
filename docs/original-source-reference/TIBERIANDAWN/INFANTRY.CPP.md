# TIBERIANDAWN/INFANTRY.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [INFANTRY.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/INFANTRY.CPP)

## Purpose

Implements infantry-specific simulation. Infantry differs from vehicles because soldiers use sub-cell positions and occupancy bits, have infantry animation/action states, can scatter, and may have special capture/combat behavior.

## Main class: `InfantryClass`

Represents an individual infantry soldier. It specializes the mobile/combat object hierarchy for infantry movement, animation, targeting, cell occupancy, firing, and scenario serialization.

## Function inventory from the source index

- **Simulation and lifecycle:** `AI`, `Init`, `Limbo`, `Unlimbo`, `Validate`, constructors/destructor, pool `operator new/delete`.
- **Orders and targeting:** `Active_Click_With`, `As_Target`, `Assign_Destination`, `Assign_Mission`, `Assign_Target`, `What_Action`, `Mission_Attack`.
- **Movement/cell occupancy:** `Can_Enter_Cell`, `Clear_Occupy_Bit`, `Set_Occupy_Bit`, `Start_Driver`, `Stop_Driver`, `Scatter`, `Per_Cell_Process`, `Overlap_List`.
- **Combat:** `Can_Fire`, `Fire_At`, `Fire_Coord`, `Greatest_Threat`, `Take_Damage`, `Made_A_Kill`, `Rearm_Delay`.
- **Animation/visuals:** `Do_Action`, `Draw_It`, `Enter_Idle_Mode`, `Random_Animate`, `Set_Primary_Facing`.
- **Scenario and feedback:** `Read_INI`, `Write_INI`, `Full_Name`, `Response_Attack`, `Response_Move`, `Response_Select`, `Receive_Message`, `Look`.

## Important distinction

Do not model infantry as a vehicle with a different sprite. Its occupancy is more granular, and animation/action state is tightly connected to movement and combat. The Rust port needs explicit infantry sub-cell reservation and action-state rules.

## Dependencies

Builds on the mobile and techno object hierarchy; interacts with `CELL.CPP` occupancy bits, map visibility, weapon/warhead logic, animation, audio feedback, and scenario INI.

## Rust port mapping

Related foundations: `crates/rts-core/src/map.rs`, `coordinate.rs`, `facing.rs`, `combat.rs`, `weapon.rs`, and `warhead.rs`. No complete infantry behavior implementation is established.
