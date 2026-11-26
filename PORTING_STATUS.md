# Porting status

**Overall status: early foundations only — not a playable game and not a 1:1 port.**

## Upstream baseline

- Upstream: `electronicarts/CnC_Remastered_Collection`
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source tree at that commit: 1,331 files (about 37.8 MB).
- Game projects: Tiberian Dawn and Red Alert C++ DLLs, both configured for Win32.
- Editor project: C# Windows Forms, .NET Framework 4.6.2.

## Implemented in this repository

| Area | State | Evidence |
|---|---|---|
| Workspace | Expanded | Eight crates, including dedicated renderer and launcher crates |
| Renderer | Initial wgpu backend | Window surface/device setup, clear/present, resize, viewport/camera/sprite/layer abstractions, frame timing |
| Launcher | Initial executable | CLI/interactive application selection, TD/RA/editor registry, shared window lifecycle and shutdown |
| Settings | Initial | Versioned TOML settings for display/audio/input/performance/network; validation and atomic replacement |
| Logging | Initial | Structured tracing subscriber with environment filter |
| Variant map extents | Foundation | TD 64×64; RA 128×128, from respective upstream `DEFINES.H` |
| Simulation clock | Foundation | 15 ticks/second, from both upstream `DEFINES.H` files |
| Cell and region addressing | Foundation | Source cell dimensions and 4×4 region dimensions |
| Facing identifiers | Foundation | Eight source-defined facings in source order |
| Binary reader | Initial | Checked little-endian reads |
| Editor CRC-32 | Initial | Standard reflected CRC-32; checked against the 123456789 vector |
| Editor cell metrics | Initial | Row-major indexing, 8-way adjacency, clipping, bounds checks |
| Legacy MIX archive | Partial | Reads legacy header/index; filename CRC mapping remains outstanding |
| Event scheduler | Initial | Stable ordering by tick and insertion sequence |
| Simulation RNG | Ported | Exact shared 256-byte table and variant-specific Sim_IRandom routines |
| Red Alert fixed point | Ported | Q16.16 constructors, arithmetic helpers, rounding, saturation, ASCII conversion |
| Map editor model | Initial | Cell layer IDs and waypoint storage only |
| Tiberian Dawn gameplay | Not ported | Variant boundary only |
| Red Alert gameplay | Not ported | Variant boundary only |
| Rendering parity | Not ported | Generic GPU surface presenter only; no SHP/TMP sprite pipeline or palette parity |
| Networking | Not implemented | Settings schema only; no async transport/session implementation |
| Map editor UI and full codecs | Not ported | No editor frontend, triggers, team types, or full scenario serialization |

## C++ port coverage

- Upstream translation units inventoried: 494.
- Fully ported C++ translation units: 281.
- Remaining C++ translation units: 213. See [the file-by-file inventory](docs/CPP_PORT_INVENTORY.md).

## Compatibility references

- `TIBERIANDAWN/DEFINES.H`: TD map grid, regions, tick rate, facing enumeration.
- `TIBERIANDAWN/RAND.CPP` and `REDALERT/RAND.CPP`: matching deterministic table and variant-specific routines.
- `REDALERT/RANDOM.H` / `REDALERT/RANDOM.CPP`: seeded 15-bit LCG RandomClass.
- `TIBERIANDAWN/MIXFILE.H` and `TIBERIANDAWN/MIXFILE.CPP`: legacy MIX header, index records, relative data offsets, and CRC-sorted lookup.
- `TIBERIANDAWN/TiberianDawn.vcxproj` and `REDALERT/RedAlert.vcxproj`: Win32, Windows SDK 8.1, v141, packing, MASM, and native libraries.
- `CnCTDRAMapEditor/CnCTDRAMapEditor.csproj`: WinForms editor and .NET Framework 4.6.2.
- `wgpu 26.0.1`: selected as the newest renderer release line compatible with the workspace's Rust 1.85 MSRV.

## Verification status

**Not yet verified in a Rust toolchain.** The repository connector can commit source files but cannot run Cargo. Do not treat compilation or tests as passing until these commands are executed on a Rust 1.85+ host:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

The launcher currently presents an application-specific window and persists selection/settings. The selected entries are integration boundaries, not completed game/editor runtimes. The renderer currently clears and presents a real wgpu surface; sprite GPU pipelines, texture upload and palette-accurate game rendering remain open.
