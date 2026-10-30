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
- [ ] Port the remaining 261 C++ translation units with source-specific tests.

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

- [x] Port both IPX95.CPP disabled THIPX32 providers and fixed stub return values to `rts-core::ipx95`.

- [x] Port REDALERT/BMP8.CPP 8-bit bitmap header, palette, pixel data, ownership and draw-plan behavior to `rts-formats::bmp8`.

- [x] Port REDALERT/CCPTR.CPP stable heap-ID pointer construction, lookup, name ordering and all source relational operator expressions.

- [x] Record REDALERT/SURFACE.CPP as inactive because the entire implementation is guarded by #if (0).

- [x] Port both TXTLABEL.CPP constructors, Set_Text and variant-specific drawing dispatch to `rts-core::text_label`.

- [x] Record REDALERT/WOLEDIT.CPP and WOL_OPT.CPP as inactive because WOLAPI_INTEGRATION is not defined in the pinned Red Alert project.

- [x] Record RAWOLAPI, WOLAPIOB, WOL_CGAM, WOL_CHAT, WOL_DNLD, WOL_GSUP, WOL_LOGN and WOL_MAIN as inactive because WOLAPI_INTEGRATION is not defined in the pinned project.

- [x] Record REDALERT/INICODE.CPP as inactive because the entire implementation is guarded by #ifdef TOFIX.

- [x] Port both DESCDLG.CPP dialog geometry, control initialization, redraw loop, OK/cancel text handling and variant-specific redraw behavior.

- [x] Port REDALERT/MPLIB.CPP interrupt register packing, MGen ordinal calls, queue/node/master operations and return-register semantics to `rts-core::mgen`.

- [x] Port all 187 ENGLISH branch TXT_WOL resources from REDALERT/WOLSTRNG.CPP.

- [x] Record REDALERT/ENDING.CPP as inactive because both function bodies are guarded by #ifdef NEVER.

- [x] Port TIBERIANDAWN/ENDING.CPP GDI ending, Nod selection loop, audio/input handling, cleanup, trailers and attract sequences.

- [x] Port TIBERIANDAWN/RULES.CPP DifficultyClass and RulesClass constructor defaults.

- [x] Port REDALERT/KEYBOARD.CPP ring buffer, Windows key modifiers, mouse events, ASCII translation and message handling.

- [x] Port TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP ring buffer, VK/ASCII/toggle remaps, modifier logic, mouse triple handling and compatibility key conversions.

- [x] Port REDALERT/MPMGRD.CPP private/global queue packet routing, connection management and MGen transport integration.
- [x] Record REDALERT/MPMGRW.CPP as inactive because its complete implementation is #if (0)-disabled.

- [x] Port both FIELD.CPP variants' scalar/string/chunk constructors and Host_To_Net/Net_To_Host conversion behavior.

- [x] Port REDALERT/TRACKER.CPP ordered target detachment, singleton cleanup and dependent trigger recursion.

- [x] Port TIBERIANDAWN/RULES.CPP DifficultyClass and RulesClass default constructor values.
- [x] Port TIBERIANDAWN/WIN32LIB/MEM.CPP paragraph pool initialization, allocation/free, ID lookup, LRU/lock/in-use markers, free-block merging, capacity queries and cleanup compaction.

- [x] Port REDALERT/DROP.CPP dropdown list geometry, item selection, peer routing, expansion/collapse and lifecycle behavior.

- [x] Port REDALERT/UDPADDR.CPP active false-return stub and record MCI/MCIMOVIE as inactive under the pinned MCIMPEG-disabled project.

- [x] Port REDALERT/CARRY.CPP techno snapshot and recreation logic, including infantry ScenarioInit handling and unconditional false return.

- [x] Port both CARGO.CPP variants' LIFO linked cargo chain, Limbo, attach, detach and quantity behavior.

- [x] Port both TOGGLE.CPP variants' forced redraw, sticky processing, press capture and variant-specific release toggling.

- [x] Record REDALERT/TOOLTIP.CPP as inactive because its complete implementation is guarded by #if (0).

- [x] Port both SCROLL.CPP variants' constructor/timer setup, autoscroll state, Init_IO and active HelpClass::AI delegation.

- [x] Port both SHAPEBTN.CPP variants' shape assignment, draw gate, frame selection and conditional mouse visibility.

- [x] Port both MiscAsm.cpp variants' coordinate/facing algorithms, fixed conversion, bit arrays, palette fading, endian helpers, string trim, fat-pixel output, palette range and screen-shake fallback.

- [x] Port REDALERT/KEY.CPP ring-buffer operations, mouse triplets, keyboard modifiers, ASCII conversion adapter, async key state and system-message pump.

