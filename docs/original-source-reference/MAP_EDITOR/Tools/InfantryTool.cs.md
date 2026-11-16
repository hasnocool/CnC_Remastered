# InfantryTool — infantry-group and stopping-slot placement

Pinned upstream: [InfantryTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/InfantryTool.cs).  
Pinned blob: a4901665d60366e9ff49a40f5e84a85f99140744. Source length: 540 lines.  
Audit status: **Body reviewed**. Every constructor, property, event handler, helper, render override, and disposal body in this file was inspected. No Rust mapping, UI test, build, or parity test is claimed.

## Role and state

InfantryTool derives from ViewTool and edits Infantry objects stored inside an InfantryGroup occupying one map cell. It supports:
- left-click selection and drag movement of an individual infantry member between stopping slots;
- Shift placement mode, left-click insertion and right-click removal;
- right-click picking of the nearest infantry member's properties;
- double-click editing of the nearest infantry member;
- a type preview rendered at the Center stopping position.

Fields:
- readonly TypeComboBox infantryTypeComboBox;
- readonly MapPanel infantryTypeMapPanel;
- readonly ObjectProperties objectProperties;
- Map previewMap, returned by protected override RenderMap;
- bool placementMode;
- readonly Infantry mockInfantry, created with a null InfantryGroup reference;
- Infantry selectedInfantry;
- ObjectPropertiesPopup selectedObjectProperties;
- InfantryType selectedInfantryType and private SelectedInfantryType property;
- private disposedValue.

The mock defaults to the first InfantryType, first map House, strength 256, South direction, and Guard mission when available (otherwise the first mission). Trigger uses Infantry's model default None. The constructor uses First() for the initial type, house, and South direction, and can therefore fail if these source collections are empty. The mission fallback still calls First() if no mission exists.

## Constructor and selector

Constructor signature:

**InfantryTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, TypeComboBox infantryTypeComboBox, MapPanel infantryTypeMapPanel, ObjectProperties objectProperties, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url)**

It delegates to ViewTool, sets previewMap to map, creates and subscribes the mock Infantry, attaches MapPanel mouse and Control key events, attaches selector SelectedIndexChanged, sets the preview panel white with MaxZoom 1, binds ObjectProperties.Object to mockInfantry, subscribes NavigationWidget.MouseCellChanged, selects the first type, and updates status.

MainForm binds all map InfantryTypes ordered by Name; it does not add a non-empty guard before constructing this tool.

SelectedInfantryType ignores unchanged values, invalidates the old and new one-cell rectangles while placement mode is active, synchronizes the ComboBox, assigns mockInfantry.Type, and refreshes the type preview.

## Exact method and event behavior

| Method / exact signature | Behavior |
|---|---|
| MapPanel_MouseDoubleClick(object sender, MouseEventArgs e) | Returns if any modifier is pressed. Resolves MouseCell; if it contains an InfantryGroup, obtains the single nearest stopping type from MouseSubPixel. Opens a popup only if that exact slot contains an Infantry. Clears selectedInfantry, closes the previous popup, subscribes SelectedInfantry_PropertyChanged, shows the popup, and updates status. It does not search the next-nearest occupied stop when the nearest stop is empty. |
| MockInfantry_PropertyChanged(object sender, PropertyChangedEventArgs e) | Rebuilds the type preview. |
| SelectedInfantry_PropertyChanged(object sender, PropertyChangedEventArgs e) | Invalidates the selected member's current InfantryGroup. |
| InfantryTypeComboBox_SelectedIndexChanged(object sender, EventArgs e) | Assigns the selected value as InfantryType. |
| InfantryTool_KeyDown(object sender, KeyEventArgs e) / InfantryTool_KeyUp(object sender, KeyEventArgs e) | Shift down enters placement mode; Shift up exits. |
| MapPanel_MouseMove(object sender, MouseEventArgs e) | Reconciles placement mode from modifiers. During placement it invalidates the current cell inflated by one cell in every direction. During a selected-infantry drag it finds/creates the destination InfantryGroup, examines stopping slots in distance order, and moves the member into the first free slot. |
| MapPanel_MouseDown(object sender, MouseEventArgs e) | In placement mode routes left/right to AddInfantry/RemoveInfantry. Otherwise routes left to SelectInfantry and right to PickInfantry. |
| MapPanel_MouseUp(object sender, MouseEventArgs e) | Clears selectedInfantry after a drag and updates status. |
| MouseoverWidget_MouseCellChanged(object sender, MouseCellChangedEventArgs e) | Only invalidates old/new inflated one-cell regions for the placement preview. Drag mutation is in MapPanel_MouseMove, not this callback. |
| AddInfantry(Point location) | Requires a non-null selected type and a valid Metrics cell. Reuses an InfantryGroup at that cell or creates/registers one. Iterates stopping types nearest-first; in the first free slot, clones mockInfantry, assigns the slot and back-reference, invalidates the group, marks dirty, and stops. |
| RemoveInfantry(Point location) | Requires a valid cell containing an InfantryGroup. Removes the first occupied slot in nearest-first order, invalidates and marks dirty, then removes the group from Technos if every slot is null. |
| EnterPlacementMode() / ExitPlacementMode() | Idempotently toggle placementMode, set NavigationWidget.MouseoverSize to empty or 1×1, invalidate the current inflated cell when a type exists, and update status. |
| PickInfantry(Point location) | At a valid cell containing a group, checks only the single nearest stopping slot. If occupied, copies Type, House, Strength, Direction, Mission, and Trigger into mockInfantry. |
| SelectInfantry(Point location) | At a valid cell containing a group, clears selection and selects only the member in the single nearest stopping slot. Updates status. |
| RefreshMapPanel() | Creates a one-tile bitmap and calls the theatre-based MapRenderer overload for mockInfantry at the Center stopping type; clears MapImage when Type is null. |
| UpdateStatus() | Displays placement instructions, selected-member drag instructions, or normal pick/select/double-click instructions. |
| PreRenderMap() | Calls base, clones the map into previewMap, and in placement mode creates/reuses a group in the clone at MouseCell. It inserts a translucent mock member into the first free stopping slot in nearest-first order. |
| PostRenderMap(Graphics graphics) | Calls base and draws a 4-pixel green rectangle around each InfantryGroup's single occupied tile. |
| Dispose(bool disposing) | Unsubscribes map mouse/key, selector, and navigation cell-change events once, then calls base disposal. It does not close selectedObjectProperties or detach SelectedInfantry_PropertyChanged from previously edited Infantry objects. |

