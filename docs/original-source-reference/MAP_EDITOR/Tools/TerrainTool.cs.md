# TerrainTool — terrain techno placement, movement, and properties

Pinned upstream: [TerrainTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/TerrainTool.cs).  
Pinned blob: 271d27be08a5a44ecfaa721bc7d8164bf16eefa4. Source length: 456 lines.  
Audit status: **Body reviewed**. All members and method bodies in this file were inspected. Supporting Terrain, TerrainType, TerrainProperties, dialog, Map, and OccupierSet contracts were checked where needed. No Rust mapping, build, UI test, or parity test is claimed.

## Role and state

TerrainTool derives from ViewTool and edits Terrain objects stored in Map.Technos. It supports single-cell placement/removal in Shift mode, selection and drag movement of existing terrain, right-click type/trigger picking, double-click trigger-property editing, a thumbnail preview, and render-time occupancy overlays.

Fields:
- readonly TypeComboBox terrainTypeComboBox;
- readonly MapPanel terrainTypeMapPanel;
- readonly TerrainProperties terrainProperties;
- Map previewMap, returned by protected override RenderMap;
- bool placementMode;
- readonly Terrain mockTerrain;
- Terrain selectedTerrain and Point selectedTerrainPivot;
- TerrainType selectedTerrainType;
- TerrainPropertiesPopup selectedTerrainProperties;
- private SelectedTerrainType property and disposedValue.

The SelectedTerrainType setter ignores unchanged references. During placement mode it invalidates the old and new OverlapBounds-sized regions at MouseCell. It updates the selector, assigns mockTerrain.Type, chooses Icon 22 for transformable types and 0 otherwise, then refreshes the thumbnail panel.

## Constructor and host setup

Constructor signature:

**TerrainTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, TypeComboBox terrainTypeComboBox, MapPanel terrainTypeMapPanel, TerrainProperties terrainProperties, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url)**

The constructor delegates to ViewTool, points previewMap at the live map, creates a new mock Terrain and subscribes its PropertyChanged event, attaches MapPanel mouse and Control key handlers, subscribes the type selector, sets the preview panel white with MaxZoom 1, binds TerrainProperties.Terrain to the mock, and makes the embedded property control visible only for GameType.TiberianDawn. It subscribes NavigationWidget.MouseCellChanged, selects the first supplied TerrainType, and updates status.

MainForm supplies terrain types whose Theaters array contains the active map theatre. The constructor calls Types.First() without an empty-list guard. TerrainType itself accepts a nullable Theaters array, while the MainForm filter calls Contains directly; the pinned call path therefore assumes non-null theatre arrays and at least one compatible type.

The TerrainToolDialog(plugin) constructor initializes its embedded TerrainProperties control with Initialize(plugin, true) before passing it to TerrainTool. The popup for an existing Terrain is a separate TerrainProperties control initialized with isMockObject=false.

## Exact method behavior

| Method / exact signature | Behavior |
|---|---|
| MapPanel_MouseDoubleClick(object sender, MouseEventArgs e) | Rejects modifier keys, resolves MouseCell, and opens TerrainPropertiesPopup only when Map.Technos at that cell is Terrain. Clears drag selection, closes the previous popup, shows the new popup at the current mouse position, and updates status. |
| MockTerrain_PropertyChanged(object sender, PropertyChangedEventArgs e) | Calls RefreshMapPanel for every mock property change. |
| TerrainTypeCombo_SelectedIndexChanged(object sender, EventArgs e) | Copies SelectedValue as TerrainType into SelectedTerrainType. |
| TerrainTool_KeyDown(object sender, KeyEventArgs e) / TerrainTool_KeyUp(object sender, KeyEventArgs e) | Shift down enters placement mode; Shift up exits. |
| MapPanel_MouseDown(object sender, MouseEventArgs e) | In placement mode, left calls AddTerrain and right calls RemoveTerrain. Otherwise, left selects a Terrain and right picks its type/trigger into the mock. |
| MapPanel_MouseUp(object sender, MouseEventArgs e) | Clears selectedTerrain and its pivot after movement, then updates status. |
| MapPanel_MouseMove(object sender, MouseEventArgs e) | Reconciles placement mode from Control.ModifierKeys: exactly Shift enters, no modifiers exits. It does not place terrain continuously while dragging. |
| MouseoverWidget_MouseCellChanged(object sender, MouseCellChangedEventArgs e) | In placement mode invalidates old/new TerrainType.OverlapBounds-sized regions. Otherwise, when a Terrain is selected, computes a non-negative top-left preserving the click pivot, invalidates/removes the old Terrain, attempts Technos.Add at the new point, invalidates on success, and attempts to restore the old point on failure. It does not set plugin.Dirty and does not call url.Track for this move. |
| AddTerrain(Point location) | Rejects locations outside Map.Metrics. If a type is selected, clones mockTerrain and adds it to Technos. On success it invalidates the terrain, registers undo (remove the captured terrain) and redo (add it at the captured location), and marks the plugin dirty. |
| RemoveTerrain(Point location) | If Technos at the point is Terrain, invalidates and removes it, registers undo (re-add at the location) and redo (remove the captured Terrain), and marks dirty. |
| EnterPlacementMode() / ExitPlacementMode() | Idempotently toggle placementMode, change NavigationWidget.MouseoverSize between empty and 1×1, invalidate the current overlap-sized area when a type exists, and update status. |
| PickTerrain(Point location) | At a valid cell containing Terrain, assigns its Type to the selector/mock and copies Trigger. The selector setter recalculates Icon from IsTransformable; it does not copy the existing Terrain.Icon. |
| SelectTerrain(Point location) | Selects Terrain at a valid cell and stores the clicked-cell offset from its stored top-left; otherwise clears selection/pivot. Updates status. |
| RefreshMapPanel() | Assigns mockTerrain.Type.Thumbnail directly to terrainTypeMapPanel.MapImage. |
| UpdateStatus() | Displays placement instructions, selected-terrain drag instructions, or normal select/move/property/pick instructions. |
| PreRenderMap() | Calls base, clones the live map into previewMap, and in placement mode creates a translucent Terrain ghost with selected type and transformable-dependent Icon. It adds the ghost only when the mouse location is in map bounds; Technos.Add performs occupancy validation. |
| PostRenderMap(Graphics graphics) | Calls base and draws a green rectangle around each Terrain's RenderSize plus red rectangles for each true cell in its OccupyMask. It iterates previewMap.Technos, so the placement ghost is included. |
| Dispose(bool disposing) | Closes the current TerrainPropertiesPopup, unsubscribes mouse/key, selector, and navigation cell-change handlers once, then calls base disposal. |

