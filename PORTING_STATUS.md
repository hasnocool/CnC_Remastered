# Porting status

**Overall status: early foundations only — not a playable game and not a 1:1 port.**

## Upstream baseline

- Upstream: `electronicarts/CnC_Remastered_Collection`
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source tree at that commit: 1,331 files (about 37.8 MB).
- Game projects: Tiberian Dawn and Red Alert C++ DLLs, both configured for Win32.
- Editor project: C# Windows Forms, .NET Framework 4.6.2.

## Implemented in this repository

| Area | State | Evidence |
|---|---|---|
| Workspace | Starter | Cargo workspace with six crates |
| Variant map extents | Foundation | TD 64×64; RA 128×128, from the respective upstream `DEFINES.H` |
| Simulation clock | Foundation | 15 ticks/second, from both upstream `DEFINES.H` files |
| Cell and region addressing | Foundation | Source cell dimensions and 4×4 region dimensions |
| Facing identifiers | Foundation | Eight source-defined facings in source order |
| Binary reader | Initial | Checked little-endian reads |
| Editor CRC-32 | Initial | Standard reflected CRC-32; checked against the 123456789 vector |
| Editor cell metrics | Initial | Row-major indexing, 8-way adjacency, clipping, bounds checks |
| Legacy MIX archive | Partial | Reads the legacy 6-byte header and 12-byte index entries used by upstream `MixFileClass`; no filename CRC mapping yet |
| Event scheduler | Initial | Stable ordering by tick and insertion sequence |
| Simulation RNG | Ported | Exact shared 256-byte table; byte index increments before lookup; both variant Sim_IRandom scaling routines |
| Red Alert fixed point | Ported | Q16.16 constructors, arithmetic helpers, rounding, saturation, ASCII conversion |
| Red Alert animation type registry | Ported | 89 animation descriptors, heap/enum order, name lookup, virtual animation links, and asset initialization plans from `REDALERT/ADATA.CPP` |
| Legacy checksum | Ported | Westwood rotate-left/add word checksum with partial-word staging |
| CRC pipe/straw adapters | Ported | Checksum observing push and pull stream adapters |
| CacheStraw | Ported | Buffered read-ahead over demand-driven input |
| SHA pipe/straw adapters | Ported | Non-transforming SHA-1 observation over push and pull streams |
| Buffer/file endpoints | Ported | Fixed-buffer, generic Read/Write, and lazy path-backed file endpoints |
| Pipe/Straw chains | Ported | Push forwarding/relink/flush and pull-demand/relink chains; cycle detection added |
| Rect geometry | Ported | Source-compatible validity, intersection, anchor adjustment, and union |
| Packed Boolean vector | Ported | Packed bit operations, resizing, set/reset, search, and equality |
| Line reader | Ported | CR/LF handling, truncation, EOF semantics, ASCII whitespace trimming |
| Launcher stub | Ported | Exact legacy console message |
| ADPCM type-1 audio decoder | Ported | Original 1,424-entry difference/index tables; low-then-high nibble decode; predictor saturation and stream state |
| LCW codec and block streams | Ported | LCW encode/decode commands, little-endian block headers, arbitrary fragmented push input, pull adapters, partial-block flush behavior |
| LZW codec and block streams | Ported | 12-bit code dictionary, 5,021-slot hash probing, EOS code 256, 16-bit little-endian codewords, framed push/pull streams |
| LZO1X codec and block streams | Ported | Deterministic 14-bit hash compressor, M2/M3/M4 match commands, EOF marker, framed push/pull streams |
| Base64 codec and streams | Ported | Four-character packet encoding, ignored invalid input characters, '=' termination, fixed-chunk Pipe/Straw adapters |
| SHA-1 and RandomStraw | Ported | Streaming SHA-1; 32 LCG state words; low-bit-first 1,024-bit seeding; chained digest scrambler |
| Blowfish engine and streams | Ported | Source-derived P/S tables, 16-round block cipher, reverse P-array decryption, 56-byte key limit, 8-byte Pipe/Straw staging |
| Empty translation unit | No behavior | `REDALERT/CLASS.CPP` contains no executable definitions |
| Map editor model | Initial | Cell layer IDs and waypoint storage only |
| Tiberian Dawn gameplay | Not ported | Variant boundary only |
| Red Alert gameplay | Not ported | Variant boundary only |
| Rendering and audio | Not ported | No renderer/audio backend |
| Networking | Not ported | No wire protocol or lockstep implementation |
| Map editor UI and full codecs | Not ported | No GUI, triggers, team types, or full scenario serialization |

