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
| CnCTDRAMapEditor/Tools/WaypointsTool.cs | Select, assign, clear waypoint cells with per-operation undo/redo. | MapDocument has waypoint-number-to-CellCoord storage and CellMetrics, but no UI selection, placement mode, or undo command. 
| CnCTDRAMapEditor/Tools/BuildingTool.cs | Place/remove/move structures, apply two occupancy masks, manage bib smudges and base-building priority order, preview and property editing. | No Building model, occupancy-set equivalent, bib lifecycle, base-priority command, or placement UI found in map-editor-model. 
| CnCTDRAMapEditor/Tools/InfantryTool.cs | Manage InfantryGroup and five stopping slots, nearest-stop selection, add/remove/move, property editing and preview. | No Infantry/InfantryGroup types, stopping-slot model, or group placement commands found. 
| CnCTDRAMapEditor/Tools/UnitTool.cs | Place/remove/move individual units with OccupierSet collision validation and rollback, plus pick/edit/preview. | No Unit entity, occupancy mask, or movement command found. |
| CnCTDRAMapEditor/Tools/ViewTool.cs | Shared ITool lifecycle, map-panel render callbacks, layer toggles, navigation widget, and waypoint/trigger/boundary overlays. | No Rust viewport, layer renderer, navigation adapter, invalidation scheduler, or trigger overlay painter found. |
| CnCTDRAMapEditor/Tools/TerrainTool.cs | Place/remove/move terrain technos with type-specific occupancy masks, transformable icons, trigger properties, preview and undo for add/remove. | CellLayers.terrain stores only an optional numeric ID; no TerrainType catalog, occupancy mask, trigger/icon model, or placement commands. |\n| CnCTDRAMapEditor/Tools/ResourcesTool.cs | Paint/erase Tiberium, gold, and gems with a square brush, adjacency-derived icons, resource totals, and per-stroke undo. | CellLayers.overlay stores only an optional numeric ID; no OverlayType flags, resource values, adjacency resolver, brush command, or stroke undo. |
| CnCTDRAMapEditor/Tools/TemplateTool.cs | Theatre-filtered multi-icon template catalog, IconMask-based placement/removal, per-cell undo, preview selection, and map-boundary resizing. | CellLayers.template stores only an optional numeric ID; no TemplateType/IconMask/icon-index catalog, map-bounds command, or undo adapter. |\n| CnCTDRAMapEditor/Tools/OverlaysTool.cs | Select, place, remove, and preview decorative/crate overlays with category filtering and per-click undo. | CellLayers.overlay stores only an optional numeric ID; no OverlayType flags, placeability policy, or overlay commands. |
| CnCTDRAMapEditor/Tools/SmudgeTool.cs | Place/remove/pick cell smudges while protecting building bib cells; per-click undo. | No typed Smudge cell, bib ownership marker, or smudge commands. |\n| CnCTDRAMapEditor/Tools/WallsTool.cs | Paint/remove wall overlays with dual occupancy checks, connected icons, and per-stroke undo. | No wall overlay type flags, Buildings/Technos wall registration, connectivity resolver, or stroke commands. |

The dialog pass also found a static resource lookup defect in GenericToolDialog: its designer asks TemplateToolDialog's resource manager for a key stored in GenericToolDialog.resx. Fix this before using the original UI as the parity oracle. No GUI test or Rust parity test was executed.


The three object tools are documented in [ObjectPlacementTools.md](MAP_EDITOR/Tools/ObjectPlacementTools.md), with per-file guides for [BuildingTool](MAP_EDITOR/Tools/BuildingTool.cs.md), [InfantryTool](MAP_EDITOR/Tools/InfantryTool.cs.md), and [UnitTool](MAP_EDITOR/Tools/UnitTool.cs.md). All three are Body reviewed only. Their source bodies do not use the inherited UndoRedoList, and the current MapDocument still stores only generic cell layers and waypoints.


The shared renderer and lifecycle base is documented in [ViewTool.cs.md](MAP_EDITOR/Tools/ViewTool.cs.md). It is Body reviewed only; its GDI overlay lifetime and synchronous pre-render work remain explicit porting concerns.


