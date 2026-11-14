# REDALERT COMBAT.CPP — source guide

> **Audit status: partially reviewed.** This first pass inspects the pinned translation unit, its direct includes, and original source function index. Paired headers, exhaustive symbols, full body explanations, call graphs, and Rust parity remain outstanding.

- Upstream: [REDALERT/COMBAT.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/COMBAT.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation-unit size: 21703 characters.

## Plain-English system purpose

Combat damage calculations and explosion resolution. TD exposes damage modification and explosion damage; RA extends this with combat animation and wide-area damage. Exact armor/warhead scaling and object iteration must be read from function bodies.

## Direct compile-time dependencies

- function.h

## Original function index

- Combat_Anim — Determines explosion animation to play.
- Explosion_Damage — Inflict an explosion damage affect.
- Modify_Damage — Adjusts damage to reflect the nature of the target.
- Wide_Area_Damage — Apply wide area damage to the map.

These entries preserve the source's own responsibility index. They do not establish all branch behavior, state changes, error paths, return contracts, or resource ownership.

## TD/RA comparison
The matching game variant exists in the other game directory. A true parity comparison requires inspecting both complete implementations and paired headers; do not assume the variants are identical.

## Rust mapping
No equivalent is asserted without inspecting the current Rust implementation and tests. Candidate port concerns: integer damage arithmetic, armor/warhead modifiers, distance falloff, and deterministic explosion target enumeration.

## Outstanding audit checklist
- [ ] Inspect paired header and enumerate all classes, structs, enums, fields, globals, constants, macros, and exact signatures.
- [ ] Explain each function body: parameters, return values, branches, loops, side effects, and failure paths.
- [ ] Trace callers/callees and shared global state.
- [ ] Compare TD/RA implementations where both exist.
- [ ] Map every behavior to Rust path and tests; distinguish implemented, partial, absent, and divergent.
