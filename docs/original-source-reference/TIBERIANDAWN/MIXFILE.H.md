# TIBERIANDAWN/MIXFILE.H

**Review status:** Partially reviewed.  
**Upstream:** [TIBERIANDAWN/MIXFILE.H](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/MIXFILE.H)

## Purpose

Declares the templated `MixFileClass` archive manager and its public operations. The implementation is in [MIXFILE.CPP](MIXFILE.CPP.md).

## Main class and nested data

- `MixFileClass<T>`: manages a mounted MIX archive and exposes caching, lookup, retrieval, offset, and freeing operations.
- `SubBlock`: describes an embedded resource using its checksum/CRC, offset within the archive data section, and byte size. It can be ordered by checksum for lookup.

## Important methods

- `Cache`: make archive data available in memory.
- `Free` / `Free_All`: release one or all cached archives.
- `Offset`: locate a resource and report its byte range.
- `Retrieve`: return the embedded resource.
- Constructors/destructor: establish and release the archive object's state.

## Rust port mapping

See `crates/rts-formats/src/mix.rs`. The current parser is a foundation; full compatibility requires reproducing the original checksum lookup and archive caching behavior without relying on native pointer/layout assumptions.
