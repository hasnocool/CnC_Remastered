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
| Workspace | Starter | Cargo workspace with six crates |
| Variant map extents | Foundation | TD 64×64; RA 128×128, from the respective upstream `DEFINES.H` |
| Simulation clock | Foundation | 15 ticks/second, from both upstream `DEFINES.H` files |
| Cell and region addressing | Foundation | Source cell dimensions and 4×4 region dimensions |
| Facing identifiers | Foundation | Eight source-defined facings in source order |
| Binary reader | Initial | Checked little-endian reads |
| Legacy MIX archive | Partial | Reads the legacy 6-byte header and 12-byte index entries used by upstream `MixFileClass`; no filename CRC mapping yet |
| Event scheduler | Initial | Stable ordering by tick and insertion sequence |
| Map editor model | Initial | Cell layer IDs and waypoint storage only |
| Tiberian Dawn gameplay | Not ported | Variant boundary only |
| Red Alert gameplay | Not ported | Variant boundary only |
| Rendering and audio | Not ported | No renderer/audio backend |
| Networking | Not ported | No wire protocol or lockstep implementation |
| Map editor UI and full codecs | Not ported | No GUI, triggers, team types, or full scenario serialization |

## Compatibility references

- `TIBERIANDAWN/DEFINES.H`: TD map grid, regions, tick rate, facing enumeration.
- `REDALERT/DEFINES.H`: RA map grid and shared timing/facing definitions.
- `TIBERIANDAWN/MIXFILE.H` and `TIBERIANDAWN/MIXFILE.CPP`: legacy MIX header, index records, relative data offsets, and CRC-sorted lookup.
- `TIBERIANDAWN/TiberianDawn.vcxproj` and `REDALERT/RedAlert.vcxproj`: Win32 DLL, Windows SDK 8.1, v141, one-byte structure packing, MASM, and native Windows libraries.
- `CnCTDRAMapEditor/CnCTDRAMapEditor.csproj`: WinForms editor and .NET Framework 4.6.2.

## Verification status

No Rust build/test result is claimed in this commit. The execution environment used to prepare this change did not have `rustc` or `cargo` installed. The workspace includes unit tests for the foundations; run the commands in the README on a Rust 1.85+ host before treating this milestone as verified.