- [x] Port REDALERT/MOUSE.CPP and TIBERIANDAWN/MOUSE.CPP cursor shape state, exact 42/34 entry animation tables, small cursor behavior, resource loading hooks and ScrollClass delegation.

- [x] Port both INTERPAL.CPP variants' palette table read/write, Red Alert midpoint nearest-color builder, Tiberian Dawn disabled builder, palette luminance scaling and disabled 2x interpolation entry point.

- [x] Port REDALERT/RGB.CPP, HSV.CPP, and PALETTEC.CPP into rgb_palette.rs, including RGB/HSV conversions, exact device-channel arithmetic, 256-color palette operations and timed host-driven palette fades.

- [x] Port REDALERT/2KEYFRAM.CPP and TIBERIANDAWN/KEYFRAME.CPP: packed animation headers, LCW decode, XOR delta streams, frame palette extraction, cached shape headers, theater reset and variant-specific buffer allocation policy.

- [x] Verify REDALERT/DEBUG.CPP and TIBERIANDAWN/DEBUG.CPP are entirely CHEAT_KEYS-guarded and that the pinned project configurations/headers do not define CHEAT_KEYS; record both as intentionally inactive.

- [x] Verify TIBERIANDAWN/ALLOC.CPP has an unconditional #if (0) around its implementation and record it as intentionally inactive.

- [x] Port REDALERT/NULLCONN.CPP active constructor/initialization, staging-buffer sizing, no-send stub, custom CRC and packet-overhead calculation; verify TIBERIANDAWN/NULLCONN.CPP is wholly #if (0).

- [x] Port REDALERT/NOSEQCON.CPP and TIBERIANDAWN/NOSEQCON.CPP: CommHeader wire layout, ACK/NOACK packet IDs, resend/duplicate suppression, out-of-order buffering, cumulative sequencing, retry/timeout queue servicing, response-delay statistics and variant-specific receive bounds.

- [x] Port REDALERT/DIAL8.CPP and TIBERIANDAWN/DIAL8.CPP geometry, input handling, shared click/drag state, direction mapping, parent ControlClass callbacks, draw gating, mouse visibility and variant-specific dial styling.

- [x] Port REDALERT/CHEKLIST.CPP and TIBERIANDAWN/CHEKLIST.CPP: check-item storage variants, text lookup/removal, read-only behavior, parent-first mouse action, checked-state toggling and variant-specific row rendering.

- [x] Port TIBERIANDAWN/VECTOR.CPP and its header's Raw_Count() and mutable raw-context interfaces using the shared vector and packed BooleanVector Rust implementations.

- [x] Port REDALERT/COLRLIST.CPP and TIBERIANDAWN/COLRLIST.CPP per-row color storage, text/localized additions, removal, selection style, highlight/box/bar/invert rendering, gradient text flags and variant-specific selected-color semantics.

- [x] Port REDALERT/TEXTBTN.CPP and TIBERIANDAWN/TEXTBTN.CPP default/string/localized constructors, text resize, box/background styles, black border, conditional mouse handling, gradient remap/text color behavior and WOLAPI branch.

- [x] Port both identical WIN32LIB/DDRAW.CPP translation units: active DirectDraw globals, capabilities, Set_DD_Palette conversion, Wait_Vert_Blank, Process_DD_Result and 20-entry SurfaceMonitorClass; Set_Video_Mode is the source's unconditional TRUE stub.

- [x] Port REDALERT/CCDDE.CPP and TIBERIANDAWN/CCDDE.CPP DDE server identity, callback lifecycle, advisory packets, multiplayer-start payload ownership, tickle/heartbeat processing, heartbeat elapsed time and network-order WChat POKE framing.

- [x] Verify REDALERT/DIBUTIL.CPP and REDALERT/DIBFILE.CPP are each entirely enclosed in #if (0) and record the Win32 DIB helpers as intentionally inactive.

- [x] Port TIBERIANDAWN/WIN32LIB/WSA.CPP open/close, resident/file-backed selection, WSA frame offsets, optional palette, frame-zero and delta LCW decode, XOR frame stepping, direct/indirect playback and metadata getters.

- [x] Port REDALERT/DIALOG.CPP and TIBERIANDAWN/DIALOG.CPP dialog box/window drawing, box palettes, text wrapping, Simple/Fancy/Plain/Clipped text, font selection, palette/shadow flags, caption rendering and variant-specific style rules.

- [x] Port REDALERT/DDE.CPP and TIBERIANDAWN/DDE.CPP active DDEML instance operations: enable callback, test/open/close poke connection, CF_TEXT POKE with 60-second timeout, destructor cleanup; preserve disabled initialize/register/dispatcher blocks as inactive.

