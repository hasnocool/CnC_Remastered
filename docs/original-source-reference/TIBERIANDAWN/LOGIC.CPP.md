# Tiberian Dawn LOGIC.CPP — source guide

> **Audit status: partially reviewed.** Grounded in the pinned upstream translation unit and its original function index. Exact header members, signatures, full function-body explanations, call graph, and Rust parity remain pending.

- Source: [TIBERIANDAWN/LOGIC.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/LOGIC.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 14,884 characters.

## Plain-English purpose

Runs the frame-level AI orchestration. The documented AI pass processes teams, then sentient objects, then rebuilds house scan bitfields; crate regeneration and optional debug/performance reporting also appear in this translation unit.

## Direct compile-time dependencies

- `function.h`
- `logic.h`

## Original source function index

- LogicClass::AI — Handles AI logic processing for game objects.
- LogicClass::Debug_Dump — Displays logic class status to the mono screen.

These are original source index descriptions, not a substitute for reading each body. Do not infer complete branch behavior, return contracts, or side effects from names alone.

## Outstanding symbol and implementation audit

- [ ] Classes, structs, enums, aliases, macros, globals, constants.
- [ ] Every field/member with type, visibility, initialization, and purpose.
- [ ] Exact function signatures, overloads, parameters, and return values.
- [ ] Every function body: branches, loops, state changes, side effects, and failure paths.
- [ ] Important callers, callees, shared state, and ownership assumptions.
- [ ] Red Alert counterpart differences.
- [ ] Rust path and test mapping, with implemented/partial/absent status.
