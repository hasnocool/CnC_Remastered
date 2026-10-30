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
| Editor CRC-32 | Initial | Standard reflected CRC-32; checked against the 123456789 vector |
| Editor cell metrics | Initial | Row-major indexing, 8-way adjacency, clipping, bounds checks |
| Legacy MIX archive | Partial | Reads the legacy 6-byte header and 12-byte index entries used by upstream `MixFileClass`; no filename CRC mapping yet |
| Event scheduler | Initial | Stable ordering by tick and insertion sequence |
| Simulation RNG | Ported | Exact shared 256-byte table; byte index increments before lookup; both variant Sim_IRandom scaling routines |
| Red Alert fixed point | Ported | Q16.16 constructors, arithmetic helpers, rounding, saturation, ASCII conversion |
| Legacy checksum | Ported | Westwood rotate-left/add word checksum with partial-word staging |
| CRC pipe/straw adapters | Ported | Checksum observing push and pull stream adapters |
| CacheStraw | Ported | Buffered read-ahead over demand-driven input |
| SHA pipe/straw adapters | Ported | Non-transforming SHA-1 observation over push and pull streams |
| Buffer/file endpoints | Ported | Fixed-buffer, generic Read/Write, and lazy path-backed file endpoints |
| Pipe/Straw chains | Ported | Push forwarding/relink/flush and pull-demand/relink chains; cycle detection added |
| Rect geometry | Ported | Source-compatible validity, intersection, anchor adjustment, and union |
| Packed Boolean vector | Ported | Packed bit operations, resizing, set/reset, search, and equality |
| Line reader | Ported | CR/LF handling, truncation, EOF semantics, ASCII whitespace trimming |
| Launcher stub | Ported | Exact legacy console message |
| ADPCM type-1 audio decoder | Ported | Original 1,424-entry difference/index tables; low-then-high nibble decode; predictor saturation and stream state |
| LCW codec and block streams | Ported | LCW encode/decode commands, little-endian block headers, arbitrary fragmented push input, pull adapters, partial-block flush behavior |
| LZW codec and block streams | Ported | 12-bit code dictionary, 5,021-slot hash probing, EOS code 256, 16-bit little-endian codewords, framed push/pull streams |
| LZO1X codec and block streams | Ported | Deterministic 14-bit hash compressor, M2/M3/M4 match commands, EOF marker, framed push/pull streams |
| Base64 codec and streams | Ported | Four-character packet encoding, ignored invalid input characters, '=' termination, fixed-chunk Pipe/Straw adapters |
| SHA-1 and RandomStraw | Ported | Streaming SHA-1; 32 LCG state words; low-bit-first 1,024-bit seeding; chained digest scrambler |
| Blowfish engine and streams | Ported | Source-derived P/S tables, 16-round block cipher, reverse P-array decryption, 56-byte key limit, 8-byte Pipe/Straw staging |
| Empty translation unit | No behavior | `REDALERT/CLASS.CPP` contains no executable definitions |
| Map editor model | Initial | Cell layer IDs and waypoint storage only |
| Tiberian Dawn gameplay | Not ported | Variant boundary only |
| Red Alert gameplay | Not ported | Variant boundary only |
| Rendering and audio | Not ported | No renderer/audio backend |
| Networking | Not ported | No wire protocol or lockstep implementation |
| Map editor UI and full codecs | Not ported | No GUI, triggers, team types, or full scenario serialization |

## C++ port coverage

