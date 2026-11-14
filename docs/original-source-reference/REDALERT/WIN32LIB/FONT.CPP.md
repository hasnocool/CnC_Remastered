# REDALERT/WIN32LIB/FONT.CPP — source guide

> **Audit status: partially reviewed.** Pinned source, direct includes, and original function index inspected. Header/member inventory, complete body explanations, native dependency graph, and Rust parity remain pending.

- Upstream: [REDALERT/WIN32LIB/FONT.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/WIN32LIB/FONT.CPP)
- Pinned source: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation unit size: 6933 characters.

## Purpose

Text metric helpers for character and string pixel widths and next text-print coordinate calculations.

## Direct includes

- font.h
- malloc.h
- dos.h
- fcntl.h
- io.h
- sys\stat.h
- string.h
- wwstd.h

## Original source function index



## Porting concerns

Pixel metrics must match original font glyph tables and encoding; use golden fixtures for kerning/width and clipping.

## Outstanding audit
- [ ] Paired headers and exact types, members, macros, globals, and signatures.
- [ ] Every method's branch/loop/side-effect/error path.
- [ ] Native API interactions, allocation ownership, and call sites.
- [ ] Compare TD/RA versions.
- [ ] Rust modules and compatibility tests.
