# Tiberian Dawn REINF.CPP — source guide

> **Audit status: partially reviewed.** This guide uses the pinned upstream translation unit and its original function index. Exact declarations, all members, full function semantics, callers, and Rust parity remain unaudited.

- Source: [TIBERIANDAWN/REINF.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/REINF.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 26,376 characters.

## Plain-English purpose

Implements reinforcement delivery and scenario-driven unit deployment, including the coordination needed to bring newly spawned forces into play.

## Direct compile-time dependencies

- `function.h`

## Original source function index

- Create_Air_Reinforcement — Creates air strike reinforcement                              *
- Create_Special_Reinforcement — Ad hoc reinforcement handler.                             *
- Do_Reinforcements — Create and place a reinforcement team.                               *

Original index text records intended responsibilities, not verified body behavior.

## Outstanding audit

- [ ] Enumerate all declarations, fields, globals, constants, macros, and exact signatures.
- [ ] Explain every function body, parameter, return value, branch, state change, and side effect.
- [ ] Trace call sites, shared state, and cross-system dependencies.
- [ ] Compare Red Alert counterpart where applicable.
- [ ] Map each behavior to Rust code and tests with explicit parity status.
