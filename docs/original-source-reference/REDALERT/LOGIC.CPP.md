# Red Alert LOGIC.CPP — source guide

> **Audit status: partially reviewed.** Grounded in the pinned Red Alert translation unit and its original function index. Exact paired-header members, complete body explanations, call graphs, and Rust parity are still pending.

- Source: [REDALERT/LOGIC.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/LOGIC.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 18,735 characters.

## Plain-English purpose

Coordinates the Red Alert frame-level AI pass across teams and live objects, plus periodic strategic updates and scan-state bookkeeping. The exact scheduling and order are defined by this translation unit.

## Direct compile-time dependencies

- `function.h`
- `logic.h`
- `vortex.h`

## Original source function index

- LogicClass::AI — Handles AI logic processing for game objects.
- LogicClass::Debug_Dump — Displays logic class status to the mono screen.
- LogicClass::Detach — Detatch the specified target from the logic system.

The original index is a responsibility map, not a semantic substitute for reading the full function body.

## Outstanding audit

- [ ] Complete declarations, members, globals, constants, and exact signatures.
- [ ] Explain each function body, including branches, state changes, return behavior, and side effects.
- [ ] Trace call sites and shared state.
- [ ] Compare exact behavior against the Tiberian Dawn counterpart.
- [ ] Map to Rust modules and tests with explicit parity status.
