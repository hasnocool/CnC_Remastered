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
| `rts-assets` | Theater-aware MIX/ICONSET lookup, MAP-indirection resolution, palette expansion, and deduplicated RGBA terrain images. | Keep decoded CPU assets independent of wgpu and egui. |
| `tiberian-dawn` / `red-alert` | Variant-specific game rules and future asset interpretation. | Keep window lifecycle and backend-specific rendering out. |

## Existing authentic terrain path

The repository's authentic template path has been extracted into `rts-assets::TemplateAssetProvider`:
- Resolves source-derived template IDs against TD/RA theater catalogs.
- Finds theater-specific ICONSET files in caller-supplied MIX archives.
- Loads ICON/SINF/SSET/TRNS/MAP chunks through `rts-formats`.
- Resolves logical icons through each ICONSET MAP chunk and expands indexed pixels through the selected `Palette256`.
- Returns a renderer-independent `TerrainAssetSet` containing row-major cell-to-image references and deduplicated RGBA tile images.

The map editor now accepts a user-supplied MIX archive and palette, lets the user select TD/RA theater and RGB6/RGB8 palette encoding, and performs archive parsing, palette loading, and image decoding on its dedicated background worker. The egui canvas uploads the returned images as nearest-neighbor textures and paints authentic template cells. The wgpu renderer keeps a small compatibility facade around the shared provider while retaining its own atlas upload and visible sprite batching.

## Next integration milestones

1. **Complete the shared asset adapter:** expose atlas packing and stable tile/UV metadata from `rts-assets` so both backends can share one packing decision where useful.
2. **Strengthen editor asset selection:** support multiple MIX archives in upstream search order and validate palette/theater combinations against the selected game variant.
3. **Runtime integration:** make both game shells consume the same shared terrain asset contract, then replace placeholder scenes with actual map/simulation state. Keep file I/O and archive decoding away from the egui frame callback.
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
