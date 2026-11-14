# Red Alert MISSION.CPP — source guide

> **Audit status: partially reviewed.** Grounded in the pinned Red Alert translation unit and its original function index. Exact paired-header members, complete body explanations, call graphs, and Rust parity are still pending.

- Source: [REDALERT/MISSION.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/MISSION.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 30,951 characters.

## Plain-English purpose

Implements Red Alert mission state and dispatch for game objects, including immediate assignment, queued/suspended mission behavior, and mission-specific execution.

## Direct compile-time dependencies

- `function.h`

## Original source function index

- MissionClass::AI — Processes order script.
- MissionClass::Assign_Mission — Give an order to a unit.
- MissionClass::Commence — Start script with new order.
- MissionClass::Debug_Dump — Dumps status values to mono screen.
- MissionClass::Get_Mission — Fetches the mission that this object is acting under.
- MissionClass::MissionClass — Default constructor for the mission object type.
- MissionClass::Mission_??? — Stub mission functions that do nothing.
- MissionClass::Mission_From_Name — Fetch order pointer from its name.
- MissionClass::Mission_Name — Converts a mission number into an ASCII string.
- MissionClass::Override_Mission — temporarily overrides the units mission
- MissionClass::Restore_Mission — Restores overridden mission
- MissionClass::Set_Mission — Sets the mission to the specified value.
- MissionClass::Is_Recruitable_Mission — Determines if this mission is recruitable for a te

The original index is a responsibility map, not a semantic substitute for reading the full function body.

## Outstanding audit

- [ ] Complete declarations, members, globals, constants, and exact signatures.
- [ ] Explain each function body, including branches, state changes, return behavior, and side effects.
- [ ] Trace call sites and shared state.
- [ ] Compare exact behavior against the Tiberian Dawn counterpart.
- [ ] Map to Rust modules and tests with explicit parity status.
