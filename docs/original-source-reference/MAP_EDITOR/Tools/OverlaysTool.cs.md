# OverlaysTool — decorative overlay selection and cell placement

Pinned upstream: [OverlaysTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/OverlaysTool.cs).  
Pinned blob: 30bba06808509875a0712d7dec7ef7e96bc857d8. Source length: 342 lines.  
Audit status: **Body reviewed**. All constructor, selector, input, placement/removal, picking, preview/render, and disposal bodies were inspected. Overlay/OverlayType, CellGrid, NavigationWidget, and MainForm catalog filtering were checked. No Rust mapping, build, UI test, or parity test is claimed.

## Role and state

OverlaysTool derives from ViewTool and edits the Map.Overlay CellGrid. It provides Shift placement mode, one-cell left-click placement, one-cell right-click removal, and normal-mode picking with either mouse button. It does not paint continuously while dragging; mouse-cell changes only invalidate the preview footprint.

Fields:
- readonly TypeComboBox overlayTypeComboBox and MapPanel overlayTypeMapPanel;
- Map previewMap, returned by protected override RenderMap;
- bool placementMode;
- OverlayType selectedOverlayType and private SelectedOverlayType property;
- private disposedValue.

MainForm supplies OverlayTypes where IsPlaceable is true and Theaters is null or contains the active theatre, ordered by Name. The TypeComboBox.Types setter computes Max thumbnail dimensions and the constructor calls Types.First(), so an empty filtered catalog fails before the tool is usable.

## Constructor and selector

Constructor signature:

**OverlaysTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, TypeComboBox overlayTypeComboBox, MapPanel overlayTypeMapPanel, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url)**

It delegates to ViewTool, sets previewMap to map, subscribes MapPanel.MouseDown/MouseMove and Control.KeyDown/KeyUp, subscribes selector SelectedIndexChanged, configures the preview panel white with MaxZoom 1, subscribes NavigationWidget.MouseCellChanged, selects the first type, and updates status.

SelectedOverlayType ignores unchanged references. While placement mode is active it invalidates the current mouse cell for the old type, stores the new type, assigns ComboBox.SelectedValue, and refreshes the preview thumbnail. It does not explicitly invalidate the current cell again after changing the type.

## Exact method behavior

| Method / exact signature | Behavior |
|---|---|
| OverlayTypeComboBox_SelectedIndexChanged(object sender, EventArgs e) | Copies SelectedValue as OverlayType into SelectedOverlayType. |
| OverlaysTool_KeyDown(object sender, KeyEventArgs e) / OverlaysTool_KeyUp(object sender, KeyEventArgs e) | Shift down enters placement mode; Shift up exits. |
| MapPanel_MouseDown(object sender, MouseEventArgs e) | In placement mode, left calls AddOverlay and right calls RemoveOverlay. Outside placement mode, either left or right calls PickOverlay. |
| MapPanel_MouseMove(object sender, MouseEventArgs e) | Enters placement mode when modifiers equal Shift; exits when placement mode is active and modifiers become None. |
| MouseoverWidget_MouseCellChanged(object sender, MouseCellChangedEventArgs e) | In placement mode, invalidates the old and new one-cell preview locations when a type is selected. It does not call AddOverlay or RemoveOverlay; placement/removal is click-only. |
| AddOverlay(Point location) | Rejects row zero and the final map row. If the overlay cell is empty and a selected type exists, creates Overlay { Type=SelectedOverlayType, Icon=0 }, writes Map.Overlay, invalidates, registers one undo/redo closure for that cell, and marks plugin.Dirty. |
| RemoveOverlay(Point location) | Removes only an existing Overlay whose Type.IsPlaceable is true. It invalidates, records undo/redo for the captured overlay, and marks dirty. |
| EnterPlacementMode() / ExitPlacementMode() | Idempotently toggle placementMode, set NavigationWidget.MouseoverSize to empty or 1×1, invalidate the current mouse cell when a type exists, and update status. |
| PickOverlay(Point location) | Resolves the cell with Metrics.GetCell, reads Map.Overlay, and selects the type whenever the overlay is non-null and not a wall. It does not require IsPlaceable. |
| RefreshMapPanel() | Assigns SelectedOverlayType?.Thumbnail to the preview panel's MapImage. |
| UpdateStatus() | Shows left/right place/remove instructions in placement mode, otherwise Shift-to-place and either-button pick instructions. |
| PreRenderMap() | Calls base, clones the live map into previewMap, and in placement mode adds a 50%-alpha white ghost only when the mouse cell is valid, the selected type is non-null, and the overlay cell is empty. |
| PostRenderMap(Graphics graphics) | Calls base and draws a 4-pixel green outline around each previewMap overlay whose type is IsPlaceable. It creates a Pen on every render and does not dispose it. |
| Dispose(bool disposing) | Unsubscribes selector, map mouse/key, and navigation cell-change handlers once, then calls base disposal. |

