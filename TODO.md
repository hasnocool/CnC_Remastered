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
- [x] Port `REDALERT/OVERLAY.CPP`: overlay pool lifecycle, Mark wall/non-wall behavior, OverlayPack and legacy INI loading, multiplayer crate rules, wall ownership selection, and LCW OverlayPack writing.
- [x] Port `TIBERIANDAWN/GOPTIONS.CPP`: source-ordered options menu, multiplayer filtering/navigation, language and locale variants, action outcomes, movie/palette sequence, caption rendering geometry, and final callbacks.
- [x] Port source-isolated `REDALERT/LOGIC.CPP` scheduler behavior: mission triggers/timers, shadow growth, time quake, object-list mutation safety, factories/houses, auto-sonar, detach and recently-created cleanup.
- [ ] Port the optional `REDALERT/LOGIC.CPP` `CHEAT_KEYS` monochrome Debug_Dump visualization and complete final verification.
- [x] Port `REDALERT/PK.CPP`: DER key import/export, block sizing, full-block RSA modular exponentiation, disabled Generate behavior, and the PKPipe/PKStraw cipher adapter.
- [x] Port `REDALERT/GOPTIONS.CPP`: resolution geometry, feature-gated menu definitions, multiplayer button filtering, navigation, redraw loop, all action/result branches, and host-driven non-blocking dialog integration.
- [x] Port `REDALERT/WSPUDP.CPP`: dotted broadcast configuration, UDP socket setup/local address discovery, broadcast fan-out, self-packet filtering, and asynchronous packet event handling.
- [x] Port `REDALERT/WSPIPX.CPP`: IPX network-card discovery, socket binding/options, asynchronous read/write event handling, address filtering, packet queues, and legacy send backpressure behavior.
- [x] Port `REDALERT/MSGBOX.CPP`: all Process overloads, source-specific three-button order/input mapping, dialog geometry, preservation, redraw, and non-blocking modal state machine.
- [x] Port `REDALERT/BBDATA.CPP`: 18-entry bullet type registry, enum/heap order, constructor defaults, complete INI overrides, safe type lookup, and debug/release shape asset planning.
- [x] Port `REDALERT/PKPIPE.CPP` and `REDALERT/PKSTRAW.CPP`: RSA session-key block sizing, push/pull handshake state machines, random 56-byte Blowfish key handoff, and embedded pipe/straw chain rewiring.
- [x] Port `REDALERT/ADATA.CPP`: all 89 animation descriptors, exact heap order, lookup helpers, and asset initialization planning.
- [x] Port independent `REDALERT/AIRCRAFT.CPP` routines: civilian evacuation, overlap lists, Sort_Y, pose/layer, pip count, flight speed, fire direction, sight range, helipad spy flags, landing-approach sweep, destination assignment decisions, takeoff/landing stage plans, shape-frame selection, cell passability, aircraft occupancy/reservation exclusion, landing-zone occupancy/contact validation, friendly support-building selection, concentric-ring fallback landing-zone search, target firing-position scanning with moving-target bias, passenger exit placement/radio handoff, edge-of-map retreat evacuation, aircraft primary/secondary rotation, fire eligibility/facing, paradrop cargo handling, voice-response selection, object/cell action selection overrides, camera reconnaissance, projectile/paradrop firing paths, landing/takeoff frame transition logic, rotor overlay frame/placement planning, aircraft-specific damage/death aftermath, fixed-wing/rotorcraft idle mission and scatter decisions, Process_Fly_To, Mission_Hunt, rotorcraft Mission_Attack, Mission_Retreat, Movement_AI, Mission_Guard and Mission_Guard_Area decision logic, plus Mission_Move navigation, landing-zone retargeting and takeoff/landing transitions.
- [ ] Finish `REDALERT/AIRCRAFT.CPP` mission, flight state-machine, movement, combat, rendering, radio, INI, and save/load behavior.
- [ ] Port the remaining 213 C++ translation units with source-specific tests.

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

