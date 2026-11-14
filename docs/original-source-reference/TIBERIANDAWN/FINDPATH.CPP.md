# TIBERIANDAWN FINDPATH.CPP — source guide

> **Audit status: partially reviewed.** This first pass inspects the pinned translation unit, its direct includes, and original source function index. Paired headers, exhaustive symbols, full body explanations, call graphs, and Rust parity remain outstanding.

- Upstream: [TIBERIANDAWN/FINDPATH.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/FINDPATH.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation-unit size: 54745 characters.

## Plain-English system purpose

Foot-unit pathfinding and movement-path support. Includes path-overlap registration, path search, edge following, loop unraveling, coordinate stepping, and movement optimization.

## Direct compile-time dependencies

- function.h

## Original function index

- Clear_Path_Overlap — clears the path overlap list
- Find_Path — Find a path from point a to point b.
- Find_Path_Cell — Finds a given cell on a specified path
- Follow_Edge — Follow an edge to get around an impassable spot.
- FootClass::Unravel_Loop — Unravels a loop in the movement path
- Get_New_XY — Get the new x,y based on current position and direction.
- Optimize_Moves — Optimize the move list.
- Set_Path_Overlap — Sets the overlap bit for given cell

These entries preserve the source's own responsibility index. They do not establish all branch behavior, state changes, error paths, return contracts, or resource ownership.

## TD/RA comparison
The matching game variant exists in the other game directory. A true parity comparison requires inspecting both complete implementations and paired headers; do not assume the variants are identical.

## Rust mapping
No equivalent is asserted without inspecting the current Rust implementation and tests. Candidate port concerns: path search tie-breaking, occupancy updates, movement budget, and deterministic path optimization.

## Outstanding audit checklist
- [ ] Inspect paired header and enumerate all classes, structs, enums, fields, globals, constants, macros, and exact signatures.
- [ ] Explain each function body: parameters, return values, branches, loops, side effects, and failure paths.
- [ ] Trace callers/callees and shared global state.
- [ ] Compare TD/RA implementations where both exist.
- [ ] Map every behavior to Rust path and tests; distinguish implemented, partial, absent, and divergent.
