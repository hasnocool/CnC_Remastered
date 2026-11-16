# MapPanel — camera transforms, incremental invalidation, and render lifecycle

Pinned upstream: [MapPanel.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/MapPanel.cs).  
Pinned blob: 358e125ada837533777ffb5830f0a18148c8a8ad. Source length: 536 lines.  
Audit status: **Body reviewed**. Every field, property, event, transform overload, invalidation overload, WinForms override, camera routine, P/Invoke, and drawing helper was inspected. No Rust mapping, build, GUI test, or parity test is claimed.

## Role and state

MapPanel derives from Panel and owns the editor viewport's map-image display, camera, zoom, scroll integration, coordinate transforms, incremental cell invalidation, and PreRender/PostRender events.

Fields:
- updatingCamera, cameraBounds, lastScrollPosition;
- nullable referencePositions tuple (Point map, SizeF client) used to preserve the cursor anchor during zoom;
- mapToViewTransform, viewToPageTransform, compositeTransform, invCompositeTransform;
- invalidateCells HashSet<Point> and fullInvalidation flag;
- Image mapImage;
- minZoom=1, maxZoom=8, zoomStep=1, zoom=1;
- quality initialized from Properties.Settings.Default.Quality.

Public properties:
- MapImage: stores a different Image reference and calls UpdateCamera.
- MinZoom/MaxZoom: update the bound and reassign Zoom to force clamping.
- ZoomStep: stores the step and rounds current Zoom down to a multiple.
- Zoom: clamps to MinZoom..MaxZoom and records the current map point plus normalized client position before updating the camera.
- Quality: stores and invalidates when changed.
- FocusOnMouseEnter: Behavior property, default false.
- PreRender and PostRender: EventHandler<RenderEventArgs> events.

## Coordinate conversion API

Point, Size, and Rectangle overloads exist for MapToClient and ClientToMap:
- Point uses TransformPoints.
- Size uses VectorTransformPoints so translation is excluded.
- Rectangle transforms only the top-left and bottom-right points and reconstructs an axis-aligned rectangle. This is valid for the current scale/translation-only camera transforms.

## Incremental invalidation API

Overloads accept:
- full map invalidation;
- Rectangle or IEnumerable<Rectangle>;
- Point or IEnumerable<Point>;
- linear cell index or IEnumerable<int>;
- ICellOverlapper.

Full invalidation clears the cell set, sets fullInvalidation=true, and calls Panel.Invalidate. Partial invalidation returns if full invalidation is already pending; otherwise it adds every point in the requested rectangles, expands the set through Map.Overlappers.Overlaps, and calls Invalidate only when new cells were added. Linear indices are converted through Metrics.GetLocation. The IEnumerable<int> overload calls GetLocation twice for each retained index.

## Exact method behavior

| Method / exact signature | Behavior |
|---|---|
| MapPanel() | Calls InitializeComponent and enables DoubleBuffered. |
| MapToClient(Point/Size/Rectangle) / ClientToMap(Point/Size/Rectangle) | Apply the current composite or inverse composite transform to points, vectors, or rectangle corners. |
| Invalidate(Map invalidateMap) | Clears partial cells, sets fullInvalidation, and requests a full WinForms repaint. |
| Invalidate(Map invalidateMap, Rectangle cellBounds) / IEnumerable<Rectangle> | Adds all cells in the rectangle(s), expands through the map Overlapper set, and requests repaint when the invalidation set grows. |
| Invalidate(Map invalidateMap, Point location) / IEnumerable<Point> | Wraps points as 1×1 rectangles and delegates. |
| Invalidate(Map invalidateMap, int cell) / IEnumerable<int> | Converts valid linear cell indices to coordinates and delegates. |
| Invalidate(Map invalidateMap, ICellOverlapper overlapper) | Looks up the overlapper's rectangle and invalidates it when present. |
| OnMouseEnter(EventArgs e) | Calls base and focuses the panel only when FocusOnMouseEnter is true. |
| OnMouseWheel(MouseEventArgs e) | Adds ZoomStep multiplied by Math.Sign(e.Delta); magnitude beyond sign is ignored. |
| OnClientSizeChanged(EventArgs e) | Calls base then UpdateCamera. |
| OnScroll(ScrollEventArgs se) | Calls base then InvalidateScroll. |
| OnPaintBackground(PaintEventArgs e) | Calls base then clears BackColor. |
| OnPaint(PaintEventArgs pe) | Calls base, reconciles scroll, raises PreRender with null cells for full invalidation or the pending cell set otherwise, draws MapImage using compositeTransform and nearest-neighbor interpolation, restores compositing/interpolation settings, raises PostRender, optionally draws developer overlap diagnostics, then clears invalidationCells and fullInvalidation. The Graphics transform is intentionally left as compositeTransform for map-coordinate PostRender drawing. |
| UpdateCamera() | Returns when MapImage is null or ClientSize.IsEmpty. Otherwise sets updatingCamera, computes camera size from map/panel aspect ratios and zoom, clamps or centers camera location, computes scroll size, recalculates transforms, restores the map point under the cursor from referencePositions when possible, suspends redraw, updates AutoScrollMinSize/AutoScrollPosition/lastScrollPosition, resumes redraw, clears updatingCamera, and invalidates. |
| RecalculateTransforms() | Rebuilds map-to-view, view-to-page, composite, and inverse matrices from cameraBounds and ClientSize. |
| InvalidateScroll() | Returns while updatingCamera. When AutoScrollPosition changed, converts the scroll delta to map units, clamps camera location, recalculates transforms, and invalidates. |
| SuspendDrawing() / ResumeDrawing() | Send WM_SETREDRAW false/true to the control Handle using user32.dll SendMessage. Return values are ignored. |