- [x] Port byte-identical Red Alert/Tiberian Dawn `LOAD.CPP` routines to `rts-formats::load`, including bounded file I/O, compression dispatch, reserved skip data, and scratch-tail staging.- [x] Port both variants' packed `GETSHAPE.CPP` metadata, shape-table extraction, prefix data and height mutation routines to `rts-core::shape_access`.

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

- [x] Port REDALERT/DOOR.CPP and TIBERIANDAWN/DOOR.CPP door constructor, open/close state changes, StageClass control adapter, completion transition and reverse closing frame calculation.

- [x] Port REDALERT/INTRO.CPP and TIBERIANDAWN/INTRO.CPP active Choose_Side branch assignments, confirming OBSOLETE/#if(0)/DEMO guards against pinned project definitions.

- [x] Verify REDALERT/ROTBMP.CPP is entirely disabled by #if(0), and REDALERT/SEDITDLG.CPP is guarded by undefined WOLAPI_INTEGRATION; record both as inactive in the pinned build.

- [x] Port REDALERT/WARHEAD.CPP constructor defaults, indexed heap registration/lookup/free, INI configuration values and fixed-point Verses modifiers.

- [x] Port REDALERT/WEAPON.CPP WeaponTypeClass defaults/destruction, indexed registry, INI data parsing, warhead/bullet references, allowed-threat calculation and weapon/armor name lookup.
- [x] Port `TIBERIANDAWN/CONNECT.CPP` base connection constructor/destructor state, packet staging capacity, retry/timeout configuration, abstract send/receive queue service order, legacy 60 Hz time conversion and packet command names to `rts-core::connection`.
- [x] Port `TIBERIANDAWN/COMBAT.CPP` damage modifiers and explosion collateral traversal into `rts-core::combat`, preserving source order, 32-object cap, `IsToDamage` duplicate suppression, building impact handling, HE aircraft bonus and post-damage terrain effects.
- [x] Port active NEWMENU code from `TIBERIANDAWN/EXPAND.CPP`: expansion and bonus mission discovery/order, first-1000-byte `[Basic] Name` parsing/default, localized mission labels, display entry styling, resolution-dependent dialog geometry, input selection and surface-restoration redraw loop.
- [x] Port `TIBERIANDAWN/HDATA.CPP` house metadata, pointer-table ordering, case-insensitive INI-name resolution, safe house reference lookup, constructor bias defaults and one-time Jurassic Park radar-color override.
- [x] Port active Tiberian Dawn tables from `TIBERIANDAWN/CONST.CPP`, including source names, coordinate/facing tables, weapons, warheads, terrain movement costs, theaters and remap palettes.
- [x] Port `TIBERIANDAWN/MSGBOX.CPP` overloads and modal behavior as a resumable Rust state machine, including bounded message staging, geometry, 1–3 button mapping, keyboard focus, no-button timer callback, surface restoration and preserved-region restore.
- [x] Port `TIBERIANDAWN/VISUDLG.CPP` and its `SliderClass` interaction semantics: 320/640 dialog layout, four option sliders, 20-unit bump behavior, reset values, six-position keyboard navigation, callback and restored-surface redraw.
- [x] Port `TIBERIANDAWN/TAB.CPP`, retaining its active code paths and `#if 0` tab-painting behavior: One_Time geometry/asset setup, highlight, click activation, scroll-mod hit testing, credits AI, sidebar AI/draw delegation and redraw invalidation.
- [x] Port `REDALERT/BAR.CPP` using the existing Fixed16 type, including Q16.16 update threshold, unsigned subtraction wrap, redraw cache, outline and source-specific horizontal/vertical rectangle coordinates.
- [x] Port active Red Alert MAPSEL.CPP support code: exact country/scenario coordinate data, ant-mission filename table, safe inclusive mouse hit testing, cursor frame parameters, and persistent palette-pulse callback step; keep the disabled Map_Selection function as the compiled NULL stub.
- [x] Port TIBERIANDAWN/SOUNDDLG.CPP sound controls: 320/640 layout and language geometry, allowed track list/selected row, volume sliders with 255-16 cap, shuffle/repeat actions, play/stop, callback/redraw handling, settings save and stop-when-muted behavior.
- [x] Port TIBERIANDAWN/GAMEDLG.CPP game-controls dialog: resolution geometry, inverse speed/scroll sliders, keyboard focus transitions, multiplayer GameSpeed event queuing, temporary save/restore, visual and sound dialog routing.
- [x] Port REDALERT/TEMPLATE.CPP TemplateClass pool lifecycle, constructor placement, footprint/icon-map based map marking, visibility bounds, matching-only mark-up clear, mark-down overlay/smudge clearing and no-double-unmark self deletion.
- [x] Port REDALERT/MPGSET.CPP MPGSettings constructors, optional MCIMPEG device enumeration and digital-video probe, default/config device matching, save-to-INI behavior, three-button test/accept/cancel dialog and movie-test cleanup/theme reset.
- [x] Port TIBERIANDAWN/RADIO.CPP RadioClass message table, base receive delegation, contact-gated HELLO negotiation, OVER_OUT teardown, transmit target fallback and limbo cleanup through a peer-network adapter.
- [x] Port REDALERT/VISUDLG.CPP as a distinct Red Alert visual-controls variant: resolution-scaled geometry/thumb bump, normalized slider-to-fixed option writes, Reset to 1/2, keyboard/button navigation and click-outside exit.
- [x] Port REDALERT/BUFF.CPP Buffer constructors, owned allocation, borrowed pointer wrappers, shallow copy/assignment ownership flags, null/zero-size edge cases and Reset/destructor semantics using safe shared leases and nonblocking access.
- [x] Port TIBERIANDAWN/CONTROL.CPP ControlClass constructor, peer setup, ID access, Action event-to-key mapping, peer notification order and Draw_Me peer/base forwarding.
- [x] Port REDALERT/STATBTN.CPP StaticButtonClass constructor/default, Set_Text copy/resize order, Draw_Me conditional mouse handling, page background capture/restore and Draw_Text center/right alignment.
- [x] Port TIBERIANDAWN/LOGIC.CPP AI order and mutation-safe iteration: crate regeneration, teams, objects, unit/infantry/aircraft/building scan bits, optional RA attribute recalculation, map logic, factories, game-mode-specific house AI and recently-created flag clearing.
- [x] Port TIBERIANDAWN/UTRACKER.CPP constructor/destructor storage, per-type increment/decrement, clear, totals/count access, guarded in-place htonl/ntohl conversion and safe invalid-index reads.
- [x] Port REDALERT/UTRACKER.CPP unit totals and dual network-conversion configurations, including non-WIN32 htonl/ntohl macros that return zero, idempotent format-state guards and bounds-safe total access.
- [x] Port REDALERT/CONTROL.CPP Red Alert ControlClass constructors including copy/NoInit forms, peer cloning, Action key update before peer callback, base gadget Action delegation and peer-first Draw_Me.
- [x] Port REDALERT/GAMEDLG.CPP option dialog: German and standard coordinates, WOLAPI feature/connection-dependent button list, inverse speed/scroll sliders, callback/map redraw, navigation, speed event, temporary GameSpeed save/restore, sound-card warning and subdialog returns.
- [x] Port REDALERT/RADIO.CPP Red Alert message catalogue, distinct-message receive history, HELLO/OVER_OUT contact rules, explicit/default-LParam transmit overloads, debug snapshot and limbo cleanup.
- [x] Port REDALERT/TAB.CPP TabClass timers, One_Time asset setup, disabled Draw_It path, credits and mission warning pulse frames, Win32/non-Win32 tab highlights, click activation rules, money flash and credit/sidebar AI calls.
- [x] Port REDALERT/CCMPATH.CPP MPATH compile-gated behavior: default no-op/success, optional packet-buffer/settings/init sequence, player ping/discovery/retry loop, focus redraw/ESC handling and destroy-connection house/player cleanup.
- [x] Port REDALERT/SOUNDDLG.CPP: locale-dependent layout, FIXIT_CSII Win32 listbox height, theme enumeration/prefix ID, selected-track mapping, sliders and immediate sound/score options, shuffle/repeat, stop/quiet, play, FIXIT_VERSION_3 multiplayer score mirror and cleanup.
- [x] Port REDALERT/AADATA.CPP static aircraft definitions and type helpers: source-order heap registry, case-insensitive INI names, asset paths, dimensions, occupancy/overlap, max pips and Create_And_Place=false.

