# Desktop platform integration

## Components
- `rts-launcher`: sole user-facing launcher; owns application selection, settings, LAN discovery UI, and child-process supervision.
- `cnc-client`: optional bootstrap that locates and starts the adjacent `rts-launcher`; it is not a second launcher UI.
- `rts-formats`: checked decoding of legacy formats including MIX, ICONSET, palettes, TD BIN, and RA MapPack.
- `rts-assets`: backend-independent theater template provider and deduplicated RGBA terrain tile sets.
- `renderer`: low-level wgpu surface/device backend, texture cache, atlas uploads, sprite batches, and camera-aware terrain composition.
- `rts-render`: egui map canvas and editor interaction/presentation helpers; consumes shared decoded terrain images.
- `rts-net`: async UDP discovery and bounded TCP lobby message framing.
- `map-editor`: standalone editor with four cell layers and waypoint placement.
- `tiberian-dawn` / `red-alert`: separate application shells.

## Authentic terrain integration
The editor accepts a user-supplied MIX archive and 768-byte palette, a TD/RA theater, and RGB6/RGB8 palette encoding. Archive parsing, palette loading, ICONSET lookup, MAP-indirection resolution, and RGBA expansion run on the editor's background worker. The shared `rts-assets::TerrainAssetSet` contains deduplicated decoded tile images and row-major cell references. `rts-render` adapts these images into nearest-neighbor egui textures and paints template cells.

The wgpu renderer retains a compatibility facade around the shared provider and its own GPU atlas/sprite adapter. Multiple-MIX archive precedence in the editor and a shared packed-atlas/UV model remain follow-up work.

## Build and test
```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
cargo run -p rts-launcher
cargo run -p map-editor
cargo run -p tiberian-dawn
cargo run -p red-alert
cargo run -p cnc-client
```

## Limitations and verification
- Game executables remain application shells, not playable game runtimes.
- The map editor JSON format remains a development interchange format, not Westwood scenario serialization.
- LAN discovery and TCP framing remain transport primitives, not authenticated multiplayer or deterministic session integration.
- The editor's initial asset UI accepts one MIX archive at a time.
- No successful build/test/Clippy result is claimed until these commands run with Rust 1.95+ and their output is recorded.
