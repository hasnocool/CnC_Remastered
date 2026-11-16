# BuildingTool — structure placement, movement, and base ordering

Pinned upstream: [BuildingTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/BuildingTool.cs).  
Pinned blob: 1c00e9a97d6cb2fb4997b376eb33f3a6a98c4a65. Source length: 503 lines.  
Audit status: **Body reviewed**. Every constructor, property, event handler, helper, render override, and disposal body in this file was inspected. This is source review only; no Rust mapping, build, UI test, or parity test is claimed.

## Role and inheritance

BuildingTool derives from ViewTool and edits the map supplied by IGamePlugin. It supports:
- selecting an existing building and dragging it while preserving the click offset inside its footprint;
- Shift placement mode, with left-click placement and right-click removal;
- right-click picking of a building's type and editable state into a mock object;
- double-click property editing;
- a separate type-preview panel;
- base-building priority normalization and occupancy-mask visualization.

ViewTool owns the shared Map reference, MapPanel, status label, UndoRedoList reference, NavigationWidget, layer selection, and map-panel pre/post-render subscriptions. ITool exposes only IDisposable and the Layers property. The derived RenderMap property redirects the base renderer to previewMap.

## State

- readonly TypeComboBox buildingTypeComboBox — selectable BuildingType catalog.
- readonly MapPanel buildingTypeMapPanel — preview surface.
- readonly ObjectProperties objectProperties — property editor bound to the mock object.
- Map previewMap; protected override Map RenderMap returns this value.
- bool placementMode — Shift-driven place/remove mode.
- readonly Building mockBuilding — template cloned for placement and used by the property editor.
- Building selectedBuilding — current drag target.
- ObjectPropertiesPopup selectedObjectProperties — popup for an existing building.
- Point selectedBuildingPivot — clicked-cell offset from the selected building's stored top-left.
- BuildingType selectedBuildingType and private SelectedBuildingType property — synchronizes selector, mock object, invalidation, and preview.

The mock object starts with the first selector type, first map house, strength 256, and the map's North direction. Its PropertyChanged event refreshes the preview. If its type is null or has no turret, MockBuilding_PropertyChanged forces the direction back to North.

## Construction and selector contract

Constructor signature:

**BuildingTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, TypeComboBox buildingTypeComboBox, MapPanel buildingTypeMapPanel, ObjectProperties objectProperties, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url)**

The constructor:
1. delegates common rendering/navigation setup to ViewTool;
2. initially sets previewMap to the live map;
3. constructs the mock Building using First() on the type, house, and North-direction sequences;
4. subscribes mock property changes;
5. subscribes MapPanel mouse down/up/double-click/move and Control key down/up;
6. subscribes selector SelectedIndexChanged;
7. sets the preview panel white with MaxZoom 1;
8. binds ObjectProperties.Object to mockBuilding;
9. subscribes NavigationWidget.MouseCellChanged;
10. selects the mock type and updates status.

MainForm.RefreshActiveTool supplies only building types whose Theater list is null or contains the current map theatre, ordered by IsFake then Name. The constructor still assumes at least one supplied type, one house, and a North direction; there are no local guards around First().

SelectedBuildingType ignores reference-identical assignments. While placement mode is active, it invalidates the old and new overlap-size rectangles at the current mouse cell. It updates the backing field and ComboBox SelectedValue, assigns mockBuilding.Type, then rebuilds the preview. ComboBox synchronization can raise SelectedIndexChanged, but the second setter call is a no-op when the value already matches.

## Event and operation behavior

