# REDALERT WARHEAD.CPP — source guide

> **Audit status: partially reviewed.** This first pass inspects the pinned translation unit, its direct includes, and original source function index. Paired headers, exhaustive symbols, full body explanations, call graphs, and Rust parity remain outstanding.

- Upstream: [REDALERT/WARHEAD.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/WARHEAD.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation-unit size: 12287 characters.

## Plain-English system purpose

Red Alert warhead-type definitions, INI loading, allocation, pointer conversion, and damage-property configuration. Warhead data is consumed by combat damage resolution.

## Direct compile-time dependencies

- function.h

## Original function index

- WarheadTypeClass::As_Pointer — Convert a warhead type number into a pointer.
- WarheadTypeClass::Read_INI — Fetches the warhead data from the INI database.
- WarheadTypeClass::WarheadTypeClass — Default constructor for warhead objects.
- WarheadTypeClass::operator delete — Returns warhead object back to special memory pool.
- WarheadTypeClass::operator new — Allocate a warhead object from the special heap.

These entries preserve the source's own responsibility index. They do not establish all branch behavior, state changes, error paths, return contracts, or resource ownership.

## TD/RA comparison
This translation unit is present in Red Alert; no corresponding Tiberian Dawn file with this exact name was found in the pinned source tree.

## Rust mapping
No equivalent is asserted without inspecting the current Rust implementation and tests. Candidate port concerns: warhead damage tables, armor interactions, and INI default compatibility.

## Outstanding audit checklist
- [ ] Inspect paired header and enumerate all classes, structs, enums, fields, globals, constants, macros, and exact signatures.
- [ ] Explain each function body: parameters, return values, branches, loops, side effects, and failure paths.
- [ ] Trace callers/callees and shared global state.
- [ ] Compare TD/RA implementations where both exist.
- [ ] Map every behavior to Rust path and tests; distinguish implemented, partial, absent, and divergent.
