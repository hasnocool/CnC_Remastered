# TIBERIANDAWN/WIN32LIB/PALETTE.CPP — platform source guide

> **Audit status: partially reviewed.** Translation unit, direct includes, and original source function index inspected. Paired headers, all declarations and function-body details, platform call graph, and Rust parity remain outstanding.

- Upstream: [TIBERIANDAWN/WIN32LIB/PALETTE.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/WIN32LIB/PALETTE.CPP)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 15277 characters.

## Purpose

Palette programming and visual transitions: sets full palettes or individual colors, fades to a target palette, computes bump rates, and animates palette bumping.

## Direct includes

- palette.h
- timer.h
- wwstd.h

## Original function index



## Platform and Rust-port considerations

Palette fade/bump operations are clock-dependent visual state. Use explicit time inputs for deterministic tests and replay; separate legacy palette behavior from modern shader/color-pipeline behavior.

## Outstanding audit
- [ ] Header declarations, classes, fields, constants, macros, and exact signatures.
- [ ] Explain each function body and error/cleanup path.
- [ ] Trace platform APIs and call sites in game code.
- [ ] Compare TD and RA source differences.
- [ ] Identify Rust modules/tests and classify parity.
