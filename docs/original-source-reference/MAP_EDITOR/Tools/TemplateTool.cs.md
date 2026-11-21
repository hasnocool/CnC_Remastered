# TemplateTool — theatre template selection, multi-icon placement, and map bounds

Pinned upstream: [TemplateTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/TemplateTool.cs).  
Pinned blob: 8d529ff073c2d8c75458e90c97b65dd17ddd1bbe. Source length: 1,009 lines.  
Audit status: **Body reviewed**. All fields, properties, constructor setup, list/preview events, placement/removal, bounds editing, undo/redo, rendering, and disposal bodies were inspected. Supporting Template, TemplateType, CellGrid, Map.Bounds, NavigationWidget, and Globals contracts were checked. No Rust mapping, build, UI test, or parity test is claimed.

## Role and state

TemplateTool is the Map tool created by MainForm. It edits Map.Templates, a CellGrid<Template>, and also provides Ctrl-driven editing of Map.Bounds. It has two mutually exclusive interaction modes:
- Shift: place a selected template type or selected individual icon; right-drag clears template cells.
- Ctrl: resize the map bounds by dragging one of eight edges/corners.

Normal mode picks a whole template with left click or an individual icon with right click. The template catalog is grouped into ListView categories and a separate preview panel lets the user select a valid icon cell.

Fields:
- static compiled Regex CategoryRegex = ^([a-z]*), used to derive a category prefix from a template name.
- readonly ListView templateTypeListView, MapPanel templateTypeMapPanel, and ToolTip mouseTooltip.
- undoTemplates and redoTemplates dictionaries keyed by linear cell index.
- Map previewMap; protected override RenderMap returns it.
- placementMode, boundsMode, dragBounds, and dragEdge (initially -1).
- selectedTemplateType and private SelectedTemplateType property.
- nullable selectedIcon and private SelectedIcon property.
- NavigationWidget templateTypeNavigationWidget for the icon preview.
- private disposedValue.

## Constructor and catalog

Constructor signature:

**TemplateTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, ListView templateTypeListView, MapPanel templateTypeMapPanel, ToolTip mouseTooltip, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url)**

It delegates to ViewTool, initializes previewMap to map, attaches main MapPanel mouse/key handlers, subscribes ListView selection, then creates the type catalog:
1. Match a leading lowercase category prefix with CategoryRegex.
2. Filter plugin.Map.TemplateTypes to types with a non-null Thumbnail, a Theaters array containing the active map theatre, and no Clear flag.
3. Group by category and order groups by key.
4. Flatten thumbnails, compute maximum image width and height, create a Depth24Bit ImageList, and add grouped ListView items with each TemplateType in Tag.
5. Configure the template preview MapPanel (black background, MaxZoom 1), attach its mouse-down and post-render handlers, and retain the shared tooltip.
6. Subscribe NavigationWidget.MouseCellChanged and url.Undone/Redone.
7. Select the first type in the first category and update status.

The catalog query is enumerated for image sizing, list population, and first selection. There are no empty-catalog guards: Max() and First() throw if no type survives the filters. The filter also calls t.Theaters.Contains without a null check.

SelectedTemplateType updates the old/new template footprint invalidations while in placement mode, synchronizes the ListView selection with event suppression, ensures the selected item is visible, then rebuilds the preview panel. SelectedIcon invalidates the preview panel and the current map footprint when placement mode is active.

## Exact method behavior

