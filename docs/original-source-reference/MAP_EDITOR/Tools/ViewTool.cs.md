# ViewTool — shared map-view lifecycle and overlay rendering

Pinned upstream: [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs).  
Pinned blob: e946a4e9033220dbdf3b446d04dac910e7573726. Source length: 316 lines.  
Audit status: **Body reviewed**. All fields, properties, constructor setup, event handlers, rendering branches, and disposal bodies in the pinned file were inspected. No Rust renderer mapping, build, UI test, or parity test is claimed.

## Role and ITool contract

ViewTool is the abstract WinForms base class for map editing tools. It implements ITool, whose complete interface is IDisposable plus MapLayerFlag Layers { get; set; }. ViewTool supplies the common map reference, view controls, navigation widget, layer state, map-panel event subscriptions, shared overlay drawing, and disposal. Derived tools override RenderMap, PreRenderMap, and/or PostRenderMap to provide previews and tool-specific overlays.

## Fields and properties

- protected readonly IGamePlugin plugin — active game plugin and GameType.
- protected readonly Map map — captured from plugin.Map at construction.
- protected readonly MapPanel mapPanel — interactive map viewport and render event source.
- protected readonly ToolStripStatusLabel statusLbl — tool instructions/status.
- protected readonly UndoRedoList<UndoRedoEventArgs> url — shared undo/redo collection exposed to derived tools; ViewTool itself does not read or write it.
- protected readonly NavigationWidget navigationWidget — mouse cell/subpixel tracking, zoom/navigation and overlay drawing.
- protected virtual Map RenderMap => map — render target; object tools override it to return previewMap.
- private MapLayerFlag layers and public MapLayerFlag Layers { get; set; } — changing the flags invalidates RenderMap.

## Constructor and subscriptions

Constructor signature:

**ViewTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url)**

It stores the supplied layer flags, plugin, undo list, panel and status label; subscribes MapPanel.PreRender and PostRender; captures map = plugin.Map; subscribes BasicSection.PropertyChanged; and creates NavigationWidget(mapPanel, map.Metrics, Globals.TileSize).

The captured Map reference is not dynamically re-read from plugin.Map. MainForm is responsible for disposing/replacing the active tool when the active plugin/map changes.

## Exact method behavior

| Method / exact signature | Behavior |
|---|---|
| Invalidate() | Calls mapPanel.Invalidate(RenderMap), so a derived preview target is invalidated rather than always the live map. |
| BasicSection_PropertyChanged(object sender, PropertyChangedEventArgs e) | Handles only PropertyName == BasePlayer. It enumerates Buildings.OfType<Building>(), selects occupiers whose BasePriority is non-negative, and invalidates each building. Other BasicSection property changes are ignored here. |
| MapPanel_PreRender(object sender, RenderEventArgs e) | Returns immediately only when e.Cells is non-null and empty. Otherwise calls PreRenderMap, creates Graphics from mapPanel.MapImage, applies NearestNeighbor interpolation and HighSpeed pixel offset when Settings.Default.Quality > 1, filters the optional cell set through map.Metrics.Contains, then calls MapRenderer.Render(plugin.GameType, RenderMap, graphics, filteredCells, Layers). |
| MapPanel_PostRender(object sender, RenderEventArgs e) | Calls PostRenderMap(e.Graphics), then NavigationWidget.Render(e.Graphics). The navigation widget is drawn after the tool/base overlays. |
| PreRenderMap() | Empty virtual extension point. Derived tools can prepare a render map or transient placement ghost before MapRenderer runs. |
| PostRenderMap(Graphics graphics) | Draws the shared overlays enabled by Layers: waypoint cells, techno trigger labels, cell-trigger labels, and map boundaries. Detailed behavior is below. |
| Dispose(bool disposing) | Uses disposedValue to run once. When disposing is true, disposes NavigationWidget and unsubscribes MapPanel.PreRender/PostRender and BasicSection.PropertyChanged. It then marks disposedValue. |
| Dispose() | Calls Dispose(true). |

