# MainForm.cs — map editor source guide

> **Audit status: partially reviewed.** This is a first-pass source inspection against the pinned upstream C# file. The extracted declarations are not a complete parser-backed symbol inventory; nested, multiline, generated, and conditional declarations need verification.

- Upstream: [`CnCTDRAMapEditor/MainForm.cs`](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/MainForm.cs)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source size: 50,213 characters.

## Plain-English purpose

Main editor window and application coordinator. It owns tool selection, visible map layers, active tool/dialog, current plugin/map file, recent files, undo/redo history, and periodic Steam UGC servicing. It is the UI orchestration point between map model, tools, dialogs, rendering, and game plugins.

## Namespaces and imports

- `MobiusEditor.Dialogs`
- `MobiusEditor.Event`
- `MobiusEditor.Interface`
- `MobiusEditor.Model`
- `MobiusEditor.Tools`
- `MobiusEditor.Tools.Dialogs`
- `MobiusEditor.Utility`
- `Steamworks`
- `System`
- `System.Collections.Generic`
- `System.ComponentModel`
- `System.Data`
- `System.Diagnostics`
- `System.Drawing`
- `System.IO`
- `System.Linq`
- `System.Text`
- `System.Windows.Forms`

## Top-level and nested type names found

- `MainForm`
- `ToolType`

## Method/declaration lines found

- `private void SteamUpdateTimer_Tick(object sender, EventArgs e)`
- `protected override void OnLoad(EventArgs e)`
- `protected override void OnClosed(EventArgs e)`
- `protected override bool ProcessCmdKey(ref Message msg, Keys keyData)`
- `private void UpdateUndoRedo()`
- `private void UndoRedo_Updated(object sender, EventArgs e)`
- `private void fileNewMenuItem_Click(object sender, EventArgs e)`
- `private void fileOpenMenuItem_Click(object sender, EventArgs e)`
- `private void fileSaveMenuItem_Click(object sender, EventArgs e)`
- `private void fileSaveAsMenuItem_Click(object sender, EventArgs e)`
- `private void fileExportMenuItem_Click(object sender, EventArgs e)`
- `private void fileExitMenuItem_Click(object sender, EventArgs e)`
- `private void editUndoMenuItem_Click(object sender, EventArgs e)`
- `private void editRedoMenuItem_Click(object sender, EventArgs e)`
- `private void settingsMapSettingsMenuItem_Click(object sender, EventArgs e)`
- `private void settingsTeamTypesMenuItem_Click(object sender, EventArgs e)`
- `private void settingsTriggersMenuItem_Click(object sender, EventArgs e)`
- `private void Mru_FileSelected(object sender, FileInfo e)`
- `private void mapPanel_MouseMove(object sender, MouseEventArgs e)`
- `private bool LoadFile(string loadFilename)`
- `private bool SaveFile(string saveFilename)`
- `private void RefreshAvailableTools()`
- `private void ClearActiveTool()`
- `private void RefreshActiveTool()`
- `private void clampActiveToolForm()`
- `private void ActiveToolForm_ResizeEnd(object sender, EventArgs e)`
- `private void Triggers_CollectionChanged(object sender, System.Collections.Specialized.NotifyCollectionChangedEventArgs e)`
- `private void mainToolStripButton_Click(object sender, EventArgs e)`
- `private void UpdateVisibleLayers()`
- `private void viewLayersMenuItem_CheckedChanged(object sender, EventArgs e)`
- `private void toolTabControl_Selected(object sender, TabControlEventArgs e)`
- `private void developerGenerateMapPreviewMenuItem_Click(object sender, EventArgs e)`
- `private void developerGoToINIMenuItem_Click(object sender, EventArgs e)`
- `private void developerGenerateMapPreviewDirectoryMenuItem_Click(object sender, EventArgs e)`
- `private void developerDebugShowOverlapCellsMenuItem_CheckedChanged(object sender, EventArgs e)`
- `private void filePublishMenuItem_Click(object sender, EventArgs e)`
- `private void mainToolStrip_MouseMove(object sender, MouseEventArgs e)`
- `private void MainForm_FormClosing(object sender, FormClosingEventArgs e)`
- `private bool PromptSaveMap()`



## Dependencies and Rust-port considerations

- The editor is a separate C# WinForms application, not part of the native C++ game runtime.
- The map model and renderer encode editor semantics that should be compared against Rust map/theatre/render crates, not blindly translated as UI code.
- UI event handlers, undo/redo, tool/plugin lifecycle, file serialization, and Steam Workshop/UGC behavior are distinct responsibilities.
- No Rust parity is claimed by this page; equivalent paths and behavioral tests must be identified explicitly.

## Outstanding audit

- [ ] Exact class/member/field/enum inventories from full declarations.
- [ ] Every method's full signature and implementation explanation.
- [ ] UI event flows, model mutation, invalidation, undo/redo, and serialization side effects.
- [ ] Cross-file dependencies and plugin/tool contracts.
- [ ] Rust mapping and test evidence.


## Tool-dialog routing (bounded call-site audit)

The pinned RefreshActiveTool implementation (MainForm.cs lines 940–1074) creates the tool form and the matching editing tool together, passes the dialog's controls into the tool constructor, assigns activeToolForm, and calls activeToolForm.Show(this).

| ToolType | Dialog | Consumer and source data |
|---|---|---|
| Map | TemplateToolDialog | TemplateTool receives its ListView and preview MapPanel |
| Smudge / Overlay / Wall | GenericToolDialog | SmudgeTool, OverlaysTool, WallsTool receive TypeComboBox and preview; MainForm filters the plugin type collections |
| Resources | ResourcesToolDialog | ResourcesTool receives count label, brush-size NumericUpDown, and Gems checkbox |
| Terrain | TerrainToolDialog(plugin) | TerrainTool receives theatre-filtered terrain types, preview, and TerrainProperties |
| Infantry / Unit / Building | ObjectToolDialog(plugin) | respective tool receives filtered type selector, preview, and ObjectProperties |
| Waypoint | WaypointsToolDialog | WaypointsTool receives a ComboBox bound to waypoint names |
| CellTrigger | CellTriggersToolDialog | CellTriggersTool receives a ComboBox bound to trigger names |

The generic selector is implemented by [TypeComboBox](Controls/TypeComboBox.cs.md); its owner-draw and thumbnail fallback behavior is documented there. The GenericToolDialog designer contains a resource-manager type mismatch: it requests its fallback image from TemplateToolDialog resources even though the key is in GenericToolDialog.resx. This can null out MissingThumbnail and fail when a selected type has no thumbnail. See [GenericToolDialog](Tools/Dialogs/GenericToolDialog.md).

All seven tool dialogs restore and persist one shared user setting, ToolDialogPosition (default point 0,0). Every OnMove saves synchronously. This means one dialog's last position becomes the initial position of all seven, and settings persistence is on the UI move path. The setting declaration is in Properties/Settings.Designer.cs; a separate settings-file audit remains open.

The cell-trigger and waypoint placement consumers are documented in [PlacementTools](Tools/PlacementTools.md). This bounded call-site pass does not complete MainForm: other handlers, the remaining tool implementations, lifecycle edges, and UI tests remain outstanding.
