# UnitTool — single-cell vehicle placement and drag movement

Pinned upstream: [UnitTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/UnitTool.cs).  
Pinned blob: 2c8bb07f40e3eec2b92cd70e22c13a0b01853bf4. Source length: 427 lines.  
Audit status: **Body reviewed**. Every constructor, property, event handler, helper, render override, and disposal body in this file was inspected. No Rust mapping, UI test, build, or parity test is claimed.

## Role and state

UnitTool derives from ViewTool and edits individual Unit objects in Map.Technos. It supports:
- left-click selection and drag movement;
- Shift placement mode, left-click insertion and right-click removal;
- right-click picking of an existing unit's editable fields;
- double-click property editing;
- a three-tile preview panel and translucent placement ghost.

Fields:
- readonly TypeComboBox unitTypeComboBox;
- readonly MapPanel unitTypeMapPanel;
- readonly ObjectProperties objectProperties;
- Map previewMap, returned by protected override RenderMap;
- bool placementMode;
- readonly Unit mockUnit;
- Unit selectedUnit;
- ObjectPropertiesPopup selectedObjectProperties;
- UnitType selectedUnitType and private SelectedUnitType property;
- private disposedValue.

The mock starts with the first UnitType, first House, strength 256, North direction, and Guard mission when available (otherwise the first mission). Unit.Trigger uses its model default None. The constructor's First() calls are not guarded.

## Constructor and MainForm filter

Constructor signature:

**UnitTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, TypeComboBox unitTypeComboBox, MapPanel unitTypeMapPanel, ObjectProperties objectProperties, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url)**

The constructor delegates shared setup to ViewTool, sets previewMap to map, creates/subscribes the mock Unit, attaches MapPanel mouse and Control key events, subscribes selector changes, configures the preview panel (white, MaxZoom 1), binds ObjectProperties.Object to mockUnit, subscribes NavigationWidget.MouseCellChanged, selects the mock type, and updates status.

MainForm supplies only UnitTypes where IsFixedWing is false, sorted by Name. The filter can yield an empty sequence; the constructor then fails at Types.First().

SelectedUnitType ignores unchanged references, invalidates an inflated 3×3 region around the mouse cell before and after changing type while in placement mode, synchronizes ComboBox.SelectedValue, assigns mockUnit.Type, and refreshes the preview.

## Exact method and event behavior