## Terrain model and property binding

Terrain implements ICellOccupier, ICellOverlapper, INotifyPropertyChanged, and ICloneable. It stores Type, Icon, Trigger (default None), and Tint. OverlapBounds and OccupyMask delegate to TerrainType. Clone copies Type, Icon, and Trigger; Tint returns to its default white value.

TerrainType stores sbyte ID, Name/DisplayName, Theaters, OccupyMask, IsTransformable, TemplateType, RenderSize, and Thumbnail. OverlapBounds is the RenderSize rounded up to whole tile dimensions. Init maps OreMine template types to the OREMINE tileset name, loads icon 22 for transformable terrain (otherwise icon 0), stores RenderSize divided by TileScale, and creates a half-size thumbnail.

TerrainProperties.Initialize binds a trigger selector to Trigger.None plus distinct map trigger names and subscribes to Map.Triggers.CollectionChanged. Its Disposed handler clears Terrain and unsubscribes. TerrainPropertiesPopup creates a separate control with isMockObject=false and clears its Terrain on close.

## Review findings and Rust requirements

- **Move dirty/undo gap:** the selected-terrain drag path removes and re-adds the Terrain but never sets plugin.Dirty and never records an undo/redo action. Add/remove clicks do both. A Rust port should make move an atomic command with rollback, dirty-state update, and undo/redo.
- **Property dirty handler wiring:** TerrainProperties.cs defines Obj_PropertyChanged to rebind on Type and set Plugin.Dirty for non-mock objects, but neither that file nor TerrainProperties.Designer.cs subscribes Terrain.PropertyChanged to this handler. The trigger ComboBox's SelectedIndexChanged writes its binding, but the defined dirty handler is not reached through the inspected control wiring. Verify this behavior in a UI test before treating property edits as save-dirty.
- **Null preview assumption:** RefreshMapPanel dereferences mockTerrain.Type.Thumbnail without checking Type. If the selector can transition to null, MockTerrain_PropertyChanged can trigger a null-reference failure.
- **Catalog assumptions:** the constructor uses Types.First() without a guard. MainForm filters by theatre and can supply an empty list; the MainForm predicate also assumes TerrainType.Theaters is non-null.
- **GDI lifetime:** PostRenderMap creates green and red Pens on every render and does not dispose them.
- **Synchronous preview work:** PreRenderMap clones the full map synchronously in ViewTool's pre-render callback.
- **Movement semantics:** drag clamps only the top-left to non-negative coordinates; Technos.Add is responsible for full occupancy-mask bounds/collision validation. On failure the code attempts to re-add at the old location.
- **Undo granularity:** AddTerrain and RemoveTerrain each create their own url.Track entry. Terrain dragging is not grouped with these operations and currently has no undo entry.

Recommended Rust command surface: place/remove/move Terrain, set type/icon/trigger, validate occupancy masks, preserve transformable icon defaults, and create a render-only ghost. Keep TerrainType theatre/tile initialization separate from map document storage.

## Related source

- [Terrain.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Terrain.cs)
- [TerrainType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/TerrainType.cs)
- [TerrainProperties.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/TerrainProperties.cs) and [local control audit](../Controls/TerrainProperties.cs.md)
- [TerrainToolDialog.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/TerrainToolDialog.cs)
- [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs)
- [OccupierSet.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OccupierSet.cs)


## Rust implementation update — generic editor wiring (partial)

The desktop editor's Terrain layer click path now calls `place_terrain_with_history` and exposes Undo/Redo controls backed by `MapEditHistory`. The current UI deliberately constructs a 1×1 generic occupancy definition because the complete TD/RA TerrainType catalog is not yet imported. This proves the command/history connection only; it does **not** map TerrainTool's mock Terrain, Map.Technos, type selector, transformable icon-22 selection, drag/pivot movement, right-click removal, trigger properties, or render-derived overlap bounds. The source TerrainTool is therefore still only partially mapped.


## Rust implementation update — source catalog and brush integration

The desktop editor Terrain brush now selects definitions from the active variant/theatre `TerrainCatalog` instead of creating a synthetic 1×1 definition. Placement uses each source OccupyMask and remains one atomic history command. `CellLayers.terrain_icon` records icon 22 for transformable types and 0 otherwise; this is undoable/redoable. Catalog filtering prevents selecting entries unavailable in the chosen theatre.

Still not mapped: actual Map.Technos object aggregation/clone semantics, Shift placement mode and ghost preview, right-click removal, drag/pivot movement, trigger properties, source occupancy collision with non-terrain techno categories, and render-size-derived overlap/redraw geometry. OreMine asset substitution and tile 22 asset loading are not yet connected to the renderer.
