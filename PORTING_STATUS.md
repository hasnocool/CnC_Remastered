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
| Legacy MIX archive | Partial | Reads the legacy 6-byte header and 12-byte index entries and resolves filenames with the upstream uppercase rotate-and-add CRC; encrypted MIX variants and loose-file fallback remain outstanding |
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
| Rendering | Initial backend | wgpu textured sprites/UI, bounded GPU texture cache, multi-texture ordered draws, TMP-derived terrain atlas composition, 16×16 chunk traversal and viewport culling; authentic map/theater integration remains pending |
| Audio | Not ported | No playback/mixing backend |
| Networking | Not ported | No wire protocol or lockstep implementation |
| Map editor UI and full codecs | Not ported | No GUI, triggers, team types, or full scenario serialization |

## C++ port coverage

- Upstream translation units inventoried: 494.
- Fully ported C++ translation units: 281 (`TIBERIANDAWN/GOPTIONS.CPP`, `REDALERT/OVERLAY.CPP`, `REDALERT/CLASS.CPP` [empty], `REDALERT/CRC.CPP`, `REDALERT/CRCPIPE.CPP`, `REDALERT/CRCSTRAW.CPP`, `REDALERT/DTABLE.CPP`, `REDALERT/FIXED.CPP`, `REDALERT/LCW.CPP`, `REDALERT/LCWPIPE.CPP`, `REDALERT/LCWSTRAW.CPP`, `REDALERT/LCWUNCMP.CPP`, `REDALERT/LZO1X_C.CPP`, `REDALERT/LZO1X_D.CPP`, `REDALERT/LZOPIPE.CPP`, `REDALERT/LZOSTRAW.CPP`, `REDALERT/LZW.CPP`, `REDALERT/LZWPIPE.CPP`, `REDALERT/LZWSTRAW.CPP`, `REDALERT/ITABLE.CPP`, `REDALERT/PIPE.CPP`, `REDALERT/RAND.CPP`, `REDALERT/RANDOM.CPP`, `REDALERT/READLINE.CPP`, `REDALERT/RECT.CPP`, `REDALERT/RNDSTRAW.CPP`, `REDALERT/SHA.CPP`, `REDALERT/STRAW.CPP`, `REDALERT/ADPCM.CPP`, `REDALERT/STUB.CPP`, `REDALERT/VECTOR.CPP`, `TIBERIANDAWN/RAND.CPP`, `REDALERT/INIBIN.CPP` [empty], `TIBERIANDAWN/WIN32LIB/REGIONSZ.CPP`, `TIBERIANDAWN/WIN32LIB/NEWDEL.CPP`, `REDALERT/Shape.cpp`, `TIBERIANDAWN/Shape.cpp`, `TIBERIANDAWN/WIN32LIB/LOADPAL.CPP`, `REDALERT/WIN32LIB/WINHIDE.CPP`, `TIBERIANDAWN/WIN32LIB/WINHIDE.CPP`, plus the remaining stream, coordinate, facing, link, vector, dipthong, legacy random, font metrics, buffer, timer, timer-system, allocator, palette fade/morph, and platform helper units recorded in the inventory).
- Remaining C++ translation units: 213. See [the file-by-file inventory](docs/CPP_PORT_INVENTORY.md).

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

No Rust build/test result is claimed in the latest renderer commits. The execution environment used to prepare these changes did not have `rustc` or `cargo` installed. The workspace includes unit tests for the foundations; run the commands in the README on a Rust 1.85+ host before treating this milestone as verified.


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

