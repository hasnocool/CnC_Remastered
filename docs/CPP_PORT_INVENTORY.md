# Upstream C++ translation-unit port inventory

Pinned source: [electronicarts/CnC_Remastered_Collection](https://github.com/electronicarts/CnC_Remastered_Collection)
Upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`

- Total C++ translation units: **494**
- Fully ported: **271**
- Not yet fully ported: **223**
- A translation unit is complete only when its active behavior has a Rust implementation. Intentionally empty source files are identified explicitly; placeholders do not count.

## Coverage by directory

| Upstream directory | C++ files | Complete | Remaining |
|---|---:|---:|---:|
| `REDALERT/` | 275 | 138 | 137 |
| `REDALERT/WIN32LIB/` | 26 | 25 | 1 |
| `TIBERIANDAWN/` | 161 | 63 | 98 |
| `TIBERIANDAWN/WIN32LIB/` | 32 | 31 | 1 |

## File-by-file coverage

### `REDALERT/`

| Status | Upstream C++ file | Rust port / notes |
|---|---|---|
| Complete | `REDALERT/2KEYFRAM.CPP` | rts-formats/src/keyframe.rs — packed keyframe header/table parsing, LCW keyframe decode, XOR key-delta/subframe reconstruction, optional 768-byte palette extraction, cached-shape metadata, frame getters and Red Alert uncompressed-buffer policy/sizing. |
| Complete | `REDALERT/AADATA.CPP` | rts-core/src/red_alert_aircraft_types.rs — all seven static aircraft records, enum/name lookup, source-order heap initialization, rotor data asset names, shape asset path generation, dimensions, occupancy/overlap lists, pip calculation and prohibited create-and-place behavior. |
| Complete | `REDALERT/ABSTRACT.CPP` | rts-core/src/abstract_object.rs — building-adjusted target distance, RTTI/ID target encoding, 24-byte INI name, Full_Name, ownership mask, identity coordinate fixup, and vtable-preserving Delete_This adapter. CHEAT_KEYS debug dump is not enabled by the pinned project. |
| Complete | `REDALERT/ADATA.CPP` | rts-core/src/red_alert_animation_types.rs — all 89 source-defined animation records in exact Init_Heap order, metadata/fixed damage/chain links, safe type/name lookup, create/place prohibition, debug/release shape lookup planning, theater-specific refresh, and virtual beacon image alias. |
| Complete | `REDALERT/ADPCM.CPP` | rts-formats/src/adpcm.rs — SOS ADPCM type-1 decoder. |
| Partial | `REDALERT/AIRCRAFT.CPP` | rts-core/src/red_alert_aircraft_helpers.rs — civilian evacuation eligibility, default/Badger overlap footprints, packed Sort_Y, pose/layer selection, rounded pip count, capped flight-speed calculation, fire direction, sight range, landed-helipad spy flags, passenger landing-approach sweep, destination reassignment decisions, takeoff/landing stage plans, table-driven aircraft shape-frame selection, source-condition cell passability, aircraft occupancy/reservation exclusion checks, landing-zone occupancy/contact validation, and friendly support-building selection, source-ordered concentric-ring fallback landing-zone search, multi-ring target firing-position scan with moving-target bias and best/second-best selection, passenger exit placement/radio handoff planning, retreating edge-of-map civilian evacuation/cleanup planning, primary/secondary aircraft rotation processing, aircraft-specific fire eligibility/facing checks, paradrop cargo state handling, and weighted attack/move/select voice-response tables, object/cell action selection including visibility, weapon, fixed-wing, passenger, allied-building and repair-building rules, plus camera reconnaissance, passenger paradrop dispatch, standard weapon fire reveal and dropping-projectile speed handling, per-tick landing/takeoff altitude, crash, layer, tether, scatter and blocked-landing corrective transitions, idle/airborne rotor frame, draw-flag and Chinook dual-rotor placement planning, in-flight damage reduction, destroyed-aircraft cargo/death/fireball/crew-paradrop aftermath, fixed-wing/rotorcraft idle mission and scatter decision trees, Process_Fly_To approach speed/heading, Mission_Hunt and rotorcraft Mission_Attack state machines, Mission_Retreat, Movement_AI, and computer/player Guard and Guard_Area mission overrides. Mission AI, movement, combat, rendering, radio, INI, and save/load integration remain unported; not included in completed count. |
| Not started | `REDALERT/ANIM.CPP` | — |
| Not started | `REDALERT/AUDIO.CPP` | — |
| Complete | `REDALERT/B64PIPE.CPP` | rts-formats/src/base64_streams.rs — Base64 push adapter. |
| Complete | `REDALERT/B64STRAW.CPP` | rts-formats/src/base64_streams.rs — Base64 pull adapter. |
| Complete | `REDALERT/BAR.CPP` | rts-core/src/progress_bar.rs — Q16.16 update threshold, unsigned wraparound redraw behavior, orientation, optional outline, exact horizontal/vertical fill and background rectangles, mouse hide/show and last-rendered tracking. |
| Not started | `REDALERT/BASE.CPP` | — |
| Complete | `REDALERT/BASE64.CPP` | rts-formats/src/base64.rs — Base64 packet encoder/decoder. |
| Not started | `REDALERT/BBDATA.CPP` | — |
| Not started | `REDALERT/BDATA.CPP` | — |
| Complete | `REDALERT/BENCH.CPP` | No active implementation in the pinned translation unit; the complete file, including all Benchmark constructor/reset/begin/end/value routines, is enclosed by an unconditional `#if (0)` through end-of-file. |
| Not started | `REDALERT/BFIOFILE.CPP` | — |
| Complete | `REDALERT/BIGCHECK.CPP` | No active implementation in the pinned RedAlert.vcxproj configuration; the file is entirely guarded by WOLAPI_INTEGRATION, which is not defined by the project. |
| Complete | `REDALERT/BLOWFISH.CPP` | rts-formats/src/blowfish.rs — Blowfish key schedule, P/S tables, encrypt/decrypt. |
| Complete | `REDALERT/BLOWPIPE.CPP` | rts-formats/src/blowfish_streams.rs — Blowfish push adapter. |
| Complete | `REDALERT/BLWSTRAW.CPP` | rts-formats/src/blowfish_streams.rs — Blowfish demand-driven adapter. |
| Complete | `REDALERT/BMP8.CPP` | rts-formats/src/bmp8.rs — 8-bit BMP headers, 256-entry palette conversion, pixel payload, RAII resource ownership and COLORONCOLOR stretch-draw plan; stale GDI member names are normalized. |
| Complete | `REDALERT/BUFF.CPP` | rts-core/src/legacy_buffer.rs — pointer/size wrapper constructors, owned allocation, borrowed mutable/const slices, explicit shared-external storage, shallow copy/assignment ownership flags, reset/destructor semantics and nonblocking access. |
| Not started | `REDALERT/BUILDING.CPP` | — |
| Not started | `REDALERT/BULLET.CPP` | — |
| Complete | `REDALERT/CARGO.CPP` | rts-core/src/cargo.rs — LIFO linked-object attach, Limbo, chain concatenation, unsigned-byte quantity, detach and debug payload. |
| Complete | `REDALERT/CARRY.CPP` | rts-core/src/carryover.rs — supported techno RTTI/type/house/strength/cell snapshot and object recreation; infantry temporarily clears and restores ScenarioInit; source's unconditional false return preserved. |
| Complete | `REDALERT/CCDDE.CPP` | rts-core/src/dde_bridge.rs — DDEServerClass lifecycle, REDALERT/WCHAT service identity, duplicate-instance detection, callback advisory handling, start-game INI payload, tickle and heartbeat updates, heartbeat elapsed time, and network-order WChat poke framing. |
| Not started | `REDALERT/CCFILE.CPP` | — |
| Not started | `REDALERT/CCINI.CPP` | — |
| Complete | `REDALERT/CCMPATH.CPP` | rts-core/src/red_alert_mpath.rs — default MPATH-disabled success/no-op build, optional initialization, settings failure exit, nonblocking ping discovery/retry loop, duplicate-address suppression, response unicast, connection-loss/left-game messages, player removal and AI takeover cleanup. |
| Complete | `REDALERT/CCPTR.CPP` | rts-core/src/ccptr.rs — stable heap-ID pointer resolution, case-insensitive Name comparison, 31 explicit template instantiations and exact relational-operator expressions. |
| Not started | `REDALERT/CCTEN.CPP` | — |
| Not started | `REDALERT/CDATA.CPP` | — |
| Not started | `REDALERT/CDFILE.CPP` | — |
| Not started | `REDALERT/CELL.CPP` | — |
| Complete | `REDALERT/CHECKBOX.CPP` | rts-core/src/checkbox.rs — ToggleClass draw gate, depressed frame/fill, green checked fill, and LEFTRELEASE toggle before base action. |
| Complete | `REDALERT/CHEKLIST.CPP` | rts-core/src/check_list.rs — CheckObject text/checked storage, add/get/current/remove/search/select operations, read-only parent-action gate, LEFTPRESS toggle after ListClass::Action, checkmark row construction, selected shadow fill and clipped text styling. |
| Complete | `REDALERT/CLASS.CPP` | Intentionally empty translation unit; no executable definitions. |
| Complete | `REDALERT/COLRLIST.CPP` | rts-core/src/color_list.rs — parallel remap-color vector, string/resource-ID additions, indexed color removal, selected-style/color overrides and all five row render modes (normal, highlight, box, bar, invert), including gradient flags and palette colors. |
| Not started | `REDALERT/COMBAT.CPP` | — |
| Not started | `REDALERT/COMBUF.CPP` | — |
| Complete | `REDALERT/COMINIT.CPP` | rts-core/src/com_init.rs — RAII OLE lifecycle adapter; host supplies platform calls. |
| Not started | `REDALERT/COMQUEUE.CPP` | — |
| Complete | `REDALERT/CONFDLG.CPP` | rts-core/src/confirmation_dialog.rs — active Process(int) returns true exactly as the source does; the interactive string overload body is disabled under #if (0). |
| Not started | `REDALERT/CONNECT.CPP` | — |
| Not started | `REDALERT/CONQUER.CPP` | — |
| Not started | `REDALERT/CONST.CPP` | — |
| Complete | `REDALERT/CONTROL.CPP` | rts-core/src/red_alert_control_gadget.rs — base-state/copy constructors, peer pointer cloning, ID access, recognized-event pseudo-key conversion before peer notification, GadgetClass action delegation and peer-first draw forwarding. |
| Not started | `REDALERT/COORD.CPP` | — |
| Complete | `REDALERT/CRATE.CPP` | rts-core/src/crate_manager.rs — tracked crate cell and expiration timer, crate removal, random legal placement retries, water/wood overlay selection, ScenarioInit save/restore and wood/steel/water pickup cleanup. |
| Complete | `REDALERT/CRC.CPP` | rts-formats/src/legacy_crc.rs — rotate-add checksum engine. |
| Complete | `REDALERT/CRCPIPE.CPP` | rts-formats/src/crc_streams.rs — checksum-aware push adapter. |
| Complete | `REDALERT/CRCSTRAW.CPP` | rts-formats/src/crc_streams.rs — checksum-aware pull adapter. |
| Complete | `REDALERT/CREDITS.CPP` | rts-core/src/credits.rs — RedAlertCreditClass frame-static throttle, null-player handling, nonnegative credits, mission-timer redraw trigger, eighth-gap step, direction-specific countdown, sound flags, RESFACTOR/Win32 position, and non-Win32 timer formatting. |
| Complete | `REDALERT/CREW.CPP` | Intentionally empty translation unit; contains only the function.h include and no executable definitions. |
| Complete | `REDALERT/CSTRAW.CPP` | rts-formats/src/cache_straw.rs — buffered read-ahead adapter |
| Complete | `REDALERT/DDE.CPP` | rts-core/src/dde_instance.rs — active Win32 wrapper state, no-op constructor before DdeInitialize, DdeUninitialize destructor, callback enable toggle, DdeConnect/DdeDisconnect, Test_Server_Running and CF_TEXT XTYP_POKE client transaction with 60-second timeout; registration and dispatcher are #if (0). |
| Complete | `REDALERT/DEBUG.CPP` | No active implementation in the pinned RedAlert.vcxproj configuration; the entire translation unit is guarded by `#ifdef CHEAT_KEYS`, which is not defined by the project or its headers. |
| Complete | `REDALERT/DESCDLG.CPP` | rts-core/src/description_dialog.rs — dialog geometry, gadget setup, redraw/callback loop, OK/cancel handling and text trimming. |
| Complete | `REDALERT/DIAL8.CPP` | rts-core/src/dial8.rs — exact 8-position geometry, shared static drag-selection state, Desired_Facing8 pointer tracking, facing-change event suppression, parent gadget delegation, mouse hiding/restoring, standard box styles and remap-scheme shadow/highlight lines. |
| Complete | `REDALERT/DIALOG.CPP` | rts-core/src/dialog_render.rs — Dialog_Box shape background, six source box styles, bevel/box/border rendering, in-place @ line wrapping, Window_Box and mouse guard, complete Simple_Text_Print font/palette/shadow/spacing selection, Fancy/Plain/Clipped print helpers, and caption layout. |
| Complete | `REDALERT/DIBFILE.CPP` | No active implementation in the pinned source; SaveDIB, ReadDIBFile, LoadDIB_FromMemory and chunked I/O helpers are all enclosed by unconditional `#if (0)`. |
| Complete | `REDALERT/DIBUTIL.CPP` | No active implementation in the pinned source; the entire Microsoft DIB helper API (CreateDIB, DIB geometry/palette helpers, palette creation, DIB/bitmap conversion and format conversion) is enclosed by unconditional `#if (0)`. |
| Not started | `REDALERT/DISPLAY.CPP` | — |
| Not started | `REDALERT/DLLInterface.cpp` | — |
| Not started | `REDALERT/DLLInterfaceEditor.cpp` | — |
| Complete | `REDALERT/DOOR.CPP` | rts-core/src/door.rs — constructor defaults, opening/closing guards, stage/rate initialization, animation completion, redraw flag and reverse closing frame calculation. |
| Complete | `REDALERT/DPMI.CPP` | No active implementation in the pinned source; the complete translation unit is enclosed by #if (0). |
| Not started | `REDALERT/DRIVE.CPP` | — |
| Complete | `REDALERT/DROP.CPP` | rts-core/src/drop_list.rs — dropdown construction geometry, item selection, peer routing, expand/collapse, position updates, redraw and zap/remove lifecycle. |
| Complete | `REDALERT/DTABLE.CPP` | rts-formats/src/adpcm.rs — exact 1,424-entry ADPCM difference table. |
| Complete | `REDALERT/DYNAVEC.CPP` | No active implementation in the pinned translation unit; the entire DynamicVectorClass template implementation is enclosed by unconditional `#if (0)` through end-of-file. The active template is defined in VECTOR.H and is covered by the existing vector module. |
| Not started | `REDALERT/EDIT.CPP` | — |
| Not started | `REDALERT/EGOS.CPP` | — |
| Complete | `REDALERT/ENDING.CPP` | No active implementation in the pinned source; both GDI_Ending and Nod_Ending bodies are enclosed by #ifdef NEVER. |
| Not started | `REDALERT/EVENT.CPP` | — |
| Not started | `REDALERT/EXPAND.CPP` | — |
| Complete | `REDALERT/FACE.CPP` | rts-core/src/face.rs — exact Desired_Facing8/256 ratio arithmetic, DirType octant constants and wrapping add/sub operators, including identical-point behavior. |
| Complete | `REDALERT/FACING.CPP` | rts-core/src/facing.rs — DirType and FacingClass current/desired headings, signed shortest-path rotation, capped rotation rate, sprite rotation zones. |
| Not started | `REDALERT/FACTORY.CPP` | — |
| Complete | `REDALERT/FIELD.CPP` | rts-core/src/field.rs — all scalar/string/chunk constructors, four-byte IDs, eight-byte header, and Host_To_Net/Net_To_Host payload conversion. |
| Not started | `REDALERT/FINDPATH.CPP` | — |
| Complete | `REDALERT/FIXED.CPP` | rts-core/src/fixed16.rs — Q16.16 constructors, arithmetic helpers, ASCII conversion. |
| Complete | `REDALERT/FLASHER.CPP` | rts-core/src/flasher.rs — global countdown, IsBlushing parity, per-player countdowns, flashing bitmask and optional debug text. |
| Complete | `REDALERT/FLY.CPP` | rts-core/src/fly.rs — speed scaling via fixed(speed,256), pixel-quantized movement accumulation, Coord_Move, and edge rollback. |
| Not started | `REDALERT/FOOT.CPP` | — |
| Complete | `REDALERT/FUSE.CPP` | rts-core/src/fuse.rs — fuse timer/arming countdown, proximity detonation, HeadTo and raw eight-byte timer/arming/coordinate/proximity save/load format. |
| Not started | `REDALERT/GADGET.CPP` | — |
| Complete | `REDALERT/GAMEDLG.CPP` | rts-core/src/red_alert_game_controls.rs — Red Alert resolution/German/WOLAPI geometry, speed/scroll thumb mapping, optional connection-aware WOL button index/navigation, session callbacks, map redraw, multiplayer-safe GAMESPEED event/save substitution, and visual/sound/WOL child dialog routing. |
| Not started | `REDALERT/GAUGE.CPP` | — |
| Complete | `REDALERT/GETCPU.CPP` | rts-core/src/cpu_info.rs — MMX/CPU-type/vendor-ID wrapper with assembly probe injected through CpuProbe and source-compatible strncpy output behavior. |
| Not started | `REDALERT/GLOBALS.CPP` | — |
| Not started | `REDALERT/GOPTIONS.CPP` | — |
| Not started | `REDALERT/GSCREEN.CPP` | — |
| Not started | `REDALERT/HDATA.CPP` | — |
| Complete | `REDALERT/HEAP.CPP` | rts-core/src/heap.rs — FixedHeapClass and FixedIHeapClass allocation/free/clear/free-all, first-false bitmap selection, active-order iteration, logical IDs, zeroed indexed allocations, raw Pipe/Straw serialization, and pointer code/decode callbacks. |
| Not started | `REDALERT/HELP.CPP` | — |
| Not started | `REDALERT/HOUSE.CPP` | — |
| Complete | `REDALERT/HSV.CPP` | rts-core/src/rgb_palette.rs — HSV adjustment, squared component distance, HSV-to-RGB six-sector conversion and palette-index write through RGBClass. |
| Not started | `REDALERT/ICONLIST.CPP` | — |
| Not started | `REDALERT/IDATA.CPP` | — |
| Not started | `REDALERT/INFANTRY.CPP` | — |
| Not started | `REDALERT/INI.CPP` | — |
| Complete | `REDALERT/INIBIN.CPP` | rts-core/src/inibin.rs — intentionally empty upstream translation unit. |
| Complete | `REDALERT/INICODE.CPP` | No active implementation in the pinned source; the entire scenario digest and INI-to-INB implementation is enclosed by #ifdef TOFIX. |
| Not started | `REDALERT/INIT.CPP` | — |
| Complete | `REDALERT/INT.CPP` | rts-core/src/bignum_globals.rs — zero-initialized Error, Carry, Borrow, and precision-specific remainder state. |
| Not started | `REDALERT/INTERNET.CPP` | — |
| Complete | `REDALERT/INTERPAL.CPP` | rts-formats/src/palette_interpolation.rs — active 256×256 midpoint nearest-color remap-table generation, 65,536-byte palette table read/write, RGB luminance increase and disabled 2× scaling entry point. |
| Complete | `REDALERT/INTRO.CPP` | rts-core/src/intro_selection.rs — active Choose_Side branch sets Whom=HOUSE_GOOD; cinematic/side-selection implementations are #if (0) or OBSOLETE-guarded in the pinned project. |
| Not started | `REDALERT/IOMAP.CPP` | — |
| Not started | `REDALERT/IOOBJ.CPP` | — |
| Not started | `REDALERT/IPX.CPP` | — |
| Complete | `REDALERT/IPX95.CPP` | rts-core/src/ipx95.rs — disabled THIPX32 loader, fixed legacy socket/connection return values, and all exported IPX95 stubs. |
| Complete | `REDALERT/IPXADDR.CPP` | rts-core/src/ipx_address.rs — broadcast/default address, source/destination header copying, Winsock IPX versus UDP raw-address handling, NETX zero-network equality exception, and raw 10-byte relational comparisons. |
| Not started | `REDALERT/IPXCONN.CPP` | — |
| Not started | `REDALERT/IPXGCONN.CPP` | — |
| Not started | `REDALERT/IPXMGR.CPP` | — |
| Complete | `REDALERT/ITABLE.CPP` | rts-formats/src/adpcm.rs — exact 1,424-entry ADPCM index table. |
| Not started | `REDALERT/JSHELL.CPP` | — |
| Complete | `REDALERT/KEY.CPP` | rts-core/src/keyboard_runtime.rs — 256-element ring buffer, key/mouse event packing, modifier capture, ASCII translation adapter, async key-state queries, double-clear/system pump and disabled window message handler. |
| Complete | `REDALERT/KEYBOARD.CPP` | rts-core/src/keyboard.rs — 256-slot ring buffer, 255-entry capacity, modifier/release capture, ASCII translation, mouse coordinate triples, double-click expansion, async key state and message pumping. |
| Complete | `REDALERT/LAYER.CPP` | rts-core/src/layer.rs — LayerClass submit/append, stable sorted insertion by Sort_Y, legacy single adjacent-swap Sort pass, and DynamicVectorClass allocation/growth policy. |
| Complete | `REDALERT/LCW.CPP` | rts-formats/src/lcw.rs — LCW block decompression. |
| Complete | `REDALERT/LCWPIPE.CPP` | rts-formats/src/lcw_blocks.rs — framed push compression/decompression and partial-block flush. |
| Complete | `REDALERT/LCWSTRAW.CPP` | rts-formats/src/lcw_blocks.rs — framed pull compression/decompression and buffered reads. |
| Complete | `REDALERT/LCWUNCMP.CPP` | rts-formats/src/lcw.rs — second LCW decompression entry point. |
| Complete | `REDALERT/LINK.CPP` | rts-core/src/link.rs — arena-backed LinkClass insertion, traversal, head/tail, unlink, and list-position assignment. |
| Complete | `REDALERT/LIST.CPP` | rts-core/src/list_box.rs — ListClass item management, pointer-equivalent item lookup/removal, selection correction, keyboard/mouse actions, viewport clamping, auto-scrollbar lifecycle, peer notifications, gadget-chain insertion/removal ordering, row painting, gradient/selection colors and mouse hide/restore callbacks. |
| Not started | `REDALERT/LOADDLG.CPP` | — |
| Not started | `REDALERT/LOGIC.CPP` | — |
| Complete | `REDALERT/LZO1X_C.CPP` | rts-formats/src/lzo1x.rs — deterministic LZO1X-1 compressor. |
| Complete | `REDALERT/LZO1X_D.CPP` | rts-formats/src/lzo1x.rs — bounds-checked LZO1X decoder. |
| Complete | `REDALERT/LZOPIPE.CPP` | rts-formats/src/lzo_blocks.rs — framed push block stream. |
| Complete | `REDALERT/LZOSTRAW.CPP` | rts-formats/src/lzo_blocks.rs — framed pull block stream. |
| Complete | `REDALERT/LZW.CPP` | rts-formats/src/lzw.rs — 12-bit dictionary engine and 16-bit little-endian codewords. |
| Complete | `REDALERT/LZWOTRAW.CPP` | Excluded from the pinned build: neither RedAlert.vcxproj nor TiberianDawn.vcxproj references this orphan source. It contains unresolved staging-buffer/pointer identifiers and is not a compilable implementation; the active stream units are LZWSTRAW.CPP and LZOSTRAW.CPP. |
| Complete | `REDALERT/LZWPIPE.CPP` | rts-formats/src/lzw_blocks.rs — framed push compression/decompression. |
| Complete | `REDALERT/LZWSTRAW.CPP` | rts-formats/src/lzw_blocks.rs — framed pull compression/decompression. |
| Not started | `REDALERT/MAP.CPP` | — |
| Not started | `REDALERT/MAPEDDLG.CPP` | — |
| Not started | `REDALERT/MAPEDIT.CPP` | — |
| Not started | `REDALERT/MAPEDPLC.CPP` | — |
| Not started | `REDALERT/MAPEDSEL.CPP` | — |
| Not started | `REDALERT/MAPEDTM.CPP` | — |
| Complete | `REDALERT/MAPSEL.CPP` | rts-core/src/map_selection.rs — exact 2×14×3 country/scenario coordinate table, ant-mission filenames, inclusive mouse hit boxes, cursor animation parameters and persistent 1/6-second palette pulse state; preserves the source-disabled Map_Selection return-NULL behavior. |
| Complete | `REDALERT/MCI.CPP` | No active implementation in the pinned RedAlert.vcxproj configuration; all functions are guarded by MCIMPEG, which is not defined. |
| Complete | `REDALERT/MCIMOVIE.CPP` | No active implementation in the pinned RedAlert.vcxproj configuration; all functions are guarded by MCIMPEG, which is not defined. |
| Not started | `REDALERT/MENUS.CPP` | — |
| Not started | `REDALERT/MISSION.CPP` | — |
| Not started | `REDALERT/MIXFILE.CPP` | — |
| Not started | `REDALERT/MONOC.CPP` | — |
| Complete | `REDALERT/MOUSE.CPP` | rts-core/src/mouse_control.rs — 42-entry cursor table, default/revert/override states, small-cursor fallback, animated frame updates, debug-only loose MOUSE.SHP probe, and scenario reset. |
| Not started | `REDALERT/MP.CPP` | — |
| Complete | `REDALERT/MPGSET.CPP` | rts-core/src/movie_settings.rs — DVD-gated MPGSettings constructors/config lookup, optional MCIMPEG digital-video enumeration/probe, case-insensitive selected-device resolution, INI save semantics, three-button Default/Test/Cancel dialog, test cleanup and Cancel restore. |
| Not started | `REDALERT/MPLAYER.CPP` | — |
| Complete | `REDALERT/MPLIB.CPP` | rts-core/src/mgen.rs — DOS yield interrupt, chunnel interrupt register packing, MGen queue/node/master operations and EAX/ECX result handling; external ordinal/device constants are injected. |
| Complete | `REDALERT/MPMGRD.CPP` | rts-core/src/player_manager.rs — MGen queue IDs, private/global packet routing, connection table, receive counts, response timing, game-player count and queue flush. |
| Complete | `REDALERT/MPMGRW.CPP` | No active implementation in the pinned source; the entire Windows DLL-backed multiplayer manager is enclosed by #if (0)//PG. |
| Complete | `REDALERT/MPU.CPP` | No active implementation for the pinned MSVC Win32 project; Get_CPU_Clock exists only under __BORLANDC__. |
| Not started | `REDALERT/MSGBOX.CPP` | — |
| Not started | `REDALERT/MSGLIST.CPP` | — |
| Complete | `REDALERT/MiscAsm.cpp` | rts-core/src/misc_asm.rs — shared facing/distance algorithms, bit-array helpers, Q16.16 conversion, fading-table search, endian helpers, text trimming, viewport fat-pixel drawing, palette update and no-op screen shake. |
| Not started | `REDALERT/NETDLG.CPP` | — |
| Complete | `REDALERT/NOSEQCON.CPP` | rts-core/src/nonsequenced_connection.rs — 8-byte CommHeaderType, independent ACK/NOACK IDs, bounded packet queues, duplicate detection, cumulative receive/read sequencing, ACK generation, resend/retry/timeout, NOACK cleanup and the exact 256-sample response-delay filter; Red Alert receive queue rejects oversized packets. |
| Complete | `REDALERT/NULLCONN.CPP` | rts-core/src/null_modem_connection.rs — base connection timing/queue settings, port initialization, actual buffer sizing, 32-bit rotate-and-add CRC and packet overhead. Active Send() returns 0; serial framing code is under #if (0). |
| Complete | `REDALERT/NULLDLG.CPP` | rts-core/src/network_stubs.rs — active Select_Serial_Dialog fallback returns GAME_NORMAL; all remaining modem/serial dialog implementation is guarded by unconditional #if (0). |
| Complete | `REDALERT/NULLMGR.CPP` | No active implementation in the pinned source; the entire modem manager implementation is enclosed by an unconditional #if (0) through end-of-file. |
| Not started | `REDALERT/OBJECT.CPP` | — |
| Not started | `REDALERT/ODATA.CPP` | — |
| Not started | `REDALERT/OPTIONS.CPP` | — |
| Not started | `REDALERT/OVERLAY.CPP` | — |
| Not started | `REDALERT/PACKET.CPP` | — |
| Complete | `REDALERT/PALETTEC.CPP` | rts-core/src/rgb_palette.rs — 256-entry palette construction/copy/equality, full and LUT-gated adjustment, closest-color search, 6-bit palette serialization and timed callback-driven palette fading. |
| Complete | `REDALERT/PIPE.CPP` | rts-formats/src/pipe.rs — push chain forwarding, relink, flush, unlink. |
| Not started | `REDALERT/PK.CPP` | — |
| Not started | `REDALERT/PKPIPE.CPP` | — |
| Not started | `REDALERT/PKSTRAW.CPP` | — |
| Not started | `REDALERT/POWER.CPP` | — |
| Complete | `REDALERT/PROFILE.CPP` | rts-core/src/profile.rs — INI string/key/hex/int get and write routines, sound/language config loading, section/entry replacement/removal, plus all Read_Bin_* and Write_Bin_* buffer initialization, cursor, length, numeric and length-prefixed string operations with original asymmetric bounds checks. |
| Not started | `REDALERT/QUEUE.CPP` | — |
| Not started | `REDALERT/RADAR.CPP` | — |
| Complete | `REDALERT/RADIO.CPP` | rts-core/src/red_alert_radio.rs — Red Alert's 34-message catalogue/texts, three-entry distinct-message history, strength/contact HELLO negotiation, matched-peer OVER_OUT teardown, target fallback, LParam overload and limbo cleanup. |
| Not started | `REDALERT/RAMFILE.CPP` | — |
| Complete | `REDALERT/RAND.CPP` | rts-core/src/random.rs — Sim_Random and Sim_IRandom. |
| Complete | `REDALERT/RANDOM.CPP` | rts-core/src/random_class.rs — seeded LCG and rejection-sampled ranges. |
| Not started | `REDALERT/RAWFILE.CPP` | — |
| Complete | `REDALERT/RAWOLAPI.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/READLINE.CPP` | rts-formats/src/readline.rs — CR/LF line reader and whitespace trimming. |
| Complete | `REDALERT/RECT.CPP` | rts-core/src/rect.rs — validity, intersection, anchor adjustment, union. |
| Not started | `REDALERT/REINF.CPP` | — |
| Complete | `REDALERT/RGB.CPP` | rts-core/src/rgb_palette.rs — 6-bit RGB gun storage, component expansion, ratio adjustment, squared color distance, RGB-to-HSV conversion and cached palette entry update; legacy Win32 DAC methods are no-ops. |
| Complete | `REDALERT/RNDSTRAW.CPP` | rts-formats/src/random_straw.rs — 32 independent LCGs, 1,024-bit seed collection, iterative SHA-1 scramble. |
| Complete | `REDALERT/ROTBMP.CPP` | Inactive in the pinned build: the entire translation unit is enclosed by unconditional `#if (0)`; Rotate_Bitmap is not compiled. |
| Not started | `REDALERT/RULES.CPP` | — |
| Not started | `REDALERT/SAVELOAD.CPP` | — |
| Not started | `REDALERT/SCENARIO.CPP` | — |
| Not started | `REDALERT/SCORE.CPP` | — |
| Complete | `REDALERT/SCROLL.CPP` | rts-core/src/scroll.rs — active constructor timer setup, autoscroll toggle, Init_IO reset and HelpClass::AI delegation; map-scroll algorithm is #if 0. |
| Not started | `REDALERT/SDATA.CPP` | — |
| Complete | `REDALERT/SEDITDLG.CPP` | Inactive in the pinned build: the entire SimpleEditDlgClass implementation is guarded by `#ifdef WOLAPI_INTEGRATION`, which is absent from the Red Alert project definitions and source tree. |
| Not started | `REDALERT/SENDFILE.CPP` | — |
| Not started | `REDALERT/SEQCONN.CPP` | — |
| Not started | `REDALERT/SESSION.CPP` | — |
| Complete | `REDALERT/SHA.CPP` | rts-formats/src/sha1.rs — streaming SHA-1 engine and non-destructive result snapshots. |
| Complete | `REDALERT/SHAPEBTN.CPP` | rts-core/src/shape_button.rs — shape assignment, first-frame bounds, disabled/pressed/reflect-state frame selection and conditional mouse hide/show. |
| Complete | `REDALERT/SHAPIPE.CPP` | rts-formats/src/sha_streams.rs — non-transforming SHA push adapter |
| Complete | `REDALERT/SHASTRAW.CPP` | rts-formats/src/sha_streams.rs — SHA pull adapter hashing returned bytes |
| Not started | `REDALERT/SIDEBAR.CPP` | — |
| Not started | `REDALERT/SIDEBARGlyphx.CPP` | — |
| Not started | `REDALERT/SLIDER.CPP` | — |
| Not started | `REDALERT/SMUDGE.CPP` | — |
| Complete | `REDALERT/SOUNDDLG.CPP` | rts-core/src/red_alert_sound_controls.rs — French/German/default layout and FIXIT_CSII listbox sizing, theme catalogue and prefixed IDs, immediate 1/256 volume/shuffle/repeat option updates, FIXIT_VERSION_3 multiplayer volume mirror, stop/quiet and play actions, focus-redraw handling and list cleanup. |
| Not started | `REDALERT/SPECIAL.CPP` | — |
| Complete | `REDALERT/SPRITE.CPP` | rts-core/src/scale_rotate.rs — exact lookup tables, fixed-point rotated corner geometry, both Bresenham rasterization cases, source pixel stepping, transparency and destination writes; Rust safely bounds framebuffer access. |
| Not started | `REDALERT/STARTUP.CPP` | — |
| Complete | `REDALERT/STATBTN.CPP` | rts-core/src/static_button.rs — default/text constructors, ignored constructor ID, -1 auto-size semantics, owned text replacement, resize background invalidation, conditional mouse hide/show, background capture/restore under page lock, and center/right text alignment. |
| Not started | `REDALERT/STATS.CPP` | — |
| Complete | `REDALERT/STRAW.CPP` | rts-formats/src/straw.rs — demand-driven pull chain and reconnect behavior. |
| Complete | `REDALERT/STUB.CPP` | rts-core/src/legacy_stub.rs — exact launcher message. |
| Not started | `REDALERT/SUPER.CPP` | — |
| Complete | `REDALERT/SURFACE.CPP` | No active implementation in the pinned source; the complete constructor, sub-surface and copy implementation is enclosed by #if (0)//PG. |
| Complete | `REDALERT/Shape.cpp` | rts-core/src/shape_buffer.rs — shape buffer capacity/configuration; caller retains memory ownership. |
| Complete | `REDALERT/TAB.CPP` | rts-core/src/red_alert_tab.rs — TabClass timers/credit lifecycle, resolution scaling, tab/credit/mission pulse frame selection, WIN32/non-WIN32 highlight variants, 8-pixel mouse hit band, Win32 right-tab suppression, money-flash invalidation and Sidebar/Credit AI delegation. |
| Not started | `REDALERT/TACTION.CPP` | — |
| Not started | `REDALERT/TARGET.CPP` | — |
| Not started | `REDALERT/TCPIP.CPP` | — |
| Not started | `REDALERT/TDATA.CPP` | — |
| Not started | `REDALERT/TEAM.CPP` | — |
| Not started | `REDALERT/TEAMTYPE.CPP` | — |
| Not started | `REDALERT/TECHNO.CPP` | — |
| Complete | `REDALERT/TEMPLATE.CPP` | rts-core/src/red_alert_template.rs — template pool allocation/lifecycle, constructor unlimbo path, template type footprint/icon-set lookup, radar-bounded map marking, mark-up ownership checks, mark-down overlay/smudge clearing, redraw/recalc and safe self-removal without duplicate unmark. |
| Not started | `REDALERT/TENMGR.CPP` | — |
| Not started | `REDALERT/TERRAIN.CPP` | — |
| Not started | `REDALERT/TEVENT.CPP` | — |
| Complete | `REDALERT/TEXTBTN.CPP` | rts-core/src/text_button.rs — default/string/resource-ID constructors, text assignment/resizing, parent redraw gate, pointer hide/restore, black border, standard/disabled box states, optional WOLAPI disabled box and remap-scheme text with gradient-bright/medium flags. |
| Not started | `REDALERT/THEME.CPP` | — |
| Complete | `REDALERT/TOGGLE.CPP` | rts-core/src/toggle.rs — constructor state, forced redraw hit-testing, sticky processing, press capture, toggle-on-release and Red Alert release-inside check. |
| Complete | `REDALERT/TOOLTIP.CPP` | No active implementation in the pinned source; the entire tooltip, icon-list hover and surface save/restore implementation is enclosed by #if (0)//PG. |
| Complete | `REDALERT/TRACKER.CPP` | rts-core/src/tracker.rs — legal-target guard, ordered global registry/singleton detachment, dependent-trigger recursion and deletion. |
| Not started | `REDALERT/TRIGGER.CPP` | — |
| Not started | `REDALERT/TRIGTYPE.CPP` | — |
| Complete | `REDALERT/TURRET.CPP` | rts-core/src/turret.rs — empty default/destructor behavior and parameterized constructor forwarding UnitType/HousesType to DriveClass; DriveClass behavior remains in its own translation units. |
| Complete | `REDALERT/TXTLABEL.CPP` | rts-core/src/text_label.rs — constructor state, Set_Text, Gadget draw gate, Simple_Text_Print and clipped text dispatch. |
| Not started | `REDALERT/UDATA.CPP` | — |
| Complete | `REDALERT/UDPADDR.CPP` | rts-core/src/udp_addr.rs — active Get_Broadcast_Addresses stub returns false; the complete dialog/address-discovery implementation is #if(0)-disabled. |
| Not started | `REDALERT/UNIT.CPP` | — |
| Complete | `REDALERT/UTRACKER.CPP` | rts-core/src/red_alert_unit_tracker.rs — 32-bit unit counters and same CRUD API, with explicit Win32 big-endian conversion versus upstream DOS htonl/ntohl macros that destructively return zero; conversion-state guard and safe index handling. |
| Not started | `REDALERT/VDATA.CPP` | — |
| Complete | `REDALERT/VECTOR.CPP` | rts-core/src/vector.rs and bool_vector.rs — VectorClass, DynamicVectorClass, DynamicVectorArrayClass, and packed BooleanVectorClass. |
| Not started | `REDALERT/VERSION.CPP` | — |
| Not started | `REDALERT/VESSEL.CPP` | — |
| Complete | `REDALERT/VISUDLG.CPP` | rts-core/src/red_alert_visual_controls.rs — resolution-scaled dialog/control geometry, four normalized visual options, SliderClass max/thumb clamp and scaled bump, direct slider persistence, reset-to-0.5, reset/options focus navigation, outside/right-click exit and callback/surface restoration redraw. |
| Not started | `REDALERT/VORTEX.CPP` | — |
| Complete | `REDALERT/W95TRACE.CPP` | rts-core/src/w95_trace.rs — active implementation is disabled under #if 0. |
| Complete | `REDALERT/WARHEAD.CPP` | rts-core/src/warhead.rs — WarheadTypeClass defaults, five armor modifiers, fixed-index heap allocation/free and ID lookup, INI property reads, Verses fixed-point parsing and organic-warhead detection. |
| Complete | `REDALERT/WEAPON.CPP` | rts-core/src/weapon.rs — WeaponTypeClass defaults/destructor, fixed-index heap registry and IDs, complete INI field resolution for projectile/warhead references, name/armor lookup, anti-air/anti-ground Allowed_Threats and Is_Wall_Destroyer. |
| Complete | `REDALERT/WINSTUB.CPP` | rts-core/src/win_stub.rs — focus suspend/restore, lock count, null main-window/dialog/focus-check stubs, no-op debug string/colour helper, queued VQA palette mutation/luminance/interpolation, debug-only Assert_Failure, fatal memory-error event flow and PCX title-screen read/blit/free. |
| Complete | `REDALERT/WOLAPIOB.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOLEDIT.CPP` | No active implementation in the pinned project; the entire translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOLSTRNG.CPP` | rts-core/src/wol_strings.rs — all 187 English TXT_WOL string constants selected by the pinned ENGLISH project configuration. |
| Complete | `REDALERT/WOL_CGAM.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_CHAT.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_DNLD.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_GSUP.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_LOGN.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_MAIN.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_OPT.CPP` | No active implementation in the pinned project; the entire translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Not started | `REDALERT/WSPIPX.CPP` | — |
| Not started | `REDALERT/WSPROTO.CPP` | — |
| Not started | `REDALERT/WSPUDP.CPP` | — |
| Complete | `REDALERT/XPIPE.CPP` | rts-formats/src/io_streams.rs — fixed-buffer and file-backed push endpoints |
| Complete | `REDALERT/XSTRAW.CPP` | rts-formats/src/io_streams.rs — fixed-buffer and file-backed pull endpoints |
| Complete | `REDALERT/_WSPROTO.CPP` | rts-core/src/winsock_state.rs — PacketTransport global initialized to null. |

### `REDALERT/WIN32LIB/`

| Status | Upstream C++ file | Rust port / notes |
|---|---|---|
| Complete | `REDALERT/WIN32LIB/ALLOC.CPP` | rts-core/src/legacy_alloc.rs — Windows malloc/free/realloc behavior, allocation flags, counters, DPMI stubs and fixed RAM estimates. |
| Complete | `REDALERT/WIN32LIB/BUFFER.CPP` | rts-core/src/buffer.rs — owned/borrowed storage, size-only allocation, default and destructor semantics. |
| Complete | `REDALERT/WIN32LIB/BUFFGLBL.CPP` | rts-core/src/buffer_globals.rs — callback/viewport null defaults and icon-cache enabled state. |
| Complete | `REDALERT/WIN32LIB/DDRAW.CPP` | rts-core/src/direct_draw.rs — identical to Tiberian Dawn's unit; active Set_Video_Mode returns TRUE, while caps, vertical blank, palette conversion, reset, surface registration/focus restore/release and the 20-surface monitor are ported. |
| Complete | `REDALERT/WIN32LIB/DELAY.CPP` | rts-core/src/delay.rs — timer threshold and duration loop. |
| Complete | `REDALERT/WIN32LIB/DIPTHONG.CPP` | rts-formats/src/dipthong.rs — dip/undip/fixup transforms and legacy string-block lookup. |
| Complete | `REDALERT/WIN32LIB/DRAWRECT.CPP` | rts-core/src/draw_rect.rs — locked four-edge Draw_Line sequence. |
| Not started | `REDALERT/WIN32LIB/DrawMisc.cpp` | — |
| Complete | `REDALERT/WIN32LIB/FONT.CPP` | rts-core/src/font_metrics.rs — character/string pixel widths and Text_Print cursor coordinate conversion. |
| Complete | `REDALERT/WIN32LIB/GBUFFER.CPP` | rts-core/src/graphic_buffer.rs — system/DirectDraw buffer initialization, viewport attach/clamp/offset math, lock nesting, surface-loss restore, mouse exclusion and blit geometry. |
| Complete | `REDALERT/WIN32LIB/GETSHAPE.CPP` | rts-core/src/shape_access.rs — packed Shape_Type metadata, special prefix words, shape-block indexing, extraction, height mutation and restoration. |
| Complete | `REDALERT/WIN32LIB/ICONSET.CPP` | rts-formats/src/iconset.rs — parses ICON/SINF/SSET/TRNS/MAP IFF chunks, builds packed IControl_Type metadata and exposes bounded icon/map/transparency slices. |
| Complete | `REDALERT/WIN32LIB/IFF.CPP` | rts-formats/src/iff.rs — FORM validation, first-match chunk search, even-byte chunk alignment, bounded reads, chunk writes and FORM-size update. |
| Complete | `REDALERT/WIN32LIB/IRANDOM.CPP` | rts-core/src/legacy_irandom.rs — assembly-compatible Random state and injectable CRT IRandom range sampling. |
| Complete | `REDALERT/WIN32LIB/LOAD.CPP` | rts-formats/src/load.rs — bounded Load_Data/Write_Data/Load_Alloc_Data, legacy compression-header parsing, skip-data extraction, alias-safe tail staging, and NOCOMPRESS/LCW dispatch; HORIZONTAL remains a source-configured no-op. |
| Complete | `REDALERT/WIN32LIB/LOADFONT.CPP` | rts-core/src/load_font.rs — validates the little-endian size prefix, uncompressed marker, five-block font layout, and retains the legacy allocation layout. |
| Complete | `REDALERT/WIN32LIB/MOUSEWW.CPP` | rts-core/src/mouse_ww.rs — live cursor visibility nesting, conditional hide regions, state queries, forced coordinates and buffer-lock coordination; disabled rendering/timer/clip paths remain no-ops. |
| Complete | `REDALERT/WIN32LIB/PALETTE.CPP` | rts-formats/src/palette_fade.rs — byte-identical shared implementation with Tiberian Dawn; IBM palette copy/set, bump-rate calculation, bounded fade steps. |
| Complete | `REDALERT/WIN32LIB/SET_FONT.CPP` | rts-core/src/font.rs — previous font return, little-endian block offsets, width/height metrics. |
| Complete | `REDALERT/WIN32LIB/TIMER.CPP` | rts-core/src/legacy_timer.rs — tick-source selection, accumulated elapsed time, start/stop/set/reset. |
| Complete | `REDALERT/WIN32LIB/TIMERDWN.CPP` | rts-core/src/legacy_timer.rs — countdown timer set/time/start/stop/reset semantics. |
| Complete | `REDALERT/WIN32LIB/TIMERINI.CPP` | rts-core/src/legacy_timer_system.rs — timer-system state, callback tick counters, active flag, frequency and teardown. |
| Complete | `REDALERT/WIN32LIB/WINDOWS.CPP` | rts-formats/src/window_text.rs — text window state, cursor bookkeeping, control codes, dipthong expansion, substitutions, word wrapping, scrolling and draw-operation emission. |
| Complete | `REDALERT/WIN32LIB/WINHIDE.CPP` | rts-core/src/window_mouse.rs — mouse hide/show calls are commented out upstream; observable no-op. |
| Complete | `REDALERT/WIN32LIB/WRITEPCX.CPP` | rts-formats/src/pcx.rs — 128-byte PCX header, source-compatible scanline RLE, 256-colour palette trailer. |
| Complete | `REDALERT/WIN32LIB/_DIPTABL.CPP` | rts-formats/src/dipthong_tables.rs — exact common-character and diphthong lookup tables. |

### `TIBERIANDAWN/`

| Status | Upstream C++ file | Rust port / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/AADATA.CPP` | — |
| Complete | `TIBERIANDAWN/ABSTRACT.CPP` | rts-core/src/abstract_object.rs — building-adjusted target distance, 9-byte INI name, Name/Full_Name, RTTI_ABSTRACTTYPE, 0xffff ownership, identity coordinate fixup, and vtable-preserving Delete_This adapter. |
| Not started | `TIBERIANDAWN/ADATA.CPP` | — |
| Not started | `TIBERIANDAWN/AIRCRAFT.CPP` | — |
| Complete | `TIBERIANDAWN/ALLOC.CPP` | No active implementation in the pinned source; all memory allocation, DPMI locking, heap statistics and realloc routines are enclosed by unconditional `#if (0)`. |
| Not started | `TIBERIANDAWN/ANIM.CPP` | — |
| Not started | `TIBERIANDAWN/AUDIO.CPP` | — |
| Not started | `TIBERIANDAWN/BASE.CPP` | — |
| Not started | `TIBERIANDAWN/BBDATA.CPP` | — |
| Not started | `TIBERIANDAWN/BDATA.CPP` | — |
| Not started | `TIBERIANDAWN/BUILDING.CPP` | — |
| Not started | `TIBERIANDAWN/BULLET.CPP` | — |
| Complete | `TIBERIANDAWN/CARGO.CPP` | rts-core/src/cargo.rs — shared cargo chain behavior; variant-specific pointer return is represented by stable object IDs. |
| Complete | `TIBERIANDAWN/CCDDE.CPP` | rts-core/src/dde_bridge.rs — shared DDE callback/server implementation with CONQUER/WCHAT service identity, duplicate-instance detection, game-info payload ownership, tickle/heartbeat handling and network-order poke send. |
| Not started | `TIBERIANDAWN/CCFILE.CPP` | — |
| Not started | `TIBERIANDAWN/CDATA.CPP` | — |
| Not started | `TIBERIANDAWN/CDFILE.CPP` | — |
| Not started | `TIBERIANDAWN/CELL.CPP` | — |
| Complete | `TIBERIANDAWN/CHECKBOX.CPP` | rts-core/src/checkbox.rs — ToggleClass draw gate, green depressed frame, dark fill, and two diagonal checked-state lines. |
| Complete | `TIBERIANDAWN/CHEKLIST.CPP` | rts-core/src/check_list.rs — in-place first-character check marker, Is_Checked lookup, read-only gate and parent-first LEFTPRESS toggle; row rendering remains delegated to ListClass. |
| Complete | `TIBERIANDAWN/COLRLIST.CPP` | rts-core/src/color_list.rs — parallel byte-color vector, localized/string additions, indexed color removal, selected-style sentinel (-1) and five row modes, including variant-specific invert text colors and raw SelectColor BAR fill behavior. |
| Complete | `TIBERIANDAWN/COMBAT.CPP` | rts-core/src/combat.rs — Tiberian Dawn armor modifier/distance falloff and bounded 3×3-cell explosion damage traversal, duplicate suppression, building direct-hit rule, high-explosive aircraft multiplier, tiberium reduction and wall destruction. |
| Not started | `TIBERIANDAWN/COMBUF.CPP` | — |
| Not started | `TIBERIANDAWN/COMQUEUE.CPP` | — |
| Complete | `TIBERIANDAWN/CONFDLG.CPP` | rts-core/src/confirmation_dialog.rs — interactive modal confirmation state machine, exact factor-scaled window/button geometry, default No focus, Yes/No/Escape/Enter/arrow input, game callback/main-loop exit handling and surface-restore redraw. |
| Complete | `TIBERIANDAWN/CONNECT.CPP` | rts-core/src/connection.rs — abstract connection base state, 8-byte Win32 packet header layout, queue-service short-circuit semantics, retry/timeout settings, 60 Hz legacy time conversion and command names. |
| Not started | `TIBERIANDAWN/CONQUER.CPP` | — |
| Complete | `TIBERIANDAWN/CONST.CPP` | rts-core/src/game_constants.rs — source names, coordinate/facing tables, 25 weapon profiles, 12 warheads, seven ground movement profiles, four theater records and all seven 256-byte remap tables. |
| Complete | `TIBERIANDAWN/CONTROL.CPP` | rts-core/src/control_gadget.rs — constructor/default flags, peer association, event-to-button pseudo-key mapping, peer-first action notification, base action delegation, peer-first redraw and base draw result. |
| Not started | `TIBERIANDAWN/COORD.CPP` | — |
| Complete | `TIBERIANDAWN/CREDITS.CPP` | rts-core/src/credits.rs — CreditClass constructor, nullable-player handling, nonnegative balance clamp, 1..143 step approach, countdown/audio direction, forced synchronization, logic-only redraw suppression, resolution-aware tab position and dirty/forced rendering. |
| Complete | `TIBERIANDAWN/CREW.CPP` | Intentionally empty translation unit; contains only the function.h include and no executable definitions. |
| Complete | `TIBERIANDAWN/DDE.CPP` | rts-core/src/dde_instance.rs — same active DDE instance/client POKE wrapper methods as Red Alert; DdeInitialize, registration and event callback dispatcher are compile-time inactive in the pinned source. |
| Complete | `TIBERIANDAWN/DEBUG.CPP` | No active implementation in the pinned TiberianDawn.vcxproj configuration; the entire translation unit is guarded by `#ifdef CHEAT_KEYS`, which is not defined by the project or its headers. |
| Complete | `TIBERIANDAWN/DESCDLG.CPP` | rts-core/src/description_dialog.rs — shared dialog flow with GreenBorder style, SeenBuff-sized background and surface-restore redraw handling. |
| Complete | `TIBERIANDAWN/DIAL8.CPP` | rts-core/src/dial8.rs — shared geometry and event state with Tiberian Dawn green depressed/raised box styles and fixed green pointer shadow/highlight colors supplied by the host palette adapter. |
| Complete | `TIBERIANDAWN/DIALOG.CPP` | rts-core/src/dialog_render.rs — green-texture dialog boxes, source 12-style palette/geometry, secondary window borders, text wrapping, embedded 16×16 font palette plus medium/bright tables, green-gradient font selection, FF stripping, shadow/spacing and clipped tabbed output. |
| Not started | `TIBERIANDAWN/DISPLAY.CPP` | — |
| Not started | `TIBERIANDAWN/DLLInterface.cpp` | — |
| Not started | `TIBERIANDAWN/DLLInterfaceEditor.cpp` | — |
| Complete | `TIBERIANDAWN/DOOR.CPP` | rts-core/src/door.rs — byte-identical state transitions and animation-stage handling shared with Red Alert. |
| Complete | `TIBERIANDAWN/DPMI.CPP` | No active implementation in the pinned source; the complete translation unit is enclosed by #if (0). |
| Not started | `TIBERIANDAWN/DRIVE.CPP` | — |
| Not started | `TIBERIANDAWN/EDIT.CPP` | — |
| Complete | `TIBERIANDAWN/ENDING.CPP` | rts-core/src/ending.rs — GDI demo/standard endings, Nod target-selection loop, sound/input gating, cleanup, ending movies, optional trailer and attract-screen sequence. |
| Not started | `TIBERIANDAWN/EVENT.CPP` | — |
| Complete | `TIBERIANDAWN/EXPAND.CPP` | rts-core/src/expansion.rs — NEWMENU expansion/bonus scenario catalogues, bounded Basic/Name INI extraction, localized bonus labels, 320/640 layout, faction selection, keyboard/button accept/cancel, callback/surface-restore redraw loop and list-entry color/gradient rendering adapter. |
| Complete | `TIBERIANDAWN/FACING.CPP` | rts-core/src/facing.rs — DirType and FacingClass current/desired headings, signed shortest-path rotation, capped rotation rate, sprite rotation zones. |
| Not started | `TIBERIANDAWN/FACTORY.CPP` | — |
| Complete | `TIBERIANDAWN/FIELD.CPP` | rts-core/src/field.rs — shared field serialization and endian conversion behavior. |
| Not started | `TIBERIANDAWN/FINDPATH.CPP` | — |
| Complete | `TIBERIANDAWN/FLASHER.CPP` | rts-core/src/flasher.rs — shared per-player flashing state implementation. |
| Complete | `TIBERIANDAWN/FLY.CPP` | rts-core/src/fly.rs — Fixed_To_Cardinal speed conversion and shared flying-object physics. |
| Not started | `TIBERIANDAWN/FOOT.CPP` | — |
| Complete | `TIBERIANDAWN/FUSE.CPP` | rts-core/src/fuse.rs — shared fuse timing and proximity algorithm with identical eight-byte save format; ICON_LEPTON_W is supplied by the caller as a game-definition constant. |
| Not started | `TIBERIANDAWN/GADGET.CPP` | — |
| Complete | `TIBERIANDAWN/GAMEDLG.CPP` | rts-core/src/game_controls.rs — exact resolution-scaled layout, inverted 0–6 speed/scroll sliders, keyboard focus traversal, event-queued multiplayer gamespeed update, save-time temporary GameSpeed substitution/restoration, visual/sound subdialog routing and no-sound-card message path. |
| Not started | `TIBERIANDAWN/GAUGE.CPP` | — |
| Not started | `TIBERIANDAWN/GLOBALS.CPP` | — |
| Not started | `TIBERIANDAWN/GOPTIONS.CPP` | — |
| Not started | `TIBERIANDAWN/GSCREEN.CPP` | — |
| Complete | `TIBERIANDAWN/HDATA.CPP` | rts-core/src/house.rs — ten-house catalogue, names/IDs, INI names, three-character suffixes, display-name resources, radar colors, remap table/color mapping, default gameplay biases, case-insensitive name lookup and One_Time Jurassic Park color adjustment. |
| Complete | `TIBERIANDAWN/HEAP.CPP` | rts-core/src/heap.rs — fixed and indexed heaps with first-false allocation, free-map and active-pointer management, byte-slot clearing, FileClass Save / raw block Load representation, and pointer code/decode callbacks. |
| Not started | `TIBERIANDAWN/HELP.CPP` | — |
| Not started | `TIBERIANDAWN/HOUSE.CPP` | — |
| Not started | `TIBERIANDAWN/IDATA.CPP` | — |
| Not started | `TIBERIANDAWN/INFANTRY.CPP` | — |
| Not started | `TIBERIANDAWN/INI.CPP` | — |
| Not started | `TIBERIANDAWN/INIT.CPP` | — |
| Not started | `TIBERIANDAWN/INTERNET.CPP` | — |
| Complete | `TIBERIANDAWN/INTERPAL.CPP` | rts-formats/src/palette_interpolation.rs — 65,536-byte table read/write, RGB luminance increase; the table generation body and 2× scaling body are compiled out in this variant. |
| Complete | `TIBERIANDAWN/INTRO.CPP` | rts-core/src/intro_selection.rs — active non-DEMO Choose_Side branch sets Whom=HOUSE_GOOD and ScenPlayer=SCEN_PLAYER_GDI then returns; extended cinematic implementation is #if (0). |
| Not started | `TIBERIANDAWN/IOMAP.CPP` | — |
| Not started | `TIBERIANDAWN/IOOBJ.CPP` | — |
| Not started | `TIBERIANDAWN/IPX.CPP` | — |
| Complete | `TIBERIANDAWN/IPX95.CPP` | rts-core/src/ipx95.rs — same fixed IPX95 stubs and socket API behavior as Red Alert. |
| Complete | `TIBERIANDAWN/IPXADDR.CPP` | rts-core/src/ipx_address.rs — broadcast/default address, source/destination header copying, VIRTUAL_SUBNET_SERVER host-mask mapping, head-to-head mask XOR, NETX equality exception and raw relational comparisons. |
| Not started | `TIBERIANDAWN/IPXCONN.CPP` | — |
| Not started | `TIBERIANDAWN/IPXGCONN.CPP` | — |
| Not started | `TIBERIANDAWN/IPXMGR.CPP` | — |
| Not started | `TIBERIANDAWN/JSHELL.CPP` | — |
| Complete | `TIBERIANDAWN/KEYFRAME.CPP` | rts-formats/src/keyframe.rs — shared keyframe and XOR-delta decoder plus Tiberian Dawn's 12,000 KiB/1,000 KiB shape buffers, memory-threshold cache decision, 200 KiB growth increment, and theater slot handling. |
| Complete | `TIBERIANDAWN/LAYER.CPP` | rts-core/src/layer.rs — shared LayerClass behavior with Tiberian Dawn's signed ActiveCount capacity check represented by bounded Rust counts. |
| Not started | `TIBERIANDAWN/LINK.CPP` | — |
| Complete | `TIBERIANDAWN/LIST.CPP` | rts-core/src/list_box.rs — shared ListClass state machine plus Tiberian Dawn's green-box style, empty-list selection reset, current-item behavior, gradient row highlight and gadget-chain operations. |
| Not started | `TIBERIANDAWN/LOADDLG.CPP` | — |
| Complete | `TIBERIANDAWN/LOGIC.CPP` | rts-core/src/logic_runtime.rs — original AI phase ordering, crate regen timing, team/object/factory/house scheduling, deletion-safe dynamic object traversal, locked/revealed scan-bit updates, optional RA attribute recalculation and recently-created flag clearing. |
| Not started | `TIBERIANDAWN/MAP.CPP` | — |
| Not started | `TIBERIANDAWN/MAPEDDLG.CPP` | — |
| Not started | `TIBERIANDAWN/MAPEDIT.CPP` | — |
| Not started | `TIBERIANDAWN/MAPEDPLC.CPP` | — |
| Not started | `TIBERIANDAWN/MAPEDSEL.CPP` | — |
| Not started | `TIBERIANDAWN/MAPEDTM.CPP` | — |
| Not started | `TIBERIANDAWN/MAPSEL.CPP` | — |
| Not started | `TIBERIANDAWN/MENUS.CPP` | — |
| Not started | `TIBERIANDAWN/MISSION.CPP` | — |
| Not started | `TIBERIANDAWN/MIXFILE.CPP` | — |
| Not started | `TIBERIANDAWN/MONOC.CPP` | — |
| Complete | `TIBERIANDAWN/MOUSE.CPP` | rts-core/src/mouse_control.rs — 34-entry cursor table, default/revert/override states, small cursor and animated frames, loose-file MOUSE.SHP probe, VTable binding hook, and scenario reset. |
| Not started | `TIBERIANDAWN/MPLAYER.CPP` | — |
| Complete | `TIBERIANDAWN/MSGBOX.CPP` | rts-core/src/message_box.rs — three Process overload paths, bounded C-string staging, 1–3 button layout and legacy order/mapping, keyboard focus navigation, timer-driven no-button callback mode, surface-restoration redraw and preserved-region restoration. |
| Not started | `TIBERIANDAWN/MSGLIST.CPP` | — |
| Complete | `TIBERIANDAWN/MiscAsm.cpp` | rts-core/src/misc_asm.rs — shared algorithms plus Tiberian Dawn's 8-bit fractional cardinal conversion/saturation. |
| Not started | `TIBERIANDAWN/NETDLG.CPP` | — |
| Complete | `TIBERIANDAWN/NOSEQCON.CPP` | rts-core/src/nonsequenced_connection.rs — same active protocol method bodies as Red Alert, including ACK/NOACK queueing, duplicate/reorder handling, cumulative sequencing, retry/timeout and response-time accounting; Tiberian Dawn's CommBuffer receive enqueue omits Red Alert's maximum-length check. |
| Complete | `TIBERIANDAWN/NULLCONN.CPP` | No active implementation in the pinned source; constructor, destructor, Init, Send, CRC and packet-overhead methods are all enclosed by unconditional #if (0). |
| Not started | `TIBERIANDAWN/NULLDLG.CPP` | — |
| Complete | `TIBERIANDAWN/NULLMGR.CPP` | No active implementation in the pinned source; the entire modem manager implementation is enclosed by an unconditional #if (0) through end-of-file. |
| Not started | `TIBERIANDAWN/OBJECT.CPP` | — |
| Not started | `TIBERIANDAWN/ODATA.CPP` | — |
| Not started | `TIBERIANDAWN/OPTIONS.CPP` | — |
| Not started | `TIBERIANDAWN/OVERLAY.CPP` | — |
| Not started | `TIBERIANDAWN/PACKET.CPP` | — |
| Not started | `TIBERIANDAWN/POWER.CPP` | — |
| Complete | `TIBERIANDAWN/PROFILE.CPP` | rts-core/src/profile.rs — case-insensitive private-profile INI get/write string/int/hex functions and sound/language NewConfigType loading, including the zero-card/IRQ/DMA result; Red Alert-only binary helpers share the module. |
| Not started | `TIBERIANDAWN/QUEUE.CPP` | — |
| Not started | `TIBERIANDAWN/RADAR.CPP` | — |
| Complete | `TIBERIANDAWN/RADIO.CPP` | rts-core/src/radio.rs — 35 message IDs/text slots, last-message tracking, contact/strength-gated HELLO negotiation, OVER_OUT cleanup, delegated base-object receive/limbo semantics, explicit peer network transport and debug snapshot. |
| Complete | `TIBERIANDAWN/RAND.CPP` | rts-core/src/random.rs — Sim_Random and Sim_IRandom. |
| Not started | `TIBERIANDAWN/RAWFILE.CPP` | — |
| Not started | `TIBERIANDAWN/REINF.CPP` | — |
| Complete | `TIBERIANDAWN/RULES.CPP` | rts-core/src/rules_defaults.rs — exact DifficultyClass and RulesClass constructor defaults. |
| Not started | `TIBERIANDAWN/SAVELOAD.CPP` | — |
| Not started | `TIBERIANDAWN/SCENARIO.CPP` | — |
| Not started | `TIBERIANDAWN/SCORE.CPP` | — |
| Complete | `TIBERIANDAWN/SCROLL.CPP` | rts-core/src/scroll.rs — active constructor/countdown start, autoscroll toggle and HelpClass::AI delegation; map-scroll algorithm is #if 0. |
| Not started | `TIBERIANDAWN/SDATA.CPP` | — |
| Not started | `TIBERIANDAWN/SEQCONN.CPP` | — |
| Complete | `TIBERIANDAWN/SHAPEBTN.CPP` | rts-core/src/shape_button.rs — shared shape-button drawing and state selection. |
| Not started | `TIBERIANDAWN/SIDEBAR.CPP` | — |
| Not started | `TIBERIANDAWN/SIDEBARGlyphx.CPP` | — |
| Not started | `TIBERIANDAWN/SLIDER.CPP` | — |
| Not started | `TIBERIANDAWN/SMUDGE.CPP` | — |
| Complete | `TIBERIANDAWN/SOUNDDLG.CPP` | rts-core/src/sound_controls.rs — resolution and locale layout, allowed-theme enumeration and formatted entries, initial music/sound slider state, shuffle/repeat toggles, play/stop actions, modal callback/restoration redraw, save/stop-on-zero-volume finalization, and music-list entry text rendering. |
| Complete | `TIBERIANDAWN/SPECIAL.CPP` | rts-core/src/special_options.rs — SpecialClass default initialization, exact ten-option checkbox table, local snapshot editing, checkbox toggles, OK/Escape event commit, Cancel discard, non-normal Main_Loop exit and surface-restore redraw/cleanup. |
| Not started | `TIBERIANDAWN/STARTUP.CPP` | — |
| Not started | `TIBERIANDAWN/STATS.CPP` | — |
| Not started | `TIBERIANDAWN/SUPER.CPP` | — |
| Complete | `TIBERIANDAWN/Shape.cpp` | rts-core/src/shape_buffer.rs — shared upstream implementation. |
| Complete | `TIBERIANDAWN/TAB.CPP` | rts-core/src/tab.rs — TabClass constructor/dirty flag, resolution-specific one-time setup, disabled tab drawing semantics, tab highlight/credits image draw, top-edge hit-testing and scroll-mod exception, options/map activation, CreditClass::AI and sidebar AI delegation. |
| Complete | `TIBERIANDAWN/TARCOM.CPP` | rts-core/src/tarcom.rs — parent turret AI delegation, primary/secondary weapon selection, fire/sound handling, facing and locked-turret rules, Flame Tank direction snapping, cloak/uncloak handling, and legal-target hull/turret alignment. CHEAT_KEYS Debug_Dump is disabled by the pinned project. |
| Not started | `TIBERIANDAWN/TARGET.CPP` | — |
| Not started | `TIBERIANDAWN/TCPIP.CPP` | — |
| Not started | `TIBERIANDAWN/TDATA.CPP` | — |
| Not started | `TIBERIANDAWN/TEAM.CPP` | — |
| Not started | `TIBERIANDAWN/TEAMTYPE.CPP` | — |
| Not started | `TIBERIANDAWN/TECHNO.CPP` | — |
| Not started | `TIBERIANDAWN/TEMPLATE.CPP` | — |
| Not started | `TIBERIANDAWN/TERRAIN.CPP` | — |
| Complete | `TIBERIANDAWN/TEXTBTN.CPP` | rts-core/src/text_button.rs — default/string/resource-ID constructors and metric preparation, text assignment/resizing, pointer bounds behavior, gradient green and ordinary box states, gradient text flags and normal/pressed/on/disabled palette colors. |
| Not started | `TIBERIANDAWN/THEME.CPP` | — |
| Complete | `TIBERIANDAWN/TOGGLE.CPP` | rts-core/src/toggle.rs — shared toggle behavior with unsigned hit-testing and Tiberian Dawn release-outside toggle semantics. |
| Not started | `TIBERIANDAWN/TRIGGER.CPP` | — |
| Not started | `TIBERIANDAWN/TURRET.CPP` | — |
| Complete | `TIBERIANDAWN/TXTLABEL.CPP` | rts-core/src/text_label.rs — variant-specific color state, Fancy_Text_Print format and shared clipped text dispatch. |
| Not started | `TIBERIANDAWN/UDATA.CPP` | — |
| Not started | `TIBERIANDAWN/UNIT.CPP` | — |
| Complete | `TIBERIANDAWN/UTRACKER.CPP` | rts-core/src/unit_tracker.rs — 32-bit Windows long counters, safe allocation/count bounds, increment/decrement/clear, in-place htonl/ntohl byte order conversion, repeated-conversion guard and invalid-index Get_Unit_Total behavior. |
| Complete | `TIBERIANDAWN/VECTOR.CPP` | rts-core/src/vector.rs and bool_vector.rs — shared VectorClass/DynamicVectorClass and packed BooleanVectorClass behavior; also ports Tiberian Dawn's Raw_Count() and mutable Raw()/Raw(context) accessors. |
| Complete | `TIBERIANDAWN/VISUDLG.CPP` | rts-core/src/visual_controls.rs — resolution-scaled dialog geometry, four slider/value bindings, source 20-unit slider bump and 235 effective maximum, six-position keyboard navigation, reset-to-128 behavior, callback/surface-restoration redraw and exit handling. |
| Complete | `TIBERIANDAWN/WINSTUB.CPP` | rts-core/src/win_stub.rs — sound-aware theme capture/stop on focus loss, focus restore, lock count, no-op main-window/dialog/focus pump and debug string, VQA palette pipeline, non-terminating GlyphX memory-error flow, and PCX title-screen blit/free. |

### `TIBERIANDAWN/WIN32LIB/`

| Status | Upstream C++ file | Rust port / notes |
|---|---|---|
| Complete | `TIBERIANDAWN/WIN32LIB/ALLOC.CPP` | rts-core/src/legacy_alloc.rs — shared Windows allocator compatibility implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/BUFFER.CPP` | rts-core/src/buffer.rs — shared BufferClass storage and ownership implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/BUFFGLBL.CPP` | rts-core/src/buffer_globals.rs — identical shared global initializers. |
| Complete | `TIBERIANDAWN/WIN32LIB/DDRAW.CPP` | rts-core/src/direct_draw.rs — byte-identical to Red Alert's unit; active DirectDraw state, capability flags, palette entries, vblank behavior and surface restoration are ported. |
| Complete | `TIBERIANDAWN/WIN32LIB/DIPTHONG.CPP` | rts-formats/src/dipthong.rs — shared implementation; Internet string table and little-endian offset lookup. |
| Complete | `TIBERIANDAWN/WIN32LIB/DRAWRECT.CPP` | rts-core/src/draw_rect.rs — identical shared implementation. |
| Not started | `TIBERIANDAWN/WIN32LIB/DrawMisc.cpp` | — |
| Complete | `TIBERIANDAWN/WIN32LIB/FONT.CPP` | rts-core/src/font_metrics.rs — shared font metrics and print cursor implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/GBUFFER.CPP` | rts-core/src/graphic_buffer.rs — byte-identical shared implementation with Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP` | rts-core/src/shape_access.rs — shared accessor implementation; variant source differs only in non-behavioral formatting. |
| Complete | `TIBERIANDAWN/WIN32LIB/ICONSET.CPP` | rts-formats/src/iconset.rs — shared IFF icon-set loader; variant differences are pointer syntax/include details. |
| Complete | `TIBERIANDAWN/WIN32LIB/IFF.CPP` | rts-formats/src/iff.rs — byte-identical shared implementation with Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/IRANDOM.CPP` | rts-core/src/legacy_irandom.rs — shared implementation; same source routine in both variants. |
| Complete | `TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP` | rts-core/src/td_keyboard.rs — 256-slot ring, locale VK/ASCII remap tables, caps/num toggle table, virtual-key detection, compatibility conversions, mouse triples and source no-op message handler. |
| Complete | `TIBERIANDAWN/WIN32LIB/LOAD.CPP` | rts-formats/src/load.rs — byte-identical shared implementation with Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/LOADFONT.CPP` | rts-core/src/load_font.rs — shared implementation of the same IBM loader and five-block validation used by Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/LOADPAL.CPP` | rts-formats/src/palette.rs — 768-byte IBM palette loading contract. |
| Complete | `TIBERIANDAWN/WIN32LIB/MEM.CPP` | rts-core/src/memory_pool.rs — paragraph-aligned first-fit allocation, block IDs, LRU timestamps, locked/in-use flags, adjacent free-block merging, capacity queries and compaction around locked blocks; legacy 20-byte pool/18-byte block headers model one-byte packing. |
| Complete | `TIBERIANDAWN/WIN32LIB/MORPHPAL.CPP` | rts-formats/src/palette_fade.rs — 768-channel 6-bit palette interpolation and time-progress update. |
| Complete | `TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP` | rts-core/src/mouse_ww.rs — byte-identical shared implementation with Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/NEWDEL.CPP` | rts-core/src/legacy_new_delete.rs — scalar/array new and delete delegation to Alloc(size, MEM_NEW) and Free(pointer). |
| Complete | `TIBERIANDAWN/WIN32LIB/PALETTE.CPP` | rts-formats/src/palette_fade.rs — palette copy/set, single-colour update, fade-rate calculation and bounded channel stepping. |
| Complete | `TIBERIANDAWN/WIN32LIB/REGIONSZ.CPP` | rts-core/src/region_size.rs — signed 32-bit width × height helper with explicit wrapping semantics. |
| Complete | `TIBERIANDAWN/WIN32LIB/SET_FONT.CPP` | rts-core/src/font.rs — identical shared implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/TIMER.CPP` | rts-core/src/legacy_timer.rs — shared TimerClass elapsed-time implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/TIMERDWN.CPP` | rts-core/src/legacy_timer.rs — shared CountDownTimerClass implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/TIMERINI.CPP` | rts-core/src/legacy_timer_system.rs — timer-system state, callback tick counters, active flag, frequency and teardown. |
| Complete | `TIBERIANDAWN/WIN32LIB/WINDOWS.CPP` | rts-formats/src/window_text.rs — byte-identical shared implementation with Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/WINHIDE.CPP` | rts-core/src/window_mouse.rs — shared upstream no-op implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/WRITEPCX.CPP` | rts-formats/src/pcx.rs — byte-identical shared implementation with Red Alert; same 128-byte header, scanline RLE, and 256-colour palette trailer. |
| Complete | `TIBERIANDAWN/WIN32LIB/WSA.CPP` | rts-formats/src/wsa.rs — WSA header/offset-table parsing, resident versus file-backed selection, optional palette capture, frame-zero LCW decode, shortest-direction frame stepping with wrap/linear-only behavior, per-frame LCW/XOR deltas, direct viewport and indirect-buffer host callbacks, metadata getters and close semantics. |
| Complete | `TIBERIANDAWN/WIN32LIB/_DIPTABL.CPP` | rts-formats/src/dipthong_tables.rs — identical shared lookup tables. |

## Completion requirements

Port each translation unit with its headers, callers, assembly dependencies, build settings, and relevant data structures. Preserve integer widths, signedness, overflow behavior, byte order, call order, state mutation order, and variant-specific behavior. Platform-specific functionality must be replaced with portable implementations, not placeholders. All file parsing must validate untrusted offsets and lengths.
