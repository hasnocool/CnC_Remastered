# SmudgeTool — decorative smudges and automatic building bib protection

Pinned upstream: [SmudgeTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/SmudgeTool.cs).  
Pinned blob: 322aa9a39a6ab6e82ddc75d8f3f1df6b2253d68f. Source length: 338 lines.  
Audit status: **Body reviewed**. All constructor, selector, input, add/remove, pick, preview/render, and disposal bodies were inspected. Smudge/SmudgeType and MainForm catalog filtering were checked. No Rust mapping, build, UI test, or parity test is claimed.

## Role and state

SmudgeTool derives from ViewTool and edits Map.Smudge, a CellGrid<Smudge>. It supports Shift placement mode, single-cell left-click placement, single-cell right-click removal, and normal-mode type picking with either button. Mouse-cell changes only invalidate the preview; this tool does not paint continuously while dragging.

Fields:
- readonly TypeComboBox smudgeTypeComboBox and MapPanel smudgeTypeMapPanel;
- Map previewMap, returned by protected override RenderMap;
- bool placementMode;
- SmudgeType selectedSmudgeType and private SelectedSmudgeType property;
- private disposedValue.

MainForm supplies SmudgeTypes where (Flag & SmudgeTypeFlag.Bib) == None, ordered by Name. TypeComboBox.Types computes Max thumbnail dimensions and the constructor calls Types.First(); an empty non-bib catalog fails before the tool is usable.

## Constructor and selector

Constructor signature:

**SmudgeTool(MapPanel mapPanel, MapLayerFlag layers, ToolStripStatusLabel statusLbl, TypeComboBox smudgeTypeComboBox, MapPanel smudgeTypeMapPanel, IGamePlugin plugin, UndoRedoList<UndoRedoEventArgs> url)**

It delegates to ViewTool, initializes previewMap to map, subscribes MapPanel.MouseDown/MouseMove and Control.KeyDown/KeyUp, subscribes selector SelectedIndexChanged, configures the preview panel white with MaxZoom 1, subscribes NavigationWidget.MouseCellChanged, selects the first type, and updates status.

SelectedSmudgeType invalidates the current cell for the old type when placement mode is active, stores the new type, synchronizes ComboBox.SelectedValue, invalidates the current cell again for the new type, and refreshes the thumbnail.

## Exact method behavior

| Method / exact signature | Behavior |
|---|---|
| SmudgeTypeComboBox_SelectedIndexChanged(object sender, EventArgs e) | Copies SelectedValue as SmudgeType into SelectedSmudgeType. |
| SmudgeTool_KeyDown(object sender, KeyEventArgs e) / SmudgeTool_KeyUp(object sender, KeyEventArgs e) | Shift down enters placement mode; Shift up exits. |
| MapPanel_MouseDown(object sender, MouseEventArgs e) | In placement mode, left calls AddSmudge and right calls RemoveSmudge. Otherwise either button calls PickSmudge. |
| MapPanel_MouseMove(object sender, MouseEventArgs e) | Enters placement mode when modifiers equal Shift; exits when placement mode is active and modifiers become None. |
| MouseoverWidget_MouseCellChanged(object sender, MouseCellChangedEventArgs e) | In placement mode, invalidates old and new one-cell locations when a type is selected. It does not add/remove smudges on pointer movement. |
| AddSmudge(Point location) | If Map.Smudge at the point is null and a type is selected, creates Smudge { Type, Icon=0, Data=0 }, writes the cell, invalidates, registers one undo/redo entry for that cell, and marks plugin.Dirty. The point indexer is used directly; the normal pointer path relies on NavigationWidget's valid MouseCell invariant. |
| RemoveSmudge(Point location) | Removes only an existing Smudge whose Type.Flag has no Bib bit. It writes null, invalidates, registers undo/redo for the captured smudge, and marks dirty. |
| EnterPlacementMode() / ExitPlacementMode() | Idempotently toggle placementMode, set NavigationWidget.MouseoverSize to empty or 1×1, invalidate the current cell when a type exists, and update status. |
| PickSmudge(Point location) | Resolves a valid cell and, if any Smudge exists there, assigns its Type to SelectedSmudgeType. It does not copy the instance Icon or Data and does not filter out Bib types. |
| RefreshMapPanel() | Assigns SelectedSmudgeType?.Thumbnail to the preview panel MapImage. |
| UpdateStatus() | Displays left-click place/right-click remove in placement mode, otherwise Shift-to-place and either-button pick instructions. |
| PreRenderMap() | Calls base, clones the live map, and in placement mode adds a translucent Smudge ghost with selected Type and Data=0 only when the cell is valid and empty. |
| PostRenderMap(Graphics graphics) | Calls base and draws a green one-tile outline around every non-Bib Smudge in previewMap. It creates a Pen on every render and does not dispose it. |
| Dispose(bool disposing) | Unsubscribes selector, map mouse/key, and navigation cell-change handlers once, then calls base disposal. |