| Method / exact signature | Behavior |
|---|---|
| Url_Redone(object sender, EventArgs e) / Url_Undone(object sender, EventArgs e) | While boundsMode is active, if map.Bounds differs from dragBounds, copies the map bounds into dragBounds, clears dragEdge, updates the tooltip, and invalidates the map panel. |
| TemplateTypeMapPanel_MouseDown(object sender, MouseEventArgs e) | Clears SelectedIcon for null or single-icon types. Otherwise, left-click reads the preview NavigationWidget cell and selects it only when in icon bounds and IconMask[x,y] is true; right-click clears the icon selection. |
| TemplateTypeMapPanel_PostRender(object sender, RenderEventArgs e) | Draws a yellow rectangle around SelectedIcon using one-quarter original-tile dimensions. If a type is selected, draws a black translucent label with DisplayName and IconWidth×IconHeight. It creates a Pen, StringFormat, brushes, and Matrix without disposing them. |
| TemplateTool_KeyDown(object sender, KeyEventArgs e) | Shift enters placement mode; Control enters bounds mode. |
| TemplateTool_KeyUp(object sender, KeyEventArgs e) | Releasing either Shift or Control calls ExitAllModes, even if the other modifier remains held. |
| TemplateTypeListView_SelectedIndexChanged(object sender, EventArgs e) | Sets SelectedTemplateType from the first selected item's Tag, or null when no item is selected, then clears SelectedIcon. |
| MapPanel_MouseDown(object sender, MouseEventArgs e) | In bounds mode detects a drag edge and updates status. In placement mode left calls SetTemplate and right calls RemoveTemplate. Otherwise left picks a whole template and right picks an individual icon. |
| MapPanel_MouseUp(object sender, MouseEventArgs e) | In bounds mode, if dragBounds differs from map.Bounds, captures old bounds, creates undo/redo closures, writes map.Bounds, tracks the command, and invalidates. It resets dragEdge and status. Outside bounds mode, commits buffered template-cell changes if either dictionary is non-empty. |
| MapPanel_MouseMove(object sender, MouseEventArgs e) | Enters Shift placement or Ctrl bounds mode based on modifiers; exits all modes when neither is held. Chooses resize cursor from dragEdge/DetectDragEdge and updates the bounds tooltip. |
| MouseoverWidget_MouseCellChanged(object sender, MouseCellChangedEventArgs e) | If dragEdge is active, clips the endpoint to map metrics and adjusts the corresponding top/bottom/left/right sides of dragBounds according to the selected edge/corner. In placement mode, right mouse drag removes templates at each new cell and invalidates old/new template footprints; left placement is performed on MouseDown, not continuously. In normal mode, a held left/right button repeatedly calls PickTemplate for each new cell. |
| RefreshMapPanel() | Disposes the previous templateTypeNavigationWidget. For a selected type, assigns its Thumbnail to the preview panel and creates a new NavigationWidget over a CellMetrics sized to IconWidth×IconHeight, using one-quarter original tile size. With no type, clears MapImage. |
| SetTemplate(Point location) | With SelectedIcon, writes one Template at a valid cell using row-major icon = y×IconWidth+x. Without SelectedIcon, first snapshots every valid cell in the full rectangular icon footprint, then writes only positions where IconMask[x,y] is true. Each write records redo state, invalidates, and marks dirty. It does not clear old values in mask holes and permits partial placement at map edges. |
| RemoveTemplate(Point location) | With SelectedIcon, snapshots and clears one valid cell. Without SelectedIcon, snapshots every valid cell in the full rectangular footprint and then clears every valid cell in that rectangle, regardless of IconMask. Each clear records redo null, invalidates, and marks dirty. |
| EnterPlacementMode() | Does nothing if placement or bounds mode is already active. Otherwise sets placementMode, empties mouseover size, invalidates the current selected-type footprint, and updates status. |
| EnterBoundsMode() | Does nothing if either mode is active. Otherwise sets boundsMode, copies map.Bounds to dragBounds, empties mouseover size, invalidates the selected-type footprint, updates tooltip, and updates status. |
| ExitAllModes() | If either mode is active, clears both modes, resets dragEdge and dragBounds, restores 1×1 mouseover size, invalidates the current type footprint, updates tooltip, and updates status. |
| UpdateTooltip() | In bounds mode formats X/Y/Width/Height, measures text, positions the tooltip near the cursor based on dragEdge, clamps to Screen.WorkingArea dimensions, and shows it. Outside bounds mode hides it. |
| PickTemplate(Point location, bool wholeTemplate) | For a valid cell, selects the stored Template.Type or falls back to the map's type named clear1 when the cell is empty. If picking an individual icon from a multi-icon type, derives SelectedIcon from the stored row-major icon index (or zero for an empty cell); otherwise clears SelectedIcon. |
| DetectDragEdge() | Uses MouseCell and MouseSubPixel to detect top, bottom, left, right, and four corners within quarter-tile bands. Returns edge codes 0–7 (top clockwise) or -1 when no edge is hit. |
| CommitChange() | Copies the undo/redo cell dictionaries into closures, applies each captured value to the supplied event map and invalidates the changed cell keys, clears live dictionaries, and registers one url.Track entry for the gesture. |
| UpdateStatus() | Displays template place/clear instructions, bounds edge drag instructions, or normal whole-template/individual-icon pick and modifier instructions. |
| PreRenderMap() | Calls base, clones the map into previewMap, and in placement mode adds either one selected icon or all IconMask-enabled cells of the selected template type at the mouse location, clipped cell-by-cell to map metrics. |
| PostRenderMap(Graphics graphics) | Calls base. In bounds mode draws a red 8-pixel outline around dragBounds; in placement mode draws a green outline around the one-cell selected icon or the full template footprint. |
| Dispose(bool disposing) | Unsubscribes main panel, key, ListView, preview panel, navigation cell-change, and undo/redo handlers, then calls base disposal. It does not dispose the current templateTypeNavigationWidget, although RefreshMapPanel disposes the previous widget whenever the selected type changes. |

