# ObjectToolDialog — source guide

**Status:** code-behind Body reviewed; designer Header reviewed; .resx Resource reviewed.

- [Code-behind](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/ObjectToolDialog.cs) — blob 9ad499130a1eb41415bb80c16750806a3c9bd3c9
- [Designer](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/ObjectToolDialog.Designer.cs) — blob b30168aa416fcb864b148616c83487a4ddb40b4a
- [Resource file](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/ObjectToolDialog.resx) — blob 17708545dcc61f2ec48998940100017c35e86ac8

## Code-behind

Partial ObjectToolDialog : Form exposes TypeComboBox ObjectTypeComboBox, MapPanel ObjectTypeMapPanel, and ObjectProperties ObjectProperties. The parameterless constructor initializes controls and restores the shared tool-dialog position. ObjectToolDialog(IGamePlugin plugin) : this() calls objectProperties.Initialize(plugin, true). OnMove(EventArgs) persists the shared point and calls Settings.Default.Save().

## Generated designer

Fields: IContainer components, TableLayoutPanel tableLayoutPanel1, MapPanel objectTypeMapPanel, TypeComboBox objectTypeComboBox, ObjectProperties objectProperties. The three-row layout places selector first, preview second, and object-properties control in the final fill row. MapPanel starts with MapImage null, MinZoom 1, MaxZoom 8, Zoom 1, ZoomStep 1, Quality 1. TypeComboBox is owner-drawn, DropDownList, displays Name, selects Type, and loads objectTypeComboBox.MissingThumbnail. ObjectProperties starts with Object null.

The resource manager uses typeof(ObjectToolDialog), matching the resource file. MainForm constructs this dialog for Infantry, Unit, and Building, calls the plugin overload, filters the relevant type list, and passes all three controls into the selected tool.

## ResX

Besides the four stock sample entries (Name1, Color1, Bitmap1, Icon1), the file contains objectTypeComboBox.MissingThumbnail as a System.Drawing.Bitmap byte-array-base64 payload. The custom PNG is 32×32 and its key matches the designer. The four sample entries are not referenced by the designer.

## Port notes

Keep common object-property initialization as a separate plugin-dependent step; the meaning of the boolean argument belongs to the ObjectProperties.Initialize audit. Use typed object-kind/type IDs, a preview-rendering service, and a property-inspector model. No corresponding desktop UI or verified editor-object property parity is present in map-editor-model.