See [TerrainTool.cs.md](MAP_EDITOR/Tools/TerrainTool.cs.md), [ResourcesTool.cs.md](MAP_EDITOR/Tools/ResourcesTool.cs.md), and [TerrainAndResourceTools.md](MAP_EDITOR/Tools/TerrainAndResourceTools.md) for the source-level contracts and identified dirty/undo/resource-recalculation gaps.


See [TemplateTool.cs.md](MAP_EDITOR/Tools/TemplateTool.cs.md), [OverlaysTool.cs.md](MAP_EDITOR/Tools/OverlaysTool.cs.md), and [TemplateAndOverlayTools.md](MAP_EDITOR/Tools/TemplateAndOverlayTools.md) for the full source contracts and porting findings.


See [SmudgeTool.cs.md](MAP_EDITOR/Tools/SmudgeTool.cs.md), [WallsTool.cs.md](MAP_EDITOR/Tools/WallsTool.cs.md), and [SmudgeAndWallsTools.md](MAP_EDITOR/Tools/SmudgeAndWallsTools.md) for bib ownership and wall occupancy/connectivity contracts.


## Additional control references

- **NavigationWidget.cs:** mouse-to-cell/subpixel conversion, pointer-cell events, odd cursor footprint, middle-button panning, and cursor rendering. The current widget can update a subpixel coordinate while retaining a stale cell outside Metrics.
- **TerrainProperties.cs:** trigger list binding, popup lifecycle, trigger collection refresh, and an unwired Obj_PropertyChanged dirty handler.
- **TerrainProperties.Designer.cs:** one-row Trigger label and PropertiesComboBox with SelectedIndexChanged wiring.
- **TerrainProperties.resx:** four resource headers and no data/metadata payload.

See [NavigationWidget.cs.md](MAP_EDITOR/Widgets/NavigationWidget.cs.md) and [TerrainProperties.cs.md](MAP_EDITOR/Controls/TerrainProperties.cs.md) for complete source and resource findings.

See [NavigationWidget.cs.md](MAP_EDITOR/Widgets/NavigationWidget.cs.md) and [TerrainProperties.cs.md](MAP_EDITOR/Controls/TerrainProperties.cs.md) for complete source and resource findings.


- **ObjectProperties.cs:** typed property bindings for Infantry/Unit/Building, mock-object dirty isolation, IsPrebuilt house normalization, and popup event lifetime; the current Rust editor has no equivalent property command/UI adapter.
- **ObjectProperties.Designer.cs / ObjectProperties.resx:** generated control layout and event wiring; the .resx has four headers and no data/metadata payload.
- **PropertiesComboBox.cs:** nested ToolStripDropDown AutoClose coordination; no equivalent Rust popup ownership adapter.
- **TypeItem.cs:** generic display-name/value wrapper used by HouseType and DirectionType selectors; replace with typed selector IDs rather than serializing this UI wrapper.

See [ObjectPropertyEditing.md](MAP_EDITOR/Controls/ObjectPropertyEditing.md), [ObjectProperties.cs.md](MAP_EDITOR/Controls/ObjectProperties.cs.md), [PropertiesComboBox.cs.md](MAP_EDITOR/Controls/PropertiesComboBox.cs.md), and [TypeItem.cs.md](MAP_EDITOR/TypeItem.cs.md).


- **MapPanel.cs / MapPanel.Designer.cs / MapPanel.resx:** camera transforms, zoom anchoring, incremental cell invalidation, render event order, Windows WM_SETREDRAW calls, component-only designer, and one Boolean tray metadata entry. The Rust UI has no equivalent camera/render/invalidation adapter.
- **ImageTooltip.cs:** owner-drawn image tooltip with MaxSize clipping; the current clamp can distort aspect ratio. No Rust image-tooltip adapter is present.
- **MenuButton.cs:** split-button ContextMenuStrip hit testing and arrow rendering. No Rust split-button adapter is present.

See [MapPanel.cs.md](MAP_EDITOR/MapPanel.cs.md), [MapPanel.Designer.cs.md](MAP_EDITOR/MapPanel.Designer.cs.md), [ImageTooltip.cs.md](MAP_EDITOR/Controls/ImageTooltip.cs.md), and [MenuButton.cs.md](MAP_EDITOR/Controls/MenuButton.cs.md).