Partial in progress: `REDALERT/AIRCRAFT.CPP` → `crates/rts-core/src/red_alert_aircraft_helpers.rs` ports source-isolated civilian evacuation, overlap, coordinate sorting, pose/layer, pip-count, speed, fire direction, sight, spy flags, landing-approach, destination assignment, takeoff/landing stage, shape-frame, cell passability, occupancy/reservation, landing-zone occupancy/contact validation, friendly support-building selection, concentric-ring fallback landing-zone scanning, moving-target-aware firing-position selection, passenger exit placement/radio handoff, edge-of-map retreat evacuation, primary/secondary aircraft rotation, fire eligibility/facing, paradrop cargo, voice-response routines, object/cell action-selection overrides, camera reconnaissance, standard/paradrop projectile handling, per-tick landing/takeoff transitions, rotor-overlay animation/placement, aircraft damage/death aftermath, fixed-wing/rotorcraft idle mission/scatter decisions, Process_Fly_To, Mission_Hunt, rotorcraft Mission_Attack, Mission_Retreat, Movement_AI, Guard/Guard_Area mission decisions, and fixed-wing/rotorcraft Mission_Move navigation and landing transitions. The translation unit remains incomplete and is not included in the fully ported count.

Latest addition: `REDALERT/PKPIPE.CPP` and `REDALERT/PKSTRAW.CPP` → `crates/rts-core/src/public_key_streams.rs` (56-byte Blowfish session-key negotiation, RSA block staging, asymmetric push/pull handshake behavior, and embedded stream chain wiring).

Latest addition: `REDALERT/BBDATA.CPP` → `crates/rts-core/src/red_alert_bullet_types.rs` (18-entry bullet type heap order, source defaults, complete INI override mapping, safe type lookup, and SHP asset request planning).

Latest addition: `REDALERT/MSGBOX.CPP` → `crates/rts-core/src/red_alert_message_box.rs` (source-accurate Red Alert three-button layout and mapping, all overloads, preservation/redraw behavior, and non-blocking modal input steps).

Latest addition: `REDALERT/WSPIPX.CPP` → `crates/rts-core/src/red_alert_winsock_ipx.rs` (IPX adapter discovery, socket setup/options, asynchronous packet handling, self-packet filtering, and source-compatible send queue semantics behind a host adapter).

Latest addition: `REDALERT/WSPUDP.CPP` → `crates/rts-core/src/red_alert_winsock_udp.rs` (UDP address setup, socket initialization and local IP discovery, broadcast fan-out, self-packet filtering, async read/write events, and legacy send backpressure behavior).

Latest addition: `REDALERT/GOPTIONS.CPP` → `crates/rts-core/src/red_alert_game_options.rs` (complete source-driven game-options menu state machine, compile-feature variants, multiplayer filters, navigation, action branches, and non-blocking host integration).

Latest addition: `REDALERT/PK.CPP` → `crates/rts-core/src/public_key_class.rs` (PKey DER handling, modulus/block-size rules, modular exponentiation, source-accurate no-op key generation, and PKPipe/PKStraw cipher adapter).

Latest addition: `REDALERT/OVERLAY.CPP` → `crates/red-alert/src/overlay.rs` (OverlayClass pool/mark behavior, packed and legacy INI parsing, map placement filtering, wall ownership, and signed-byte OverlayPack serialization using framed LCW).

Latest addition: `TIBERIANDAWN/GOPTIONS.CPP` → `crates/rts-core/src/tiberian_dawn_game_options.rs` (resolution/layout, complete seven-button TD options flow, multiplayer resign/quit, restate movie/palette handling, caption filigree, and source-ordered cleanup callbacks).

Partial in progress: `REDALERT/LOGIC.CPP` → `crates/rts-core/src/red_alert_logic.rs` ports the active AI scheduling, mission timer triggers and reminders, shadow growth, time-quake damage planning, Chronal Vortex ordering, mutation-safe object iteration, house/auto-sonar branches, trigger detach and recently-created cleanup. The optional `CHEAT_KEYS` monochrome `Debug_Dump` screen is still outstanding; this translation unit is not in the completed count.

## Application Runtime & Rendering Foundation — initial implementation

