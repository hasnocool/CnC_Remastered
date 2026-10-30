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
