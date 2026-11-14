# MapPanel.cs — editor source reference

> **Audit status: partially reviewed.** Source declarations and method signatures were mechanically enumerated from the pinned file; generated UI details and full behavioral explanation are incomplete.

- Upstream: [CnCTDRAMapEditor/Controls/MapPanel.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/MapPanel.cs)
- Pinned source commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 16945 characters.

## Purpose

Scrollable map viewport with coordinate transforms, zoom, camera management, incremental invalidation, painting, and native drawing suspension/resumption.

## Types

- MapPanel

## Fields detected

- `updatingCamera`
- `cameraBounds`
- `lastScrollPosition`
- `mapToViewTransform`
- `viewToPageTransform`
- `compositeTransform`
- `invCompositeTransform`
- `invalidateCells`
- `fullInvalidation`
- `mapImage`
- `minZoom`
- `maxZoom`
- `zoomStep`
- `zoom`
- `quality`

## Method declarations detected

- `public Point MapToClient(Point point)`
- `public Size MapToClient(Size size)`
- `public Rectangle MapToClient(Rectangle rectangle)`
- `public Point ClientToMap(Point point)`
- `public Size ClientToMap(Size size)`
- `public Rectangle ClientToMap(Rectangle rectangle)`
- `public void Invalidate(Map invalidateMap)`
- `public void Invalidate(Map invalidateMap, Rectangle cellBounds)`
- `public void Invalidate(Map invalidateMap, IEnumerable<Rectangle> cellBounds)`
- `public void Invalidate(Map invalidateMap, Point location)`
- `public void Invalidate(Map invalidateMap, IEnumerable<Point> locations)`
- `public void Invalidate(Map invalidateMap, int cell)`
- `public void Invalidate(Map invalidateMap, IEnumerable<int> cells)`
- `public void Invalidate(Map invalidateMap, ICellOverlapper overlapper)`
- `protected override void OnMouseEnter(EventArgs e)`
- `protected override void OnMouseWheel(MouseEventArgs e)`
- `protected override void OnClientSizeChanged(EventArgs e)`
- `protected override void OnScroll(ScrollEventArgs se)`
- `protected override void OnPaintBackground(PaintEventArgs e)`
- `protected override void OnPaint(PaintEventArgs pe)`
- `private void UpdateCamera()`
- `private void RecalculateTransforms()`
- `private void InvalidateScroll()`
- `private static extern int SendMessage(IntPtr hWnd, Int32 wMsg, bool wParam, Int32 lParam);`
- `private void SuspendDrawing()`
- `private void ResumeDrawing()`

## Implementation notes

The source contains point/size/rectangle coordinate conversion in both directions, several invalidation overloads (map, cell bounds, points, cell IDs, and overlapper), paint and scroll overrides, camera/transform recomputation, and Win32 SendMessage-based SuspendDrawing/ResumeDrawing. The exact transform order, clipping, invalidation aggregation, and exception behavior need line-by-line audit.

## Rust mapping
No equivalent path is asserted. MapPanel behavior should be mapped to the Rust editor viewport/render crate only after transform, invalidation, zoom, and input semantics are fully characterized. Dialogs and WinForms layout are UI-specific and should be reimplemented at the presentation layer.

## Remaining audit
- [ ] Exact signatures and full method-body explanations.
- [ ] Designer controls, layout dimensions, tab order, event hookup, and resource keys.
- [ ] Disposal and native handle lifetime.
- [ ] Settings persistence and interaction with App.config.
- [ ] Rust implementation paths and behavioral tests.