## C++ port coverage

- Upstream translation units inventoried: 494.
- Fully ported C++ translation units: 271 (`REDALERT/CLASS.CPP` [empty], `REDALERT/CRC.CPP`, `REDALERT/CRCPIPE.CPP`, `REDALERT/CRCSTRAW.CPP`, `REDALERT/DTABLE.CPP`, `REDALERT/FIXED.CPP`, `REDALERT/LCW.CPP`, `REDALERT/LCWPIPE.CPP`, `REDALERT/LCWSTRAW.CPP`, `REDALERT/LCWUNCMP.CPP`, `REDALERT/LZO1X_C.CPP`, `REDALERT/LZO1X_D.CPP`, `REDALERT/LZOPIPE.CPP`, `REDALERT/LZOSTRAW.CPP`, `REDALERT/LZW.CPP`, `REDALERT/LZWPIPE.CPP`, `REDALERT/LZWSTRAW.CPP`, `REDALERT/ITABLE.CPP`, `REDALERT/PIPE.CPP`, `REDALERT/RAND.CPP`, `REDALERT/RANDOM.CPP`, `REDALERT/READLINE.CPP`, `REDALERT/RECT.CPP`, `REDALERT/RNDSTRAW.CPP`, `REDALERT/SHA.CPP`, `REDALERT/STRAW.CPP`, `REDALERT/ADPCM.CPP`, `REDALERT/STUB.CPP`, `REDALERT/VECTOR.CPP`, `TIBERIANDAWN/RAND.CPP`, `REDALERT/INIBIN.CPP` [empty], `TIBERIANDAWN/WIN32LIB/REGIONSZ.CPP`, `TIBERIANDAWN/WIN32LIB/NEWDEL.CPP`, `REDALERT/Shape.cpp`, `TIBERIANDAWN/Shape.cpp`, `TIBERIANDAWN/WIN32LIB/LOADPAL.CPP`, `REDALERT/WIN32LIB/WINHIDE.CPP`, `TIBERIANDAWN/WIN32LIB/WINHIDE.CPP`, plus the remaining stream, coordinate, facing, link, vector, dipthong, legacy random, font metrics, buffer, timer, timer-system, allocator, palette fade/morph, and platform helper units recorded in the inventory).
- Remaining C++ translation units: 223. See [the file-by-file inventory](docs/CPP_PORT_INVENTORY.md).

## Compatibility references

- `TIBERIANDAWN/DEFINES.H`: TD map grid, regions, tick rate, facing enumeration.
- `TIBERIANDAWN/RAND.CPP` and `REDALERT/RAND.CPP`: matching 256-byte deterministic table and variant-specific Sim_IRandom calls.
- `REDALERT/RANDOM.H` / `REDALERT/RANDOM.CPP`: separate seeded 15-bit LCG RandomClass and rejection-based ranged selection.
- `REDALERT/CRCPIPE.*` and `REDALERT/CRCSTRAW.*`: checksum-observing stream adapter behavior; `PIPE.CPP` and `STRAW.CPP` chain behavior is still outstanding.
- `TIBERIANDAWN/COORDA.ASM` and `REDALERT/COORDA.ASM`: distinct 8-bit and 16-bit Fixed_To_Cardinal arithmetic.
- `REDALERT/FIXED.H` / `REDALERT/FIXED.CPP`: Q16.16 fixed-point value layout and methods.
- `REDALERT/CRC.H` / `REDALERT/CRC.CPP`: 32-bit rotate-and-add checksum engine.
- `REDALERT/DEFINES.H`: RA map grid and shared timing/facing definitions.
- `TIBERIANDAWN/MIXFILE.H` and `TIBERIANDAWN/MIXFILE.CPP`: legacy MIX header, index records, relative data offsets, and CRC-sorted lookup.
- `TIBERIANDAWN/TiberianDawn.vcxproj` and `REDALERT/RedAlert.vcxproj`: Win32 DLL, Windows SDK 8.1, v141, one-byte structure packing, MASM, and native Windows libraries.
- `CnCTDRAMapEditor/CnCTDRAMapEditor.csproj`: WinForms editor and .NET Framework 4.6.2.

## Verification status

No Rust build/test result is claimed in this commit. The execution environment used to prepare this change did not have `rustc` or `cargo` installed. The workspace includes unit tests for the foundations; run the commands in the README on a Rust 1.85+ host before treating this milestone as verified.