## Milestone 4A — Application Runtime & Rendering Foundation

### Renderer — `crates/renderer`

- [x] Add a dedicated renderer crate and keep it independent of authoritative simulation state.
- [x] Integrate a wgpu window-surface backend compatible with the workspace Rust 1.85 MSRV.
- [x] Initialize an adapter/device, configure a window surface, clear and present frames.
- [x] Add viewport, camera, color, render-layer, texture-handle, sprite-instance, and stable sprite-batch abstractions.
- [x] Add resize handling, frame timing, vsync selection, and low-power/high-performance adapter preference.
- [x] Add GPU textured-sprite pipeline, WGSL shaders, RGBA texture upload, nearest-neighbor sampling, and ordered sprite execution.
- [ ] Add palette-aware SHP/TMP rendering and game-specific presentation composition.
- [ ] Add headless/offscreen renderer tests and platform-specific surface tests.

### Launcher — `crates/launcher`

- [x] Add a unified executable and application registry for Tiberian Dawn, Red Alert, and Map Editor.
- [x] Add `--app td|ra|editor`, `--list`, and interactive terminal application selection.
- [x] Add shared window lifecycle, renderer initialization, resize/redraw, and graceful shutdown.
- [x] Run GPU initialization on a worker thread and poll completion without blocking the event loop.
- [x] Add versioned TOML configuration for display, audio, input, performance, and network preferences.
- [x] Validate settings and atomically replace the persisted configuration file.
- [x] Add structured logging and startup error reporting.
- [x] Add an in-window launcher menu and editable settings UI for application selection, display, performance, audio, input, and network enablement.
- [ ] Connect each registry entry to a functioning game/editor runtime when those runtime APIs exist.

