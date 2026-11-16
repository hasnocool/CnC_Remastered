# NavigationWidget — mouse-to-cell mapping, subpixels, panning, and cursor footprint

Pinned upstream: [NavigationWidget.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Widgets/NavigationWidget.cs).  
Pinned blob: cca72bebb4cc22860cfd48ee1ac19a61d1c8e136. Source length: 167 lines.  
Audit status: **Body reviewed**. Every field, property, event argument, constructor, pointer callback, render method, and disposal body was inspected. No Rust mapping, build, UI test, or parity test is claimed.

## Role and state

NavigationWidget implements IWidget and centralizes mouse-to-map conversion for editor tools. It exposes the current map cell, the mouse's subpixel coordinate within that cell, a MouseCellChanged event, a configurable odd-sized cursor footprint, middle-button panning, and cursor rendering.

Fields:
- static readonly Pen defaultMouseoverPen = yellow, width 4;
- readonly MapPanel mapPanel;
- readonly Size cellSize;
- bool dragging;
- Point lastMouseLocation;
- CellMetrics Metrics;
- Point MouseCell and MouseSubPixel;
- private Size mouseoverSize, default 1×1;
- MouseCellChanged event.

MouseCellChangedEventArgs contains OldCell and NewCell, both private-set Point properties.

## Exact property and method behavior

| Method / exact signature | Behavior |
|---|---|
| MouseoverSize.get/set | Returns the current footprint. For any non-empty Size, setter forces both dimensions odd using width|1 and height|1. Size.Empty remains empty. |
| NavigationWidget(MapPanel mapPanel, CellMetrics metrics, Size cellSize) | Stores panel/metrics/cell size and subscribes to MapPanel.MouseDown and MouseMove. It does not initially call OnMouseMove; MouseCell and MouseSubPixel remain default (0,0) until a pointer move or Refresh. |
| Refresh() | Recomputes from the current global mouse position converted to panel-client coordinates by calling OnMouseMove. |
| MapPanel_MouseDown(object sender, MouseEventArgs e) | On middle-button down, stores e.Location and sets dragging=true. Other buttons do nothing. |
| MapPanel_MouseMove(object sender, MouseEventArgs e) | Passes e.Location to OnMouseMove. |
| OnMouseMove(Point location) | If dragging, checks whether the middle button is still held; if released, clears dragging, otherwise pans AutoScrollPosition by the delta from lastMouseLocation. It then updates lastMouseLocation, converts the point with ClientToMap, calculates MouseSubPixel using Globals.PixelWidth/Height and cellSize, calculates the new integer MouseCell, returns if the cell is unchanged, returns if Metrics.Contains is false, otherwise updates MouseCell, raises MouseCellChanged(old,new), and invalidates the panel. |
| Render(Graphics graphics) | When MouseoverSize is non-empty, builds a rectangle at MouseCell×cellSize, inflates by half the odd footprint dimensions in cell pixels, and draws it with defaultMouseoverPen. |
| Dispose(bool disposing) | Runs once and, when disposing, unsubscribes MapPanel.MouseDown and MouseMove. |
| Dispose() | Calls Dispose(true). |

## Pointer and coordinate invariants

- MouseCell changes only when the newly calculated cell differs from the current value and Metrics.Contains(newMouseCell) is true.
- MouseSubPixel is recalculated before the unchanged-cell and bounds checks. Therefore it can change even when MouseCell does not.
- If the pointer maps outside Metrics, the method returns after updating MouseSubPixel but leaves MouseCell unchanged. The pair can then describe different map cells.
- For negative map-space coordinates, C# integer division and remainder semantics can produce negative subpixel values. The code does not clamp them.
- MouseCellChanged is not raised for movement within the same cell, even though MouseSubPixel changes. Consumers needing subcell precision must observe MapPanel.MouseMove or another continuous pointer source.
- The constructor does not initialize from the current pointer. Until the first MouseMove/Refresh, the default cell is (0,0).
- The default mouseover footprint is 1×1. The setter normalizes even sizes upward: 2 becomes 3, 4 becomes 5. This matters if a caller assumes exact even brush dimensions.
- The Render rectangle is centered around the current cell for odd sizes. Size.Empty suppresses the cursor outline.

## Middle-button navigation

Middle-button down starts panning and stores the current client location. While middle remains held, each move applies the inverse scroll-position plus the delta to MapPanel.AutoScrollPosition. The widget stops dragging only when a later OnMouseMove observes that the middle button is no longer held; it does not subscribe to MouseUp.

## Review findings and Rust requirements

1. **MouseCell/MouseSubPixel can diverge:** out-of-bounds pointer movement updates MouseSubPixel but leaves MouseCell unchanged. InfantryTool uses MouseCell for the group and MouseSubPixel for nearest stopping-slot selection, so a pointer outside the map can combine a stale cell with a new or negative subpixel position.
2. **Subpixel changes have no event:** MouseCellChanged only fires when the cell changes. Consumers that need precise movement inside one cell must subscribe to a continuous pointer event or expose a separate MouseSubPixelChanged event.
3. **No initial pointer refresh:** a newly constructed widget starts at (0,0) until the first move/Refresh. A preview or brush can initially appear at the origin even when the cursor is elsewhere.
4. **Odd-size normalization:** the setter forces odd dimensions. Keep this explicit in a Rust brush/cursor API rather than silently accepting even sizes.
5. **Middle drag termination:** panning ends on the next mouse move after the middle button is released; there is no MouseUp handler.
6. **Static drawing resource:** defaultMouseoverPen is a static process-lifetime Pen and is not disposed by instances. It is not allocated per render, but its lifetime is intentionally process-wide.
7. **Consumer ownership:** ViewTool disposes its primary NavigationWidget. TemplateTool creates a second preview NavigationWidget and its Dispose method does not dispose the current preview widget; callers must explicitly own/dispose every instance.

Recommended Rust boundary: a pure coordinate converter returning a validated cell and bounded subpixel pair together, a continuous pointer state stream, an explicit odd-brush normalization policy, and a navigation adapter that owns pan gesture state. Never publish a new subpixel coordinate with an unrelated stale cell.

## Related source

- [MapPanel.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/MapPanel.cs)
- [CellMetrics.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/CellMetrics.cs)
- [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs)
- [InfantryTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/InfantryTool.cs)
- [TemplateTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/TemplateTool.cs)