Latest addition: `TIBERIANDAWN/ALLOC.CPP` is recorded as intentionally inactive because its full implementation is under unconditional `#if (0)`.

Latest additions: both `PROFILE.CPP` variants → `crates/rts-core/src/profile.rs` (case-insensitive INI/profile access, sound config parsing, and Red Alert binary-buffer serialization).
Latest additions: both SHAPEBTN.CPP variants → rts-core/src/shape_button.rs with shape bounds, disabled/pressed/reflect-state frames and mouse visibility behavior.

Latest addition: `TIBERIANDAWN/VECTOR.CPP` → `crates/rts-core/src/vector.rs` and `bool_vector.rs`, including Tiberian Dawn's `Raw_Count()`/raw-context accessors.

Latest additions: both `WIN32LIB/DDRAW.CPP` variants → `crates/rts-core/src/direct_draw.rs` (disabled Set_Video_Mode branch, active capability, palette, vertical-blank and surface-monitor behavior).

Latest addition: `TIBERIANDAWN/WIN32LIB/WSA.CPP` → `crates/rts-formats/src/wsa.rs` (WSA file parsing, resident/disk storage, LCW/XOR playback and viewport host adapter).

Latest addition: `REDALERT/BENCH.CPP` is inactive under an unconditional #if (0); both `CARGO.CPP` variants share the cargo-chain implementation in `crates/rts-core/src/cargo.rs`.

Latest additions: both `HEAP.CPP` variants → `crates/rts-core/src/heap.rs` (fixed/indexed allocators, slot tracking, logical IDs, save/load formats and pointer callbacks).

Latest additions: both `NULLMGR.CPP` modem managers and TD `NULLCONN.CPP` are verified inactive under unconditional #if (0); `REDALERT/NULLDLG.CPP` ports its active GAME_NORMAL selection stub in `network_stubs.rs`.

Latest additions: both `WINSTUB.CPP` variants → `crates/rts-core/src/win_stub.rs` (focus, palette, title-screen, error and platform-stub behavior).

Latest additions: both `IPXADDR.CPP` variants → `crates/rts-core/src/ipx_address.rs` (broadcast addresses, packet-header mapping, VSS masks, NETX equality and raw byte ordering).

Latest addition: `REDALERT/DYNAVEC.CPP` is recorded as inactive under an unconditional #if (0); active DynamicVectorClass template code remains represented by VECTOR.H's Rust port.

Latest additions: both `LIST.CPP` variants → `crates/rts-core/src/list_box.rs` (item, selection, scrolling, peer-gadget and rendering behavior).

Latest additions: both `ABSTRACT.CPP` variants → `crates/rts-core/src/abstract_object.rs` (abstract target distance, type metadata/name buffers, ownership masks and vtable-preserving deletion adapter).

Latest additions: both `CONFDLG.CPP` variants → `crates/rts-core/src/confirmation_dialog.rs` (Red Alert's active true stub and Tiberian Dawn's full interactive confirmation modal).

Latest additions: both `LAYER.CPP` variants → `crates/rts-core/src/layer.rs` (stable sorted insert, append, one-pass Sort and vector capacity/growth semantics).

Latest addition: `TIBERIANDAWN/SPECIAL.CPP` → `crates/rts-core/src/special_options.rs` (SpecialClass defaults, ten-checkbox modal editor and queued event commit/cancel transaction).

Latest additions: `REDALERT/CREDITS.CPP` and `TIBERIANDAWN/CREDITS.CPP` → `crates/rts-core/src/credits.rs` with variant-specific animation, throttle, sound, mission-timer and redraw behavior.

Latest additions: both `FUSE.CPP` variants → `crates/rts-core/src/fuse.rs` (arming/lifetime timers, proximity detonation and eight-byte save data).

Latest addition: `REDALERT/CRATE.CPP` → `crates/rts-core/src/crate_manager.rs` (crate placement, overlays, tracked cell, expiry and pickup handling).

Latest addition: `REDALERT/TURRET.CPP` → `crates/rts-core/src/turret.rs` (empty derived constructor/destructor and DriveClass constructor forwarding).

Latest addition: `REDALERT/SPRITE.CPP` → `crates/rts-core/src/scale_rotate.rs` (lookup-table scale/rotate rasterizer with bounded framebuffer writes).

Latest inventory clarification: `REDALERT/LZWOTRAW.CPP` is recorded inactive because neither pinned game project references it and its source contains unresolved staging-buffer/pointer identifiers.