### Integration and verification

- [ ] Integrate the Tiberian Dawn runtime, Red Alert runtime, and map editor into the launcher lifecycle.
- [ ] Add async networking services, session discovery, cancellation, and shutdown integration.
- [ ] Add configuration migration tests and GPU initialization/recovery tests.
- [ ] Run `cargo fmt --all -- --check`.
- [ ] Run `cargo check --workspace --all-targets`.
- [ ] Run `cargo test --workspace`.
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Verify Vulkan, Metal, and DX12 backends where supported and document platform limitations.
- [ ] Record actual build/test output; do not infer success from source inspection.
 
## Milestone 4A — authentic asset ingestion (first tranche)
- [x] Add explicit 256-color RGB8/RGB6 palette parsing and checked indexed-image-to-RGBA conversion; transparency is opt-in.
- [x] Add bounds-checked SHP shape-block offsets and packed Shape_Type metadata extraction with malformed-input tests.
- [x] Add retail TD/RA SHP header/frame-table parsing, bounded LCW keyframe decoding, Format40 referenced-frame XOR decoding with cycle detection, Format20 previous-frame XOR decoding, palette expansion to renderer-compatible RGBA images, and synthetic pixel-level fixtures.
- [ ] Implement pixel/run decoding for the legacy in-memory Shape_Type payload variants (normal, compact, variable and uncompressed); this is separate from the retail keyframe SHP decoder.
- [x] Add bounded TD and RA TMP readers with variant-specific signatures/header offsets, tile-index extraction, blank-tile handling, palette expansion, and synthetic binary fixtures.
- [ ] Validate TMP parsing and tile output against legally supplied original TD/RA terrain archives.
- [x] Add synthetic binary fixtures and golden pixel vectors; do not commit retail game assets.
- [ ] Validate retail SHP parsing and decoding against legally supplied original TD/RA asset files.
- [x] Add renderer API to upload palette-expanded RGBA images from `rts-formats` into a bounded GPU texture cache.
- [x] Add a renderer-owned 128 MiB FIFO GPU texture cache with explicit GPU resource lifetime; bounded CPU-side decoded-image caching remains pending.
- [ ] Implement camera-aware terrain chunks, adjacency transitions and viewport culling.
- [ ] Wire TD, RA and editor asset/runtime providers into launcher application selection.


