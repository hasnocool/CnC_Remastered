# REDALERT/CELL.CPP

**Review status:** Partially reviewed — function index inspected.
**Upstream:** [CELL.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/CELL.CPP)

## Purpose
Implements Red Alert's individual map cell. It handles terrain appearance and attributes, object lookup, occupancy/overlap, radar color, Tiberium growth, and local gameplay behavior. It is a central part of the map simulation and terrain renderer.

## Main class
`CellClass` represents one cell in the Red Alert world. It connects the static ground/template data with dynamic objects and cell-level state.

## Function inventory (source-index responsibilities)
- **Geometry/lookups:** `Adjacent_Cell`, `Cell_Coord`, `Cell_Find_Object`, `Cell_Building`, `Cell_Infantry`, `Cell_Unit`, `Cell_Vessel`, `Cell_Techno`, `Cell_Terrain`, `Cell_Object`.
- **Appearance:** `Cell_Color`, `Clear_Icon`, `Concrete_Calc`, `Draw_It`, `Flag_Place`, `Flag_Remove`.
- **Occupancy and availability:** `Closest_Free_Spot`, `Occupy_Down/Up`, `Occupy_Unit`, `Overlap_Down/Up`, `Overlap_Unit`, `Reserve_Cell`, `Is_Generally_Clear`.
- **Tiberium simulation:** `Can_Tiberium_Germinate`, `Can_Tiberium_Grow`, `Can_Tiberium_Spread`, `Grow_Tiberium`, plus cell-level Tiberium adjustment and reduction routines.
- **Other behavior:** `Adjust_Threat`, `Goodie_Check`, `Incoming`, `Get_Trigger`, `Read`, `Recalc_Attributes`, `Redraw_Objects`, `Reduce_Wall`, `Shimmer`, `Validate`, `Wall_Update`.

## Key RA-specific distinction
Red Alert adds explicit Tiberium growth/germination/spread behavior and vessel lookup support compared with the TD cell function index. Do not assume identical cell fields, flags, or update order.

## Dependencies
`MAP.CPP` owns the grid; `CDATA.CPP` and icon-set loading provide template definitions; terrain/overlay objects contribute cell appearance; objects and units update occupancy; triggers and display consume cell state.

## Rust mapping
Related modules: `crates/rts-core/src/map.rs`, `coordinate.rs`, `region_size.rs`, `crates/red-alert/src/overlay.rs`. Status: partial foundations only; Tiberium growth and complete cell occupancy/render behavior are not established.

## Remaining audit
Inspect `CELL.H` for every member and flag; trace exact Tiberium tick conditions, adjacency order, save/load layout, and renderer calls.