The Layers setter compares the old and new flags; only a change stores the new value and calls Invalidate().

## Shared post-render overlays

### Waypoints

When MapLayerFlag.Waypoints is enabled, the method creates a translucent black background brush, translucent dark-orange text brush, and dark-orange outline Pen. For every waypoint with a Cell value, it converts the linear cell index to x/y using Metrics.Width, scales to tile pixels, fills/outlines a tile-sized rectangle, centers the waypoint number, chooses an adjusted font, and draws the label.

### Techno triggers

When MapLayerFlag.TechnoTriggers is enabled, the method iterates Map.Technos and builds one or more (trigger, bounds) records according to the runtime object type:
- Terrain: trigger text over Type.RenderSize.
- Building: trigger text over Type.Size in tile pixels.
- Unit: trigger text over one Globals.TileSize rectangle.
- InfantryGroup: one record for each non-null Infantry member. The slot index is interpreted as InfantryStoppingType; UpperLeft/UpperRight/LowerLeft/LowerRight shift the tile rectangle by one quarter tile in the corresponding directions, while Center uses the unshifted tile.

It ignores trigger names equal to None using ordinal case-insensitive comparison. For each remaining trigger it computes a fitted font and measured text size, centers a translucent dark background behind the label, outlines it in lime green, and draws lime-green text.

### Cell triggers

When MapLayerFlag.CellTriggers is enabled, it iterates Map.CellTriggers. Each linear cell index is converted to x/y with Metrics.Width; a translucent black background and white outline are drawn over the tile, with the trigger name centered in translucent white text.

### Map boundaries

When MapLayerFlag.Boundaries is enabled, it converts Map.Bounds to pixel coordinates using tile width/height and draws a cyan rectangle with an 8-pixel Pen.

## Resource and execution characteristics

- All rendering and overlay work is invoked directly from MapPanel render events; the file contains no task scheduling, worker dispatch, or asynchronous boundary.
- Derived PreRenderMap implementations in BuildingTool, InfantryTool, and UnitTool clone a map and prepare a placement ghost before MapRenderer draws. This happens synchronously inside MapPanel_PreRender.
- The Graphics wrapper for MapImage is disposed with a using scope.
- The waypoint, techno-trigger, cell-trigger, and boundary branches allocate Brushes, Pens, and StringFormat objects without disposing them in this method. Repeated redraws can therefore accumulate GDI resources. The derived placement tools also allocate undisposed Pens in their PostRenderMap methods.
- Dispose removes the base subscriptions and disposes NavigationWidget, but derived classes remain responsible for unsubscribing their own handlers and closing their own popups.
- The shared url field is only stored here; undo/redo semantics are implemented by individual derived tools, not by ViewTool.

## Rust port boundary and tests

The current map-editor-model crate is a data document, not a view system. It stores cell layers and waypoints but has no MapLayerFlag rendering adapter, viewport, navigation widget, invalidation scheduler, trigger overlay painter, or render snapshot API.

Recommended Rust separation:
1. Keep MapDocument and occupancy/model commands independent of UI frameworks.
2. Build immutable render snapshots from the document and current theatre/type catalogs.
3. Render enabled layers and labels from the snapshot; do not mutate the document during paint.
4. Keep mouse-to-cell/subpixel conversion in a navigation adapter with checked coordinate conversion.
5. Make subscriptions scoped and disposal deterministic.
6. Coalesce invalidation and expensive preview preparation so pointer handling never waits on full-map cloning or synchronous disk work.

Required tests include layer-toggle invalidation, null versus empty render-cell sets, off-map cell filtering, BasePlayer-only building invalidation, all four shared overlay types, infantry trigger offsets for all five stops, trigger None case-insensitivity, disposal/unsubscription, and repeated-render GDI/resource stability. No tests were run in this audit tranche.

## Related source

- [ITool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Interface/ITool.cs)
- [MainForm.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/MainForm.cs)
- [BuildingTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/BuildingTool.cs)
- [InfantryTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/InfantryTool.cs)
- [UnitTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/UnitTool.cs)
