# TemplateToolDialog — source guide

**Status:** code-behind Body reviewed; designer Header reviewed; .resx Resource reviewed.

- [Code-behind](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/TemplateToolDialog.cs) — blob 54794e6c77149f72110d57bc48697f0838ad69f0
- [Designer](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/TemplateToolDialog.Designer.cs) — blob cd31b3efff193f2206318316d4ea3f4570c86dd6
- [Resource file](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs/TemplateToolDialog.resx) — blob 1af7de150c99c12dd67a509fe57c10d63e4eeb04

## Code-behind

Partial TemplateToolDialog : Form exposes ListView TemplateTypeListView and MapPanel TemplateTypeMapPanel. Constructor initializes the generated layout and restores the shared tool-dialog location. OnMove(EventArgs) calls base, writes the shared location, and synchronously saves settings.

## Generated designer

Fields: IContainer components, SplitContainer splitContainer1, ListView templateTypeListView, MapPanel templateTypeMapPanel. The split container is horizontal: a single-select ListView occupies the upper pane (MultiSelect=false, HideSelection=false), and MapPanel occupies the lower pane. Preview starts with MapImage null, MinZoom 1, MaxZoom 8, Quality 1, Zoom 1, ZoomStep 1. The form is a non-taskbar, iconless sizable tool window titled “Map”.

MainForm uses this dialog only for the Map/Template tool and passes the ListView and preview panel to TemplateTool. TemplateTool populates grouped template entries and renders selected template/icon preview. This dialog does not use TypeComboBox.

## ResX

Only the four standard Visual Studio sample rows (Name1, Color1, Bitmap1, Icon1) are present. There are no custom image resources or designer GetObject lookups.

## Port notes

Preserve single selection, category grouping, preview-map lifecycle, and template icon selection as explicit view-model state. The current map document has no template catalog or preview widget; no UI parity is claimed.