| Component | State | Evidence / remaining work |
|---|---|---|
| Workspace | Expanded | Eight crates; renderer and launcher are workspace members |
| Renderer | Initial textured 2D backend | wgpu surface, RGBA upload, 128 MiB FIFO GPU texture cache, cached icon/TMP image upload, ordered multi-texture draw runs, nearest-sampled sprites, camera-aware generic terrain atlas composition, 16×16 chunk traversal and viewport culling |
| Launcher | Graphical initial release | `cnc-remastered` opens an in-window application selector/settings UI; `--app` direct preview, `--list`, mouse selection, Escape/close handling and shared lifecycle |
| Application registry | Initial | Tiberian Dawn, Red Alert, and Map Editor identities; runtime integration is not yet complete |
| Settings | Initial | Versioned TOML display/audio/input/performance/network settings, validation, atomic replacement |
| GPU startup | Non-blocking | Renderer initialization is performed on a worker; event loop polls a one-shot channel |
| Logging | Initial | tracing subscriber, environment filter, startup and presentation error reporting |
| Rendering parity | Partial foundation | Palette conversion, retail SHP decoding, TMP decoding, icon-set access, cached GPU upload and generic terrain composition exist; real-file validation, authentic map/theater adjacency, TRNS semantics, legacy Shape_Type pixels and game HUD remain outstanding |
| Audio | Not implemented | Audio settings are persisted; playback/mixing remains outstanding |
| Networking | Not implemented | Network preferences are persisted; async transport, lobby, and lockstep remain outstanding |
| Application integration | Not implemented | Launcher registry entries and preview windows exist; TD, RA, and editor runtime APIs are not yet wired |

Dependency choice: wgpu 26.0.1 and winit 0.30.12 are used while preserving the workspace Rust 1.85 MSRV. The latest wgpu 30.0.2 release requires Rust 1.87, so adopting it would require an explicit MSRV change.

**Verification status:** source files have been committed to the feature branch, but Cargo formatting, compilation, tests, and Clippy have not been run. The GitHub repository connector cannot execute Cargo commands. Do not consider this implementation verified or merge-ready until the commands in the README pass on a Rust 1.85+ host.

## Graphical Launcher & Textured 2D Presentation — implementation tranche

- The default `cargo run -p launcher` path now opens a graphical menu with Tiberian Dawn, Red Alert, and Map Editor cards.
- The settings screen exposes fullscreen, four resolution presets, vsync, low-power GPU selection, frame limit, master-audio mute, edge scrolling, mouse-Y inversion, and networking enablement. Changes are persisted automatically; display/GPU changes are applied on the next launch.
- The renderer now has separate textured-sprite and UI pipelines, an RGBA texture upload API, atlas UV regions, nearest-neighbor sampling, sprite tint/rotation, render-layer ordering, alpha blending, and a bundled 5x7 bitmap font.
- A deterministic four-region procedural terrain atlas is drawn behind both the launcher and direct application preview windows. It is a renderer fixture, not authentic TD/RA artwork.
- The launcher initializes the GPU on a worker thread and polls completion from the event loop. Frame pacing uses the configured frame limit.

**Remaining:** authentic TD/RA map and theater ingestion, source-derived terrain transitions, real-file asset validation, legacy Shape_Type pixel decoding, in-game HUD/object rendering, audio playback, async multiplayer networking, and runtime integration for the two games and editor.

**Verification:** Cargo is not installed in the current execution environment and outbound GitHub clone access is unavailable. Formatting, compilation, tests, and Clippy have therefore not been run. Treat this branch as implementation-in-progress until the Rust 1.85+ validation commands pass.

 
## Authentic asset ingestion — first tranche