## InfantryGroup contract

The pinned Infantry.cs declares:
- InfantryStoppingType values Center=0, UpperLeft=1, UpperRight=2, LowerLeft=3, LowerRight=4.
- Infantry has InfantryGroup, Type, House, Strength, Direction, Mission, Trigger (default None), Tint, PropertyChanged, Clone, and the SetField/OnPropertyChanged notification helpers.
- InfantryGroup has a fixed Infantry array sized by Globals.NumInfantryStops, OverlapBounds (-1,-1,3,3), and a 1×1 true OccupyMask.
- Its static constructor assigns five pixel-space stopping positions: center and four quarter-tile corners.
- ClosestStoppingTypes(Point subPixel) computes squared Euclidean distance from the mouse subpixel to every stop, sorts ascending, and yields the corresponding enum values.

Infantry.Clone initially copies its current InfantryGroup reference as well as the editable properties. AddInfantry overwrites the cloned reference with the destination group after placing the clone in a slot.

## Movement and edge semantics

During a drag, MapPanel_MouseMove records the old group, the member's old array index, and the target cell's current techno. A null target cell gets a new InfantryGroup registered in Technos; an existing InfantryGroup is reused; any other techno blocks the move. It then checks stopping positions nearest-first. On the first free slot it clears the old slot, writes the selected member to the destination slot, invalidates the old group and removes it from Technos if now empty when crossing groups, updates the member's InfantryGroup back-reference, invalidates the destination group, and marks the plugin dirty. The loop exits once the destination group is the member's current group.

There are two distinct selection policies:
- Add, remove, and drag scan nearest-first until they find a free/occupied slot.
- Pick, select, and double-click inspect only the single nearest slot. They do not fall through to a farther occupied slot.

The model collection stores the group in Technos; the five individual Infantry instances are not separate map occupiers. The group has one-cell occupancy and a larger overlap region for rendering.

## Review findings and Rust requirements

- Constructor First() calls are unchecked. An empty infantry catalog, house list, or South direction list can throw; the mission fallback can also throw when MissionTypes is empty.
- Existing Infantry.PropertyChanged handlers are subscribed on every property-popup open and never removed on popup close or tool disposal. The popup Closed callback only refreshes NavigationWidget. Repeated edits can accumulate handlers and retain the tool.
- PostRenderMap allocates a Pen on each render without disposing it.
- PreRenderMap clones the complete map synchronously for each non-empty map-panel pre-render; a Rust renderer should consume a stable snapshot and represent the placement ghost as a transient overlay.
- The UndoRedoList is passed to ViewTool but this class never accesses it or records commands. Placement, removal, and drag writes are direct model mutations, so this class does not itself supply undo/redo parity.
- NavigationWidget recalculates MouseSubPixel before checking Metrics.Contains, but leaves MouseCell unchanged when the pointer maps outside the map. InfantryTool uses MouseCell for the group and MouseSubPixel for nearest-stop resolution, so these values can describe different cells. Validate and publish the cell/subpixel pair together; see [NavigationWidget audit](../../Widgets/NavigationWidget.cs.md).

Recommended Rust model: a typed InfantryGroup with exactly five named/ordered stopping slots, a deterministic nearest-stop resolver, explicit member-to-group ownership, and commands for add/remove/move/edit. Keep single-nearest selection semantics distinct from nearest-free placement semantics.

## Related source

- [InfantryTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/InfantryTool.cs)
- [Infantry.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Infantry.cs)
- [OccupierSet.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OccupierSet.cs)
- [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs)
- [MainForm.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/MainForm.cs)