## Smudge and SmudgeType model

Smudge stores Type, Icon, Data, and Tint (default white). It is a cell-grid value, not an ICellOccupier and not part of Map.Technos.

SmudgeType stores sbyte ID, Name/DisplayName, Size, SmudgeTypeFlag, RenderSize, and Thumbnail. Flags are None=0, Bib=1, Bib1=3, Bib2=5, Bib3=9. Bib1/Bib2/Bib3 all contain the Bib bit, so the tool's bitwise test excludes all three variants.

SmudgeType.Init loads tile data by Name and calculates RenderSize while preserving the source tile aspect ratio within one map tile. It creates a full-resolution Thumbnail from the source tile image.

## Bib ownership and cross-system behavior

Map.AddBibs is the owner of automatic building bib creation. It returns when BuildingType.HasBib is false; otherwise it finds Bib1/Bib2/Bib3 smudge types and selects by building width: width 2 uses Bib3, width 3 uses Bib2, width 4 uses Bib1. It places each bib icon beneath the building footprint, stores the resulting linear cells in Building.BibCells, and copies the building tint.

Map.RemoveBibs finds Bib-flagged smudges intersecting Building.BibCells, clears those cells, and clears the building's BibCells set. SmudgeTool therefore filters Bib types from its list and refuses to remove existing bib cells.

## Review findings and Rust requirements

- **Pick/catalog mismatch:** PickSmudge accepts any non-null Smudge, including Bib types, even though MainForm's selector excludes Bib types. The selected type can therefore be outside the supplied ComboBox catalog; verify ComboBox behavior and whether AddSmudge can subsequently use that value.
- **Add does not enforce the Bib filter:** AddSmudge checks only that the cell is empty and SelectedSmudgeType is non-null. The Bib exclusion is a catalog policy, not a mutation-layer invariant.
- **Per-click undo:** each successful AddSmudge/RemoveSmudge creates a separate url.Track entry; there is no drag-stroke batching.
- **No drag painting:** MouseCellChanged only invalidates. Placement/removal occurs on MapPanel.MouseDown.
- **GDI lifetime:** PostRenderMap allocates an undisposed Pen on every render.
- **Synchronous preview:** PreRenderMap clones the full map synchronously inside ViewTool's render callback.
- **Bounds assumption:** AddSmudge/RemoveSmudge use the CellGrid point indexer directly. NavigationWidget currently updates MouseCell only for coordinates contained by Metrics; preserve that validation in a new input adapter.

Recommended Rust model: typed SmudgeKind and SmudgeCell, separate generated Bib ownership tied to Building.BibCells, explicit add/remove commands that reject bib-owned cells, and a preview-only ghost. Preserve Icon/Data when editing existing smudges rather than assuming type pick is a full object copy.

## Related source

- [Smudge.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Smudge.cs)
- [SmudgeType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/SmudgeType.cs)
- [Map.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Map.cs)
- [NavigationWidget.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Widgets/NavigationWidget.cs)
- [ViewTool.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/ViewTool.cs)
