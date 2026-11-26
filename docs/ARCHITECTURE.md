# Rust port architecture

## Goals

Reimplement the released engine and editor behavior in Rust, preserving externally observable rules and data compatibility. The target is behavioral parity, not line-for-line translation. Platform services are replaceable; simulation rules and serialized formats remain explicit and deterministic.

## Workspace

- `rts-core`: platform-independent value types shared by both variants.
- `rts-formats`: untrusted binary readers and codecs for archive, asset, scenario, and save data.
- `rts-sim`: deterministic scheduling and shared simulation runtime.
- `map-editor-model`: editor-side document representation, operations, validation, and undoable edits.
- `tiberian-dawn`: TD-specific definitions, rules, content tables, AI, campaign, and runtime composition.
- `red-alert`: RA-specific definitions, rules, content tables, AI, campaign, and runtime composition.
- `renderer`: presentation-only GPU abstractions and the wgpu window-surface backend.
- `launcher`: executable entry point, application registry, settings, logging, and shared lifecycle.

## Application runtime

The launcher owns process startup/shutdown and selects one of three application identities: Tiberian Dawn, Red Alert, or Map Editor. Variant crates remain independent; shared rendering and platform services are injected at the application boundary rather than imported into deterministic simulation modules.

The initial launcher exposes `--app td|ra|editor`, `--list`, and an interactive terminal selector. It opens an application-specific window, initializes the renderer, drives redraw/resize events, and persists versioned settings under the platform configuration directory. The registry is an integration seam: the existing game and editor crates do not yet expose complete runtime APIs.

## Rendering boundary

`renderer` owns GPU objects, presentation surfaces, viewport/camera presentation values, render-layer ordering, and sprite submission data. It does not own game rules, simulation clocks, random state, map mutation, or authoritative coordinates. Simulation emits presentation commands; the renderer consumes them.

The initial backend uses wgpu 26.0.1 and winit 0.30.12. The workspace MSRV remains Rust 1.85. The current backend initializes an adapter/device, configures a surface, clears and presents frames, and responds to resize events. GPU sprite pipelines, texture upload/cache management, palette-aware SHP/TMP decoding, and offscreen rendering remain follow-up work.

## Settings and lifecycle

Launcher configuration is versioned TOML and separates display, audio, input, performance, and network preferences. Values are validated before use. Writes go to a sibling temporary file and are atomically renamed into place. Network settings are configuration only at this stage; transport, session discovery, cancellation, and lockstep are separate milestones.

## Compatibility constraints recovered from upstream

1. The original solution builds two separate Win32 DLL projects; they remain separate variant entry points.
2. TD defines a 64×64 global cell grid; RA defines a 128×128 global cell grid.
3. Both variants define 15 simulation ticks per second.
4. Both variants use 4×4 map regions with a two-region border.
5. Legacy MIX entries are bounds-checked and preserve source integer ordering.
6. The editor is a separate C# WinForms application; porting requires translating editor semantics, not merely replacing its window toolkit.
7. Editor CellMetrics uses row-major indexing and eight adjacent facings.

## Determinism

- Every simulation time value is an integer tick.
- State iteration must use stable ordering.
- Input and network commands must be applied in a defined sequence.
- Random number generators must be ported from upstream algorithms and covered by golden vectors.
- Floating-point values are presentation-only; they must not enter authoritative simulation state.

## Data format policy

Parsing code is fallible and bounds-checked. Every count, offset, size, string length, and index read from a file must be validated before allocation or slicing. Format codecs must preserve unknown fields where necessary for lossless editor round trips. Golden fixtures must be legally distributable and must not bundle copyrighted game content.

## Port completion criteria

A module is complete only when all source responsibilities are mapped, error handling and boundary behavior match, binary layouts are proven, tests cover normal/edge/malformed cases, and intentional divergences are documented.
