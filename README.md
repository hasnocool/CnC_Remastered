# Unofficial Rust reimplementation — work in progress

This repository is the start of a **native Rust reimplementation** of the game-engine and map-editor source released by Electronic Arts in [CnC_Remastered_Collection](https://github.com/electronicarts/CnC_Remastered_Collection).

It is not a C++ source mirror, a wrapper around the original DLLs, or a completed 1:1 port. The current commit establishes Rust workspace boundaries and a small set of compatibility foundations. Gameplay and editor parity remain incomplete.

## Scope

The upstream release contains two Win32 C++ game DLL projects: `TIBERIANDAWN/TiberianDawn.vcxproj` and `REDALERT/RedAlert.vcxproj`. It also contains a Windows Forms map editor targeting .NET Framework 4.6.2 at `CnCTDRAMapEditor/CnCTDRAMapEditor.csproj`.

The native projects target Win32, Windows SDK 8.1, Visual Studio toolset v141, and MASM. This Rust project is intended to replace those platform-bound implementations with portable Rust code while preserving observable game rules, data formats, simulation timing, and editor behavior.

## Current implementation

- `rts-core`: source-derived map extents, cell/region addressing, 8-way facing, 15 Hz clock, both variants' simulation RNGs (including Red Alert's seeded 15-bit LCG), and Red Alert Q16.16 fixed-point arithmetic.
- `rts-formats`: checked binary readers, a bounds-checked legacy MIX parser, the original rotate-and-add `CRCEngine` checksum, plus checksum-aware push/pull stream adapters, a bounds-checked LCW codec, and framed LCW block streams.
- `rts-assets`: backend-independent TD/RA theater template lookup, ICONSET MAP-indirection decoding, palette expansion, and deduplicated RGBA terrain images.
- `rts-sim`: deterministic event scheduling with stable same-tick ordering.
- `map-editor-model`: an initial typed map document with cell layers and waypoints.
- `tiberian-dawn` / `red-alert`: separate variant boundary crates; actual game rules are not yet ported.

See [PORTING_STATUS.md](PORTING_STATUS.md), [TODO.md](TODO.md), [the 494-file C++ port inventory](docs/CPP_PORT_INVENTORY.md), and [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md). These files are the source of truth for parity work.

## Build and test

Requires Rust and Cargo 1.95 or newer:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

No proprietary game data, Steam files, compiled game binaries, or Electronic Arts artwork are included. A future complete game build will require legally obtained game data in a format supported by the port.

## Upstream provenance and licensing

The upstream source snapshot used for compatibility research is pinned at:

- Repository: https://github.com/electronicarts/CnC_Remastered_Collection
- Commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`

The upstream `LICENSE.md` is retained in this repository. It contains GPL v3 **and additional Section 7 terms**. Read it before copying, modifying, or redistributing any derived code. In particular, the additional terms do not grant EA trademark rights, prohibit claiming EA affiliation, and require modified versions to be clearly marked as modified. The repository name was specified by the maintainer; it does not imply EA endorsement or affiliation. Choose a non-trademarked name for distributed modified builds and promotion.

This is an independent, unofficial community project. It is not affiliated with, sponsored by, or endorsed by Electronic Arts.

## Parity policy

A subsystem is not considered ported merely because a Rust module with a similar name exists. Parity requires source traceability, behavior-focused tests, compatible fixtures where legally available, and explicit notes for any intentional differences. No 1:1 compatibility claim should be made until the corresponding subsystem has passed that process.

## Desktop platform architecture

The sole user-facing launcher is `rts-launcher` (`crates/launcher/rts-launcher`). It owns application selection, settings, LAN discovery UI, and process supervision for the TD, RA, and map-editor executables. The optional `cnc-client` binary is only a bootstrap that locates and starts the adjacent `rts-launcher`; it does not implement a second launcher UI.

Run the launcher and bootstrap with:

```bash
cargo run -p rts-launcher
cargo run -p cnc-client
```

The launcher reads `configs/launcher.toml` by default; copy `configs/launcher.toml.example` and adjust executable paths for your build layout. It supervises child processes on a dedicated Tokio worker thread. Settings persistence is moved off the egui frame callback.

Rendering and asset responsibilities are currently split as follows:

- `renderer`: low-level wgpu surface, texture cache, atlas uploads, sprite batches, camera-aware terrain composition, and GPU presentation. It also currently contains `TemplateAssetProvider` and CPU terrain-atlas construction; these backend-independent pieces should be extracted.
- `rts-render`: egui-based map-canvas widget and camera/palette presentation helpers. It currently paints placeholder layer colors and does not yet consume authentic terrain assets.
- `rts-formats`: variant-aware TD BIN, RA MapPack, TMP, ICONSET, MIX, and palette decoding.
- `rts-assets`: source-derived theater template resolution and renderer-independent RGBA terrain tile sets.

The editor can load a user-supplied MIX archive and palette, select a TD/RA theater and RGB6/RGB8 palette encoding, and decode authentic template imagery on its background asset worker. `rts-assets` returns a renderer-independent tile set; `rts-render` adapts those images to egui textures. The wgpu renderer retains its own atlas upload and sprite-batch adapter. The TD and RA binaries remain application shells, not playable game runtimes. The editor's JSON format remains a development interchange format, not Westwood scenario serialization. LAN discovery and TCP framing remain transport primitives, not authenticated multiplayer or deterministic session integration.

See [docs/PLATFORM_IMPLEMENTATION.md](docs/PLATFORM_IMPLEMENTATION.md) and [docs/PLATFORM_ARCHITECTURE.md](docs/PLATFORM_ARCHITECTURE.md) for current boundaries and remaining work.