## Authentic TD/RA terrain icon sets — indexed tile access

- [x] Reuse the existing IFF/ICON/SINF/SSET/TRNS/MAP icon-set parser; do not introduce a second, incompatible TMP container parser.
- [x] Validate SSET length against SINF dimensions and bitplane count.
- [x] Track physical SSET image count separately from logical MAP count.
- [x] Expose bounded 8-bit physical tile pixel slices and logical MAP resolution.
- [x] Treat the legacy 0xFF map entry as an empty logical tile.
- [ ] Validate TD and RA icon sets from legally supplied original game data, including actual MAP and TRNS semantics.
- [ ] Connect decoded tile pixels to palette conversion and renderer texture upload.
- [ ] Implement terrain chunk composition, adjacency transitions, camera scrolling and GPU texture caching.


## TD/RA TMP terrain palette bridge

- [x] Reuse the IFF `ICON`/`SINF`/`SSET`/`MAP `/`TRNS` loader for original TD/RA terrain icon sets; `Read_Iff_Chunk` copies raw chunk bytes and does not decompress SSET.
- [x] Expand physical and logical indexed terrain icons through an explicitly selected `Palette256` into renderer-ready RGBA8 images.
- [x] Preserve empty logical entries (`MAP=0xFF`) as `None`; do not invent per-pixel transparency for terrain.
- [ ] Validate with legally supplied TD and RA theater icon sets and compare pixel output against a trusted extractor.
- [ ] Connect logical terrain images to `WgpuRenderer::cache_rgba_image` and draw them in a scrollable terrain chunk.

## Canonical IFF terrain-to-renderer bridge

- Added `WgpuRenderer::cache_iconset_tile`, which resolves a logical TD/RA IFF icon through the existing `IconSet` MAP table, expands it through an explicit `Palette256`, and uploads it through the existing bounded GPU cache.
- The caller supplies a stable `TextureId`; empty MAP entries return `None` without allocating a texture. This keeps cache identity policy outside the renderer and avoids conflating logical indices across theaters.
- The separate flat-header `tmp.rs` archive reader is not used by this bridge. Its retail-format compatibility remains an audit item and must be checked against source definitions and real files before it is treated as the canonical TD/RA path.
- No GPU-backed test or real theater asset validation has been performed. Cargo formatting, compilation, tests and Clippy remain unverified.

## Camera-aware TMP terrain atlas and chunk composition

- Added renderer::terrain::TerrainMap as a checked row-major map-cell view whose optional indices address the existing TmpArchive logical tile table.
- Added TerrainAtlas::build: collects only referenced, nonblank tiles; palette-expands them; packs them into one bounded RGBA atlas; and retains deterministic tile-index-to-atlas-slot mapping.
- Added TerrainAtlas::upload to reuse the existing bounded GPU texture cache and visible_batch to cull off-screen cells while traversing 16x16-cell chunks and emitting a single-texture SpriteBatch with atlas UVs.
- Atlas dimensions are capped at 8192 per axis and 128 MiB. The caller must use a texture ID unique to the map/theater/palette identity; the existing cache treats an already-cached ID as immutable.
- Added CPU-only tests for atlas packing, deterministic tile slots, UV generation, camera culling and empty terrain cells. Tests are authored but have not been executed.
- This TMP-backed composition path remains separate from the IFF icon-set parser pending source-backed confirmation of each format's asset role.


- Added cached-texture frame submission: WgpuRenderer::render_cached resolves a TextureId internally, avoiding an impossible external borrow of a texture held inside the renderer cache.
- Added TerrainAtlas::render_visible to build the camera-culled chunk batch and submit it against its cached atlas ID in one call.


## Application renderer — current implementation tranche

