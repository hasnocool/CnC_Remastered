# Desktop platform integration

## Components
- `renderer`: wgpu sprite/UI backend, texture cache, atlas and camera-aware terrain composition.
- `rts-render`: egui map canvas widget used by current editor/game shells.
- `launcher`: graphical command-center and renderer preview.
- `rts-launcher`: process launcher, settings UI and child process supervisor.
- `rts-net`: async UDP discovery and bounded TCP lobby message framing.
- `map-editor`: standalone editor with four cell layers and waypoint placement.
- `cnc-client`: bootstrap process that starts `rts-launcher`.
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
- The graphical `launcher` currently selects an application identity; its Continue action does not spawn the selected application. The separate `rts-launcher` process supervisor does spawn configured executables.
- The two launcher entry points and two rendering crates are provisional and should be consolidated.
- Game executables are render shells, not playable games.
- The map editor JSON format is not a legacy scenario codec.
- LAN discovery and TCP framing are transport primitives, not production multiplayer or Westwood wire compatibility.
- No successful build/test/Clippy result is claimed for this integration because a Rust toolchain was unavailable during review.
