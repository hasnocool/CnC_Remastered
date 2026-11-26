# Desktop platform implementation

## Workspace additions

- crates/engine/rts-render: shared egui painter renderer. eframe's native backend uses wgpu; renderer owns presentation primitives, not authoritative simulation.
- crates/engine/rts-net: asynchronous UDP LAN discovery, TCP length-prefixed JSON lobby transport, protocol fingerprints, and deterministic lockstep command ordering.
- crates/launcher/rts-launcher: standalone desktop command center, TOML settings, process supervision, and LAN server discovery.
- crates/editor/map-editor: standalone map editor with terrain-ID painting and background-thread JSON file operations.
- apps/cnc-client: small bootstrap process that starts the launcher.
- tiberian-dawn and red-alert now have independent executable shells using the shared renderer.

## Dependency direction

cnc-client -> rts-launcher
rts-launcher -> process boundary -> tiberian-dawn | red-alert | map-editor
tiberian-dawn + red-alert + map-editor -> rts-render -> map-editor-model -> rts-core
rts-net -> protocol/transport only
rts-sim -> deterministic simulation only

No simulation crate depends on a renderer or asynchronous runtime. Networking carries commands and protocol messages; it does not mutate simulation state. The caller must validate compatibility and apply commands at deterministic simulation ticks.

## Build and run

- cargo fmt --all -- --check
- cargo test --workspace
- cargo clippy --workspace --all-targets -- -D warnings
- cargo build --workspace
- cargo run -p rts-launcher
- cargo run -p map-editor
- cargo run -p tiberian-dawn
- cargo run -p red-alert
- cargo run -p cnc-client

The launcher defaults to target/debug/tiberian-dawn, target/debug/red-alert, and target/debug/map-editor. Build the workspace before launching from the repository root, or edit executable paths in configs/launcher.toml.

## Current functional scope

### Renderer
- GPU-backed desktop presentation through eframe/wgpu.
- Map-cell rendering for template, overlay, smudge, terrain, and ground layers.
- Grid, cell selection, and camera transform primitives.
- No decoded SHP/TMP/ICONSET/WSA sprite pipeline, animation renderer, palette conversion, fog-of-war, or game-object render extraction yet.

### Launcher
- Separate desktop process and bootstrap executable.
- Game/editor process launch, exit monitoring, and error reporting on a background Tokio worker.
- Persistent TOML settings with atomic temp-file replacement.
- LAN discovery client and server list.
- No installer, self-updater, asset acquisition, mod dependency resolver, or production online-account system yet.

### Map editor
- Create a bounded TD or RA map document.
- Paint terrain, template, overlay, and smudge IDs into the existing editor model.
- Place uniquely numbered waypoints on selected cells.
- Grid, zoom, cell selection, JSON save/load on a file-I/O worker.
- JSON is an editor interchange format only; it is not the original Westwood scenario format. Building/unit placement, triggers, teams, theatre palettes, validation, undo/redo, and playtest integration remain to be implemented.

### Networking
- UDP broadcast discovery request and async beacon responder.
- Length-prefixed JSON TCP framing with a 1 MiB maximum frame.
- Typed lobby messages, build/simulation/mod fingerprints, and stable lockstep command sort key.
- No cryptographic authentication, NAT traversal, internet matchmaking, relay, anti-cheat, reconnect state restoration, or end-to-end deterministic replay yet.

### Game applications
The two executable entry points deliberately identify themselves as application shells. The source port's current gameplay coverage is not sufficient to label either executable a playable game. Keep variant-specific simulation and data ownership separate.