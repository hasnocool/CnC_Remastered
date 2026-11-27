# Desktop platform integration

## Components
- `rts-launcher`: the sole user-facing launcher; owns application selection, settings, LAN discovery UI, and child-process supervision.
- `cnc-client`: optional bootstrap that locates and starts the adjacent `rts-launcher`; it is not a second launcher UI or process supervisor.
- `renderer`: low-level wgpu surface/device backend, texture cache, atlas uploads, sprite batches, and camera-aware terrain composition.
- `rts-render`: egui map-canvas widget and editor interaction/presentation helpers; it does not own the wgpu surface lifecycle or legacy asset decoding.
- `rts-net`: async UDP discovery and bounded TCP lobby message framing.
- `map-editor`: standalone editor with four cell layers and waypoint placement.
- `tiberian-dawn` / `red-alert`: separate application shells.

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

## Review findings and limitations
- The former duplicate `launcher` package has been removed. `rts-launcher` is now the canonical user-facing launcher and uses the process supervisor.
- `cnc-client` remains as a thin bootstrap only; it forwards arguments to the adjacent `rts-launcher` executable.
- The two rendering crates have distinct responsibilities: `renderer` owns low-level wgpu rendering; `rts-render` owns the egui editor canvas. Both remain because the APIs are not interchangeable.
- The egui canvas currently paints placeholder layer colors. Authentic TD/RA theater asset providers and shared renderer-independent tile visuals remain the next integration milestone.
- Game executables are render/application shells, not playable games.
- The map editor JSON format is not a legacy scenario codec.
- LAN discovery and TCP framing are transport primitives, not authenticated multiplayer, deterministic session integration, or Westwood wire compatibility.
- No successful build/test/Clippy result is claimed until these commands run with the declared Rust toolchain.
