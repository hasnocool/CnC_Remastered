# Original Source Reference Library

This directory explains the original Electronic Arts / Westwood Command & Conquer Remastered Collection source code in plain English, as the reference for the Rust port in this repository.

## Start here

1. [Scope, evidence and documentation rules](00-scope-and-method.md)
2. [System map](01-system-map.md)
3. [Complete original file inventory](02-original-file-inventory.md)
4. [Tiberian Dawn source guide](TIBERIANDAWN/README.md)
5. [Red Alert source guide](REDALERT/README.md)
6. [Map editor source guide](MAP_EDITOR/README.md)
7. [Shared graphics and asset formats](SHARED/README.md)
8. [Cross-reference: original files to Rust port](03-rust-port-cross-reference.md)

## What each file guide contains

- The job of the file, in everyday language.
- The important classes, structs, enums, and global data it defines or uses.
- Its functions/methods, explained by responsibility rather than merely repeating names.
- Inputs, outputs, state changes, and important side effects.
- The other original files it depends on and the files that call into it.
- The equivalent Rust module(s), or an explicit statement that the behavior is not yet ported.
- Evidence and confidence notes where the source behavior is not fully established.

## Source baseline

- Original repository: [electronicarts/CnC_Remastered_Collection](https://github.com/electronicarts/CnC_Remastered_Collection)
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Original tree: 1,331 files.
- This Rust repository's current port status remains authoritative in [PORTING_STATUS.md](../../PORTING_STATUS.md) and [CPP_PORT_INVENTORY.md](../CPP_PORT_INVENTORY.md).

## Important

This is a living reverse-engineering reference, not a claim that every original file has already been exhaustively explained. The inventory records every upstream path. Detailed pages are added only after inspecting the corresponding source, so unknown functions are not guessed. Tiberian Dawn and Red Alert are documented independently where their behavior differs.
