# TIBERIANDAWN/OVERLAY.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [OVERLAY.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/OVERLAY.CPP)

## Purpose
Implements overlay objects associated with map cells. Overlays include cell-level graphics/objects such as walls, resource deposits, and other map decorations with gameplay meaning. They are not the base template terrain.

## Main class
`OverlayClass` represents an overlay instance and its registration/serialization/marking lifecycle.

## Functions
`Read_INI` and `Write_INI` load/save overlay data; `new/delete` allocate and return objects to the pool; `Init` resets the overlay system; the constructor initializes an instance; `Mark` updates its map footprint; `Validate` checks object validity.

## Dependencies
Cell and map overlay slots, overlay type definitions, terrain drawing, wall/resource behavior, scenario INI, and object pooling.

## Rust mapping
Related foundation: `crates/red-alert/src/overlay.rs` is Red Alert-specific; no complete Tiberian Dawn overlay equivalent is established. Keep TD and RA overlay IDs and semantics variant-specific.
