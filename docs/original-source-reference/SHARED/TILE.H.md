# TIBERIANDAWN/WIN32LIB/TILE.H

**Review status:** Reviewed for active control structure and loader declarations.  
**Upstream:** [TIBERIANDAWN/WIN32LIB/TILE.H](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/WIN32LIB/TILE.H)

## Purpose

Declares the icon-set loader functions and the in-memory control structure used by the original graphics library.

## Main declarations

- `Load_Icon_Set`: loads and initializes an icon set from a file or supplied buffer.
- `Free_Icon_Set`: frees icon-set allocations.
- `IControl_Type` (active TD layout): stores icon width, height, count, allocation count, total size, and offsets/pointers for icon pixels, palettes, remaps, transparency flags, and the template map.

The active TD structure has four 16-bit values followed by six 32-bit values (32 bytes under the original 32-bit layout/packing assumptions). This is not the same active structure as Red Alert's header.

## Porting implications

Represent the decoded meaning in Rust types rather than reading a native C struct directly. Keep raw layout parsing variant-specific and use checked offsets/lengths. See [ICONSET.CPP](ICONSET.CPP.md) and [Red Alert comparison](TILE-H-REDALERT.md).
