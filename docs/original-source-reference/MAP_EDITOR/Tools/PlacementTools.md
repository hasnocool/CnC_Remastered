# CellTriggersTool and WaypointsTool — placement-tool source guide

Pinned upstream: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.

## CellTriggersTool

- Source: [CellTriggersTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/CellTriggersTool.cs) — blob 086dd8205230686d6eed1347ab18cd4fca926e64.
- Status: **Body reviewed**; all methods in this contained file were inspected.
- Constructor: CellTriggersTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, ComboBox triggerCombo, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url). It delegates common setup to ViewTool, subscribes map mouse/key and navigation-widget cell-change events, stores the combo, and updates status.
- State: trigger ComboBox; undo/redo dictionaries keyed by linear cell index; placementMode; disposedValue.
- Normal mode: left or right click picks an existing cell trigger into the combo. Shift enters placement mode; left click sets only an empty cell; right click clears an occupied cell. While a mouse button is held, MouseCellChanged repeats set/remove across cells.
- Set resolves Point through map.Metrics.GetCell. If the cell is empty, it snapshots the prior value once, constructs CellTrigger { Trigger = triggerCombo.SelectedItem as string }, writes map.CellTriggers[cell], stores the redo value, invalidates mapPanel, and sets plugin.Dirty=true. It does not replace a non-null trigger.
- Remove resolves the cell, snapshots the existing trigger once, writes null, stores null for redo, invalidates, and marks the plugin dirty.
- MouseUp commits only when an undo/redo dictionary is non-empty. Commit copies both dictionaries, clears the live buffers, and registers closures with url.Track; undo/redo replay each cell value against e.Map.CellTriggers and invalidate e.MapPanel.
- Shift KeyDown/KeyUp and MouseMove both maintain placement mode; status text changes between pick and paint instructions.
- Dispose unsubscribes all map and navigation-widget handlers before base disposal.

## WaypointsTool

- Source: [WaypointsTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/WaypointsTool.cs) — blob 4c0fbda55298911f4d46ca39b2affb612afbced1.
- Status: **Body reviewed**; all methods in this contained file were inspected.
- Constructor: WaypointsTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, ComboBox waypointCombo, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url). It initializes ViewTool, subscribes mouse/key events, stores the combo, and updates status.
- State: waypoint ComboBox; nullable undo/redo tuples (Waypoint waypoint, int? cell); placementMode; disposedValue.
- Normal mode: left or right click picks the waypoint occupying the clicked cell and selects its array index. Shift placement mode: left click assigns the selected waypoint to a valid cell; right click finds the first waypoint at that cell and clears it. MouseMove also enters/exits placement mode based on modifier state.
- Set indexes map.Waypoints[waypointCombo.SelectedIndex], checks that the target differs from its current cell, captures the previous cell, writes the new cell, captures redo state, immediately commits one undo entry, invalidates the panel, and marks the plugin dirty.
- Remove searches map.Waypoints for the first waypoint whose Cell matches, captures the prior cell, sets it null, immediately commits, invalidates, and marks dirty.
- Commit captures the nullable tuple values in undo/redo closures, clears the live tuples, and registers them with url.Track. The closures update the captured Waypoint reference and invalidate mapPanel.
- Dispose removes mouse/key handlers before base disposal.

## Edge conditions and port requirements

- CellTriggersTool can construct a CellTrigger with null Trigger if no ComboBox item is selected; MainForm binds a string array but does not guard an empty list here.
- WaypointsTool does not validate SelectedIndex before indexing the waypoint array. An empty waypoint collection or no selection can produce an invalid index.
- Cell-trigger edits are batched across a drag and committed on MouseUp; waypoint edits commit individually at each set/remove call. Preserve this difference if matching undo granularity.
- CellTriggersTool undo actions use the map supplied by UndoRedoEventArgs; WaypointsTool closures retain the original Waypoint object reference. Verify object lifetime across map replacement.
- Both tools mutate the model from WinForms event callbacks and invalidate the panel synchronously. A Rust UI should serialize mutations through the editor/model thread, expose undoable commands, and schedule rendering without blocking input handling.

## Rust mapping

crates/map-editor-model/src/lib.rs stores waypoint number to CellCoord in a BTreeMap; MapDocument::add_waypoint rejects duplicate waypoint numbers and out-of-bounds coordinates. cell_metrics.rs supplies checked coordinate/index conversion. There is no CellTrigger collection, paint command, undo/redo stack, placement-mode state, or UI adapter in this crate. These are partial model building blocks only; neither tool is Rust-mapped or parity-tested.
