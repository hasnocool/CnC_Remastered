# Desktop platform architecture

## Canonical application lifecycle

`rts-launcher` is the only user-facing launcher. It owns:
- Application selection for Tiberian Dawn, Red Alert, and the map editor.
- Versioned TOML settings and installation paths.
- LAN discovery UI.
- Child-process launch and exit supervision through a dedicated Tokio worker.

`cnc-client` is a minimal bootstrap executable. It locates the adjacent `rts-launcher` (or the explicit `RTS_LAUNCHER` override), forwards arguments, and returns the launcher's exit status. It has no launcher UI, settings model, application registry, or process-supervision policy.

The former `launcher` package was removed. Its wgpu preview menu and duplicate application registry/configuration are no longer separate application surfaces. The standalone `rts-launcher` package is the canonical entry point.

## Rendering and asset boundaries

| Component | Current responsibility | Boundary to enforce |
|---|---|---|
| `renderer` | wgpu surface/device lifecycle, GPU texture cache, atlas uploads, sprite batching, camera-aware terrain drawing. It also currently contains `template_assets` and CPU `TerrainAtlas` construction. | Keep GPU resources and draw submission here; move MIX/ICONSET/template resolution and renderer-independent terrain images out. |
| `rts-render` | egui map canvas, camera/palette presentation helpers, editor interaction-to-cell mapping. | Keep editor widget and input mapping here; accept decoded tile images/UV data without depending on wgpu. |
| `rts-formats` | Bounds-checked decoding and encoding of supported legacy binary formats, including TD BIN, RA MapPack, TMP, ICONSET, palettes, and MIX. | Keep format decoding independent of GPU objects and UI widgets. |
| `tiberian-dawn` / `red-alert` | Variant-specific game rules and future asset interpretation. | Keep window lifecycle and backend-specific rendering out. |

## Existing authentic terrain path

The repository already contains a substantial authentic template path in `renderer::TemplateAssetProvider`:
- Resolves source-derived template IDs against TD/RA theater catalogs.
- Finds theater-specific ICONSET files in caller-supplied MIX archives.
- Loads ICON/SINF/SSET/TRNS/MAP chunks through `rts-formats`.
- Builds TD terrain from the TD editor BIN decoder and RA terrain from the RA MapPack decoder.
- Expands indexed pixels through the selected `Palette256` and composes a packed `TerrainAtlas`.

This path is not yet used by the map editor. The current `rts-render::MapCanvas` still paints placeholder colors based on whether a cell layer is populated. Also, the asset provider and CPU atlas model live inside the wgpu renderer crate, which makes the editor's reuse awkward and couples asset decoding to a backend.

## Next integration milestones

1. **Extract renderer-independent terrain assets:** create a small shared asset/terrain crate containing the theater-aware provider, decoded tile image model, map-to-tile resolution, and atlas/UV metadata. It may depend on `rts-formats` and `rts-core`, but not on wgpu or egui.
2. **Keep backend adapters thin:** `renderer` uploads the shared RGBA atlas to wgpu; `rts-render` uploads the same image to an egui `TextureHandle` and paints the correct tile UV per cell.
3. **Wire the editor:** add user-configurable asset/MIX and palette paths, load assets off the UI thread, select TD/RA theater, and render actual template tiles. Keep file I/O and archive decoding away from the egui frame callback.
4. **Preserve format differences:** TD BIN remains a row-major pair stream; RA MapPack remains its own compressed two-plane format. Do not normalize away unknown IDs or transparency rules.
5. **Runtime integration:** make both game shells consume the same shared terrain asset contract, then replace placeholder scenes with actual map/simulation state.

## Verification policy

Run these from the repository root with the workspace's declared Rust toolchain:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

A GitHub commit or successful source review is not a build result. Record exact toolchain versions and command output before marking a platform milestone verified.
