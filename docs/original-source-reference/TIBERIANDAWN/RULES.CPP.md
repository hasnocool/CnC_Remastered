# Tiberian Dawn RULES.CPP — source guide

> **Audit status: partially reviewed.** This guide uses the pinned upstream translation unit and its original function index. Exact declarations, all members, full function semantics, callers, and Rust parity remain unaudited.

- Source: [TIBERIANDAWN/RULES.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/RULES.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 6,884 characters.

## Plain-English purpose

Defines and initializes gameplay rule data used to configure units, buildings, weapons, costs, timing, and other balance parameters.

## Direct compile-time dependencies

- `function.h`

## Original source function index

- DifficultyClass::DifficultyClass — Default constructor for difficulty class object.
- RulesClass::RulesClass — Default constructor for rules class object.

Original index text records intended responsibilities, not verified body behavior.

## Outstanding audit

- [ ] Enumerate all declarations, fields, globals, constants, macros, and exact signatures.
- [ ] Explain every function body, parameter, return value, branch, state change, and side effect.
- [ ] Trace call sites, shared state, and cross-system dependencies.
- [ ] Compare Red Alert counterpart where applicable.
- [ ] Map each behavior to Rust code and tests with explicit parity status.