Latest addition: `TIBERIANDAWN/TARCOM.CPP` → `crates/rts-core/src/tarcom.rs` (weapon fallback, fire result handling, sound, uncloak and target-facing decisions).

Latest addition: `REDALERT/FACE.CPP` → `crates/rts-core/src/face.rs` with exact octant and 256-direction facing ratios.

Latest additions: both `DOOR.CPP` variants → `crates/rts-core/src/door.rs` (shared four-state door animation and stage logic).

Latest additions: both `INTRO.CPP` variants → `crates/rts-core/src/intro_selection.rs` (active Choose_Side assignments; extended cinematic branches are compile-time disabled).

Latest inactive units: `REDALERT/ROTBMP.CPP` is wholly #if(0)-guarded; `REDALERT/SEDITDLG.CPP` is WOLAPI_INTEGRATION-only and excluded from the pinned project.

Latest addition: `REDALERT/WARHEAD.CPP` → `crates/rts-core/src/warhead.rs` (indexed warhead registry, INI rules and five armor damage multipliers).

Latest addition: `REDALERT/WEAPON.CPP` → `crates/rts-core/src/weapon.rs` (weapon type records, INI references, allowed threats and wall destruction).

Latest addition: `TIBERIANDAWN/CONNECT.CPP` → `crates/rts-core/src/connection.rs` (abstract connection base state, Win32 packet-header layout, retry/timeout configuration, send-then-receive queue servicing with source short-circuit behavior, 60 Hz fallback time conversion and packet command names).

Latest addition: `TIBERIANDAWN/COMBAT.CPP` → `crates/rts-core/src/combat.rs` (Tiberian Dawn fixed/cardinal armor scaling, distance-based blast attenuation, deterministic adjacent-cell object collection, direct building hits, HE aircraft bonus, tiberium reduction and wall-destruction rules).

Latest addition: `TIBERIANDAWN/EXPAND.CPP` → `crates/rts-core/src/expansion.rs` (the pinned build does not define DEMO, so DEFINES.H enables NEWMENU; port includes expansion availability, GDI/NOD scenario discovery, bonus missions, selection/cancel behavior, redraw lifecycle and EListClass text-render decisions).

Latest addition: `TIBERIANDAWN/HDATA.CPP` → `crates/rts-core/src/house.rs` (typed ten-house metadata catalogue, `From_Name`, safe `As_Reference`, constructor default biases and `One_Time` JP radar-color override).

Latest addition: `TIBERIANDAWN/CONST.CPP` → `crates/rts-core/src/game_constants.rs` (active static weapon, warhead, coordinate, facing, ground, theater and remap tables; disabled NEVER/OBSOLETE/PETROGLYPH_EXAMPLE_MOD alternatives are excluded from the active data).

Latest addition: `TIBERIANDAWN/MSGBOX.CPP` → `crates/rts-core/src/message_box.rs` (resumable modal state machine, string/resource overload adapters, source button ordering, keyboard navigation, surface-restoration redraw and preserve-region restore).

Latest addition: `TIBERIANDAWN/VISUDLG.CPP` → `crates/rts-core/src/visual_controls.rs` (layout initialization, slider controls, `SliderClass::Bump` thumb-size semantics, title highlighting, keyboard navigation/reset logic and responsive modal steps).

Latest addition: `TIBERIANDAWN/TAB.CPP` → `crates/rts-core/src/tab.rs` (TabClass lifecycle, source-disabled tab rendering, highlight and credits-tab drawing adapters, pointer hit-testing, options/map activation, existing CreditClass AI integration and SidebarClass delegation).

Latest addition: `REDALERT/BAR.CPP` → `crates/rts-core/src/progress_bar.rs` (uses the existing Fixed16; preserves the 0.1 redraw threshold, wrapping subtraction, horizontal/vertical pixel rectangles, border-line coordinates and mouse lifecycle).

