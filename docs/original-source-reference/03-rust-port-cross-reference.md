# Original source to Rust port cross-reference

This page maps the first audited original files to the current Rust workspace. The mapping is a navigation aid, not proof of complete parity. Consult [PORTING_STATUS.md](../../PORTING_STATUS.md) and [CPP_PORT_INVENTORY.md](../CPP_PORT_INVENTORY.md) for the authoritative implementation status.

| Original file | Original responsibility | Current Rust location / status |
|---|---|---|
| `TIBERIANDAWN/MAP.CPP` | Map grid lifecycle, map binary read/write, placement/overlap, visibility, validation, map logic. | `crates/map-editor-model` only contains an initial editor model; runtime map behavior is not ported. |
| `TIBERIANDAWN/MAP.H` | Declares map class and its public state/operations. | No complete runtime `MapClass` equivalent yet. |
| `TIBERIANDAWN/TEMPLATE.CPP` | Runtime template objects placed on map cells and serialized through scenario INI. | Not fully ported. |
| `TIBERIANDAWN/CDATA.CPP` | Static template type definitions and theatre-specific graphics initialization. | No complete theatre/template registry yet. |
| `TIBERIANDAWN/INI.CPP` | Scenario INI loading and scenario object placement. | Scenario parsing/assembly remains incomplete. |
| `TIBERIANDAWN/MIXFILE.CPP/H` | Legacy MIX archive caching, lookup, retrieval, and offsets. | `crates/rts-formats/src/mix.rs` (partial; see port status). |
| `REDALERT/MAP.CPP` | Red Alert map runtime, map binary read/write, zones, bridge operations, placement and visibility. | No complete Red Alert runtime map implementation yet. |
| `REDALERT/CDATA.CPP` | Red Alert static type tables, template definitions, theatre and template initialization. | No complete theatre/template registry yet. |
| `TIBERIANDAWN/WIN32LIB/ICONSET.CPP` | Reads IFF icon sets and exposes image/map/transparency data. | `crates/rts-formats/src/iconset.rs` (initial implementation; variant layout audit required). |
| `TIBERIANDAWN/WIN32LIB/TILE.H` | Tiberian Dawn icon-set control structure and loader declarations. | `crates/rts-formats/src/iconset.rs`; do not serialize a native C struct as a universal cross-variant header. |
| `REDALERT/WIN32LIB/TILE.H` | Red Alert icon-set control structure, including map width/height fields. | `crates/rts-formats/src/iconset.rs`; Red Alert metadata needs explicit support. |
| `CnCTDRAMapEditor/Model/` | Editor-side map representation and related models. | `crates/map-editor-model/` is only an initial subset. |

## Required architecture direction

Keep these as separate responsibilities in Rust:

- `rts-formats`: byte-level parsing of MIX, IFF, icon sets, maps, palettes, and related legacy formats.
- Theatre/template catalog: theatre identity, suffixes, supported template types, template dimensions, and icon-set associations.
- Map document: decoded map cells and object/layer references, with TD and RA decoders kept distinct.
- Terrain resolver: converts a map's template ID + icon index into the actual tile image and occupancy behavior.
- Renderer: draws already-resolved terrain and layers; it should not guess template identity from atlas coordinates.


## Map-editor plugin and serialization findings

| Original file | Responsibility established from source | Current Rust location / status |
|---|---|---|
| CnCTDRAMapEditor/Interface/IGamePlugin.cs | Shared editor plugin contract, file/game enums, map/image/dirty state, load/save and disposal boundary. | No matching plugin trait or lifecycle implementation found. |
| CnCTDRAMapEditor/TiberianDawn/GamePlugin.cs | TD scenario load/save, INI plus per-cell binary map data, object/team/trigger sections. | Not ported; map-editor-model is only an initial generic document model. |
| CnCTDRAMapEditor/RedAlert/GamePlugin.cs | RA scenario load/save, INI records, LCW MapPack/OverlayPack, team/trigger schemas and preview output. | Not ported; no RA editor serializer or LCW section codec in the current editor model. |
| CnCTDRAMapEditor/Utility/INI.cs | Ordered case-insensitive INI collections, parsing, reflection conversion, and diff model. | No editor INI parser/writer found. rts-core/inibin.rs is an empty-translation-unit marker, not this serializer. |
| CnCTDRAMapEditor/Model/SteamSection.cs | Workshop metadata embedded in map INI. | No Rust equivalent found. |
| CnCTDRAMapEditor/Utility/SteamworksUGC.cs | Steam init/shutdown, callback pump, single active Workshop publish operation. | No Rust Steamworks service found. |

The current crates/map-editor-model source contains MapDocument, CellLayers, dimensions, and waypoints, plus CellMetrics and object-selection planning. Its tests cover generic cell/waypoint behavior, not original TD/RA INI/BIN/MEG/PGM parity. No test execution or format-parity result is claimed by this audit.


## Map-editor tool UI and placement

| Original source | Responsibility established from source | Current Rust location / status |
|---|---|---|
| CnCTDRAMapEditor/Tools/Dialogs/*.cs and generated designers | WinForms tool-window composition, shared position persistence, selectors, preview panels, property controls. | No Rust desktop tool-dialog layer found; keep UI adapters separate from map-format parsing. |
| CnCTDRAMapEditor/Controls/TypeComboBox.cs | IBrowsableType binding, owner-drawn names, thumbnail measurement/drawing, missing-thumbnail fallback. | No Rust type catalog/thumbnail widget; map-editor-model has no UI layer. |
| CnCTDRAMapEditor/Tools/CellTriggersTool.cs | Shift-to-paint cell-trigger placement, pick/remove, drag-batched undo/redo. | No CellTrigger collection or placement command in map-editor-model. |
| CnCTDRAMapEditor/Tools/WaypointsTool.cs | Select, assign, clear waypoint cells with per-operation undo/redo. | MapDocument has waypoint-number-to-CellCoord storage and CellMetrics, but no UI selection, placement mode, or undo command. |\n| CnCTDRAMapEditor/Tools/BuildingTool.cs | Place/remove/move structures, apply two occupancy masks, manage bib smudges and base-building priority order, preview and property editing. | No Building model, occupancy-set equivalent, bib lifecycle, base-priority command, or placement UI found in map-editor-model. |\n| CnCTDRAMapEditor/Tools/InfantryTool.cs | Manage InfantryGroup and five stopping slots, nearest-stop selection, add/remove/move, property editing and preview. | No Infantry/InfantryGroup types, stopping-slot model, or group placement commands found. |\n| CnCTDRAMapEditor/Tools/UnitTool.cs | Place/remove/move individual units with OccupierSet collision validation and rollback, plus pick/edit/preview. | No Unit entity, occupancy mask, or movement command found. |

The dialog pass also found a static resource lookup defect in GenericToolDialog: its designer asks TemplateToolDialog's resource manager for a key stored in GenericToolDialog.resx. Fix this before using the original UI as the parity oracle. No GUI test or Rust parity test was executed.


The three object tools are documented in [ObjectPlacementTools.md](MAP_EDITOR/Tools/ObjectPlacementTools.md), with per-file guides for [BuildingTool](MAP_EDITOR/Tools/BuildingTool.cs.md), [InfantryTool](MAP_EDITOR/Tools/InfantryTool.cs.md), and [UnitTool](MAP_EDITOR/Tools/UnitTool.cs.md). All three are Body reviewed only. Their source bodies do not use the inherited UndoRedoList, and the current MapDocument still stores only generic cell layers and waypoints.
