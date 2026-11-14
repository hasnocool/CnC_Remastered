# REDALERT/MIXFILE.CPP

**Review status:** Reviewed for class/function responsibilities.  
**Upstream:** [REDALERT/MIXFILE.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/MIXFILE.CPP)

## Purpose

Implements Red Alert's MIX archive manager. It locates resources inside MIX containers, caches archive data, returns embedded file bytes, and reports offsets/sizes.

## Main class

### `MixFileClass`

A mounted archive resource manager. It is responsible for finding and retrieving a packed file, not interpreting the file's internal format.

## Related files

- [MIXFILE.H](MIXFILE.H.md): public class declaration.
- [Tiberian Dawn MIXFILE.CPP](../TIBERIANDAWN/MIXFILE.CPP.md): variant comparison.
- `WIN32LIB/ICONSET.CPP`: consumes icon-set bytes after archive retrieval.
- `CDATA.CPP`: requests theatre-specific template resources.

## Rust port mapping

See `crates/rts-formats/src/mix.rs`. Verify checksum lookup, archive variants, offsets, caching, and failure behavior against both implementations.