- [x] Port both CARGO.CPP variants: attach/limbo, linked-chain append, unsigned-byte quantity, attached-object query, detach and destructor reset.
- [x] Verify REDALERT/BENCH.CPP is entirely guarded by unconditional #if (0) through end-of-file and record as inactive.

- [x] Port REDALERT/HEAP.CPP and TIBERIANDAWN/HEAP.CPP FixedHeapClass/FixedIHeapClass allocation, free, clear, active pointer ordering, serialization and pointer translation routines.

- [x] Port REDALERT/PROFILE.CPP and TIBERIANDAWN/PROFILE.CPP private INI get/write APIs, case-insensitive key lookup, double-NUL key enumeration, section/entry edits, sound/language config parsing, and Red Alert read/write binary cursors.

- [x] Record REDALERT/NULLMGR.CPP and TIBERIANDAWN/NULLMGR.CPP as fully disabled by unconditional #if (0) blocks.
- [x] Record TIBERIANDAWN/NULLCONN.CPP as fully disabled by unconditional #if (0).
- [x] Port REDALERT/NULLDLG.CPP active Select_Serial_Dialog() fallback returning GAME_NORMAL; remaining content is disabled.

- [x] Port REDALERT/WINSTUB.CPP and TIBERIANDAWN/WINSTUB.CPP focus, palette queue, title screen, lock-state, debug and memory-error behaviors.

- [x] Port REDALERT/IPXADDR.CPP and TIBERIANDAWN/IPXADDR.CPP constructors, source/destination header transfer, broadcast detection, NETX zero-network equality exception, all relational operators, RA Winsock UDP address copy and TD virtual-subnet host mask.

- [x] Verify REDALERT/DYNAVEC.CPP is entirely #if (0)-guarded through EOF; record it as inactive because DynamicVectorClass's active template implementation is in VECTOR.H.

- [x] Port REDALERT/LIST.CPP and TIBERIANDAWN/LIST.CPP list items, selection, keyboard/mouse actions, scrollbar lifecycle, view clamping, peer notifications, drawing and gadget link operations.

- [x] Port REDALERT/ABSTRACT.CPP and TIBERIANDAWN/ABSTRACT.CPP abstract object/type constructors, building-adjusted target distance, target identity, coordinate fixup, name buffers, ownership masks, active/recent flags and vtable-preserving deletion adapter.

- [x] Port REDALERT/CONFDLG.CPP active Process(int) true stub and TIBERIANDAWN/CONFDLG.CPP modal confirmation behavior, layout, selection, input, game-loop callback and surface-restoration redraw.

- [x] Port REDALERT/LAYER.CPP and TIBERIANDAWN/LAYER.CPP LayerClass submit, sorted insertion, one-pass adjacent-swap sorting, init/no-op One_Time and vector allocation/growth rules.

- [x] Port TIBERIANDAWN/SPECIAL.CPP and SPECIAL.H default settings plus Special_Dialog's ten checkbox settings, local transactional copy, OK/Escape event queue, Cancel discard and surface restore redraw.

- [x] Port REDALERT/CREDITS.CPP and TIBERIANDAWN/CREDITS.CPP counter logic with their variant-specific animation step sizes, frame throttling, sound, mission-timer redraw, and UI positioning.

- [x] Port REDALERT/FUSE.CPP and TIBERIANDAWN/FUSE.CPP fuse construction, Arm_Fuse clamping, timer/arming order, proximity detonation checks, target getter and raw save/load format.

- [x] Port REDALERT/CRATE.CPP crate placement/retry logic, ScenarioInit preservation, overlay selection and cleanup, tracked-cell state and expiration timer.

- [x] Port REDALERT/TURRET.CPP default/parameterized constructors and empty destructor; preserve the base DriveClass forwarding boundary.

- [x] Port REDALERT/SPRITE.CPP GraphicBufferClass::Scale_Rotate, exact sine/cosine lookup tables, rotated corner setup and both Bresenham source/destination stepping cases.

- [x] Verify REDALERT/LZWOTRAW.CPP is excluded from both pinned project files and contains unresolved staging-buffer/pointer references; record it as a non-build orphan rather than fabricating an implementation.

- [x] Port TIBERIANDAWN/TARCOM.CPP target-computer AI: parent turret callback, primary/secondary firing fallback, sound, facing/error handling, cloak, and target-dependent hull/turret alignment.

- [x] Port REDALERT/FACE.CPP Desired_Facing8/256, DirType values and wrapping direction arithmetic using its exact ratio/diagonal logic.
