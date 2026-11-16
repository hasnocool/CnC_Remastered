# WallsTool — connected wall overlays and occupancy integration

Pinned upstream: [WallsTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/WallsTool.cs).  
Pinned blob: 28c75f0f35edf74c49ea6893b65a2cd311cd7a49. Source length: 385 lines.  
Audit status: **Body reviewed**. All constructor, selector, mouse/key, add/remove, grouped undo, preview/render, and disposal bodies were inspected. Supporting Overlay/OverlayType, Map overlay events, wall adjacency, OccupierSet, and MainForm catalog filtering were checked. No Rust mapping, build, UI test, or parity test is claimed.

## Role and state

WallsTool derives from ViewTool and edits wall Overlay objects in Map.Overlay. A wall is stored in the overlay cell grid but is also registered into Map.Buildings and Map.Technos through Map.Overlay_CellChanged and the Buildings collection events. This dual registration makes walls block incompatible techno/building placement and allows the map model to update wall connectivity.

Fields:
- readonly TypeComboBox wallTypeComboBox and MapPanel wallTypeMapPanel;
- undoOverlays and redoOverlays dictionaries keyed by linear cell index;
- Map previewMap, returned by protected override RenderMap;
- bool placementMode;
- OverlayType selectedWallType and private SelectedWallType property;
- private disposedValue.

MainForm supplies every OverlayType where IsWall is true, ordered by Name. Unlike the decorative overlay tool, this filter does not check OverlayType.Theaters. The constructor calls Types.First() without an empty-list guard.

## Constructor and selector

Constructor signature:

**WallsTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, TypeComboBox wallTypeComboBox, MapPanel wallTypeMapPanel, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url)**

It delegates to ViewTool, initializes previewMap to map, subscribes MapPanel mouse/key handlers and selector changes, configures the preview panel white with MaxZoom 1, subscribes NavigationWidget.MouseCellChanged, selects the first wall type, and updates status.

SelectedWallType ignores unchanged values. In placement mode it invalidates the current cell for the old type, stores the new type, synchronizes ComboBox.SelectedValue, and refreshes the thumbnail. It does not explicitly invalidate the current cell again after changing the type.

## Exact method behavior

| Method / exact signature | Behavior |
|---|---|
| WallTypeComboBox_SelectedIndexChanged(object sender, EventArgs e) | Copies SelectedValue as OverlayType into SelectedWallType. |
| WallTool_KeyDown(object sender, KeyEventArgs e) / WallTool_KeyUp(object sender, KeyEventArgs e) | Shift down enters placement mode; Shift up exits. |
| MapPanel_MouseDown(object sender, MouseEventArgs e) | In placement mode, left calls AddWall and right calls RemoveWall. Otherwise either button calls PickWall. |
| MapPanel_MouseUp(object sender, MouseEventArgs e) | Commits buffered overlay changes when either undoOverlays or redoOverlays is non-empty. |
| MapPanel_MouseMove(object sender, MouseEventArgs e) | Enters placement mode when modifiers equal Shift; exits when placement mode is active and modifiers become None. |
| MouseoverWidget_MouseCellChanged(object sender, MouseCellChangedEventArgs e) | While in placement mode, left mouse movement calls AddWall and right mouse movement calls RemoveWall for each new cell. It invalidates old/new one-cell rectangles inflated by one cell to refresh connected wall neighbors. |
| AddWall(Point location) | Resolves a valid linear cell, requires a selected wall type, constructs Overlay { Type, Icon=0 }, and requires both Technos.CanAdd(cell, overlay) and Buildings.CanAdd(cell, overlay). It snapshots the original overlay once, writes the wall overlay, records redo state, invalidates a 3×3 neighborhood, and marks dirty. It does not require Map.Overlay[cell] to be empty; a non-wall overlay can be replaced. |
| RemoveWall(Point location) | Resolves a valid cell and removes only an overlay whose Type.IsWall is true. It snapshots the original overlay once, writes null, records redo null, invalidates a 3×3 neighborhood, and marks dirty. |
| CommitChange() | Copies the undo/redo dictionaries into closures, restores captured overlay values on undo/redo, invalidates each changed cell's 3×3 neighborhood, clears the live dictionaries, and registers one url.Track entry for the complete mouse stroke. |
| EnterPlacementMode() / ExitPlacementMode() | Idempotently toggle placementMode, change NavigationWidget.MouseoverSize between empty and 1×1, invalidate a 3×3 neighborhood around the current cell, and update status. |
| PickWall(Point location) | Resolves a valid cell and selects the overlay type only when the current overlay is a wall. |
| RefreshMapPanel() | Assigns SelectedWallType?.Thumbnail to the preview panel MapImage. |
| UpdateStatus() | Displays drag-to-add/remove instructions in placement mode, otherwise Shift-to-place and either-button pick instructions. |
| PreRenderMap() | Calls base, clones the live map, and in placement mode creates a translucent wall ghost for a valid cell when both previewMap.Technos.CanAdd and previewMap.Buildings.CanAdd accept it. It does not require the overlay cell itself to be empty, matching AddWall's replacement behavior. |
| PostRenderMap(Graphics graphics) | Calls base and draws a green one-tile outline around every wall overlay in previewMap. It creates a Pen on each render and does not dispose it. |
| Dispose(bool disposing) | Unsubscribes map mouse/key, selector, and navigation cell-change handlers once, then calls base disposal. |