- Upstream translation units inventoried: 494.
- Fully ported C++ translation units: 83 (`REDALERT/CLASS.CPP` [empty], `REDALERT/CRC.CPP`, `REDALERT/CRCPIPE.CPP`, `REDALERT/CRCSTRAW.CPP`, `REDALERT/DTABLE.CPP`, `REDALERT/FIXED.CPP`, `REDALERT/LCW.CPP`, `REDALERT/LCWPIPE.CPP`, `REDALERT/LCWSTRAW.CPP`, `REDALERT/LCWUNCMP.CPP`, `REDALERT/LZO1X_C.CPP`, `REDALERT/LZO1X_D.CPP`, `REDALERT/LZOPIPE.CPP`, `REDALERT/LZOSTRAW.CPP`, `REDALERT/LZW.CPP`, `REDALERT/LZWPIPE.CPP`, `REDALERT/LZWSTRAW.CPP`, `REDALERT/ITABLE.CPP`, `REDALERT/PIPE.CPP`, `REDALERT/RAND.CPP`, `REDALERT/RANDOM.CPP`, `REDALERT/READLINE.CPP`, `REDALERT/RECT.CPP`, `REDALERT/RNDSTRAW.CPP`, `REDALERT/SHA.CPP`, `REDALERT/STRAW.CPP`, `REDALERT/ADPCM.CPP`, `REDALERT/STUB.CPP`, `REDALERT/VECTOR.CPP`, `TIBERIANDAWN/RAND.CPP`, `REDALERT/INIBIN.CPP` [empty], `TIBERIANDAWN/WIN32LIB/REGIONSZ.CPP`, `TIBERIANDAWN/WIN32LIB/NEWDEL.CPP`, `REDALERT/Shape.cpp`, `TIBERIANDAWN/Shape.cpp`, `TIBERIANDAWN/WIN32LIB/LOADPAL.CPP`, `REDALERT/WIN32LIB/WINHIDE.CPP`, `TIBERIANDAWN/WIN32LIB/WINHIDE.CPP`, plus the remaining stream, coordinate, facing, link, vector, dipthong, legacy random, font metrics, buffer, timer, timer-system, allocator, palette fade/morph, and platform helper units recorded in the inventory).
- Remaining C++ translation units: 411. See [the file-by-file inventory](docs/CPP_PORT_INVENTORY.md).

## Compatibility references

- `TIBERIANDAWN/DEFINES.H`: TD map grid, regions, tick rate, facing enumeration.
- `TIBERIANDAWN/RAND.CPP` and `REDALERT/RAND.CPP`: matching 256-byte deterministic table and variant-specific Sim_IRandom calls.
- `REDALERT/RANDOM.H` / `REDALERT/RANDOM.CPP`: separate seeded 15-bit LCG RandomClass and rejection-based ranged selection.
- `REDALERT/CRCPIPE.*` and `REDALERT/CRCSTRAW.*`: checksum-observing stream adapter behavior; `PIPE.CPP` and `STRAW.CPP` chain behavior is still outstanding.
- `TIBERIANDAWN/COORDA.ASM` and `REDALERT/COORDA.ASM`: distinct 8-bit and 16-bit Fixed_To_Cardinal arithmetic.
- `REDALERT/FIXED.H` / `REDALERT/FIXED.CPP`: Q16.16 fixed-point value layout and methods.
- `REDALERT/CRC.H` / `REDALERT/CRC.CPP`: 32-bit rotate-and-add checksum engine.
- `REDALERT/DEFINES.H`: RA map grid and shared timing/facing definitions.
- `TIBERIANDAWN/MIXFILE.H` and `TIBERIANDAWN/MIXFILE.CPP`: legacy MIX header, index records, relative data offsets, and CRC-sorted lookup.
- `TIBERIANDAWN/TiberianDawn.vcxproj` and `REDALERT/RedAlert.vcxproj`: Win32 DLL, Windows SDK 8.1, v141, one-byte structure packing, MASM, and native Windows libraries.
- `CnCTDRAMapEditor/CnCTDRAMapEditor.csproj`: WinForms editor and .NET Framework 4.6.2.

## Verification status

No Rust build/test result is claimed in this commit. The execution environment used to prepare this change did not have `rustc` or `cargo` installed. The workspace includes unit tests for the foundations; run the commands in the README on a Rust 1.85+ host before treating this milestone as verified.


Latest addition: `TIBERIANDAWN/WIN32LIB/NEWDEL.CPP` → `crates/rts-core/src/legacy_new_delete.rs` (scalar/array allocation operator delegation).

Latest additions: `TIBERIANDAWN/WIN32LIB/PALETTE.CPP` and `MORPHPAL.CPP` → `crates/rts-formats/src/palette_fade.rs`.
Latest addition: `REDALERT/WIN32LIB/WRITEPCX.CPP` → `crates/rts-formats/src/pcx.rs`.
