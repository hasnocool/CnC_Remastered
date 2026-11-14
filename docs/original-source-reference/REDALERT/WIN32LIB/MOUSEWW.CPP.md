# REDALERT/WIN32LIB/MOUSEWW.CPP — source guide

> **Audit status: partially reviewed.** Pinned source, direct includes, and original function index inspected. Header/member inventory, complete body explanations, native dependency graph, and Rust parity remain pending.

- Upstream: [REDALERT/WIN32LIB/MOUSEWW.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/WIN32LIB/MOUSEWW.CPP)
- Pinned source: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation unit size: 20098 characters.

## Purpose

Westwood mouse input adapter, including initialization and mouse-coordinate integration with the keyboard/input subsystem.

## Direct includes

- mouse.h
- mmsystem.h

## Original source function index

- WWMouseClass::WWMouseClass — Constructor for the Mouse Class

## Porting concerns

Input buffering and event order affect gameplay responsiveness. Separate platform input collection from deterministic simulation consumption; ensure thread-safe queues and avoid blocking polling.

## Outstanding audit
- [ ] Paired headers and exact types, members, macros, globals, and signatures.
- [ ] Every method's branch/loop/side-effect/error path.
- [ ] Native API interactions, allocation ownership, and call sites.
- [ ] Compare TD/RA versions.
- [ ] Rust modules and compatibility tests.