| Method / exact signature | Behavior |
|---|---|
| MapPanel_MouseDoubleClick(object sender, MouseEventArgs e) | Rejects any modifier key. Resolves MouseCell with Metrics.GetCell; only a Building at that cell opens an ObjectPropertiesPopup. Clears drag selection and pivot, closes the previous popup, attaches SelectedBuilding_PropertyChanged, shows at the current mouse position, and updates status. |
| MockBuilding_PropertyChanged(object sender, PropertyChangedEventArgs e) | Resets direction to North when the type is null or lacks a turret, then refreshes the type preview. |
| SelectedBuilding_PropertyChanged(object sender, PropertyChangedEventArgs e) | Invalidates the sender Building on the live map. |
| UnitTypeComboBox_SelectedIndexChanged(object sender, EventArgs e) | Copies SelectedValue as BuildingType into SelectedBuildingType. The legacy handler name is UnitTypeComboBox even though this is the building selector. |
| UnitTool_KeyDown(object sender, KeyEventArgs e) / UnitTool_KeyUp(object sender, KeyEventArgs e) | Shift down enters placement mode; Shift up exits. The legacy handler names retain UnitTool. |
| MapPanel_MouseMove(object sender, MouseEventArgs e) | Also reconciles placement mode from Control.ModifierKeys, entering when exactly Shift is held and exiting when no modifiers are held. It does not perform building dragging. |
| MapPanel_MouseDown(object sender, MouseEventArgs e) | In placement mode: left calls AddBuilding, right calls RemoveBuilding. Otherwise: left selects a building, right picks a building into the mock state. |
| MapPanel_MouseUp(object sender, MouseEventArgs e) | Clears selectedBuilding and pivot after a drag and refreshes status. |
| MouseoverWidget_MouseCellChanged(object sender, MouseCellChangedEventArgs e) | In placement mode invalidates old/new overlap-size rectangles. In drag mode, computes a non-negative new top-left from MouseCell minus selectedBuildingPivot, removes the building, checks Technos.CanAdd using BaseOccupyMask, and tries Buildings.Add. On success it invalidates and marks the plugin dirty; on failure it restores the old location. |
| AddBuilding(Point location) | Clones mockBuilding. Requires a non-null selected type, Technos.CanAdd(location, building, BaseOccupyMask), and Buildings.Add. For base buildings (BasePriority >= 0), increments existing priorities at or after the inserted priority, sorts, renumbers contiguously from zero, and invalidates all base buildings. It then invalidates the new building and marks dirty. |
| RemoveBuilding(Point location) | Looks up a Building through Technos at the cell, invalidates it, removes it through Buildings, renumbers remaining non-negative base priorities contiguously, invalidates those buildings, and marks dirty. |
| EnterPlacementMode() / ExitPlacementMode() | Idempotently toggle placementMode, change NavigationWidget.MouseoverSize between empty and 1×1, invalidate the current overlap-sized preview area when a type exists, and update status. |
| PickBuilding(Point location) | For a Building at a valid cell, copies Type, House, Strength, Direction, Trigger, BasePriority, IsPrebuilt, Sellable, and Rebuild into the mock object. |
| SelectBuilding(Point location) | Selects the Building at a valid cell and records the click pivot as clicked location minus the building's stored top-left. Updates status. |
| RefreshMapPanel() | Calls MapRenderer.Render with plugin.GameType, map.Theater, origin (0,0), tile geometry/scaling, and mockBuilding. If the returned render size is non-empty, creates a bitmap and invokes the returned draw callback; otherwise clears MapImage. |
| UpdateStatus() | Displays distinct instructions for placement, active drag, or normal pick/select/property-edit mode. |
| PreRenderMap() | Calls base, clones the entire map into previewMap, and in placement mode adds a white 50%-alpha mock building to the clone only when the selected type is non-null and occupancy permits it. |
| PostRenderMap(Graphics graphics) | Calls base, draws a 4-pixel green rectangle around each building's Type.Size, then draws 2-pixel red rectangles for every true cell in Type.BaseOccupyMask. |
| Dispose(bool disposing) | Unsubscribes map mouse/key, selector, and navigation cell-change handlers once, then calls base disposal. It does not close selectedObjectProperties or detach SelectedBuilding_PropertyChanged from the model object. |

## Occupancy and model side effects

Building implements ICellOccupier and ICellOverlapper. Its OverlapBounds and OccupyMask delegate to BuildingType; its editable fields include Type, House, Strength, Direction, Trigger, BasePriority (default -1), IsPrebuilt (default true), Sellable, Rebuild, BibCells, and Tint. Clone copies the editable placement properties but does not copy the BibCells set.

The placement precheck deliberately uses BuildingType.BaseOccupyMask. Buildings is a separate OccupierSet; its Add path uses the Building's OccupyMask. Map subscribes Buildings.OccupierAdded/Removed:
- adding a Building to Buildings adds it to Technos with BaseOccupyMask and generates bib smudges where the building type has a supported bib;
- removing it removes bib smudges and removes it from Technos.

Thus the editor's two occupancy sets and two masks are not interchangeable. Preserve both the Building collection and the Map event side effects in a port.

OccupierSet.Add rejects null/already-present objects and rejects any occupied-mask point that is out of map bounds or already occupied. A failed drag therefore restores the old position; it does not partially move the object.

## Review findings and Rust requirements

- The constructor's First() calls are unchecked. An empty theatre-filtered building catalog, empty house list, or missing North direction can throw during tool activation.
- Existing-object PropertyChanged handlers are added on each double-click and are not removed when the popup closes or the tool is disposed. The Closed callback only refreshes NavigationWidget. This can retain the tool and cause stale invalidations after the object or tool is no longer active.
- PostRenderMap allocates two Pen instances on each render and does not dispose them. ViewTool's base post-render also creates drawing objects without local disposal. Repeated redraws should be profiled for GDI resource growth.
- PreRenderMap clones the full map synchronously during each non-empty map-panel pre-render before MapRenderer draws. This work is on the WinForms rendering path; a Rust implementation should use an immutable render snapshot or cached preview overlay rather than block pointer/input processing with a full document clone.
- The url UndoRedoList is passed to ViewTool but BuildingTool never reads it or records commands. The mutations in this class are direct model writes; this class itself does not provide undo/redo parity.

Recommended Rust command coverage: place/remove building, move building with rollback, pick/select, update properties, preserve click pivot, apply both occupancy masks, generate/remove bibs, and normalize base priorities. Keep preview rendering separate from committed document state.

## Related source

- [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs)
- [Building.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Building.cs)
- [Map.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Map.cs)
- [OccupierSet.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OccupierSet.cs)
- [MainForm.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/MainForm.cs)
- [TypeComboBox.cs](https://github.com/hasnocool/CnC_Remastered/blob/main/docs/original-source-reference/MAP_EDITOR/Controls/TypeComboBox.cs.md)
