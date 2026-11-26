# Port plan and source of truth

## Milestone 0 — compatibility foundations

- [x] Create a Rust workspace with independent variant crates.
- [x] Encode source-defined TD and RA global map extents.
- [x] Encode the source-defined 15 simulation ticks per second.
- [x] Add cell/region addressing foundations and 8-way facing IDs.
- [x] Add a bounds-checked legacy MIX parser based on upstream `MixFileClass`.
- [x] Add a deterministic ordered event queue.
- [x] Port both variants' Sim_Random lookup table and Sim_IRandom routines.
- [x] Port Red Alert Q16.16 fixed-point C++ implementation.
- [x] Port Red Alert's legacy CRCEngine rotate-and-add checksum.
- [x] Port CRC-aware Pipe and Straw adapters.
- [x] Port base Pipe and Straw chain linking, forwarding, flushing, and unlinking.
- [x] Port Rect validity, intersection, anchor adjustment, and union.
- [x] Port packed BooleanVectorClass operations.
- [x] Port legacy CR/LF line reader and string trimming.
- [x] Port ADPCM type-1 decode, including the original lookup tables.
- [x] Port LCW compression/decompression and framed Pipe/Straw block streams.
- [x] Port Westwood 12-bit LZW and framed Pipe/Straw streams.
- [x] Port deterministic LZO1X and framed Pipe/Straw streams.
- [x] Port Base64 encoding/decoding and Pipe/Straw adapters.
- [x] Port SHA-1 engine and RandomStraw entropy mixer.
- [x] Port Blowfish engine and 8-byte Pipe/Straw adapters.
- [x] Add a starter editor document model.
- [x] Port the map-editor cell metric operations: row-major conversion, 8-way adjacency, and clipping.
- [x] Port the map-editor standard CRC-32 helper.
- [ ] Run formatting, tests, and Clippy with an installed Rust toolchain.

## Milestone 1 — behavior inventory and fixtures

- [x] Inventory all 494 upstream C++ translation units by exact path.
- [x] Port the currently completed source units recorded in the source inventory and porting status.
- [ ] Finish `REDALERT/AIRCRAFT.CPP` and the remaining 213 translation units.
- [ ] Build a symbol-to-module inventory for every source file in `TIBERIANDAWN` and `REDALERT`.
- [ ] Inventory the complete map-editor model, tools, renderers, dialogs, and serialization paths.
- [ ] Identify every serialized structure, integer width, packing rule, endianness assumption, and version gate.
- [ ] Create legal, minimal binary fixtures for each supported format; do not include proprietary game assets.
- [ ] Record exact behavior and compatibility tests before porting each subsystem.

## Milestone 2 — core runtime and formats

- [ ] Port fixed-point arithmetic, coordinate/lepton conversion, direction/facing math, deterministic RNG, timer behavior, and memory/container primitives.
- [ ] Complete MIX support, including all upstream variants and edge cases.
- [ ] Port SHP, TMP, PAL, CSF, scenario/map, save-game, briefing, and theater data formats.
- [ ] Add round-trip and malformed-input tests for every parser and writer.

## Milestone 3 — deterministic simulation

- [ ] Port object/type hierarchies and stable object identifiers.
- [ ] Port map cells, occupancy/overlap, zones, regions, movement, pathfinding, and collision rules.
- [ ] Port infantry, vehicles, aircraft, buildings, projectiles, terrain, overlays, smudges, animations, and damage states.
- [ ] Port house economy, power, production, repair, selling, harvesting, cargo, transport, and special weapons.
- [ ] Port missions, triggers, teams, team missions, scenario progression, campaign state, and AI.
- [ ] Keep TD-specific and RA-specific differences explicit and testable.
- [ ] Verify deterministic replay and state hashes across repeated runs.

## Milestone 4 — Application Runtime & Rendering Foundation

### Renderer — `crates/renderer`

- [x] Add a dedicated renderer crate and isolate it from authoritative simulation state.
- [x] Add a portable wgpu window-surface backend with adapter/device selection and presentation.
- [x] Add viewport, camera, color, render-layer, texture-handle, sprite-instance, and stable sprite-batch abstractions.
- [x] Add frame timing and resize handling.
- [ ] Add texture upload/cache lifecycle and palette-aware SHP/TMP rendering.
- [ ] Execute ordered sprite batches with GPU pipelines and shaders.
- [ ] Add headless/offscreen renderer tests and platform-specific surface tests.

### Launcher — `crates/launcher`

- [x] Add a unified executable and application registry for Tiberian Dawn, Red Alert, and Map Editor.
- [x] Add application selection via command line and interactive terminal menu.
- [x] Add shared window lifecycle, renderer startup, resize, redraw, and graceful shutdown.
- [x] Add versioned TOML configuration for display, audio, input, performance, and networking settings.
- [x] Add validated, atomic settings persistence and structured logging.
- [ ] Add an in-window launcher menu and editable settings UI.
- [ ] Connect each application registry entry to its game/editor runtime once those runtime APIs exist.
- [ ] Add async networking services, session discovery, cancellation, and shutdown integration.
- [ ] Add startup diagnostics and recovery for unavailable GPU adapters.

### Integration and verification

- [ ] Integrate the Tiberian Dawn runtime, Red Alert runtime, and map editor into the launcher lifecycle.
- [ ] Add non-blocking networking interfaces and versioned network/session configuration.
- [ ] Run `cargo fmt --all -- --check`.
- [ ] Run `cargo check --workspace --all-targets`.
- [ ] Run `cargo test --workspace`.
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Verify the renderer on Vulkan, Metal, and DX12 where supported; document unsupported platforms.
- [ ] Record actual build/test output and remaining failures in `PORTING_STATUS.md`.

## Milestone 5 — platform, rendering, audio, and input

- [ ] Replace Win32, DirectDraw, DirectSound, WinMM, sockets, and MASM dependencies behind portable interfaces.
- [ ] Reproduce palette-accurate rendering, scaling, viewport, sidebar, radar, animations, and theater handling.
- [ ] Port audio playback, mixing, music, speech, and timing behavior.
- [ ] Port keyboard, mouse, controller-independent input mapping, window lifecycle, and configuration.

## Milestone 6 — networking and online integration

- [ ] Port packet layouts, framing, checksums, session state, lobby behavior, and lockstep synchronization.
- [ ] Verify compatibility boundaries and document services that are no longer available.
- [ ] Keep platform services optional and isolated from simulation determinism.

## Milestone 7 — map editor

- [ ] Port map model and all map section codecs.
- [ ] Port terrain/template/overlay/smudge layers, object placement, waypoints, houses, triggers, team types, and briefing metadata.
- [ ] Port selection, painting, transforms, property editing, undo/redo, validation, and preview rendering.
- [ ] Replace Windows Forms UI with a portable editor frontend without changing map semantics.
- [ ] Verify load-save-load equivalence against legal fixtures.

## Milestone 8 — parity and release

- [ ] Run golden scenario tests for both game variants.
- [ ] Compare tick-by-tick state, map serialization, object transitions, and deterministic replays.
- [ ] Complete gameplay, asset-format, editor, rendering, audio, and networking parity matrices.
- [ ] Document known deviations and supported data versions.
- [ ] Make a release only after license/trademark review and full verification.
