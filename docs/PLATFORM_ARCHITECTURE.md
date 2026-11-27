# Desktop platform architecture

## Canonical application lifecycle

`rts-launcher` is the only user-facing launcher. It owns:
- Application selection for Tiberian Dawn, Red Alert, and the map editor.
- Versioned TOML settings and installation paths.
- LAN discovery UI.
- Child-process launch and exit supervision through a dedicated Tokio worker.

`cnc-client` is a minimal bootstrap executable. It locates the adjacent `rts-launcher` (or the explicit `RTS_LAUNCHER` override), forwards arguments, and returns the launcher's exit status. It has no launcher UI, settings model, application registry, or process-supervision policy.

The former `launcher` package was removed. Its wgpu preview menu and duplicate application registry/configuration are no longer separate application surfaces. The standalone `rts-launcher` package is the canonical entry point.

## Rendering boundaries

| Component | Owns | Must not own |
|---|---|---|
| `renderer` | wgpu surface/device lifecycle, GPU texture cache, atlas uploads, sprite batching, low-level draw submission, camera-aware terrain chunk traversal | Editor widgets, simulation state, legacy asset decoding policy |
| `rts-render` | egui map canvas, camera/palette presentation helpers, editor interaction-to-cell mapping | wgpu surface/device lifecycle, simulation state, legacy asset decoding policy |
| `rts-formats` | Bounds-checked decoding and encoding of supported legacy binary formats | GPU objects, egui widgets, game rules |
| `tiberian-dawn` / `red-alert` | Variant-specific asset interpretation and game rules as those ports are implemented | Window lifecycle and backend-specific rendering |

Both renderer crates remain because they serve different layers. The egui canvas currently uses placeholder colors and is not yet connected to authentic TD/RA theater assets. The next renderer milestone should introduce a shared, renderer-independent terrain visual model (stable tile identity, palette-expanded pixels or indexed image data, and explicit transparency) that can be consumed by either backend without making the editor depend on wgpu internals.

## Next integration milestones

1. **Asset-provider contract:** define a shared terrain asset provider contract for theater metadata, tile identity, decoded pixel data, and transparency semantics. Keep TD and RA format differences explicit.
2. **Authentic theater loading:** implement TD/RA providers over the existing TMP/ICONSET/shape decoding APIs; validate palette and tile dimensions with fixtures.
3. **Renderer adapters:** implement a wgpu atlas adapter in `renderer` and an egui texture adapter in `rts-render`. Avoid duplicating legacy decoding.
4. **Editor integration:** have the map editor request tile visuals through the provider and display real theater tiles; retain JSON only as a development interchange format until Westwood scenario codecs are implemented.
5. **Runtime integration:** share the same asset-provider contract with game runtime shells, then replace placeholder render scenes with actual map/simulation state.

## Verification policy

Run these from the repository root with the workspace's declared Rust toolchain:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

A GitHub commit or successful source review is not a build result. Record exact toolchain versions and command output before marking a platform milestone verified.
