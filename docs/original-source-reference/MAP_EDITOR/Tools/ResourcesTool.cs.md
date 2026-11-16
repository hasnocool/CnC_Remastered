# ResourcesTool — brush painting, resource values, and grouped undo

Pinned upstream: [ResourcesTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ResourcesTool.cs).  
Pinned blob: 5124dc021ffe6a6dd9e698812f8253a18d8f08be. Source length: 365 lines.  
Audit status: **Body reviewed**. All constructor, event, brush, mutation, undo/redo, update, render, and disposal bodies were inspected. Supporting Overlay/OverlayType, Map resource recalculation, and ResourcesToolDialog brush constraints were checked. No Rust mapping, build, UI test, or parity test is claimed.

## Role and state

ResourcesTool derives from ViewTool but renders directly against the live Map; it does not override RenderMap or PreRenderMap. It paints Tiberium/gold or gem Overlay cells using a square brush, removes only resource overlays, displays the map's computed total resource value, and groups each mouse gesture into one undo/redo entry.

Fields:
- readonly Label totalResourcesLbl;
- readonly NumericUpDown brushSizeNud;
- readonly CheckBox gemsCheckBox;
- bool placementMode and bool additivePlacement;
- Dictionary<int, Overlay> undoOverlays and redoOverlays, keyed by linear cell index;
- private disposedValue.

The ResourcesToolDialog designer constrains brush size to minimum 1, maximum 9, increment 2, initial value 1. The resulting supported sizes are 1, 3, 5, 7, and 9. The NavigationWidget.MouseoverSize is initialized from this value.

## Constructor and subscriptions

Constructor signature:

**ResourcesTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, Label totalResourcesLbl, NumericUpDown brushSizeNud, CheckBox gemsCheckBox, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url)**

It delegates to ViewTool, subscribes MapPanel.MouseDown/MouseUp and Control.KeyDown, stores the controls, subscribes brush ValueChanged, sets NavigationWidget.MouseoverSize, subscribes MouseCellChanged, subscribes url.Undone and url.Redone to Url_UndoRedo, calls Update, then updates status.

## Exact method behavior

| Method / exact signature | Behavior |
|---|---|
| Url_UndoRedo(object sender, EventArgs e) | Calls Update so the total label and Gems checkbox visibility reflect the current map after undo/redo. |
| BrushSizeNud_ValueChanged(object sender, EventArgs e) | Copies the NumericUpDown value into NavigationWidget.MouseoverSize as a square Size. |
| ResourceTool_KeyDown(object sender, KeyEventArgs e) | OemOpenBrackets calls DownButton and invalidates the panel; OemCloseBrackets calls UpButton and invalidates. NumericUpDown bounds/increment enforce the supported sizes. |
| MapPanel_MouseDown(object sender, MouseEventArgs e) | On left mouse down, if not already in placement mode, enters additive mode and immediately paints at MouseCell. On right mouse down, enters subtractive mode and immediately removes resources. |
| MapPanel_MouseUp(object sender, MouseEventArgs e) | Exits only when the released button matches the active additive/subtractive mode. If either change dictionary is non-empty, commits the entire gesture. |
| MouseoverWidget_MouseCellChanged(object sender, MouseCellChangedEventArgs e) | While placement mode is active, calls AddResource or RemoveResource at the new cell. For brush sizes above one, invalidates old and new brush rectangles after converting them to client coordinates. |
| AddResource(Point location) | Builds a one-cell rectangle and inflates it by half the brush width/height. For every point, skips map row 0 and the final row, requires Metrics.GetCell success and an empty Overlay cell, selects the first gem or TiberiumOrGold OverlayType according to GemsCheckBox.Checked, snapshots the old cell once, writes Overlay { Type, Icon=0 }, records redo state, and marks dirty. It then inflates the invalidation rectangle by one, invalidates the map, and calls Update. |
| RemoveResource(Point location) | Uses the same brush rectangle but does not skip the first/final map rows. For each valid cell it removes only overlays whose Type.IsResource is true, snapshots the original value once, records null as redo state, and marks dirty. It invalidates the inflated rectangle and calls Update. |
| EnterPlacementMode(bool additive) / ExitPlacementMode() | Idempotently set placementMode and additivePlacement, then update the status message. Unlike TerrainTool/object tools, this tool does not require Shift; the mouse button starts and ends a brush gesture. |
| CommitChange() | Copies the undo/redo dictionaries into closures, clears the live dictionaries, and calls url.Track once. Each closure writes all captured Overlay values to the supplied event map and invalidates a one-cell rectangle inflated by one around every changed cell. |
| Update() | Sets TotalResourcesLbl.Text from map.TotalResources. If any OverlayType is a gem, shows the Gems checkbox; otherwise hides it and clears Checked. |
| UpdateStatus() | Shows drag-to-add, drag-to-remove, or normal left/right drag instructions. |
| PostRenderMap(Graphics graphics) | Calls base and draws a green rectangle around each resource Overlay cell. It creates one Pen per render and does not dispose it. |
| Dispose(bool disposing) | Unsubscribes MapPanel mouse/key handlers, brush ValueChanged, NavigationWidget.MouseCellChanged, and url.Undone/Redone once, then calls base disposal. |

