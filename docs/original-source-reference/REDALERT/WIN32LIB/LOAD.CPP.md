# REDALERT/WIN32LIB/LOAD.CPP — source guide

> **Audit status: partially reviewed.** Pinned source, direct includes, and original function index inspected. Header/member inventory, complete body explanations, native dependency graph, and Rust parity remain pending.

- Upstream: [REDALERT/WIN32LIB/LOAD.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/WIN32LIB/LOAD.CPP)
- Pinned source: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation unit size: 17744 characters.

## Purpose

Resource file loading and writing, including allocation-aware loads and decompression helpers. This sits between file/archive access and decoded game assets.

## Direct includes

- iff.h
- file.h
- misc.h
- wwstd.h
- dos.h
- wwmem.h

## Original source function index



## Porting concerns

Preserve decompression format, expected output sizes, short-read behavior, and archive/file ownership. Test corrupt and truncated assets.

## Outstanding audit
- [ ] Paired headers and exact types, members, macros, globals, and signatures.
- [ ] Every method's branch/loop/side-effect/error path.
- [ ] Native API interactions, allocation ownership, and call sites.
- [ ] Compare TD/RA versions.
- [ ] Rust modules and compatibility tests.