## Wall occupancy and adjacency contract

OverlayType is a one-cell ICellOccupier. Its IsWall property tests the Wall flag. Map.Overlay_CellChanged handles old and new wall values before its general invalidation/update batching:
- if the old overlay is a wall, it removes it from Map.Buildings;
- if the new overlay is a wall, it adds it to Map.Buildings at the changed location.

Map.Buildings_OccupierAdded treats a Building specially; for any other occupier (including a wall Overlay), it adds the object to Map.Technos with its OccupyMask. Buildings_OccupierRemoved removes the occupier from Technos. Consequently a wall overlay participates in both Buildings and Technos occupancy sets even though its serialized/storage representation is Map.Overlay.

Map.UpdateWallOverlays examines the four orthogonal neighbors. It sets icon bits only when a neighbor has the same OverlayType:
- North = 1;
- East = 2;
- South = 4;
- West = 8.

The map overlay cell-change path invalidates a one-cell neighborhood and recalculates wall icons for affected cells. WallsTool also invalidates a 3×3 neighborhood around pointer changes and mutations.

## Review findings and Rust requirements

- **Overwrite semantics:** AddWall checks Technos and Buildings occupancy but does not check Map.Overlay[cell] for null. It can replace a resource, crate, flag, or decorative overlay that is not registered as a techno/building occupier. The original overlay is captured for undo. Preserve this only if intentional.
- **Two occupancy checks:** both Technos and Buildings must accept the one-cell wall overlay. A Rust port that checks only the overlay grid will permit walls through occupied units/buildings; one that checks only Technos may miss collection-specific blockers.
- **Connected icon state:** wall Icon is derived from same-type orthogonal neighbors by Map.UpdateWallOverlays. Do not make the UI the source of truth for these bits.
- **Grouped undo:** AddWall/RemoveWall mutate per cell during a drag but store original/final overlay values in dictionaries and create one undo entry on mouse up.
- **Dirty timing:** plugin.Dirty is set for each changed cell before the stroke is committed to UndoRedoList. Undo/redo closures restore overlay cells and invalidate but do not directly set plugin.Dirty in their bodies; verify the shared undo host's dirty policy.
- **Catalog theatre policy:** MainForm filters only IsWall, not Theaters. This differs from OverlaysTool's theatre-aware selector. Verify whether wall type catalogs are intentionally global or whether incompatible types can be shown.
- **Selection invalidation:** SelectedWallType invalidates the old preview cell before changing the type but does not explicitly invalidate the new preview cell. Verify redraw after changing wall type while placement mode is held without moving the mouse.
- **GDI lifetime:** PostRenderMap allocates an undisposed Pen on each render.
- **Synchronous preview:** PreRenderMap clones the full map synchronously inside ViewTool's render callback.

Recommended Rust model: WallOverlay as a typed one-cell overlay with transactional occupancy registration in both relevant indexes, derived connectivity icon recomputation, and a stroke command that records every cell's before/after value. Make overlay replacement policy explicit and test all competing occupant classes.

## Related source

- [Overlay.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Overlay.cs)
- [OverlayType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OverlayType.cs)
- [Map.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Map.cs)
- [OccupierSet.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OccupierSet.cs)
- [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs)
- [MainForm.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/MainForm.cs)