## Resource type and map recalculation contracts

Overlay stores Type, Icon, Tint, and a one-cell OccupyMask delegated to OverlayType. OverlayType flags are:
- TiberiumOrGold = 1;
- Gems = 2;
- Wall = 4;
- Crate = 8;
- Flag = 16.

IsResource is true for either TiberiumOrGold or Gems. IsTiberiumOrGold and IsGem expose the individual bits. IsWall, IsCrate, and IsFlag expose the other categories. IsPlaceable is true when no flag other than Crate is present.

Map.TotalResources sums (Icon + 1) multiplied by GemValue for gems or TiberiumOrGoldValue for Tiberium/gold. The tool does not calculate this formula itself; it refreshes the label from Map.TotalResources.

Map.Overlay_CellChanged reacts to resource overlay changes by marking the Resources layer dirty around the changed cell (a one-cell inflation in each direction). When map updates are not batched, it immediately runs UpdateResourceOverlays. That routine counts adjacent resource overlays across CellMetrics.AdjacentFacings and selects the icon index from fixed tables:
- Tiberium/gold: 0, 1, 3, 4, 6, 7, 8, 10, 11 for zero through eight adjacent resource cells.
- Gems: 0, 0, 0, 1, 1, 1, 2, 2, 2.

The adjacency test uses IsResource, so neighboring gems and Tiberium/gold both contribute to the count. The final icon table depends on the current overlay's IsGem value.

## Review findings and Rust requirements

- **Asymmetric border rule:** AddResource skips row 0 and the final map row, but RemoveResource does not. A strict compatibility test must preserve this asymmetry or document an intentional fix.
- **Brush footprint:** supported sizes are odd values 1..9. Inflation by integer brushSize/2 produces centered odd square footprints. Keep the cursor footprint and mutation footprint identical.
- **No overwrite:** AddResource only writes empty Overlay cells; it does not replace walls, crates, flags, or other overlay types. RemoveResource deletes only IsResource overlays.
- **Type selection:** the tool chooses the first matching OverlayType in registry order. It does not select by a stable ID or sort by name.
- **Grouped undo:** all changed cells from one mouse-down/mouse-up gesture are stored in dictionaries and committed as one url.Track entry. Each cell's original value is captured only once; redo stores its final value.
- **Resource update cost:** AddResource/RemoveResource call Update after every mouse-cell callback, and Update reads Map.TotalResources across the overlay collection. Large brushes or long drags can therefore repeat full resource-total scans on the input path.
- **GDI lifetime:** PostRenderMap creates an undisposed Pen on every render.
- **Undo/redo dirty state:** the closures restore map.Overlay and invalidate the map; they do not set plugin.Dirty in the closure bodies. Verify whether the shared undo host marks the plugin dirty elsewhere before porting this behavior.
- **No async boundary:** brush mutation, adjacency updates, total calculation, invalidation, and undo recording all occur synchronously in WinForms event callbacks. A Rust implementation should keep each input command bounded, batch a stroke, and schedule derived overlay/total recalculation without blocking pointer handling.

Recommended Rust model: typed OverlayKind and resource amount, deterministic adjacency-to-icon resolver, brush footprint iterator, one command transaction per stroke, and derived resource total. Preserve the difference between map overlay storage and techno occupancy.

## Related source

- [Overlay.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Overlay.cs)
- [OverlayType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OverlayType.cs)
- [Map.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Map.cs)
- [ResourcesToolDialog.Designer.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/ResourcesToolDialog.Designer.cs)
- [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs)