## Render lifecycle and camera contract

OnPaint raises PreRender before drawing MapImage. ViewTool's pre-render callback uses this point to update MapImage. MapPanel then draws the image under compositeTransform and raises PostRender with the same transformed Graphics so tool overlays and NavigationWidget can draw in map coordinates.

Zoom records the map coordinate beneath the mouse and the mouse's normalized position in the client area. UpdateCamera uses those two values to keep that map coordinate anchored at the same relative client position after zoom.

AutoScrollPosition is treated as a camera movement. InvalidateScroll converts the change in scroll position back into map units, updates cameraBounds, and recalculates the transforms.

## Review findings and Rust requirements

- **MapImage clearing does not repaint/reset camera:** setting MapImage=null calls UpdateCamera, which immediately returns. It does not invalidate, reset AutoScrollMinSize, clear cameraBounds, or rebuild transforms. A cleared preview can remain visually stale and retain previous scroll state until another repaint/camera update.
- **Image ownership is undefined:** MapImage setter replaces the reference without disposing the previous image, and MapPanel's designer Dispose does not dispose the current image. Callers that create Bitmaps must own and dispose replaced images; do not blindly dispose because thumbnails may be shared.
- **Matrix lifetime:** the four Matrix instances are allocated as fields and never disposed by the designer or code-behind. They wrap GDI+ resources.
- **ZoomStep validation:** setting ZoomStep=0 divides by zero in the rounding expression. Negative values are also accepted without validation.
- **Zoom bounds validation:** MinZoom and MaxZoom accept arbitrary integers. There is no invariant enforcing MinZoom>0 or MinZoom<=MaxZoom; zero zoom can make camera calculations divide by zero.
- **Camera redraw safety:** UpdateCamera sets updatingCamera=true and suspends drawing, but does not use try/finally. An exception during camera/scroll updates can leave updatingCamera true and WM_SETREDRAW disabled.
- **Paint cleanup safety:** OnPaint does not use try/finally around PreRender/PostRender, image drawing, graphics-state restoration, or invalidation-set clearing. A thrown render handler can leave stale invalidation state and altered Graphics state for subsequent handlers.
- **Scroll null assumption:** InvalidateScroll dereferences mapImage when a scroll delta is observed; it assumes AutoScrollPosition cannot change while MapImage is null.
- **Windows coupling:** SuspendDrawing/ResumeDrawing depend on user32.dll WM_SETREDRAW. A Rust renderer should use its windowing toolkit's invalidation/scheduling mechanism rather than porting this P/Invoke.
- **Invalidation expansion:** partial invalidation expands cells through Overlappers, not just the changed cell. Preserve this so multi-cell sprites redraw when their covered cells change.
- **Coordinate event contract:** PreRender receives null for full invalidation and a live HashSet for partial invalidation; consumers must treat null as full render and must not retain the mutable set after the synchronous callback.

Recommended Rust architecture: a camera value object with validated zoom bounds, a single invertible transform pair, immutable invalidation batches, explicit image/snapshot ownership, and render callbacks that cannot leave the viewport in a half-suspended state. Use RAII for matrices/bitmaps and guaranteed state restoration.

## Related source

- [MapPanel.Designer.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/MapPanel.Designer.cs)
- [MapPanel.resx](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/MapPanel.resx)
- [NavigationWidget.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Widgets/NavigationWidget.cs)
- [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs)