## Editor map aggregate and spatial model — source-body pass

| Pinned C# source | Verified source contract | Current Rust path/status |
|---|---|---|
| CnCTDRAMapEditor/Model/Map.cs | Aggregate owns sections, type catalogs, cell grids, occupier/overlapper indexes, houses, triggers, teams, resource values, derived overlay icons, bib lifecycle, clone and TGA preview behavior. | crates/map-editor-model/src/lib.rs is only MapDocument/CellLayers/waypoints; no equivalent aggregate. |
| CnCTDRAMapEditor/Model/MapSection.cs | X/Y/Width/Height and Theater with INotifyPropertyChanged; MapContext-scoped string TypeConverter falls back to first theatre on unknown string. | No theatre/bounds section or property-notification adapter. |
| CnCTDRAMapEditor/Model/CellMetrics.cs | Row-major coordinate conversion, eight-direction adjacency, and two margin-aware clipping overloads. | crates/map-editor-model/src/cell_metrics.rs maps dimensions/index/adjacency/basic clipping; margin overloads and exact invalid-dimension behavior differ. |
| CnCTDRAMapEditor/Model/CellGrid.cs | Generic T[,] storage, synchronous CellChanged/Cleared events, copy/intersection/enumeration helpers. | Vec<CellLayers> in MapDocument provides flattened typed layers only; no generic event grid. |
| CnCTDRAMapEditor/Model/OccupierSet.cs | Object-to-anchor dictionary plus per-cell mask index; CanAdd/Add/Remove/Clear and synchronous events. | No occupancy set in map-editor-model. |
| CnCTDRAMapEditor/Model/OverlapperSet.cs | Rectangle index with iterative transitive overlap closure; no clipping or collision rejection. | No overlapper index in map-editor-model. |
| CnCTDRAMapEditor/Model/Template.cs | One-cell ICellOccupier instance with TemplateType and Icon. | CellLayers.template is only Option<u16>; no typed icon instance or mask catalog integration. |

See [MapModel-Contracts.md](MAP_EDITOR/MapModel-Contracts.md) and the per-file guides in MAP_EDITOR. Source files are Body reviewed where marked in the inventory; none of these model files is marked Rust-mapped, Tested, or Complete by this documentation pass.


## Editor type definitions and catalogs

| Original source | Verified responsibility | Current Rust location / status |
|---|---|---|
| CnCTDRAMapEditor/Model/BuildingType.cs | Building identity, localized name, owner/theatre/fake/turret/factory metadata, base/full occupancy masks, bib expansion and preview. | No typed BuildingType/catalog in map-editor-model; CellLayers stores numeric IDs only. |
| CnCTDRAMapEditor/Model/DirectionType.cs | Byte ID, name and FacingType with cross-type equality. | No DirectionType catalog or selector adapter in map-editor-model. |
| CnCTDRAMapEditor/Model/HouseType.cs | House ID/name, unit/building team colors and per-type color overrides. | No HouseType catalog or color resolver. |
| CnCTDRAMapEditor/Model/InfantryType.cs | Infantry identity, owner, tile-index-4 lookup, center-slot preview. | No typed InfantryType/InfantryGroup catalog. |
| CnCTDRAMapEditor/Model/OverlayType.cs | Resource/wall/crate/flag bitfields, placeability rule and one-cell occupancy. | CellLayers.overlay is only an optional numeric ID; no typed flags/catalog. |
| CnCTDRAMapEditor/Model/SmudgeType.cs | Logical size, render size, bib bit flags and theatre image fitting. | No typed SmudgeType catalog or bib metadata. |
| CnCTDRAMapEditor/Model/TemplateType.cs | Multi-icon dimensions, row-major tile lookup, [x,y] IconMask and template flags. | CellLayers.template is only an optional numeric ID; no editor IconMask catalog. crates/rts-core/src/red_alert_template.rs ports part of the Red Alert engine TemplateClass lifecycle, not this C# editor registry. |
| CnCTDRAMapEditor/Model/TerrainType.cs | Occupancy mask, render-derived overlap rectangle, transformable/OreMine tile selection. | CellLayers.terrain is only an optional numeric ID; no TerrainType catalog. |
| CnCTDRAMapEditor/Model/TheaterType.cs | Theatre identity and ordered tileset-name sequence. | No theatre catalog or tileset source abstraction. |
| CnCTDRAMapEditor/Model/UnitType.cs | sbyte ID masks for aircraft/vessels, owner/turret/fixed-wing metadata, occupancy and overlap footprints. | No typed UnitType catalog or variant ID-mask mapping. |
| CnCTDRAMapEditor/Interface/IBrowsableType.cs | DisplayName and Image Thumbnail getters. | No UI-facing browsable-type trait; avoid System.Drawing in the core model. |
| CnCTDRAMapEditor/Interface/ICellOccupier.cs | Two-dimensional occupancy-mask getter. | No mask-backed occupancy abstraction. |
| CnCTDRAMapEditor/Interface/ICellOverlapper.cs | Integer-space overlap rectangle getter. | No overlapper spatial index. |
| CnCTDRAMapEditor/Interface/ITechnoType.cs | sbyte ID and Name getters. | No typed techno definition identity trait. |

