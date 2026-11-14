# REDALERT MIXFILE.CPP — source guide

> **Audit status: partially reviewed.** This first pass inspects the pinned translation unit, its direct includes, and original source function index. Paired headers, exhaustive symbols, full body explanations, call graphs, and Rust parity remain outstanding.

- Upstream: [REDALERT/MIXFILE.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/MIXFILE.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation-unit size: 28494 characters.

## Plain-English system purpose

MIX archive access and cache management. Opens archives, retrieves named entries, reports offsets, finds matching archives, caches content, and releases cache/archive resources.

## Direct compile-time dependencies

- buff.h
- function.h
- direct.h
- fcntl.h
- io.h
- dos.h
- errno.h
- share.h
- mixfile.h
- cdfile.h

## Original function index

- MixFileClass::Cache — Caches the named mixfile into RAM.
- MixFileClass::Cache — Loads this particular mixfile's data into RAM.
- MixFileClass::Finder — Finds the mixfile object that matches the name specified.
- MixFileClass::Free — Uncaches a cached mixfile.
- MixFileClass::MixFileClass — Constructor for mixfile object.
- MixFileClass::Offset — Searches in mixfile for matching file and returns offset if found.
- MixFileClass::Retrieve — Retrieves a pointer to the specified data file.
- MixFileClass::~MixFileClass — Destructor for the mixfile object.

These entries preserve the source's own responsibility index. They do not establish all branch behavior, state changes, error paths, return contracts, or resource ownership.

## TD/RA comparison
The matching game variant exists in the other game directory. A true parity comparison requires inspecting both complete implementations and paired headers; do not assume the variants are identical.

## Rust mapping
No equivalent is asserted without inspecting the current Rust implementation and tests. Candidate port concerns: archive index parsing, offset/length validation, cache lifetime, and concurrent-safe reads.

## Outstanding audit checklist
- [ ] Inspect paired header and enumerate all classes, structs, enums, fields, globals, constants, macros, and exact signatures.
- [ ] Explain each function body: parameters, return values, branches, loops, side effects, and failure paths.
- [ ] Trace callers/callees and shared global state.
- [ ] Compare TD/RA implementations where both exist.
- [ ] Map every behavior to Rust path and tests; distinguish implemented, partial, absent, and divergent.
