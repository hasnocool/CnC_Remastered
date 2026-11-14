# TIBERIANDAWN/DISPLAY.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [DISPLAY.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/DISPLAY.CPP)

## Purpose
Implements the tactical map display controller. It converts between screen pixels and map coordinates, draws the visible map, processes mouse interaction, manages redraw requests, initializes theatre display state, and determines which cell/object the player clicked.

## Main class
`DisplayClass` coordinates the visible tactical viewport and its interaction with the map. It is a presentation/input controller, not the authoritative map data model.

## Function inventory highlights
`AI`, `Calculated_Cell`, `Cell_Object`, `Cell_Shadow`, `Click_Cell_Calc`, `Coord_To_Pixel`, `Cursor_Mark`, constructor, `Draw_It`, `Flag_To_Redraw`, `Get_Occupy_Dimensions`, `Init_Clear`, `Init_IO`, `Init_Theater`, `Map_Cell`, mouse left/right press/hold/release/up handlers, `Next_Object`, `One_Time`, `Passes_Proximity_Check`, `Pixel_To_Coord`, and `Read_INI`.

## Why this matters for terrain
`Init_Theater` is a key connection between the selected theatre and display resources. `Draw_It` and cell drawing consume already interpreted terrain and object data. They should not be responsible for guessing the identity of a template from flattened atlas coordinates.

## Dependencies
Map/cell model, template and theatre registry, icon sets/palettes, object render methods, mouse/input, viewport scrolling, and redraw tracking.

## Rust mapping
Related foundations: `crates/rts-core/src/map.rs`, `scroll.rs`, `coordinate.rs`, `rgb_palette.rs`, `visual_controls.rs`, and `crates/map-editor-model/src/cell_metrics.rs`. Status: partial helpers only; no complete tactical map display/input renderer is established.
