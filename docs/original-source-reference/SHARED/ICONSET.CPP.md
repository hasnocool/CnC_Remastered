# WIN32LIB/ICONSET.CPP

**Review status:** Reviewed for loader structure and format responsibilities.  
**Upstream TD:** [TIBERIANDAWN/WIN32LIB/ICONSET.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/WIN32LIB/ICONSET.CPP)  
**Upstream RA:** [REDALERT/WIN32LIB/ICONSET.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/WIN32LIB/ICONSET.CPP)

## Purpose

Loads an icon set from an IFF container and builds the in-memory representation used by the game to draw its frames and interpret template composition metadata.

The important point is that an icon set contains more than a flat sequence of pictures. Image pixels, transparency flags, and the template map are separate pieces of information with different jobs.

## Main functions

The original file's function index identifies:

- `Load_Icon_Set`: reads the icon-set container, allocates/initializes its control structure, and loads the relevant chunks.
- `Free_Icon_Set`: releases allocations made while loading an icon set.

## Main data concepts

- **SINF:** image-set information such as icon width/height in 8-pixel units and bitplane count.
- **SSET:** the packed image-frame pixel data.
- **TRNS:** transparency flags associated with frames.
- **MAP:** template composition/index metadata. It is not interchangeable with the SSET pixel bytes.

The loader calculates bytes per icon from the dimensions and bitplanes. The logical icon count may be constrained/overridden by the MAP chunk's size. The in-memory control structure stores offsets/pointers to the image, palette/remap, transparency, and map data.

## Why this matters

A template's MAP data determines how its frames fit into a multi-cell template. The game code reads this map and iterates it in order; an entry of `0xFF` means that position is empty. If the MAP chunk is ignored, treated as pixels, or detached from the template type, the output becomes a misleading pile of unrelated images.

## Variant compatibility warning

The TD and RA `TILE.H` control structures are not identical. The loader source files are closely related, but the control layout and metadata expectations must be represented explicitly per variant. Do not hard-code one native C struct as a universal serialized header.

## Rust port mapping

Current parser: `crates/rts-formats/src/iconset.rs`. It needs a variant-aware representation and tests for SINF/SSET/TRNS/MAP boundaries, counts, offsets, and TD/RA control-layout semantics. The semantic Rust model should own decoded vectors/slices; it should not expose raw C pointers as the primary data model.
