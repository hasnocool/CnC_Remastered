# TIBERIANDAWN/CELL.CPP

**Review status:** Partially reviewed — source function index inspected; detailed state layout and call graph pending.
**Upstream:** [CELL.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/CELL.CPP)

## Purpose

Implements the per-cell part of the world. A cell is more than a coordinate: it keeps the information needed to draw terrain, decide whether something can be built or moved there, track occupying/overlapping objects, and respond to cell-level gameplay events.

This is one of the most important bridges between terrain graphics and game simulation. The cell determines the terrain state and its derived attributes, while the template/type system supplies the graphics and composition definitions.

## Main class

### `CellClass`
Represents one map cell and its cell-level properties/operations. It provides queries for what is present in a cell, updates occupancy, computes terrain/wall/tiberium appearance, and performs cell-local behavior.

## Function inventory from the source index

| Function | Plain-English job |
|---|---|
| `Adjacent_Cell` | Finds a neighboring cell in a requested facing/direction. |
| `Adjust_Threat` | Changes the threat value recorded for this cell. |
| `Cell_Building`, `Cell_Infantry`, `Cell_Unit`, `Cell_Techno`, `Cell_Terrain` | Returns the requested category of object associated with the cell. |
| `Cell_Find_Object`, `Cell_Object`, `Cell_Occupier` | Finds an object of a requested type, the clickable object, or the current occupier. |
| `Cell_Color` | Chooses the cell's radar/minimap color. |
| `Cell_Coord`, `Spot_Index` | Converts between cell identity and finer coordinate/sub-cell position. |
| `Clear_Icon` | Calculates the graphic index for clear/open terrain. |
| `Closest_Free_Spot` | Finds an available sub-position near a requested coordinate. |
| `Concrete_Calc` | Calculates which concrete graphic should be displayed. |
| `Draw_It` | Draws the cell's terrain imagery. |
| `Flag_Place` / `Flag_Remove` | Adds or removes a house flag at this cell. |
| `Get_Trigger` | Retrieves the trigger associated with the cell. |
| `Goodie_Check` | Handles discovery/activation of a crate in the cell. |
| `Incoming` | Makes objects in the cell react to an incoming threat. |
| `Is_Generally_Clear` | Checks whether the cell is generally suitable for construction/use. |
| `Occupy_Down` / `Occupy_Up` | Adds or removes an occupancy claim. |
| `Occupy_Unit` | Marks the cell as occupied by a unit. |
| `Overlap_Down` / `Overlap_Up` | Adds or removes a visual/footprint overlap claim. |
| `Overlap_Unit` | Marks overlap caused by a unit. |
| `Read` | Reads a cell's saved-game representation. |
| `Recalc_Attributes` | Recomputes gameplay attributes derived from the cell's ground/terrain. |
| `Redraw_Objects` | Redraws objects whose footprint overlaps this cell. |
| `Reduce_Tiberium` | Removes a specified amount of Tiberium from the cell. |
| `Reduce_Wall` | Applies damage to a wall and updates it when appropriate. |
| `Reserve_Cell` | Reserves the cell for a specific unit identifier. |
| `Shimmer` | Starts/updates shimmer effects for objects in the cell. |
| `Tiberium_Adjust` | Selects/updates Tiberium imagery to create a connected/smoothed appearance. |
| `Validate` | Checks that the cell identity/state is valid. |
| `Wall_Update` | Updates wall imagery based on neighboring wall state. |

## Important variables/concepts to document from the header

The full header audit must enumerate the exact members and bit flags. At minimum, the implementation clearly depends on: terrain/template identity, cell coordinates, occupancy and overlap state, object references, trigger association, threat/radar state, and terrain-derived attributes. These are conceptual categories, not asserted member names.

## Dependencies

- `MAP.CPP`: owns the world grid and coordinates cell access.
- `CDATA.CPP` and icon-set code: provide terrain/template type and image metadata.
- `OBJECT.CPP` and derived object classes: occupy, overlap, and query cells.
- `DISPLAY.CPP`: consumes cell drawing and redraw operations.
- `TRIGGER.CPP`: cell-associated triggers.
- `TERRAIN.CPP` and `OVERLAY.CPP`: terrain/overlay objects and cell appearance.

## Rust port mapping

- Current related code: `crates/rts-core/src/map.rs`, `crates/rts-core/src/coordinate.rs`, `crates/rts-core/src/region_size.rs`, `crates/map-editor-model/src/cell_metrics.rs`.
- Status: foundational map and coordinate modules exist, but a behaviorally complete TD cell implementation (terrain attributes, occupier lists, sub-cell reservations, wall/tiberium transitions, trigger hooks, and rendering) is not established.
- Required split: immutable terrain definition, mutable cell simulation state, occupancy/reservation indexes, and rendering extraction should be explicit Rust structures/systems.

## Evidence limits

Function responsibilities come from the source index. The header's exact fields, bit packing, save format, and call-site ordering remain to be audited before full completion.