- **Palette conversion:** crates/rts-formats/src/palette_rgba.rs adds a 256-entry palette, explicit RGB8/RGB6 decoding, checked indexed-image expansion and opt-in transparent-index handling. Upstream LOADPAL.CPP loads 768 bytes for IBM; component interpretation remains an explicit caller choice.
- **SHP container metadata:** crates/rts-formats/src/shp.rs reads the u16 shape count, 32-bit frame offsets (relative to the offset-table base), and packed 26-byte Shape_Type header, validating ranges before exposing encoded frame bytes. Upstream references: TIBERIANDAWN/WIN32LIB/SHAPE.H and GETSHAPE.CPP.
- **Retail SHP decoding:** `crates/rts-formats/src/shp_retail.rs` parses the retail 14-byte TD/RA header and 8-byte frame records, uses the bounded LCW decoder for Format80 keyframes, resolves Format40 references through their u16 frame offsets with recursive cycle detection, and applies Format20 deltas to the preceding frame. XOR commands and output extents are bounded. `decode_rgba_frame(s)` expands indexed pixels through `Palette256` into tightly packed `RgbaImage` data accepted by the renderer's RGBA texture-upload API. The legacy `shp::ShpBlock` remains separate for the in-memory ShapeBlock_Type layout.
- **TMP terrain decoding:** `crates/rts-formats/src/tmp.rs` adds bounded Tiberian Dawn and Red Alert archive readers with their distinct header/signature offsets, fixed-size indexed tile extraction, blank-tile handling, and palette expansion to renderer-compatible RGBA images. Header/index layouts follow OpenRA's `TmpTDLoader` and `TmpRALoader`; validation against legally supplied original game archives remains outstanding.
- **Still outstanding:** validate SHP/TMP against real TD/RA assets; implement the separate in-memory Shape_Type pixel/run variants; integrate terrain chunks/adjacency and a persistent GPU texture cache; connect game/editor runtimes to the launcher.
- **Verification:** tests have been added but not executed. Formatting, compilation, tests and Clippy remain unverified.


## Authentic TD/RA terrain icon-set decoding — indexed pixel access

The repository already contained a shared IFF and ICON/SINF/SSET/TRNS/MAP icon-set implementation based on upstream WIN32LIB/ICONSET.CPP. This tranche extends that existing path rather than adding a second parser for later Tiberian Sun/Red Alert 2 TMP formats.

- Added SINF-aware SSET size validation; malformed partial images are rejected.
- Exposed the physical image count separately from the logical count (derived from MAP when present).
- Added bounded accessors for 8-bit indexed physical tiles and logical-to-physical MAP lookup.
- Treats 0xFF as an empty logical map entry and rejects out-of-range physical references.
- Synthetic tests cover tile extraction, MAP indirection, empty entries and physical bounds.

**Still unverified:** no real TD/RA theater icon sets have been loaded in this environment. The precise meaning of TRNS bytes and any non-8-bit/packed variants still require source-backed and real-file validation. Palette conversion is available and the renderer now exposes cached logical-icon upload, but this path has not been validated against a retail theater set or GPU-backed image comparison.

**Verification:** tests have been added but not executed; Cargo is unavailable in this environment.


## TMP-to-renderer texture upload integration

Inspection confirmed `crates/rts-formats/src/tmp.rs` already implements bounded TD/RA TMP header parsing, tile index lookup, blank tile handling and `Palette256` expansion to `RgbaImage`. This tranche connects that existing output to the GPU renderer instead of duplicating TMP decoding.

- Renderer now depends on `rts-formats` and accepts its `RgbaImage` output.
- Added a renderer-owned 128 MiB FIFO GPU texture cache. Cached `GpuTexture` objects retain their wgpu texture and bind group; eviction drops both together.
- Added cache lookup and byte-accounting APIs. Oversized images are rejected instead of exceeding the cache budget.
- Existing uncached `create_texture_rgba` remains available.

**Not yet verified:** no real retail TMP or SHP file was available for validation; TRNS semantics and palette encoding selection remain caller-controlled. A generic TerrainAtlas now composes referenced TMP tiles, traverses 16×16 chunks, culls to the camera viewport and submits cached textures; this is not yet an authentic TD/RA map/theater renderer. Cargo formatting, compilation, tests and Clippy have not been run in this environment.