| CnCTDRAMapEditor/RedAlert/DirectionTypes.cs | 16-entry RA direction byte catalog, canonical Facing values and reflection enumeration. | No Rust direction catalog; body reviewed. |
| CnCTDRAMapEditor/RedAlert/HouseTypes.cs | 20 RA houses, colors and faction/base-player helper. | No typed RA house/color catalog; body reviewed. |
| CnCTDRAMapEditor/RedAlert/InfantryTypes.cs | 26 RA infantry IDs, tokens, text keys and owner strings. | No typed RA infantry catalog; body reviewed. |
| CnCTDRAMapEditor/RedAlert/UnitTypes.cs | 22 ground + 7 air + 7 vessels; encoded sbyte masks and flags. | No typed RA unit catalog; body reviewed. |
| CnCTDRAMapEditor/TiberianDawn/DirectionTypes.cs | 16-entry TD direction catalog, same encoded values as RA. | No Rust direction catalog; body reviewed. |
| CnCTDRAMapEditor/TiberianDawn/HouseTypes.cs | 10 TD houses, split BadGuy colors and harv/mcv overrides. | No typed TD house/color catalog; body reviewed. |
| CnCTDRAMapEditor/TiberianDawn/InfantryTypes.cs | 20 TD infantry; field E7 aliases Name token e6. | No typed TD infantry catalog; body reviewed. |
| CnCTDRAMapEditor/TiberianDawn/UnitTypes.cs | 22 ground + 5 air; no vessels. | No typed TD unit catalog; body reviewed. |

See [TypeCatalog-Contracts.md](MAP_EDITOR/TypeCatalog-Contracts.md) and [TypeCatalog-Interfaces.md](MAP_EDITOR/TypeCatalog-Interfaces.md). The current Rust crate has no typed equivalents, so none of these sources is Rust-mapped. Eighteen TD/RA registry bodies are Body reviewed (nine classes per variant). TD/RA TemplateTypes are Partially reviewed for IDs0–159; remaining declarations and full initializer/consumer/serializer parity remain open. Catalog validation, initialization parity and tests are pending.


## Static registry body review — Tranche 6S-A

| Source | Review evidence | Rust status |
|---|---|---|
| TD/RA BuildingTypes.cs | Complete catalog fields, masks, reflection and GetTypes documented. | No typed BuildingType catalog. |
| TD/RA OverlayTypes.cs | IDs, text keys, flags, theatre restrictions and consumers documented. | CellLayers.overlay remains numeric-only. |
| TD/RA SmudgeTypes.cs | Identical 15-entry catalogs, logical sizes and bib flags documented. | No typed SmudgeType/bib model. |

See [Variant-Static-Catalogs-6S-A.md](MAP_EDITOR/Variant-Static-Catalogs-6S-A.md) and the six per-file guides. Arrays are reflection-derived without explicit sort; GetTypes returns array-backed sequences. Map.InitTheater mutates shared render metadata. Concurrency/stale-state behavior needs runtime tests.

## Static registry body review — Tranche 6S-B