## Template and map contracts

Template stores TemplateType Type and integer Icon. It is a one-cell ICellOccupier with a 1×1 true OccupyMask. Templates are not Technos; they live in Map.Templates, a CellGrid<Template>.

TemplateType stores ushort ID, Name/DisplayName, IconWidth, IconHeight, NumIcons, IconMask, Thumbnail, Theaters, and TemplateTypeFlag. Flags are None=0, Clear=2, Water=4, OreMine=8. Init creates a thumbnail grid sized to max(IconWidth, IconHeight), clears a bool[IconWidth, IconHeight] mask, then for each row-major icon asks TheTilesetManager for tile data. Found icons are drawn and set IconMask[x,y]=true. Thumbnail is null when no icon tile is found.

CellGrid indexers directly access the underlying array and do not perform bounds checks. TemplateTool protects its writes with Map.Metrics.GetCell before using a linear cell or coordinate. Map.Bounds is a Rectangle backed by MapSection X, Y, Width, and Height; setting it writes all four MapSection values.

## Review findings and Rust requirements

- **Map-bounds dirty gap:** the bounds-mode mouse-up path writes map.Bounds and records undo/redo but does not set plugin.Dirty. Verify save-dirty behavior; the tool contains no direct dirty update for bounds changes.
- **Template removal is rectangular:** whole-template removal clears every valid cell in the type's rectangular IconWidth×IconHeight footprint, including positions where IconMask is false. Whole-template placement writes only mask-true positions. This asymmetry can clear unrelated template cells in mask holes.
- **No-overwrite distinction:** whole placement overwrites mask-true cells but leaves mask-false cells untouched. It does not reject another template type already at a target cell.
- **Partial edge placement:** writes are clipped per cell with GetCell, so a multi-cell template can be partially placed at map edges.
- **Empty catalog:** the image Max and first-type First calls throw when no template survives thumbnail/theatre/Clear filtering.
- **Clear fallback:** PickTemplate assumes a type named clear1 exists. If absent, SelectedTemplateType becomes null and the subsequent individual-icon size check dereferences it.
- **Mode release:** releasing either Shift or Control exits both modes, even when the other modifier remains pressed.
- **Bounds picking fall-through:** MouseoverWidget_MouseCellChanged checks dragEdge and placementMode but not boundsMode before its normal-mode pick branch. In bounds mode with no edge selected, dragging across cells can call PickTemplate.
- **Preview widget lifetime:** the current templateTypeNavigationWidget is not disposed by Dispose(bool); only replacement in RefreshMapPanel disposes the previous instance.
- **GDI lifetime:** preview and map post-render paths allocate Pens, brushes, StringFormat, and Matrix instances without deterministic disposal.
- **Synchronous render cost:** ViewTool invokes PreRenderMap synchronously; this tool clones the complete map and writes the preview grid during paint.
- **Bounds tooltip positioning:** UpdateTooltip clamps screen coordinates against WorkingArea.Width/Height but does not offset by WorkingArea.Left/Top; verify behavior on secondary monitors with non-zero origins.
- **Edge threshold constant:** DetectDragEdge uses PixelHeight for horizontal quarter-tile thresholds. The pinned Globals constants currently set PixelWidth and PixelHeight both to 24, so this is numerically equivalent today but couples horizontal hit-testing to the vertical constant.

Recommended Rust design: typed TemplateType catalog with per-theatre IconMask, cell-level template IDs plus icon indices, atomic multi-cell placement/removal commands, explicit mask-hole semantics, checked map-boundary clipping, and a separate MapBounds resize command. Keep the preview icon selector and resize interaction in a UI adapter.

## Related source

- [Template.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Template.cs)
- [TemplateType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/TemplateType.cs)
- [CellGrid.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/CellGrid.cs)
- [NavigationWidget.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Widgets/NavigationWidget.cs)
- [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs)
- [MainForm.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/MainForm.cs)


## TemplateTypes catalog follow-up — Tranche 6S-C-A

See [Variant-TemplateTypes-6S-C-A.md](../Variant-TemplateTypes-6S-C-A.md) for catalog slice, theatre/flag differences, model mask construction, serializer call-site formats and whole-placement/removal asymmetry. Body-review status unchanged; no runtime parity tests executed.