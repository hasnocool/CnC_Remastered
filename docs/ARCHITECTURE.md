# Rust port architecture

## Goals

Reimplement the released engine and editor behavior in Rust, preserving externally observable rules and data compatibility. The target is behavioral parity, not line-for-line translation. Platform services are replaceable; simulation rules and serialized formats remain explicit and deterministic.

## Workspace

- `rts-core`: platform-independent value types shared by both variants: map coordinates, variant dimensions, directions, and simulation ticks.
- `rts-formats`: untrusted binary readers and codecs for archive, asset, scenario, and save data.
- `rts-sim`: deterministic scheduling and, over later milestones, the shared simulation runtime.
- `map-editor-model`: editor-side document representation, operations, validation, and undoable edits.
- `tiberian-dawn`: TD-specific definitions, rules, content tables, AI, campaign, and runtime composition.
- `red-alert`: RA-specific definitions, rules, content tables, AI, campaign, and runtime composition.

Future platform crates should isolate rendering, audio, input, filesystem, network transport, and optional online services. The simulation must not depend on a renderer or wall-clock time.

## Compatibility constraints recovered from upstream

1. The original solution builds two separate Win32 DLL projects; they should remain separate variant entry points in Rust.
2. TD defines a 64×64 global cell grid; RA defines a 128×128 global cell grid.
3. Both variants define 15 simulation ticks per second.
4. Both variants use 4×4 map regions with a two-region border in the region-grid dimensions.
5. The legacy MIX implementation reads a 16-bit count followed by a 32-bit data size, then 12-byte entries (CRC, data-relative offset, size). The engine project sets structure member alignment to one byte. The parser here validates bounds and keeps CRCs as signed 32-bit values to preserve the legacy comparison ordering.
6. The editor is a separate C# WinForms application targeting .NET Framework 4.6.2; porting it requires translating editor semantics, not merely replacing its window toolkit.
7. The editor's CellMetrics uses row-major indexing (y × width + x), eight adjacent facings, an unchanged-location None direction, and edge clipping. Its Utility/CRC.cs helper is standard reflected CRC-32; it is not the legacy MIX filename checksum.

## Determinism

- Every simulation time value is an integer tick.
- State iteration must use stable ordering.
- Input and network commands must be applied in a defined sequence.
- Random number generators must be ported from the upstream algorithm and covered by golden vectors before gameplay uses them.
- Floating-point values are prohibited in authoritative simulation state unless an upstream dependency makes exact fixed-point parity impossible and the deviation is documented.

## Data format policy

Parsing code is fallible and bounds-checked. Every count, offset, size, string length, and index read from a file must be validated before allocation or slicing. Format codecs must preserve unknown fields where doing so is necessary for lossless editor round trips. Golden fixtures must be legally distributable and must not bundle copyrighted game content.

## Port completion criteria

A module is complete only when:
- all source responsibilities have a mapped Rust implementation;
- relevant error handling and boundary behavior match;
- binary layouts are proven against source and legal fixtures;
- tests cover normal, edge, malformed, and compatibility cases;
- any intentional divergence is recorded in the porting status.

## Application runtime

The launcher owns process startup/shutdown and selects one of three application identities: Tiberian Dawn, Red Alert, or Map Editor. Variant crates remain independent; shared rendering and platform services are injected at the application boundary rather than imported into deterministic simulation modules.

The initial launcher exposes `--app td|ra|editor`, `--list`, and an interactive terminal selector. It opens an application-specific window, initializes the renderer, drives redraw/resize events, and persists versioned settings under the platform configuration directory. GPU initialization runs on a worker thread; the event loop polls a one-shot channel rather than blocking while the adapter/device are created. The registry is an integration seam: the existing game and editor crates do not yet expose complete runtime APIs.

## Rendering boundary

`renderer` owns GPU objects, presentation surfaces, viewport/camera presentation values, render-layer ordering, and sprite submission data. It exposes a backend-independent `PresentationBackend` contract and a wgpu implementation. It does not own game rules, simulation clocks, random state, map mutation, or authoritative coordinates. Simulation emits presentation commands; the renderer consumes them.

The initial backend uses wgpu 26.0.1 and winit 0.30.12 while retaining the workspace Rust 1.85 MSRV. It initializes an adapter/device, configures a surface, clears and presents frames, handles resize, records frame timing, and honors vsync/low-power preferences. GPU sprite pipelines, texture upload/cache management, palette-aware SHP/TMP decoding, and offscreen rendering remain follow-up work.

## Settings and lifecycle

Launcher configuration is versioned TOML and separates display, audio, input, performance, and network preferences. Values are validated before use. Writes are synced to a temporary file in the destination directory and atomically replace the settings file. Network settings are configuration only at this stage; transport, session discovery, cancellation, and lockstep are separate milestones.
