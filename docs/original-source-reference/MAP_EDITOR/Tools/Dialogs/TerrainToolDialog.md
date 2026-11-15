# TerrainToolDialog — source guide

**Status:** code-behind Body reviewed; designer Header reviewed; .resx Resource reviewed.

- [Code-behind](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/TerrainToolDialog.cs) — blob 7630be751a6ffee96aa2e76ce8a3cd826c6dfbea
- [Designer](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/TerrainToolDialog.Designer.cs) — blob 4576b98867f5191342cf9f66696843344354ddf1
- [Resource file](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/TerrainToolDialog.resx) — blob 54a26ecb9b4ee21710f946b21856662b8d4bf12b

## Code-behind

Partial TerrainToolDialog : Form exposes TypeComboBox TerrainTypeComboBox, MapPanel TerrainTypeMapPanel, and TerrainProperties TerrainProperties. The parameterless constructor initializes the designer and restores the shared position. TerrainToolDialog(IGamePlugin plugin) : this() calls terrainProperties.Initialize(plugin, true). OnMove(EventArgs) persists the common point and synchronously saves settings.

## Generated designer

Fields: IContainer components, TableLayoutPanel tableLayoutPanel1, MapPanel terrainTypeMapPanel, TypeComboBox terrainTypeComboBox, TerrainProperties terrainProperties. Map preview is dock-filled with MapImage null, MinZoom 1, MaxZoom 8, Quality 1, Zoom 1, ZoomStep 1. The selector displays Name, selects Type, uses OwnerDrawVariable and DropDownList, and requests terrainTypeComboBox.MissingThumbnail. TerrainProperties is docked at the top of the final percentage-height row and starts with Terrain null. The form uses AutoSize/GrowAndShrink and is a sizable tool window.

The resource manager uses typeof(TerrainToolDialog), matching the resource file. MainForm initializes the properties control with the active plugin, filters terrain types to the current theatre, and passes selector, preview, and properties control to TerrainTool.

## ResX

Four stock sample entries plus terrainTypeComboBox.MissingThumbnail occur. The custom entry is a System.Drawing.Bitmap byte-array-base64 payload containing a 32×32 PNG; the key matches the designer. Sample entries are not referenced by the designer.

## Port notes

Keep terrain type/theatre filtering, preview rendering, and property editing separate. The TerrainProperties.Initialize(plugin, true) flag and TerrainTool event/placement behavior still require their own audit. No equivalent terrain UI or thumbnail catalog is in map-editor-model.
