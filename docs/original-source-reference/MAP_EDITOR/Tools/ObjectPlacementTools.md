# Object placement tools — shared lifecycle and cross-tool contracts

Pinned upstream: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.  
This page compares the fully inspected BuildingTool, InfantryTool, and UnitTool bodies. Individual behavior is documented in [BuildingTool](BuildingTool.cs.md), [InfantryTool](InfantryTool.cs.md), and [UnitTool](UnitTool.cs.md).

## MainForm construction and tool lifecycle

MainForm.RefreshActiveTool creates an ObjectToolDialog and passes its selector, preview MapPanel, and ObjectProperties control to each object tool. The catalogs differ:
- Infantry: every plugin.Map.InfantryTypes item, ordered by Name.
- Unit: UnitTypes filtered to not IsFixedWing, then ordered by Name.
- Building: BuildingTypes with no theatre restriction or a theatre list containing the active map theatre, ordered by IsFake then Name.

MainForm assigns the constructed tool to activeTool, assigns the dialog to activeToolForm, and shows the form with MainForm as owner. ClearActiveTool calls activeTool.Dispose(), clears it, detaches the form resize handler, closes the form, and clears the status label.

ITool is only IDisposable plus MapLayerFlag Layers { get; set; }. ViewTool supplies the shared implementation:
- retains plugin, map, mapPanel, statusLbl, url, and NavigationWidget;
- subscribes MapPanel.PreRender/PostRender and Map.BasicSection.PropertyChanged;
- invalidates RenderMap when Layers changes;
- renders through MapRenderer in the pre-render callback and draws waypoint, trigger, cell-trigger, and boundary overlays in the post-render callback;
- disposes NavigationWidget and removes its shared subscriptions.

The three derived tools override RenderMap to return a cloned previewMap. Their PreRenderMap methods clone the entire map on each non-empty map-panel pre-render and add a translucent placement ghost to the clone. This is source behavior, not a claim that Map.Clone is cheap.

## Behavior comparison

| Concern | BuildingTool | InfantryTool | UnitTool |
|---|---|---|---|
| Stored map object | Building in Buildings and Technos | Infantry members inside one InfantryGroup in Technos | Unit in Technos |
| Placement default direction | North | South | North |
| Default mission | Not assigned by tool | Guard, else first mission | Guard, else first mission |
| Mouse drag handler | NavigationWidget.MouseCellChanged | MapPanel.MouseMove | NavigationWidget.MouseCellChanged |
| Drag identity | Preserves click pivot inside building footprint | Moves one member between five stopping slots | Moves whole Unit between cells |
| Destination acceptance | BaseOccupyMask precheck plus Buildings.Add | First free stopping slot in target group | Technos.Add / Unit OccupyMask |
| Failed move | Restores old building location | Leaves member in original group unless destination slot found | Restores old Unit location |
| Placement preview | Building ghost on cloned map | Ghost member in cloned/new InfantryGroup | Unit ghost on cloned map |
| Pick copies | Type, House, Strength, Direction, Trigger, BasePriority, IsPrebuilt, Sellable, Rebuild | Type, House, Strength, Direction, Mission, Trigger | Type, House, Strength, Direction, Mission, Trigger |
| UndoRedoList use in class | None | None | None |
| Popup closed in Dispose | No | No | Yes |
| Model PropertyChanged handler detached | No | No | No |

All three classes subscribe model PropertyChanged handlers when an existing object is opened in ObjectPropertiesPopup. Their popup Closed callbacks refresh NavigationWidget but do not remove the model handler. None removes those per-object handlers in Dispose. UnitTool closes its current popup during disposal; BuildingTool and InfantryTool do not.

## Occupancy model that must survive the port

Building has two distinct placement collections/masks. BuildingTool checks Technos.CanAdd using BuildingType.BaseOccupyMask and then calls Buildings.Add, whose OccupierSet validation uses Building.OccupyMask. Map's Buildings.OccupierAdded handler adds the Building to Technos using BaseOccupyMask and creates bib smudges; Buildings.OccupierRemoved removes bib smudges and the corresponding Technos entry.

