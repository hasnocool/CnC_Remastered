# Tiberian Dawn MISSION.CPP — source guide

> **Audit status: partially reviewed.** Grounded in the pinned upstream translation unit and its original function index. Exact header members, signatures, full function-body explanations, call graph, and Rust parity remain pending.

- Source: [TIBERIANDAWN/MISSION.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/MISSION.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 26,549 characters.

## Plain-English purpose

Implements the mission state machine used by game objects. It initializes active, suspended, and queued mission state and exposes mission assignment, commencement, override, restore, and mission-dispatch operations.

## Direct compile-time dependencies

- `function.h`

## Original source function index

- MissionClass::AI — Processes order script.
- MissionClass::Assign_Mission — Give an order to a unit.
- MissionClass::Commence — Start script with new order.
- MissionClass::Debug_Dump — Dumps status values to mono screen.
- MissionClass::Get_Mission — Fetches the mission that this object is acting under.        *
- MissionClass::MissionClass — Default constructor for the mission object type.            *
- MissionClass::Mission_From_Name — Fetch order pointer from its name.
- MissionClass::Mission_Name — Converts a mission number into an ASCII string.             *
- MissionClass::Overide_Mission — temporarily overides the units mission
- MissionClass::Restore_Mission — Restores overidden mission
- MissionClass::Set_Mission — Sets the mission to the specified value.                     *

These are original source index descriptions, not a substitute for reading each body. Do not infer complete branch behavior, return contracts, or side effects from names alone.

## Outstanding symbol and implementation audit

- [ ] Classes, structs, enums, aliases, macros, globals, constants.
- [ ] Every field/member with type, visibility, initialization, and purpose.
- [ ] Exact function signatures, overloads, parameters, and return values.
- [ ] Every function body: branches, loops, state changes, side effects, and failure paths.
- [ ] Important callers, callees, shared state, and ownership assumptions.
- [ ] Red Alert counterpart differences.
- [ ] Rust path and test mapping, with implemented/partial/absent status.