| Method / exact signature | Behavior |
|---|---|
| MapPanel_MouseDoubleClick(object sender, MouseEventArgs e) | Rejects any modifier key. Resolves MouseCell; only a Unit at that cell opens an ObjectPropertiesPopup. Clears drag selection, closes the prior popup, subscribes SelectedUnit_PropertyChanged, shows the popup, and updates status. |
| MockUnit_PropertyChanged(object sender, PropertyChangedEventArgs e) | Rebuilds the type preview. |
| SelectedUnit_PropertyChanged(object sender, PropertyChangedEventArgs e) | Invalidates the sender Unit on the live map. |
| UnitTypeComboBox_SelectedIndexChanged(object sender, EventArgs e) | Assigns SelectedValue as UnitType. |
| UnitTool_KeyDown(object sender, KeyEventArgs e) / UnitTool_KeyUp(object sender, KeyEventArgs e) | Shift down enters placement mode; Shift up exits. |
| MapPanel_MouseMove(object sender, MouseEventArgs e) | Reconciles placement mode from modifier state. It does not move a selected Unit; drag movement is handled by NavigationWidget.MouseCellChanged. |
| MapPanel_MouseDown(object sender, MouseEventArgs e) | In placement mode routes left/right to AddUnit/RemoveUnit. Otherwise routes left to SelectUnit and right to PickUnit. |
| MapPanel_MouseUp(object sender, MouseEventArgs e) | Clears selectedUnit after a drag and updates status. |
| MouseoverWidget_MouseCellChanged(object sender, MouseCellChangedEventArgs e) | In placement mode invalidates old/new inflated one-cell rectangles. Otherwise, when a unit is selected, captures its old location, invalidates and removes it, attempts Technos.Add at the new cell, then invalidates/marks dirty on success or restores the old location on failure. |
| AddUnit(Point location) | Clones mockUnit and calls Technos.Add only when a selected type exists. A successful add invalidates the unit and marks dirty. |
| RemoveUnit(Point location) | If Technos at the cell is a Unit, invalidates it, removes it, and marks dirty. |
| EnterPlacementMode() / ExitPlacementMode() | Idempotently toggle placementMode, change NavigationWidget.MouseoverSize between empty and 1×1, invalidate the current inflated one-cell area when a type exists, and update status. |
| PickUnit(Point location) | For a Unit at a valid Metrics cell, copies Type, House, Strength, Direction, Mission, and Trigger into mockUnit. |
| SelectUnit(Point location) | Selects a Unit at a valid Metrics cell, otherwise clears selection, then updates status. |
| RefreshMapPanel() | Creates a 3×3-tile bitmap and calls MapRenderer.Render with plugin.GameType, map.Theater, origin (1,1), tile geometry, and mockUnit. Clears MapImage when Type is null. |
| UpdateStatus() | Displays placement instructions, selected-unit drag instructions, or normal pick/select/property-edit instructions. |
| PreRenderMap() | Calls base, clones the map into previewMap, and in placement mode adds a 50%-alpha white mock Unit to the clone when Technos.Add accepts the current cell. |
| PostRenderMap(Graphics graphics) | Calls base and draws a 4-pixel green rectangle around each Unit's one-tile map cell. |
| Dispose(bool disposing) | Closes selectedObjectProperties, unsubscribes map mouse/key, selector, and navigation cell-change handlers once, then calls base disposal. It does not detach SelectedUnit_PropertyChanged from previously edited Unit objects. |

## Occupancy and movement

Unit implements ICellOccupier and ICellOverlapper; OverlapBounds and OccupyMask delegate to UnitType. Its editable properties are Type, House, Strength, Direction, Mission, Trigger (default None), and Tint. Clone copies Type, House, Strength, Direction, Mission, and Trigger; Tint is not explicitly copied by Unit.Clone, so the placement ghost sets Tint on the cloned preview instance after cloning.

OccupierSet.Add validates that the occupier is non-null and not already registered, computes the true cells in its OccupyMask, and rejects any point outside map bounds or already occupied. The UnitTool move operation removes first and attempts the destination add; if the add fails it restores the original location. Preserve that rollback as an atomic move command in Rust so a failed destination never leaves the Unit detached.

## Review findings and Rust requirements

- Constructor First() calls are unchecked for UnitTypes, Houses, North direction, and the fallback mission list. MainForm filters out fixed-wing types, so an empty post-filter catalog can fail tool activation.
- SelectedUnit.PropertyChanged is subscribed each time a popup opens and is not detached when the popup closes or the tool is disposed. The popup Closed callback only refreshes NavigationWidget. UnitTool does close the active popup during Dispose, unlike BuildingTool and InfantryTool, but it still leaves the model event subscription attached.
- PostRenderMap allocates a Pen per render and does not dispose it.
- PreRenderMap clones the full map synchronously on the map-panel render path; avoid coupling high-frequency input to full-document cloning in the Rust UI.
- The UndoRedoList is passed through ViewTool but UnitTool never accesses it or registers commands. Add, remove, and move are direct model mutations and do not provide undo/redo parity in this class.
- The move callback relies on OccupierSet.Add for destination bounds and collision validation; retain both failure cases and old-location restoration in tests.

Recommended Rust commands: place/remove unit, move with rollback, pick/select, edit properties, and create a render-only ghost. Keep fixed-wing exclusion as an explicit UI catalog policy, not a hidden limitation in the underlying Unit model.

## Related source

- [UnitTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/UnitTool.cs)
- [Unit.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Unit.cs)
- [OccupierSet.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OccupierSet.cs)
- [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs)
- [MainForm.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/MainForm.cs)
