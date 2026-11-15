# WaypointsToolDialog — source guide

**Status:** code-behind Body reviewed; designer Header reviewed; .resx Resource reviewed.

- [Code-behind](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/WaypointsToolDialog.cs) — blob 455e407ac1f03eeb9261d172aa51d7e44484f983
- [Designer](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/WaypointsToolDialog.Designer.cs) — blob e3e592d7648b0bdca57c08f2d2e2cefd46da1a9d
- [Resource file](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/WaypointsToolDialog.resx) — blob 1af7de150c99c12dd67a509fe57c10d63e4eeb04

## Code-behind

Partial WaypointsToolDialog : Form exposes ComboBox WaypointCombo. Constructor initializes the designer and restores the shared ToolDialogPosition. OnMove(EventArgs) saves the new location to that shared setting.

## Generated designer

Fields: IContainer components, TableLayoutPanel tableLayoutPanel4, Label label1, ComboBox waypointCombo. The combo is dock-filled, DropDownList, DisplayMember Name, and ValueMember Type. The form is auto-sized, FixedToolWindow, iconless, and absent from the taskbar. MainForm binds a string array of waypoint names, while WaypointsTool consumes SelectedIndex to address the waypoint array; the display/value member names are not the actual identity contract.

## ResX

Only the four stock ResX sample rows are present; no custom payload or designer resource lookup exists.

## Port notes

Bind the selector to stable waypoint numbers/IDs rather than array position and display text. Define empty-waypoint-set/no-selection behavior: the current tool indexes map.Waypoints[waypointCombo.SelectedIndex] without a range check. No Rust waypoint selector or placement-mode UI is implemented in map-editor-model.