## TD/RA TMP terrain palette bridge

The original TD/RA terrain tile path is the IFF `ICON` icon-set format (`SINF`, `SSET`, `MAP `, `TRNS`), not the later Tiberian Sun/Red Alert 2 isometric TMP format. Upstream `WIN32LIB/ICONSET.CPP` reads raw SSET bytes; the IFF reader does not decompress them. The existing `IconSet` parser already exposes physical indexed tiles and logical MAP indirection.

This tranche adds `physical_icon_rgba` and `logical_icon_rgba`, using an explicitly supplied `Palette256`. Empty `MAP=0xFF` entries remain absent, and transparency remains caller-controlled rather than assuming palette index zero is transparent. The returned `RgbaImage` is accepted directly by `WgpuRenderer::cache_rgba_image`.

**Not yet verified:** no legally supplied retail theater icon sets were available for pixel-level comparison, and no GPU-backed rendering test was run. The separate `tmp.rs` flat-header parser remains under audit; it must not be treated as validation of the original IFF TMP path.

**Verification:** Cargo formatting, compilation, tests and Clippy remain unexecuted in this environment.


## Renderer and terrain integration update

- rts-formats TMP archives provide bounded TD/RA tile extraction and palette expansion. Real retail TMP validation remains pending.
- TerrainAtlas packs only nonblank tiles referenced by a generic row-major TerrainMap, with deterministic tile slots and a bounded atlas size.
- Visible terrain generation traverses 16×16 map chunks and clips work to the camera viewport.
- WgpuRenderer::render_cached_multi resolves cached texture IDs and emits adjacent texture runs after stable render-layer sorting, preserving compositing order across texture changes.
- The 128 MiB GPU cache remains FIFO. Texture IDs are treated as immutable content identities for the life of a cache entry.
- Not implemented: authentic TD/RA map-file decoding, theater-specific adjacency/transition resolution, terrain animation, SHP unit/building rendering, camera input, CPU decoded-image cache, and real retail asset verification.


## Map template representation correction

The map editor model now retains both the template type ID and the per-cell logical icon index. These are distinct fields in the original TD map cell record (TType and TIcon); the renderer must not treat a template type ID as a TMP archive tile index.

The authoritative TD engine path obtains the template type's image data, reads its ICON set MAP table, and uses the cell's logical icon slot to resolve the physical image. The generic TMP TerrainAtlas remains a separate fixture/adapter path. Authentic TD/RA rendering needs theater-specific template catalogs plus an IFF ICON-set atlas/composition path. RA MapPack serialization is variant-specific and must be handled independently from TD's per-cell record path.

Source inspection: EA's public CnC_Tiberian_Dawn/TEMPLATE.CPP and MAP.CPP (TemplateClass::Mark, Map load/validation); the existing project source-audit notes also identify the editor's missing per-cell icon field. No proprietary game assets were added. Cargo validation remains unexecuted.


## Template ICON-set atlas path

Added TerrainAtlas::build_from_template_icon_sets. The caller supplies the theater-selected catalog of (template type, IFF IconSet) pairs. For every occupied map cell, the renderer resolves (template type, logical icon) through the corresponding IconSet MAP table, expands the physical SSET tile through the selected palette, deduplicates repeated logical references, and packs the result into the same bounded RGBA atlas used by the generic terrain renderer.

The adapter rejects missing template types, duplicate catalog IDs, inconsistent icon pixel dimensions, invalid/empty MAP references, malformed image storage, invalid map dimensions, and atlas allocations beyond the 8192-axis / 128 MiB limits. It does not invent theater catalogs or asset filenames; those remain the caller's responsibility and require real TD/RA theater data validation.

Cargo formatting, compilation, and tests remain unexecuted in this environment.


## Source-derived TD/RA template catalogs