Latest addition: REDALERT/MAPSEL.CPP → crates/rts-core/src/map_selection.rs (active coordinate/hit-test, mouse animation and pulsing palette helper code; Map_Selection's legacy body remains disabled by upstream #if 0, so its executable behavior is None).

Latest addition: TIBERIANDAWN/SOUNDDLG.CPP → crates/rts-core/src/sound_controls.rs (SoundControlsClass Init/Process and MusicListClass Draw_Entry; slider value cap follows SliderClass::Set_Value maximum minus thumb size).

Latest addition: TIBERIANDAWN/GAMEDLG.CPP → crates/rts-core/src/game_controls.rs (parent options state machine with multiplayer-safe GAMESPEED event enqueue and save-time temporary option mutation/restore).

Latest addition: REDALERT/TEMPLATE.CPP → crates/rts-core/src/red_alert_template.rs (TemplateClass pool, constructor, mark-up/down and destruction paths are represented with a map/pool adapter).

Latest addition: REDALERT/MPGSET.CPP → crates/rts-core/src/movie_settings.rs (DVD feature gate and nested MCIMPEG paths represented explicitly, with a resumable three-button dialog).

Latest addition: TIBERIANDAWN/RADIO.CPP → crates/rts-core/src/radio.rs (contact handshake, message forwarding, over/out teardown, inherited REDRAW receive handling and limbo teardown).

Latest addition: REDALERT/VISUDLG.CPP → crates/rts-core/src/red_alert_visual_controls.rs (Red Alert-specific option scale, resolution-dependent thumb values, and Reset/Options button navigation).

Latest addition: REDALERT/BUFF.CPP → crates/rts-core/src/legacy_buffer.rs (safe Rust Buffer analogue preserving BufferPtr/Size/IsAllocated states, with explicit errors for non-shareable borrowed pointer copies).

Latest addition: TIBERIANDAWN/CONTROL.CPP → crates/rts-core/src/control_gadget.rs (ControlClass event/key conversion, peer notifications and GadgetClass delegation).

Latest addition: REDALERT/STATBTN.CPP → crates/rts-core/src/static_button.rs (StaticButtonClass text storage, auto-sizing, page background buffer lifecycle and drawing).

Latest addition: TIBERIANDAWN/LOGIC.CPP → crates/rts-core/src/logic_runtime.rs (ordered logic frame scheduler and per-house new/active scan accumulation).

Latest addition: TIBERIANDAWN/UTRACKER.CPP → crates/rts-core/src/unit_tracker.rs (32-bit counter storage and network/host format transitions).

Latest addition: REDALERT/UTRACKER.CPP → crates/rts-core/src/red_alert_unit_tracker.rs (preserves the source's WIN32 byte-order path and the non-WIN32 zero-return macro behavior).

Latest addition: REDALERT/CONTROL.CPP → crates/rts-core/src/red_alert_control_gadget.rs (Red Alert-specific copy constructor and key-before-peer Action call ordering).

Latest addition: REDALERT/GAMEDLG.CPP → crates/rts-core/src/red_alert_game_controls.rs (Red Alert option dialog, German layout variant, optional WOLAPI action and the same multiplayer-safe event/save ordering).

Latest addition: REDALERT/RADIO.CPP → crates/rts-core/src/red_alert_radio.rs (Red Alert message IDs/texts and last-three-distinct-message history, with radio contact lifecycle).

Latest addition: REDALERT/TAB.CPP → crates/rts-core/src/red_alert_tab.rs (Red Alert tab controller and pulse behavior, reusing RedAlertCreditClass for its credit-count animation).

Latest addition: REDALERT/CCMPATH.CPP → crates/rts-core/src/red_alert_mpath.rs (models the default Win32 MPATH-disabled build and an optional nonblocking MPATH-enabled adapter).

Latest addition: REDALERT/SOUNDDLG.CPP → crates/rts-core/src/red_alert_sound_controls.rs (Red Alert sound/music options, locale layout and immediate settings mutation; shares MusicList entry rendering primitives).

Latest addition: REDALERT/AADATA.CPP → crates/rts-core/src/red_alert_aircraft_types.rs (seven Red Alert aircraft definitions and source-derived type helpers).

Latest addition: `REDALERT/ADATA.CPP` → `crates/rts-core/src/red_alert_animation_types.rs` (all 89 animation descriptors, source-order IDs, constructor metadata, chain/virtual links, name lookup, and non-blocking asset request plans).

Partial in progress: `REDALERT/AIRCRAFT.CPP` → `crates/rts-core/src/red_alert_aircraft_helpers.rs` ports source-isolated civilian evacuation, overlap, coordinate sorting, pose/layer, pip-count, speed, fire direction, sight, spy flags, landing-approach, destination assignment, takeoff/landing stage, shape-frame, cell passability, occupancy/reservation, landing-zone occupancy/contact validation, and friendly support-building selection. The translation unit remains incomplete and is not included in the fully ported count.
