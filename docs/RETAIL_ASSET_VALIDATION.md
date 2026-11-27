# Retail asset validation

The `asset-validate` binary exercises the existing TD and RA ingestion APIs against files supplied from a user's own game installation. It does not download, redistribute, modify, or copy game assets into the repository.

## Build and run

```bash
# Tiberian Dawn editor BIN + scenario metadata
cargo run -p renderer --bin asset-validate -- \
  --variant td \
  --scenario /path/to/SCG01EA.INI \
  --map-bin /path/to/SCG01EA.BIN \
  --palette /path/to/TEMPERAT.PAL \
  --palette-encoding rgb8 \
  --mix /path/to/TEMPERAT.MIX \
  --mix /path/to/SCORES.MIX

# Red Alert scenario [MapPack]
cargo run -p renderer --bin asset-validate -- \
  --variant ra \
  --scenario /path/to/SCG01EA.INI \
  --palette /path/to/TEMPERAT.PAL \
  --mix /path/to/TEMPERAT.MIX \
  --mix /path/to/LOCAL.MIX
```

Use `--help` for the complete option list. `--theater` can override scenario theater metadata when the scenario's INI dialect stores it in an unsupported location. `--palette-encoding` is required so the tool never guesses: choose RGB8 or RGB6 based on the supplied palette's known encoding.

## Validation performed

- Parse each supplied legacy MIX archive with the existing bounded `MixArchive` reader. The report includes archive count and indexed-entry count.
- Derive the theater-specific ICON filenames from the source-derived template catalog, calculate their Westwood filename CRCs, and resolve them through the caller-ordered MIX archive list.
- TD: read map dimensions and theater from scenario INI metadata, decode the supplied raw editor BIN byte-pair terrain stream, load referenced ICON sets, validate logical MAP indirection and palette expansion, then compose the RGBA atlas.
- RA: read dimensions/theater and numeric `[MapPack]` entries from the scenario INI, concatenate entries in numeric order, decode Base64 + framed LCW + the new-format two-plane map, resolve referenced ICON sets, validate logical MAP indirection and palette expansion, then compose the RGBA atlas.
- Validate that every occupied map cell resolves to a real logical icon. Report the final atlas dimensions, byte count, resolved-cell count, and deterministic FNV-1a checksum.
- Retain synthetic parser/ordering/theater/checksum tests in the binary module. Retail files remain external and are never fixtures in the repository.

## Important scope limits

- MIX support here is the existing legacy unencrypted MIX layout. Encrypted/extended MIX variants are not decoded by this harness; archives that fail the parser are reported as failures, not silently skipped.
- TD input is the Remastered Collection editor's raw `.BIN` pair stream. It is not the engine's separate LCW straw/stream representation.
- RA input is the new-format `[MapPack]` plane layout. Legacy interleaved `NewINIFormat` variants are not supported by this path.
- The INI reader is deliberately small and targets scenario metadata plus numeric `[MapPack]` keys; it is not a complete Westwood INI compatibility implementation.
- The tool validates CPU decoding and atlas composition only. It does not initialize a GPU, compare against a known-good retail screenshot, or prove gameplay/rendering parity.
- A successful run against actual user-supplied retail files is the evidence required to mark retail validation complete. Synthetic unit tests alone do not satisfy that milestone.

## Exit status

Returns zero only after the selected scenario has decoded, every referenced template asset has been found in the supplied MIX archives, every cell's logical icon has resolved, and the palette-backed atlas has been composed. Any missing file, malformed archive/map, unsupported theater, invalid palette, missing template asset, or unresolved icon returns a nonzero exit status.
