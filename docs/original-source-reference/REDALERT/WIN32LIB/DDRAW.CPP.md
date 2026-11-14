# REDALERT/WIN32LIB/DDRAW.CPP — source guide

> **Audit status: partially reviewed.** Pinned source, direct includes, and original function index inspected. Header/member inventory, complete body explanations, native dependency graph, and Rust parity remain pending.

- Upstream: [REDALERT/WIN32LIB/DDRAW.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/WIN32LIB/DDRAW.CPP)
- Pinned source: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation unit size: 51508 characters.

## Purpose

DirectDraw video subsystem: mode selection/reset, surface capability checks, hardware memory/capability queries, vertical-blank/blit waits, palette integration, and surface lifecycle tracking.

## Direct includes

- misc.h
- gbuffer.h
- palette.h
- stdio.h

## Original source function index



## Porting concerns

Legacy DirectDraw surfaces, cooperative display mode, lost-surface restoration, blit synchronization, and palette state must be separated behind a platform renderer. Do not carry forward busy-waiting or blocking waits on async/UI threads.

## Outstanding audit
- [ ] Paired headers and exact types, members, macros, globals, and signatures.
- [ ] Every method's branch/loop/side-effect/error path.
- [ ] Native API interactions, allocation ownership, and call sites.
- [ ] Compare TD/RA versions.
- [ ] Rust modules and compatibility tests.
