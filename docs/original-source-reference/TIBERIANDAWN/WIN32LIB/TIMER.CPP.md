# TIBERIANDAWN/WIN32LIB/TIMER.CPP — platform source guide

> **Audit status: partially reviewed.** Translation unit, direct includes, and original source function index inspected. Paired headers, all declarations and function-body details, platform call graph, and Rust parity remain outstanding.

- Upstream: [TIBERIANDAWN/WIN32LIB/TIMER.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/WIN32LIB/TIMER.CPP)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 10496 characters.

## Purpose

Timer/timebase primitives: obtains elapsed time/ticks and starts, stops, sets, and resets timer state. These primitives underpin simulation timing and frame pacing.

## Direct includes

- wwstd.h
- timer.H
- stdio.h
- stdlib.h

## Original function index



## Platform and Rust-port considerations

Establish tick units, clock source, wraparound, pause/resume, and monotonicity. Avoid blocking waits in modern async code; simulation time should be distinct from wall-clock and audio clocks.

## Outstanding audit
- [ ] Header declarations, classes, fields, constants, macros, and exact signatures.
- [ ] Explain each function body and error/cleanup path.
- [ ] Trace platform APIs and call sites in game code.
- [ ] Compare TD and RA source differences.
- [ ] Identify Rust modules/tests and classify parity.
