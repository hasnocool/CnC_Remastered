# REDALERT/WIN32LIB/ICONSET.CPP — platform source guide

> **Audit status: partially reviewed.** Translation unit, direct includes, and original source function index inspected. Paired headers, all declarations and function-body details, platform call graph, and Rust parity remain outstanding.

- Upstream: [REDALERT/WIN32LIB/ICONSET.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/WIN32LIB/ICONSET.CPP)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 12106 characters.

## Purpose

Loads and frees IFF-backed icon/template set data using the file and tile abstractions. This is a low-level asset-loading boundary rather than the game-level map renderer.

## Direct includes

- stdlib.h
- stdio.h
- wwstd.h
- file.h
- tile.h
- iff.h

## Original function index



## Platform and Rust-port considerations

IFF parsing must validate chunk boundaries, dimensions, icon counts, and offset arithmetic before allocation or access. Ownership of loaded icon data must be explicit; concurrent reads should not mutate shared buffers unsafely.

## Outstanding audit
- [ ] Header declarations, classes, fields, constants, macros, and exact signatures.
- [ ] Explain each function body and error/cleanup path.
- [ ] Trace platform APIs and call sites in game code.
- [ ] Compare TD and RA source differences.
- [ ] Identify Rust modules/tests and classify parity.
