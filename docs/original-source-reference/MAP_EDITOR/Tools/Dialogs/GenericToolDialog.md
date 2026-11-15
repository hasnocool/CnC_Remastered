# GenericToolDialog — source guide

**Status:** code-behind Body reviewed; designer Header reviewed; .resx Resource reviewed.

- [Code-behind](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/GenericToolDialog.cs) — blob 4a2e22e7e508ad5ba0b65e0ecaecdf28ac9d27c7
- [Designer](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/GenericToolDialog.Designer.cs) — blob 587d5c988eeab3cb078219e2eec4f058a8251e75
- [Resource file](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/GenericToolDialog.resx) — blob a67669d25b6490b1b93602632fcfdce38740a533

## Code-behind

Partial GenericToolDialog : Form exposes TypeComboBox GenericTypeComboBox and MapPanel GenericTypeMapPanel. The parameterless constructor initializes the designer and restores the shared ToolDialogPosition. OnMove(EventArgs) calls base, writes the shared location, and synchronously saves user settings.

## Generated designer

Fields: IContainer components, TableLayoutPanel tableLayoutPanel1, MapPanel genericTypeMapPanel, TypeComboBox genericTypeComboBox. The table has one percentage-width column and two rows; selector above map preview. The map panel is dock-filled with MinZoom 1, MaxZoom 8, Quality 1, Zoom 1. The combo uses DisplayMember Name, ValueMember Type, OwnerDrawVariable, and DropDownList.

The generated code constructs ComponentResourceManager(typeof(TemplateToolDialog)), then assigns resources.GetObject("templateTypeComboBox.MissingThumbnail") to genericTypeComboBox.MissingThumbnail. The Generic dialog resource file contains that key, but TemplateToolDialog.resx does not. Static lookup therefore targets the wrong resource base name and returns null. The generated control names are also template-derived (templateTypeMapPanel and templateTypeComboBox) despite generic-named fields.

The form is an iconless, non-taskbar sizable tool window titled “Map”. MainForm reuses it for Smudge, Overlay, and Wall, changes its title, and supplies filtered type sequences. It is not the Map/Template dialog.

## ResX

Four stock sample entries (Name1, Color1, Bitmap1, Icon1) plus templateTypeComboBox.MissingThumbnail occur. The custom entry is a System.Drawing.Bitmap byte-array-base64 payload containing a 32×32 PNG. It is intended as the fallback for types without thumbnails, but the designer queries the Template dialog resource set instead of this file.

## Runtime consequence and porting

TypeComboBox defaults MissingThumbnail to SystemIcons.Error.ToBitmap(), but the designer assignment overwrites it with the failed lookup. Its Types setter and owner-draw/measure paths dereference MissingThumbnail for entries without a type thumbnail. Fix the manager type and test a no-thumbnail entry before considering this control behavior mapped.

Rust should separate the generic type catalog/thumbnail model from the UI widget and keep selected type as a typed ID. Current map-editor-model has no equivalent selector, preview widget, or fallback-thumbnail contract.
