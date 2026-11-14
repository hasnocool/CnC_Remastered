# REDALERT/MIXFILE.H

**Review status:** Partially reviewed.  
**Upstream:** [REDALERT/MIXFILE.H](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/MIXFILE.H)

## Purpose

Declares Red Alert's templated `MixFileClass` archive manager and its resource lookup/caching interface. Its implementation is [MIXFILE.CPP](MIXFILE.CPP.md).

## Main elements

- `MixFileClass<T>`: tracks an archive and exposes cache, free, offset, and retrieve operations.
- `SubBlock`: records a resource checksum, data-relative offset, and byte length.

## Porting note

Keep the MIX archive reader separate from the IFF/icon-set decoder. Archive lookup returns bytes; it does not identify template frames or compose terrain.
