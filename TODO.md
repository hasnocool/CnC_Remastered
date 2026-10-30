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
- [x] Port Red Alert RandomClass's 32-bit LCG and 15-bit output.
- [x] Port CRC-aware Pipe and Straw adapters.
- [x] Port base Pipe and Straw chain linking, forwarding, flushing, and unlinking.
- [x] Port Rect validity, intersection, anchor adjustment, and union.
- [x] Port packed BooleanVectorClass operations.
- [x] Port legacy CR/LF line reader and string trimming.
- [x] Port the legacy command-launcher stub output.
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
- [ ] Port the remaining 379 C++ translation units with source-specific tests.

- [ ] Build a symbol-to-module inventory for every source file in `TIBERIANDAWN` and `REDALERT`.
- [ ] Inventory the complete map-editor model, tools, renderers, dialogs, and serialization paths.
- [ ] Identify every serialized structure, integer width, packing rule, endianness assumption, and version gate.
- [ ] Implement a source-compatible filename CRC routine and test it against upstream vectors.
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

## Milestone 4 — platform, rendering, audio, and input

- [ ] Replace Win32, DirectDraw, DirectSound, WinMM, sockets, and MASM dependencies behind portable interfaces.
- [ ] Reproduce palette-accurate rendering, scaling, viewport, sidebar, radar, animations, and theater handling.
- [ ] Port audio playback, mixing, music, speech, and timing behavior.
- [ ] Port keyboard, mouse, controller-independent input mapping, window lifecycle, and configuration.

## Milestone 5 — networking and online integration

- [ ] Port packet layouts, framing, checksums, session state, lobby behavior, and lockstep synchronization.
- [ ] Verify compatibility boundaries and document services that are no longer available.
- [ ] Keep platform services optional and isolated from simulation determinism.

## Milestone 6 — map editor

- [ ] Port map model and all map section codecs.
- [ ] Port terrain/template/overlay/smudge layers, object placement, waypoints, houses, triggers, team types, and briefing metadata.
- [ ] Port selection, painting, transforms, property editing, undo/redo, validation, and preview rendering.
- [ ] Replace Windows Forms UI with a portable editor frontend without changing map semantics.
- [ ] Verify load-save-load equivalence against legal fixtures.

## Milestone 7 — parity and release

- [ ] Run golden scenario tests for both game variants.
- [ ] Compare tick-by-tick state, map serialization, object transitions, and deterministic replays.
- [ ] Complete gameplay, asset-format, editor, rendering, audio, and networking parity matrices.
- [ ] Document known deviations and supported data versions.
- [ ] Make a release only after license/trademark review and full verification.


- [x] Port `TIBERIANDAWN/WIN32LIB/NEWDEL.CPP` to `rts-core::legacy_new_delete`.

- [x] Port `TIBERIANDAWN/WIN32LIB/PALETTE.CPP` and `MORPHPAL.CPP` palette set/fade/morph routines.

- [x] Port `REDALERT/WIN32LIB/WRITEPCX.CPP` PCX header, RLE scanline encoder, and palette trailer.

- [x] Port `REDALERT/WIN32LIB/LOADFONT.CPP` font-file loading and validation to `rts-core::load_font`.

- [x] Share `rts-core::load_font` across both variants' `LOADFONT.CPP` implementations.

- [x] Port byte-identical Red Alert/Tiberian Dawn `IFF.CPP` FORM chunk read/write behavior to `rts-formats::iff`.

- [x] Port byte-identical Red Alert/Tiberian Dawn `MOUSEWW.CPP` active mouse state and conditional-hide behavior to `rts-core::mouse_ww`.

- [x] Port byte-identical Red Alert/Tiberian Dawn `LOAD.CPP` routines to `rts-formats::load`, including bounded file I/O, compression dispatch, reserved skip data, and scratch-tail staging.

- [x] Port both variants' packed `GETSHAPE.CPP` metadata, shape-table extraction, prefix data and height mutation routines to `rts-core::shape_access`.

- [x] Port both variants' IFF-based `ICONSET.CPP` loader and packed control structure to `rts-formats::iconset`.

- [x] Port byte-identical Red Alert/Tiberian Dawn `WINDOWS.CPP` text window state, control codes, dipthong decoding, substitutions, wrapping, scrolling and more-prompt operations to `rts-formats::window_text`.

- [x] Port byte-identical Red Alert/Tiberian Dawn `GBUFFER.CPP` buffer/viewport geometry, DirectDraw state, lock nesting, restore handling and blit geometry to `rts-core::graphic_buffer`.

- [x] Record both include-only CREW.CPP translation units and both #if (0)-disabled DPMI.CPP translation units as having no active implementation.
- [x] Record REDALERT/MPU.CPP as inactive under the pinned MSVC target and port REDALERT/_WSPROTO.CPP's null PacketTransport initializer.

- [x] Record REDALERT/BIGCHECK.CPP as inactive in the pinned project because WOLAPI_INTEGRATION is not defined.

- [x] Port REDALERT/GETCPU.CPP CPU/MMX/vendor wrapper and preserve bounded vendor-ID copying in `rts-core::cpu_info`.

- [x] Port both variants' CHECKBOX.CPP drawing gates, frames, checked visuals and Red Alert LEFTRELEASE action.

- [x] Port both variants' FLASHER.CPP per-player countdowns, IsBlushing state, flashing bitmask and debug text.

- [x] Port both FLY.CPP variants' speed scaling, pixel-lepton movement accumulation, coordinate movement and edge rollback to `rts-core::fly`.
