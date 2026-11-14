# TIBERIANDAWN/WIN32LIB/IFF.CPP — source guide

> **Audit status: partially reviewed.** Pinned source, direct includes, and original function index inspected. Header/member inventory, complete body explanations, native dependency graph, and Rust parity remain pending.

- Upstream: [TIBERIANDAWN/WIN32LIB/IFF.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/WIN32LIB/IFF.CPP)
- Pinned source: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation unit size: 13976 characters.

## Purpose

IFF container file operations: open/close, chunk-size lookup, chunk read, and chunk write.

## Direct includes

- iff.h
- file.h

## Original source function index



## Porting concerns

Validate FORM/chunk lengths, endianness, padding, and stream bounds; malformed chunk sizes must fail safely without partial out-of-bounds reads.

## Outstanding audit
- [ ] Paired headers and exact types, members, macros, globals, and signatures.
- [ ] Every method's branch/loop/side-effect/error path.
- [ ] Native API interactions, allocation ownership, and call sites.
- [ ] Compare TD/RA versions.
- [ ] Rust modules and compatibility tests.
