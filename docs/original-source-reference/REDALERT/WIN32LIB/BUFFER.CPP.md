# REDALERT/WIN32LIB/BUFFER.CPP — source guide

> **Audit status: partially reviewed.** Pinned source, direct includes, and original function index inspected. Header/member inventory, complete body explanations, native dependency graph, and Rust parity remain pending.

- Upstream: [REDALERT/WIN32LIB/BUFFER.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/WIN32LIB/BUFFER.CPP)
- Pinned source: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation unit size: 7046 characters.

## Purpose

Generic buffer abstraction with constructors/destructor and ownership of a byte/pixel buffer; detailed allocation/locking contract is defined by buffer.h and callers.

## Direct includes

- buffer.h

## Original source function index



## Porting concerns

Inspect buffer.h for allocation size, pitch, ownership, and lock semantics. Rust should use explicit lifetimes/RAII and checked dimensions to avoid overflow.

## Outstanding audit
- [ ] Paired headers and exact types, members, macros, globals, and signatures.
- [ ] Every method's branch/loop/side-effect/error path.
- [ ] Native API interactions, allocation ownership, and call sites.
- [ ] Compare TD/RA versions.
- [ ] Rust modules and compatibility tests.
