# Tiberian Dawn TARGET.CPP — source guide

> **Audit status: partially reviewed.** Based on the pinned source translation unit and original function index. Exact class/member inventory, full implementation walkthrough, call graph, and verified Rust parity remain outstanding.

- Source: [TIBERIANDAWN/TARGET.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/TARGET.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 32,545 characters.

## Plain-English purpose

Implements typed interpretation of compact target identifiers, converting them into legal object pointers, team/trigger references, cells, or coordinates.

## Direct compile-time dependencies

- `function.h`
- `target.h`

## Original source function index

- As_Aircraft — Converts the target value into an aircraft pointer.
- As_Animation — Converts target value into animation pointer.
- As_Building — Converts a target value into a building object pointer.
- As_Bullet — Converts the target into a bullet pointer.
- As_Cell — Converts a target value into a cell number.
- As_Coord — Converts a target value into a coordinate value.
- As_Infantry — If the target is infantry, return a pointer to it.
- As_Movement_Coord — Fetches coordinate if trying to move to this target.
- As_Object — Converts a target value into an object pointer.
- As_Team — Converts a target number into a team pointer.
- As_TeamType — Converts a target into a team type pointer.
- As_Techno — Converts a target value into a TechnoClass pointer.
- As_Trigger — Converts specified target into a trigger pointer.
- As_Unit — Converts a target value into a unit pointer.
- Target_Legal — Determines if the specified target is legal.

This is the original source's index, not a complete semantic description of each implementation.

## Outstanding audit

- [ ] Every declaration, field, global, constant, macro, and exact signature.
- [ ] Every function's control flow, state mutation, return contract, and side effects.
- [ ] Callers, callees, data dependencies, and invariants.
- [ ] TD/RA differences where applicable.
- [ ] Rust implementation path and parity-test evidence.