Unit is a single ICellOccupier in Technos; OccupierSet.Add validates all cells in UnitType.OccupyMask. A failed move is rolled back to the old location.

InfantryGroup is one Technos occupier with a 1×1 OccupyMask and a fixed array of Globals.NumInfantryStops. Its OverlapBounds is (-1,-1,3,3). Infantry members are slots within the group, not independent map occupiers. ClosestStoppingTypes orders the center and four quarter-tile positions by squared distance from MouseSubPixel.

## Cross-tool correctness findings

1. **Undo/redo gap:** all three constructors receive UndoRedoList through ViewTool, but none of the three class bodies reads url or records commands. The direct mutations in these classes do not themselves supply undo/redo parity. Verify whether the Rust editor should intentionally improve this behavior; if so, add commands and tests rather than silently copying the gap.
2. **Event lifetime:** per-object PropertyChanged subscriptions are not detached on popup close or tool disposal. Model-to-view callbacks can outlive the active tool. Use scoped subscriptions or explicit unsubscribe on popup close and disposal.
3. **GDI lifetime:** each derived PostRenderMap creates a Pen on every render without disposing it. The shared ViewTool post-render also creates drawing resources. Add repeated-render resource tests or use deterministic disposable scopes in the original implementation.
4. **Catalog assumptions:** all three constructors use Types.First(), Houses.First(), and a matching direction First(). Unit's MainForm filter excludes fixed-wing types; Building's theatre filter can remove every type. Empty catalogs or incomplete map registries can throw before the dialog becomes usable.
5. **Synchronous preview work:** ViewTool invokes PreRenderMap from MapPanel_PreRender; the derived methods clone the full map and create GDI previews synchronously. A Rust UI should publish immutable model snapshots, process input as short commands, and render transient ghosts without cloning the complete document on each paint.
6. **Tool-specific selection differences:** Infantry pick/select/double-click inspect only the nearest stop, while add/remove/drag scan nearest-first for a matching free/occupied slot. Preserve this if strict behavioral compatibility is required.

## Rust mapping as verified on target main

Current target source: [crates/map-editor-model/src/lib.rs](https://github.com/hasnocool/CnC_Remastered/blob/main/crates/map-editor-model/src/lib.rs), blob ab12b41d5d7bf64f970dae290456377e864eba6c.

MapDocument currently stores variant, dimensions, a Vec<CellLayers> (template, terrain, overlay, smudge IDs), and a BTreeMap of waypoint numbers to CellCoord. It provides checked cell access/update and waypoint insertion with bounds/duplicate checks. The inspected file contains no Building, Unit, InfantryGroup, House, mission, direction, trigger, base priority, bib, occupancy mask, placement command, popup, preview, or undo/redo implementation.

Therefore the three source files are **Body reviewed**, but none is Rust-mapped, Tested, or Complete. The existing generic document model is a starting point only.

## Recommended command and test boundary

Implement typed model commands independently of the desktop toolkit:
- PlaceBuilding / RemoveBuilding / MoveBuilding, including both masks, bib generation/removal, and base-priority normalization.
- PlaceUnit / RemoveUnit / MoveUnit with occupancy validation and rollback.
- AddInfantry / RemoveInfantry / MoveInfantry with a fixed five-slot group and stable nearest-stop ordering.
- UpdateObjectProperties and PickObject as explicit state transitions.
- Undo/redo records containing before/after values and map identity/version checks.

Test at minimum:
- every map edge for each occupancy mask, including multi-cell buildings;
- blocked and out-of-bounds move rollback;
- building bib lifecycle and base-priority renumbering;
- Unit occupancy masks and type restrictions;
- five infantry stop positions, tie ordering, full-group behavior, and removal of empty groups;
- property edits after popup close/tool replacement;
- repeated render cycles for GDI/resource growth in the legacy UI and bounded render work in the Rust UI;
- command undo/redo restores byte-identical relevant map state.

No tests were run in this documentation tranche.
