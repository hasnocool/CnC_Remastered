# CellTriggersToolDialog — source guide

**Status:** code-behind Body reviewed; designer Header reviewed; .resx Resource reviewed.

- [Code-behind](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/CellTriggersToolDialog.cs) — blob f8e00aaee09c2913372aa9001cdef7c585846e45
- [Designer](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/CellTriggersToolDialog.Designer.cs) — blob 5e1dae25c7cc66e7715804c8c71b366b1ec3d0e8
- [Resource file](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/CellTriggersToolDialog.resx) — blob 1af7de150c99c12dd67a509fe57c10d63e4eeb04

## Code-behind

Namespace MobiusEditor.Tools.Dialogs; partial class CellTriggersToolDialog : Form.

- public ComboBox TriggerCombo => triggerCombo exposes the generated combo to MainForm and CellTriggersTool.
- public CellTriggersToolDialog() calls InitializeComponent(), then restores Properties.Settings.Default.ToolDialogPosition.
- protected override void OnMove(EventArgs e) calls base.OnMove(e), writes Location to the shared setting, and synchronously calls Settings.Default.Save().

There is no validation or fallback for the persisted point.

## Generated designer

Fields: IContainer components, TableLayoutPanel tableLayoutPanel4, Label label1, ComboBox triggerCombo. Dispose(bool) disposes components when disposing is true and the container is non-null, then calls base disposal.

InitializeComponent builds a two-column table with label “Trigger” and the combo. The combo is dock-filled and DropDownList; no designer event handler is attached. The form is auto-sized, has no control box, uses FixedToolWindow, disables maximize/minimize, hides its icon and taskbar entry, starts at the manually assigned location, and is titled “Cell Triggers”.

MainForm binds plugin.Map.Triggers.Select(t => t.Name).ToArray() and passes the combo to CellTriggersTool. The consumer's placement and undo behavior is in [PlacementTools](../PlacementTools.md).

## ResX

Only ResX 2.0 headers and four stock Visual Studio sample entries occur: Name1 (string with sample comment), Color1 (System.Drawing.Color), Bitmap1 (binary-base64 sample), and Icon1 (System.Drawing.Icon byte-array-base64 sample). No key is requested by this designer; these rows are not live UI assets.

## Port notes

Use a typed trigger identifier/name binding rather than relying on ComboBox string selection. Define empty-trigger-list and no-selection behavior explicitly: the current tool can create a CellTrigger whose Trigger is null if the combo has no selected item. Keep model mutations and undo state on the editor-model thread, while UI invalidation is scheduled by the host. No equivalent trigger-placement UI or parity test was found in map-editor-model.
