# TIBERIANDAWN/WIN32LIB/GBUFFER.CPP — platform source guide

> **Audit status: partially reviewed.** Translation unit, direct includes, and original source function index inspected. Paired headers, all declarations and function-body details, platform call graph, and Rust parity remain outstanding.

- Upstream: [TIBERIANDAWN/WIN32LIB/GBUFFER.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/WIN32LIB/GBUFFER.CPP)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 32817 characters.

## Purpose

Graphics-buffer and viewport abstraction: allocates/configures graphic buffers, initializes DirectDraw-backed state, locks/unlocks memory, changes viewport geometry, and selects the logic page.

## Direct includes

- gbuffer.h
- misc.h

## Original function index



## Platform and Rust-port considerations

Buffer lock/unlock must be treated as a resource-lifetime invariant. Preserve viewport clipping, pitch/stride, device initialization and failure cleanup. A Rust backend should isolate DirectDraw/Win32 specifics behind a renderer abstraction and use RAII guards for mapped/locked buffers.

## Outstanding audit
- [ ] Header declarations, classes, fields, constants, macros, and exact signatures.
- [ ] Explain each function body and error/cleanup path.
- [ ] Trace platform APIs and call sites in game code.
- [ ] Compare TD and RA source differences.
- [ ] Identify Rust modules/tests and classify parity.