## Overlay type and map behavior

Overlay stores Type, Icon, Tint, and a one-cell OccupyMask delegated to OverlayType. OverlayTypeFlag values are None=0, TiberiumOrGold=1, Gems=2, Wall=4, Crate=8, Flag=16.

OverlayType.IsResource is true for TiberiumOrGold or Gems. IsWall, IsCrate, and IsFlag expose their flags. IsPlaceable is computed as (Flag & ~Crate) == None: types with no flags and crate-only types pass; walls, resources, and flag overlays do not.

Map.Overlay is a CellGrid<Overlay>, not the Technos OccupierSet. CellGrid's coordinate and linear indexers directly index the backing array and do not check bounds. NavigationWidget updates MouseCell only when Metrics.Contains the newly calculated cell, so the tool's normal pointer path relies on that valid-cell invariant when it indexes Map.Overlay directly.

Map.Overlay_CellChanged has additional behavior for resources and walls: resource changes invalidate a one-cell neighborhood and recalculate adjacency icons; wall changes update the Buildings collection. Other decorative overlays do not enter those derived update paths.

## Review findings and Rust requirements

- **Preview/commit boundary mismatch:** AddOverlay rejects the first and final map rows, but PreRenderMap only checks map bounds and emptiness. The placement ghost can therefore appear on a row where clicking will not commit an overlay.
- **Pick/catalog mismatch:** MainForm's list contains only IsPlaceable types, but PickOverlay accepts any non-wall overlay. A resource or flag overlay can be picked even though its type is outside the supplied selector catalog; the ComboBox/SelectedValue interaction should be tested.
- **Removal category:** RemoveOverlay checks IsPlaceable, which includes crate-only types but excludes resource, wall, and flag overlays. Those categories require their dedicated tools or other workflows.
- **No drag painting:** MouseCellChanged only invalidates; Add/Remove run only from MapPanel.MouseDown. Preserve click-only semantics unless intentionally changing the tool.
- **Per-click undo:** each successful AddOverlay/RemoveOverlay registers its own url.Track entry. There is no gesture-level dictionary or stroke batching.
- **Catalog assumption:** empty theatre-filtered placeable overlay types cause TypeComboBox.Types Max() and the tool's First() to throw.
- **Selection invalidation:** SelectedOverlayType invalidates the old cell before changing the type but does not explicitly invalidate the new preview cell in the setter. Verify redraw when the type changes while placement mode is active without mouse movement.
- **GDI lifetime:** PostRenderMap allocates an undisposed Pen on every render.
- **Synchronous preview:** PreRenderMap clones the full map synchronously on ViewTool's render path.
- **Bounds invariant:** AddOverlay/RemoveOverlay use the CellGrid point indexer directly. NavigationWidget currently filters mouse-cell updates through Metrics.Contains; keep that validation if input handling is refactored.

Recommended Rust command surface: PlaceOverlay, RemoveOverlay, PickOverlay, and a typed overlay catalog that distinguishes placeable decorative/crate types from resource, wall, and flag categories. Keep resource adjacency and wall connectivity updates in the map model, not in the UI adapter.

## Related source

- [Overlay.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Overlay.cs)
- [OverlayType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OverlayType.cs)
- [CellGrid.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/CellGrid.cs)
- [NavigationWidget.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Widgets/NavigationWidget.cs)
- [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs)
- [MainForm.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/MainForm.cs)


## Rust implementation update — partial mapping

The editor's Overlay layer now resolves the selected numeric brush ID through the variant/theatre-specific `OverlayCatalog` and calls `place_overlay_with_history`. The model rejects non-placeable resource, wall and flag categories, enforces the source tool's first/final-row add restriction, rejects occupied cells, and records one undoable cell command. A matching history-backed removal API is available. The desktop UI does not yet implement Shift placement mode, right-click removal, normal-mode picking, ghost preview, or selector presentation; resource adjacency and wall registration remain separate unfinished map-model behavior.
