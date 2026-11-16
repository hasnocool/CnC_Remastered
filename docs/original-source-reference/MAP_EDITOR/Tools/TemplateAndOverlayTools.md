# Template and overlay tools — map layers, icons, bounds, and undo

Pinned upstream: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae. Individual guides: [TemplateTool](TemplateTool.cs.md) and [OverlaysTool](OverlaysTool.cs.md).

## Layer and input comparison

| Concern | TemplateTool | OverlaysTool |
|---|---|---|
| Storage | Map.Templates CellGrid<Template> | Map.Overlay CellGrid<Overlay> |
| Object granularity | One template icon per cell, with multi-icon type footprints | One Overlay per cell |
| Catalog | Thumbnail available, active theatre, Clear flag excluded | IsPlaceable and theatre-compatible |
| Selection | Whole type or individual icon through preview NavigationWidget | TypeComboBox selection; normal click picks any non-wall overlay |
| Placement modifier | Shift | Shift |
| Bounds modifier | Ctrl enables eight-edge/corner map bounds resizing | None |
| Placement input | Left click places once; right drag clears cells | Left/right click only; MouseCellChanged does not paint |
| Undo grouping | Buffered per-cell dictionary committed on mouse up; bounds change one command | One command per successful click |
| Preview | Cloned map with selected icon or full IconMask footprint | Cloned map with translucent one-cell overlay ghost |
| Render outline | Full template footprint or selected one-cell icon; red map bounds | Green outline for IsPlaceable overlays |

## Template data and behavior

TemplateType carries ushort ID, IconWidth/Height, a bool IconMask, theatre list, flags, and thumbnail. TemplateTool groups eligible types by the leading lowercase name prefix. The preview panel uses a separate NavigationWidget to select only mask-true icon positions. Icon selection maps to a row-major icon index.

Whole placement snapshots every valid cell in the rectangular type footprint, but writes only IconMask-true positions. Whole removal snapshots and clears every valid cell in the rectangle, including mask holes. Placement can be partial at map edges because each cell is checked independently.

Ctrl bounds mode keeps a local dragBounds rectangle, detects one of eight edges/corners from mouse cell and subpixel position, clips the drag endpoint to map metrics, and writes Map.Bounds on mouse up. Undo/redo updates local dragBounds only while bounds mode remains active.

## Overlay data and behavior

OverlayType flags separate resource, wall, crate, and flag categories. IsPlaceable allows no flags or crate-only types; resource, wall, and flag overlays are excluded from MainForm's Overlay selector. Overlay storage is separate from Technos and uses a one-cell grid.

OverlaysTool is click-only: mouse-cell movement invalidates the preview but does not paint. Add skips the first and final map rows and only writes into empty cells. Remove clears only IsPlaceable overlays. Pick accepts any non-wall overlay, so it can encounter types outside the selector's placeable catalog.

## Correctness findings

1. TemplateTool's map-bounds commit updates Map.Bounds and records undo/redo but does not directly set plugin.Dirty.
2. Template whole removal clears mask-hole cells that whole placement deliberately leaves untouched.
3. TemplateTool's selected preview NavigationWidget is not disposed when the tool is disposed.
4. TemplateTool's Clear fallback assumes a type named clear1 exists.
5. TemplateTool exits both modes when either Shift or Ctrl is released.
6. TemplateTool's bounds-mode mouse-cell callback can fall through to PickTemplate when no drag edge is selected because the callback does not test boundsMode in that branch.
7. TemplateTool's GDI pens/brushes/StringFormat/Matrix and OverlaysTool's render Pen are not disposed deterministically.
8. OverlaysTool can show a placement ghost on the first/final map row even though AddOverlay rejects those rows.
9. OverlaysTool's pick predicate is broader than its placeable type catalog.
10. Both tools clone the full map synchronously in ViewTool's pre-render path.

## Rust implementation boundary

MapDocument currently has template and overlay numeric IDs in CellLayers, but no TemplateType/IconMask catalog, per-cell icon index, theatre resolver, OverlayType flags, placement commands, Map.Bounds edit command, or undo/redo stack.

Implement:
- typed template catalog and theatre-specific icon masks;
- SetTemplateIcon and SetTemplateFootprint commands with explicit mask-hole overwrite/clear semantics;
- MapBounds resize command with edge/corner constraints and dirty/undo state;
- PlaceOverlay/RemoveOverlay commands constrained by category and row rules;
- preview-only ghost data, separate from committed cell layers.

Tests should cover mask holes, partial footprints at map edges, selected-icon indexing, clear1 fallback, empty catalogs, all eight bounds edges/corners, modifier release behavior, bounds undo/dirty, overlay category filtering, crate-only placeability, non-wall picking, top/bottom ghost versus commit rules, and per-click undo. No Rust code or parity tests were changed/run in this tranche.