- [x] Add a bounded renderer-owned FIFO GPU texture cache and palette-expanded RGBA upload.
- [x] Add cached logical IFF icon upload through MAP indirection.
- [x] Compose deterministic TMP terrain atlases from the nonblank tile indices referenced by a generic map grid.
- [x] Generate camera-visible terrain sprites while traversing 16×16 chunks and culling off-screen cells.
- [x] Submit sprites from multiple cached textures in stable render-layer order, switching bind groups only between adjacent texture runs.
- [ ] Validate TMP, SHP, icon-set and palette behavior against legally supplied original TD/RA assets.
- [ ] Implement theater-specific adjacency/transition resolution from source-derived map/theater metadata.
- [x] Implement Red Alert MapPack INI Base64/LCW decoding and connect decoded RA template references to the theater MIX asset provider and catalog-validated terrain atlas builder.
- [x] Decode the Tiberian Dawn editor BIN terrain layer as row-major (template_type: u8, logical_icon: u8) pairs using externally supplied map metrics; keep it independent of RA MapPack/LCW decoding.
- [x] Connect decoded TD template references through the TD theater MIX asset provider and catalog-validated IFF/ICON terrain atlas builder.
- [ ] Validate TD BIN decoding and theater ICON lookup against legally supplied retail scenarios and MIX archives.
- [ ] Implement TRNS transparency semantics and the separate legacy Shape_Type pixel decoder.
- [ ] Add camera input/panning, terrain animation, SHP unit/building rendering and CPU-side decoded-image caching.
- [ ] Connect the rendered terrain and object layers to playable TD/RA and map-editor runtime providers.


## Map-document to terrain renderer adapter

- [x] Expose row-major MapDocument::template_indices() without leaking the document's internal cell storage.
- [x] Add TerrainMap::from_template_cells() with an explicit game/theater resolver from map template IDs to TMP archive tile indices; do not assume the numeric IDs are interchangeable.
- [x] Reject unresolved non-empty template IDs rather than silently rendering the wrong tile.
- [x] Preserve both template type and logical icon index in map-editor cells; expose a row-major iterator of complete (type, icon) references.
- [x] Add source-derived TD/RA template catalogs with numeric type IDs, theater masks, and ICON-set asset stems from EA DEFINES.H / CDATA.CPP metadata (excluding optional RA FIXIT_ANTS-only entry).
- [x] Add a catalog-validated renderer entry point that rejects template types unsupported by the selected game variant/theater before resolving their ICON-set MAP indirection.
- [x] Add a renderer atlas/composition path for multiple template ICON sets; resolve each logical icon through its set's MAP indirection and reject empty/missing logical slots.
- [ ] Add reusable per-chunk CPU composition keyed by map revision and invalidate only edited chunks.
- [ ] Derive adjacency/transition selection from source-backed TD/RA template rules and real theater fixtures.
- [x] Validate map dimensions before consuming template iterators; reserve the exact expected cell capacity fallibly and reject short or oversized iterators.
- Verification: catalog and atlas CPU regression tests are authored; Cargo formatting, compilation and tests remain unexecuted in this environment.


## Red Alert MapPack binary decoding

- [x] Add a dedicated RA MapPack decoder for the Remastered Collection/editor chunk framing: little-endian compressed length + decompressed length followed by an LCW block.
- [x] Enforce the upstream 8,192-byte per-chunk output buffer limit, exact map output size, checked dimensions/allocation, complete chunk framing, and bounded LCW output.
- [x] Decode the RA new-format payload as a row-major u16 little-endian template-type plane followed by a u8 logical-icon plane, preserving raw IDs for later theater/catalog validation.
- [ ] Add a strict Base64 INI-section adapter and validate against legally supplied original RA scenario files.
- [x] Add a theater-aware MIX-backed template ICON-set provider using source-derived catalog names, theater suffix selection, upstream filename CRCs, and the existing IFF/ICON parser.
- [x] Connect decoded RA MapPack cells through TemplateAssetProvider to the catalog-validated terrain atlas builder.
- [ ] Validate the complete path against legally supplied original RA scenarios and theater MIX archives.
- Verification: synthetic chunk fixtures and malformed-input tests are authored; Cargo formatting, compilation and tests remain unexecuted.


## Theater-aware MIX template asset provider