Added upstream template type IDs, symbolic names, theater-availability masks, and ICON-set filename stems. TD metadata is derived from EA CnC_Tiberian_Dawn DEFINES.H / CDATA.CPP; RA metadata is derived from CnC_Red_Alert CODE/DEFINES.H / CODE/CDATA.CPP. The optional RA TEMPLATE_HILL01 entry behind FIXIT_ANTS is excluded because it is not part of the default enum/build.

The catalog distinguishes TD Desert/Jungle/Temperate/Winter numbering from RA Temperate/Snow/Interior numbering. TerrainAtlas::build_from_template_catalog validates caller-loaded ICON sets against the selected variant and theater before using the existing logical-icon MAP indirection and RGBA atlas path. Asset providers still need to resolve theater suffixes and load IFF files from the user's game data; no proprietary assets are bundled.

The lower-level build_from_template_icon_sets API remains available for custom mods and synthetic fixtures. Cargo formatting, compilation, tests, and Clippy remain unexecuted in this environment.


## Red Alert MapPack decoder

Added `rts_formats::ra_map_pack::RaTemplateMap` for the Red Alert MapPack binary payload. The decoder consumes the editor's framed LCW chunks (u16 compressed length, u16 decoded length, then LCW data), caps each decoded chunk at the upstream 8,192-byte scratch-buffer size, and requires the decoded stream to equal exactly three bytes per map cell. It then separates the new-format payload into a little-endian u16 template-type plane and a u8 logical-icon plane, preserving raw references for later theater/catalog checks.

The caller still needs to concatenate and Base64-decode the INI `[MapPack]` values before invoking this API. No retail scenario has been supplied or validated, and this work does not implement the older interleaved `NewINIFormat` variants handled by the legacy game reader. Cargo formatting, compilation and tests remain unverified in this environment.


## Red Alert MapPack and theater-aware template asset ingestion

- Added `rts_formats::ra_map_pack::RaTemplateMap`. It decodes the Remastered Collection editor's framed LCW chunks (little-endian u16 compressed length and u16 decoded length), enforces the 8,192-byte chunk output cap, checks exact map output size, and separates the new-format stream into the row-major u16 template-type plane followed by the u8 logical-icon plane. Raw IDs are preserved for later catalog validation.
- Added `mix::filename_crc` and `MixArchive::get_by_filename`, matching the upstream MIX filename lookup convention: ASCII-uppercase the full filename and feed it to the Westwood rotate-left/add checksum.
- Added renderer `TemplateAssetProvider`: validates catalog type/theater availability, selects the variant-specific theater extension, searches caller-provided MIX archives in registration order, and loads the result through the existing IFF/ICON parser.
- No proprietary assets are bundled. The provider has not yet been validated against retail MIX archives, and the RA MapPack decoder still needs a strict INI Base64 adapter and connection to the catalog-validated atlas builder. The legacy interleaved `NewINIFormat` map encoding remains separate.
- Cargo formatting, compilation, tests and Clippy remain unverified in this environment.

The provider now also exposes `build_ra_terrain_atlas`, which gathers unique template IDs from a decoded RA map, loads only the matching theater ICON sets, and passes the row-major (template type, logical icon) references into the catalog-validated renderer atlas builder. The caller still supplies the decoded map, palette, and parsed MIX archives. Retail archive/scenario validation remains outstanding.

The RA MapPack decoder now also accepts the concatenated INI `[MapPack]` Base64 value directly. Its strict decoder ignores ASCII whitespace, validates quartet/padding structure, bounds the decoded compressed payload based on map dimensions and the upstream chunk limit, then invokes the LCW chunk and two-plane decoder. The API does not parse INI text or determine section key order; callers pass the already-concatenated values in section order.

The TODO now separates the completed Red Alert MapPack path from the still-outstanding Tiberian Dawn binary map decoder. The legacy MIX status also reflects the newly implemented filename-to-CRC lookup; this does not claim encrypted MIX support or retail archive validation.
