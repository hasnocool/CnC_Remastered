# Tool-dialog source audit

Pinned upstream: [editor tool dialogs](https://github.com/electronicarts/CnC_Remastered_Collection/tree/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Tools/Dialogs).

This tranche inspected all seven tool-dialog code-behind files, their seven generated designers and seven linked .resx files, the MainForm tool-construction call sites, TypeComboBox, and the CellTriggersTool/WaypointsTool consumers.

## MainForm routing

| Tool | Dialog | Data supplied | Consumer |
|---|---|---|---|
| Map | [TemplateToolDialog](TemplateToolDialog.md) | ListView and preview MapPanel | TemplateTool |
| Smudge | [GenericToolDialog](GenericToolDialog.md) | Non-Bib smudge types | SmudgeTool |
| Overlay | [GenericToolDialog](GenericToolDialog.md) | Placeable overlays valid for current theatre | OverlaysTool |
| Resources | [ResourcesToolDialog](ResourcesToolDialog.md) | Count label, brush size, gems checkbox | ResourcesTool |
| Terrain | [TerrainToolDialog](TerrainToolDialog.md) | Plugin initialization and current-theatre terrain types | TerrainTool |
| Infantry | [ObjectToolDialog](ObjectToolDialog.md) | Plugin initialization and infantry types | InfantryTool |
| Unit | [ObjectToolDialog](ObjectToolDialog.md) | Plugin initialization and non-fixed-wing unit types | UnitTool |
| Building | [ObjectToolDialog](ObjectToolDialog.md) | Plugin initialization and theatre-compatible building types | BuildingTool |
| Wall | [GenericToolDialog](GenericToolDialog.md) | Overlay types flagged as walls | WallsTool |
| Waypoint | [WaypointsToolDialog](WaypointsToolDialog.md) | Waypoint names | WaypointsTool |
| Cell trigger | [CellTriggersToolDialog](CellTriggersToolDialog.md) | Trigger names | CellTriggersTool |

MainForm creates each dialog, passes its exposed controls to the tool constructor, assigns the form to activeToolForm, and shows it with MainForm as owner. The tool windows are a WinForms host contract, not game-runtime APIs.

## Shared location persistence

All seven constructors restore Location from Properties.Settings.Default.ToolDialogPosition. All seven OnMove overrides call base, assign the new Location to that same setting, and call Settings.Default.Save(). The setting is a single user-scoped System.Drawing.Point with default 0, 0; it is not keyed by dialog type. Moving one dialog therefore changes the restored position for every tool dialog.

Save is called synchronously for every move event. A Rust UI should intentionally preserve the shared-versus-per-window policy, but avoid synchronous disk writes on every pointer movement: coalesce/debounce updates and serialize persistence away from the input/render thread.

## Resource findings

All seven .resx files contain ResX 2.0 headers and four stock Visual Studio sample rows: Name1 (string plus sample comment), Color1 (System.Drawing.Color), Bitmap1 (binary-base64 sample), and Icon1 (System.Drawing.Icon byte-array-base64 sample). These sample rows are not referenced by their generated designers.

Three files additionally contain a 32×32 PNG fallback:
- GenericToolDialog.resx: templateTypeComboBox.MissingThumbnail
- ObjectToolDialog.resx: objectTypeComboBox.MissingThumbnail
- TerrainToolDialog.resx: terrainTypeComboBox.MissingThumbnail

Object and Terrain designers create their resource manager for their own dialog type and request the matching key. Generic creates ComponentResourceManager(typeof(TemplateToolDialog)) even though the key is in GenericToolDialog.resx, not TemplateToolDialog.resx. The lookup therefore targets the wrong resource set and returns null. That null overwrites TypeComboBox.MissingThumbnail's default error-icon image. TypeComboBox.Types, OnMeasureItem, and OnDrawItem dereference MissingThumbnail.Height/Width when a type has no thumbnail, so this can become a null-reference failure for a no-thumbnail entry. Correct the resource-manager type to GenericToolDialog and test a type with Thumbnail == null.

CellTriggers, Resources, Template, and Waypoints .resx files have no custom resources and no designer resource lookups.

## Rust boundary

The current crates/map-editor-model provides MapDocument, CellLayers, CellMetrics, and object-selection planning. It does not provide tool windows, thumbnail selectors, settings persistence, placement-mode event routing, or CellTrigger/Waypoint undo commands. Implement these as UI-host adapters over typed model commands, not inside the byte-format codec or simulation crates.

## Status and limitations

The seven code-behind files are Body reviewed; the seven generated designers are Header reviewed; the seven .resx files are Resource reviewed. TypeComboBox.cs and the two placement-tool source files are Body reviewed, and TypeComboBox.Designer.cs is Header reviewed. None is claimed Rust-mapped, Tested, or Complete. Broader tool implementations, ViewTool, other controls, and runtime UI tests remain open.


## Object placement consumers

The shared ObjectToolDialog also hosts three independently audited placement tools:
- [BuildingTool](../BuildingTool.cs.md): multi-cell structure occupancy, building bibs, and base-priority normalization.
- [InfantryTool](../InfantryTool.cs.md): one-cell InfantryGroup with five stopping slots.
- [UnitTool](../UnitTool.cs.md): single Unit occupiers with occupancy-checked drag rollback.

See the [object placement comparison](../ObjectPlacementTools.md) for MainForm filters, inherited ViewTool rendering, lifecycle differences, and Rust command requirements. These source files are Body reviewed; none is Rust-mapped or parity-tested.
