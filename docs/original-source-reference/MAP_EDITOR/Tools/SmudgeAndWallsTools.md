# Smudge and wall tools — bib ownership, occupancy, and stroke semantics

Pinned upstream: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae. Individual guides: [SmudgeTool](SmudgeTool.cs.md) and [WallsTool](WallsTool.cs.md).

## Storage and interaction comparison

| Concern | SmudgeTool | WallsTool |
|---|---|---|
| Storage | Map.Smudge CellGrid | Map.Overlay CellGrid |
| Model role | Smudge is a cell value, not an occupier | Wall Overlay is a cell value plus Buildings/Technos occupancy |
| Catalog | Smudge types with Bib bit excluded | Overlay types with IsWall true; MainForm does not theatre-filter |
| Placement | Shift + left click | Shift + left drag |
| Removal | Shift + right click | Shift + right drag |
| Pick | Either button; any smudge including Bib | Either button; wall only |
| Undo | One entry per click | One entry per mouse stroke |
| Preview | Empty-cell translucent smudge ghost | Ghost only when both Technos and Buildings occupancy permit |
| Render outline | Non-Bib smudges | All wall overlays |

## Smudge and building bib relationship

SmudgeTypeFlag values are None=0, Bib=1, Bib1=3, Bib2=5, Bib3=9. The Bib bit test excludes all three bib variants. MainForm excludes bib types from the Smudge selector and SmudgeTool refuses to remove bib cells. Map.AddBibs creates bib rows automatically for buildings with HasBib: width 2 selects Bib3, width 3 selects Bib2, width 4 selects Bib1. It records every generated cell in Building.BibCells. Map.RemoveBibs clears the corresponding Bib-flagged cells and empties that set.

SmudgeTool's PickSmudge does not apply the Bib filter, and AddSmudge checks only whether the cell is empty. Verify how an out-of-catalog selected type behaves before treating the catalog filter as a model-level safety rule.

## Wall overlay integration

Walls are stored in Map.Overlay but Map.Overlay_CellChanged also adds/removes wall overlays from Map.Buildings. The Buildings collection event then adds/removes the same wall object in Map.Technos. AddWall requires both occupancy sets to accept the location. This lets walls block incompatible units, terrain, buildings, and other wall placement.

Map.UpdateWallOverlays derives a four-bit connection icon from same-type north/east/south/west neighbors (1/2/4/8). The overlay cell-change path recalculates affected neighboring icons. WallsTool records before/after overlay values for every changed cell and commits one undo entry per drag stroke.

AddWall does not require an empty overlay cell; it can replace non-wall overlay data when the two occupancy checks pass. The previous value is captured for undo. RemoveWall deletes only wall overlays.

## Correctness and porting findings

1. Smudge placement/removal is click-only; pointer movement only invalidates the preview.
2. SmudgeTool's pick path can select Bib types outside the filtered catalog, while its add path has no Bib-flag guard.
3. WallsTool checks both Technos and Buildings occupancy; preserve both indexes and their transactional update.
4. WallsTool can overwrite non-wall overlays, including resources/decorative types, because it checks occupancy sets rather than overlay emptiness.
5. Wall connectivity is derived from same-type orthogonal neighbors; it belongs in the model transaction, not the renderer.
6. SmudgeTool and WallsTool both allocate undisposed Pens per render and synchronously clone the map in ViewTool's pre-render callback.
7. Smudge undo is per click; wall undo is per drag stroke.
8. Wall selector filtering differs from decorative overlay filtering: MainForm filters IsWall only and does not check theatre compatibility.

## Rust implementation boundary

MapDocument currently stores overlay as an optional numeric ID and has no typed Smudge cell, Bib ownership set, wall occupancy indexes, wall connectivity resolver, or stroke commands.

Implement:
- SmudgeCell with type/icon/data and an explicit generated-bib ownership marker.
- Building bib creation/removal as a transactional side effect keyed by Building.BibCells.
- WallOverlay placement/removal commands that update Overlay, Buildings occupancy, Technos occupancy, and derived neighbor icon bits atomically.
- One undo transaction per wall stroke and one command per smudge click unless a new interaction is intentionally chosen.

Tests should cover every Bib flag combination, building widths 2/3/4, bib cell cleanup, non-bib smudge removal, bib pick/add behavior, wall placement against each occupant type, non-wall overlay replacement and undo, same-type versus different-type neighbor bits, edge/corner connectivity, and stroke undo/redo. No Rust code or tests were changed/run.
