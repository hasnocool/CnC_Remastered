# TIBERIANDAWN/MIXFILE.CPP

**Review status:** Reviewed for public class/function responsibilities.  
**Upstream:** [TIBERIANDAWN/MIXFILE.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/MIXFILE.CPP)  
**Declaration:** [MIXFILE.H](MIXFILE.H.md)

## Purpose

Implements the legacy MIX archive manager. MIX files are containers that hold many named game resources. This class locates a resource inside an archive and makes its bytes available to the rest of the game.

It is an asset transport layer. It does not decode an icon set, understand a theatre, or decide which terrain tile a map cell needs.

## Main class

### `MixFileClass`

Represents a mounted/cached MIX archive and provides static or instance operations for retrieving embedded files. The class also tracks offsets and sizes so callers can access a file without needing to understand the archive's entire layout.

## Functions

| Function | Plain-English responsibility |
|---|---|
| `Cache` (static) | Opens/caches a named MIX archive for later resource lookup. |
| `Cache` (instance) | Loads this archive's data into memory, optionally using a supplied buffer. |
| `Finder` | Finds the mounted archive object corresponding to a requested archive name. |
| `Free` | Releases the archive's loaded data or uncaches the named archive. |
| `MixFileClass` constructor | Creates an archive manager entry and initializes its lookup state. |
| `Offset` | Finds an embedded file and returns its offset and size within the archive data. |
| `Retrieve` | Returns access to a named embedded file's bytes. |
| Destructor | Releases resources owned by the archive object. |

## How it participates in terrain loading

The theatre/template code constructs a theatre-specific graphics filename and asks the MIX layer to retrieve it. MIX returns bytes. The IFF/icon-set decoder interprets those bytes. The template registry then interprets the icon-set's template map. These are three distinct responsibilities.

## Rust port mapping

The Rust workspace has `crates/rts-formats/src/mix.rs`. The current implementation is partial: it handles the legacy archive header/index shape, but the full original filename checksum/name lookup and all cache/retrieval semantics need to be verified against the original behavior.
