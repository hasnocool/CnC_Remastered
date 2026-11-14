# REDALERT/FOOT.CPP

**Review status:** Partially reviewed — function index inspected.
**Upstream:** [FOOT.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/FOOT.CPP)

## Purpose
Implements common movement behavior for Red Alert's mobile ground objects. It connects missions and player orders to navigation queues, pathfinding, cell legality, formation adjustment, and movement state.

## Main class
`FootClass` is the mobile-object base used by vehicles and infantry. It coordinates a navigation computer/queue with map occupancy and object missions.

## Function index highlights
- **Navigation:** `Adjust_Dest`, `Approach_Target`, `Assign_Destination`, `Basic_Path`, `Clear_Navigation_List`, `Handle_Navigation_List`.
- **Orders/missions:** `Active_Click_With`, `Mission_Attack`, plus inherited/overridden mission handlers in the rest of the file.
- **Movement state:** `AI`, `Can_Enter_Cell`, `Body_Facing`, `Likely_Coord`, `Start_Driver`, `Stop_Driver`, `Mark`.
- **Map-edge/team behavior:** `Is_Allowed_To_Leave_Map`, `Is_On_Priority_Mission`, `Is_Recruitable`.
- **Lifecycle/tracking:** constructors, `Detach`, `Detach_All`, and object lifecycle overrides.

## Difference from TD
Red Alert's function index exposes navigation-list processing, destination adjustment for formations, leaving-map rules, recruitability, and priority-mission queries. These must be represented as explicit RA behavior, not silently omitted in a shared movement implementation.

## Dependencies
RA map/cell occupancy, navigation/pathfinding, techno combat and ownership, radio messages, team recruitment, and unit/infantry subclasses.

## Rust mapping
Related foundations: `crates/rts-core/src/map.rs`, `coordinate.rs`, `red_alert_mpath.rs`, `facing.rs`, `tracker.rs`. Status: incomplete; no complete RA FootClass navigation queue and mission implementation.

## Remaining audit
Read full `FOOT.H`, map every navigation-list field, and trace how queue entries are created, consumed, canceled, and synchronized with cell reservations.