| Source | Review evidence | Rust status |
|---|---|---|
| TD/RA TerrainTypes.cs | Complete 32/35 catalogs, masks, theatre lists, transformable state, flags, reflection and GetTypes documented. | No typed terrain catalog, masks, theatre restrictions or render cache. |
| TD/RA TheaterTypes.cs | Three theatres per variant, ordered terrain/common tilesets, reflection/GetTypes; TD duplicate ID2 recorded. | No typed theatre catalog or tileset source abstraction. |

See [Variant-Terrain-Theater-Catalogs-6S-B.md](MAP_EDITOR/Variant-Terrain-Theater-Catalogs-6S-B.md) and four per-file guides. Map.InitTheater initializes all terrain entries before UI theatre filtering. Missing tile lookup leaves TerrainType metadata unchanged. Body-reviewed only; not Rust-mapped/Tested/Complete


## TemplateTypes partial catalog review — Tranche 6S-C-A

TD and RA TemplateTypes are **Partially reviewed**: IDs0–31 documented (32 entries each), static constructor/GetTypes inspected, totals 216/401 established. Full catalogs and Rust mapping remain open. See [Variant-TemplateTypes-6S-C-A.md](MAP_EDITOR/Variant-TemplateTypes-6S-C-A.md). TD Water entries lack Water flag; RA Water/Water2 carry it. TemplateType.Init derives masks from successful tile lookup; TemplateTool excludes Clear and missing-thumbnail types. Serialization call-site traces are in the comparison; plugin files remain not fully body-reviewed.

## TemplateTypes partial catalog review — Tranche 6S-C-B

TD/RA IDs32–63 documented in [comparison](MAP_EDITOR/Variant-TemplateTypes-6S-C-B.md) and per-variant guides. Cumulative slices0–63 catalog-documented; source paths remain Partially reviewed. TD: Slope20–38, Shore32/33, desert Shore20–23, Brush1–7. RA: Shore30–56, ShoreCliff01–05. All Flag=None. IDs64+ and consumer parity open; no Rust mapping/tests.


## TemplateTypes partial catalog review — Tranche 6S-C-C

TD/RA declarations IDs64–95 documented in [the comparison](MAP_EDITOR/Variant-TemplateTypes-6S-C-C.md) and per-variant guides. Cumulative coverage IDs0–95 (96 each), full totals TD216/RA401. TD theatre variation and RA ShoreCliff06–37 dimensions/tokens recorded. Both rows remain Partially reviewed; complete body/dependency review, Rust mapping and tests open. Next IDs96–127.


## TemplateTypes partial catalog review — Tranche 6S-C-D

TD/RA declarations IDs96–127 are documented in [the comparison](MAP_EDITOR/Variant-TemplateTypes-6S-C-D.md) and per-variant guides. Cumulative coverage is IDs0–127 (128/216 TD; 128/401 RA). TD IDs96–127 are Road4–Road35, tokens d04–d35, all three theatres. RA IDs96–127 cover ShoreCliff38, Boulder1–6, selected patches, River01–13 and Falls1/1a/2, all Temperate/Snow. RA Boulder4–6 (IDs100–102) and Patch15 (ID111) have source dimensions 0×0; Map.InitTheater iterates all template types and TemplateType.Init requests a zero-sized Bitmap. Exact runtime exception was not executed. Both registry files remain Partially reviewed, not Rust-mapped/Tested/Complete. The existing rts-core Red Alert TemplateClass module is a runtime object port, not the editor TemplateType catalog.


## TemplateTypes partial catalog review — Tranche 6S-C-E

TD/RA declarations IDs128–159 are documented in [the comparison](MAP_EDITOR/Variant-TemplateTypes-6S-C-E.md) and per-variant guides. Cumulative coverage is IDs0–159 (160/216 TD; 160/401 RA). TD IDs128–135 are Road36–Road43 (all three theatres); IDs136–148 are River1–13 (Winter/Temperate); IDs149–159 are River14–24 (Desert-only). RA IDs128–134 are Falls2a/fords/bridges and IDs135–159 are Slope01–25, all Temperate/Snow. All entries default to TemplateTypeFlag.None through the verified five-argument constructor. No 0×0 declarations occur in this slice. Both rows remain Partially reviewed; IDs160+ and full dependency/serializer review remain open. No Rust mapping or tests.
