# Desktop platform architecture

## Canonical application lifecycle

`rts-launcher` is the only user-facing launcher. It owns application selection, versioned TOML settings, installation paths, LAN discovery UI, and child-process supervision through a dedicated Tokio worker.

`cnc-client` is a minimal bootstrap. It locates the adjacent `rts-launcher` (or the explicit `RTS_LAUNCHER` override), forwards arguments, and returns the launcher's exit status. It has no launcher UI or process-supervision policy.

The former `launcher` package was removed. Its wgpu preview menu and duplicate application registry/configuration are no longer separate application surfaces.

## Rendering and asset boundaries

| Component | Owns | Must not own |
|---|---|---|
| `rts-formats` | Checked decoding/encoding of MIX, ICONSET, TMP, palettes, TD BIN, RA MapPack and other supported legacy formats | GPU objects, UI widgets, game rules |
| `rts-assets` | Source-derived theater template lookup, MIX archive precedence, ICONSET MAP indirection, palette expansion, deduplicated CPU RGBA tile images | wgpu, egui, window lifecycle |
| `renderer` | wgpu surface/device lifecycle, texture cache, GPU atlas upload, sprite batches, camera-aware terrain drawing | Legacy archive lookup and format decoding policy |
| `rts-render` | egui map canvas, editor interaction-to-cell mapping, egui texture handles and tile painting | wgpu surface lifecycle, legacy archive parsing, simulation state |
| `tiberian-dawn` / `red-alert` | Variant-specific game rules and future runtime asset interpretation | Window lifecycle and backend-specific rendering |

## Authentic theater asset flow

The map editor uses the following path:

1. A user selects a MIX archive, palette file, theater, RGB6/RGB8 palette encoding, template type, and logical icon index.
2. The editor's dedicated background worker reads and parses the archive and palette.
3. `rts-assets::TemplateAssetProvider` resolves source-derived template IDs to theater-specific filenames and searches the supplied MIX archive.
4. `rts-formats::IconSet` parses ICON/SINF/SSET/TRNS/MAP chunks. Logical icon indices are resolved through MAP indirection.
5. `rts-assets` expands indexed pixels through the selected palette and returns a `TerrainAssetSet` with row-major cell-to-image references and deduplicated RGBA tile images.
6. `rts-render` creates nearest-neighbor egui textures and paints each populated template cell.

The wgpu renderer retains a compatibility facade around the shared provider and its own packed `TerrainAtlas` upload path. It does not own the authoritative format decoding implementation.

## Remaining integration work

1. Support multiple MIX archives in upstream search order in the editor UI.
2. Validate theater choices against the active game variant before starting background work.
3. Move atlas packing and stable UV metadata into a shared optional asset utility if both backends need identical packing.
4. Integrate authentic theater terrain into the TD/RA runtime shells and replace placeholder scenes with actual map/simulation state.
5. Keep TD BIN and RA MapPack serialization distinct; do not normalize away unknown IDs or transparency rules.

## Verification policy

Run these from the repository root with Rust/Cargo 1.95 or newer:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

A source review or GitHub commit is not a build result. Record exact toolchain versions and command output before marking a platform milestone verified.
