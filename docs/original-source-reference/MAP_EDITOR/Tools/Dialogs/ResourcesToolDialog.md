# ResourcesToolDialog — source guide

**Status:** code-behind Body reviewed; designer Header reviewed; .resx Resource reviewed.

- [Code-behind](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/ResourcesToolDialog.cs) — blob 56d2f196447691a32b02f0aec5b4a849ea7f3192
- [Designer](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/ResourcesToolDialog.Designer.cs) — blob 900a03a6794fafafd5aa0a46cb3e0fdcdde7d6be
- [Resource file](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/ResourcesToolDialog.resx) — blob 1af7de150c99c12dd67a509fe57c10d63e4eeb04

## Code-behind

Partial ResourcesToolDialog : Form exposes Label TotalResourcesLbl, NumericUpDown ResourceBrushSizeNud, and CheckBox GemsCheckBox. Constructor initializes the designer and restores the shared ToolDialogPosition. OnMove(EventArgs) persists the shared point and synchronously saves user settings.

## Generated designer

Fields: IContainer components, TableLayoutPanel tableLayoutPanel6, Label label10, Label totalResourcesLbl, CheckBox gemsCheckBox, NumericUpDown resourceBrushSizeNud, Label label1.

The layout shows “Total Resources:” with initial value “0”, a Brush Size control, and a Gems checkbox. Brush size has Minimum 1, Maximum 9, Increment 2, initial Value 1. The checkbox is not initialized as checked. The form is a sizable tool window without control box, maximize/minimize buttons, icon, or taskbar entry. No designer event handlers are attached; MainForm passes these controls into ResourcesTool, whose event/mutation behavior remains a separate audit.

## ResX

Only the four stock Visual Studio sample entries are present: Name1, Color1, Bitmap1, and Icon1. No custom key is requested by the designer; the sample payloads are not live resource dependencies.

## Port notes

Model brush-size constraints explicitly (1–9, step 2, default 1) and define the Gems toggle semantics in the ResourcesTool audit. Keep resource painting and map mutation outside the widget. No Rust resource-painting dialog/tool adapter was found in map-editor-model.
