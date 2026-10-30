# Unofficial Rust reimplementation — work in progress

This repository is the start of a **native Rust reimplementation** of the game-engine and map-editor source released by Electronic Arts in [CnC_Remastered_Collection](https://github.com/electronicarts/CnC_Remastered_Collection).

It is not a C++ source mirror, a wrapper around the original DLLs, or a completed 1:1 port. The current commit establishes Rust workspace boundaries and a small set of compatibility foundations. Gameplay and editor parity remain incomplete.

## Scope

The upstream release contains two Win32 C++ game DLL projects: `TIBERIANDAWN/TiberianDawn.vcxproj` and `REDALERT/RedAlert.vcxproj`. It also contains a Windows Forms map editor targeting .NET Framework 4.6.2 at `CnCTDRAMapEditor/CnCTDRAMapEditor.csproj`.

The native projects target Win32, Windows SDK 8.1, Visual Studio toolset v141, and MASM. This Rust project is intended to replace those platform-bound implementations with portable Rust code while preserving observable game rules, data formats, simulation timing, and editor behavior.

## Current implementation

- `rts-core`: source-derived map extents, cell/region addressing, 8-way facing, 15 Hz clock, both variants' simulation RNGs (including Red Alert's seeded 15-bit LCG), and Red Alert Q16.16 fixed-point arithmetic.
- `rts-formats`: checked binary readers, a bounds-checked legacy MIX parser, the original rotate-and-add `CRCEngine` checksum, plus checksum-aware push/pull stream adapters.
- `rts-sim`: deterministic event scheduling with stable same-tick ordering.
- `map-editor-model`: an initial typed map document with cell layers and waypoints.
- `tiberian-dawn` / `red-alert`: separate variant boundary crates; actual game rules are not yet ported.

See [PORTING_STATUS.md](PORTING_STATUS.md), [TODO.md](TODO.md), [the 494-file C++ port inventory](docs/CPP_PORT_INVENTORY.md), and [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md). These files are the source of truth for parity work.

## Build and test

Requires a Rust toolchain with Cargo (Rust 1.85 or newer):

```bash
cargo fmt --all -- --check
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