- [x] Add the upstream MIX filename-key calculation: ASCII-uppercase the complete asset filename, then use the Westwood rotate-left/add checksum used by `MIXFILE.CPP` and `CRC.CPP`.
- [x] Add `MixArchive::get_by_filename` without changing the existing CRC-index lookup API.
- [x] Add renderer::TemplateAssetProvider to validate the variant/theater catalog entry, construct the theater-suffixed filename, search caller-supplied MIX archives in caller order, and parse the resolved bytes with the existing IFF/ICON loader.
- [x] Keep asset archives caller-owned and do not bundle proprietary game files.
- [ ] Add loose-file directory lookup.
- [x] Add strict Base64 decoding for concatenated INI [MapPack] values, including whitespace, padding validation, and an output-size limit.
- [ ] Validate the complete path against legally supplied retail files.
- Verification: synthetic filename/theater checks are authored; Cargo formatting, compilation and tests remain unexecuted.

- [x] Add `TemplateAssetProvider::build_ra_terrain_atlas`: collect the unique template types referenced by a decoded RA map, load their theater-specific ICON sets from MIX archives, then call the catalog-validated renderer atlas builder with each cell's (template type, logical icon) reference.

- [x] Add `RaTemplateMap::decode_ini_section` so callers can pass the concatenated INI Base64 value directly; it bounds decoded compressed bytes before LCW chunk parsing.

## Retail TD/RA asset validation harness

- [x] Add `asset-validate`, a CPU-only CLI that accepts external scenario INI, TD editor BIN, palette, and caller-ordered legacy MIX archive paths.
- [x] Validate supplied legacy MIX indexes and resolve each map-referenced theater ICON filename using the existing Westwood filename CRC and MIX lookup APIs.
- [x] Decode TD raw editor BIN terrain pairs and RA numeric `[MapPack]` Base64/LCW chunks through their existing format readers.
- [x] Validate theater catalog availability, referenced ICON-set parsing, logical MAP indirection, palette expansion, complete map-cell resolution, and deterministic RGBA atlas composition.
- [x] Keep all retail assets external; add synthetic CLI parser/order/theater/checksum tests only.
- [ ] Run the harness against legally supplied TD and RA retail scenarios, MIX archives, and palettes; record exact file/build provenance without copying assets into the repository.
- [ ] Run Cargo formatting, workspace check, tests, and Clippy on a Rust 1.85+ host. No successful run is claimed from the current environment.

Scope limits: current harness supports the existing legacy unencrypted MIX parser, TD editor BIN representation, and RA new-format MapPack representation. Encrypted/extended MIX variants, the TD engine LCW stream, older interleaved RA NewINIFormat, full INI dialect compatibility, GPU rendering, and image-to-image retail parity remain outside this validation milestone. See `docs/RETAIL_ASSET_VALIDATION.md`.

## Reusable terrain chunk composition cache

- [x] Add a CPU-side cache for map-space terrain composition, keyed by 16x16 chunk coordinates.
- [x] Compare exact row-major cell contents so only changed chunks are recomposed; no global map revision or hash-collision assumption is required.
- [x] Cache map coordinates and atlas slots rather than screen-space vertices, allowing camera/zoom changes to reuse composition safely.
- [x] Expose per-chunk invalidation, full cache clearing, and hit/miss/rebuild counters.
- [x] Reset cached composition when the atlas identity (texture ID, tile size, or dimensions) changes.
- [ ] Add bounded LRU eviction for very large/editor-generated maps and validate memory behavior under repeated map resizing.
- [ ] Run renderer and workspace tests, formatting, compilation, and Clippy on a Rust 1.85+ host.
- Verification: regression tests are authored for unchanged chunks, single-chunk edits, and camera movement; they have not been executed in this environment.


## Map editor tranche — generic CellGrid<T>

- [x] Port row-major optional-value cell storage with coordinate and flattened-index access.
- [x] Return explicit cell-change events with previous/current values; suppress no-op assignments.
- [x] Return one aggregate clear event and preserve flattened-length-only CopyTo behavior.
- [x] Add synthetic tests for indexing, events, clear, same-area shape copy, and bounds failures.
- [ ] Add Map aggregate event wiring, typed per-layer grids, update batching, invalidation, and derived wall/resource recalculation.
- [ ] Run cargo fmt/check/test/clippy once the Rust toolchain is available.
