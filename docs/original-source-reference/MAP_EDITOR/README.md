# Original C# map editor source guide

The original editor is under `CnCTDRAMapEditor/`. It is a separate C# Windows Forms application, not part of either game's C++ runtime DLL.

## Major areas

| Folder | Responsibility (high-level; detailed audit pending) |
|---|---|
| `Controls/` | Reusable UI controls for map panels, settings, object/terrain properties, and editor widgets. |
| `Dialogs/` | Modal workflows such as map creation/settings, team types, and Steam-related dialogs. |
| `Model/` | Editor-side map/scenario data model. |
| `RedAlert/` | Red Alert-specific editor integration and rules. |
| `TiberianDawn/` | Tiberian Dawn-specific editor integration and rules. |
| `Tools/` | Editing tools and user operations on map content. |
| `Utility/` | Shared editor helpers. |
| `Interface/` | Editor interface abstractions. |
| `Render/` | Map drawing/preview support. |
| `Properties/` and `Resources/` | Application settings, embedded resources, and generated metadata. |
| `Steamworks.NET/` | Steamworks interop/wrapper code included by the editor. |

## Documentation rule

Designer-generated files and resource files still receive a role description, but they should not be treated as handwritten game logic. For each handwritten editor class, document its events, data model, user interactions, and serialization effects.

The editor's data model should be compared with the game runtime's map classes; do not assume the editor model exactly matches the runtime object graph.


## Plugin, serialization, and Workshop boundary

The common host contract is [IGamePlugin.cs](IGamePlugin.cs.md). Its concrete implementations are game-specific:

- [Red Alert GamePlugin](RedAlert/GamePlugin.cs.md) — INI/MEG/PGM dispatch, team/trigger/object records, LCW MapPack and OverlayPack.
- [Tiberian Dawn GamePlugin](TiberianDawn/GamePlugin.cs.md) — paired INI/BIN data, per-cell two-byte binary map records, and TD-specific team/trigger records.
- [Shared INI utility](Utility/INI.cs.md) — case-insensitive ordered sections, reflection converters, and unknown-section preservation.
- [SteamSection](SteamSection.cs.md), [SteamworksUGC](Utility/SteamworksUGC.cs.md), and [SteamDialog](Dialogs/SteamDialog.cs.md) — Workshop metadata, callback state machine, and publish UI.
- [Serialization contract comparison](Serialization-Contract.md) — exact cross-variant differences and required parity tests.

The current Rust map-editor-model crate is still an initial document model. It does not yet implement either C# game plugin, the INI codec, the TD binary sidecar, the RA LCW map sections, or Workshop integration.


## Tool windows and placement controls

The tool-window layer is now documented in [Tools/Dialogs](Tools/Dialogs/README.md), with individual guides for CellTriggersToolDialog, GenericToolDialog, ObjectToolDialog, ResourcesToolDialog, TemplateToolDialog, TerrainToolDialog, and WaypointsToolDialog. [TypeComboBox](Controls/TypeComboBox.cs.md) documents the owner-drawn type/thumbnail selector. [PlacementTools](Tools/PlacementTools.md) documents the CellTriggersTool and WaypointsTool event, model-mutation, and undo paths.

The GenericToolDialog designer has a resource-manager type mismatch that can null its missing-thumbnail fallback; the issue and regression-test requirement are recorded in the source guide. The tool windows share one persisted location setting and currently save synchronously on every move event. These UI details are not implemented by the current portable map-editor-model crate.


## Property editing and selector controls

- [ObjectProperties](Controls/ObjectProperties.cs.md) — Infantry, Unit, and Building property binding, dirty tracking, mock-object isolation, and popup lifecycle.
- [PropertiesComboBox](Controls/PropertiesComboBox.cs.md) — nested dropdown AutoClose coordination.
- [TypeItem<T>](TypeItem.cs.md) — display-name/value adapter for typed selector entries.
- [Object property editing overview](Controls/ObjectPropertyEditing.md) — control relationships and porting findings.
- [TerrainProperties](Controls/TerrainProperties.cs.md) — terrain trigger binding and the unwired dirty-notification handler.
- [NavigationWidget](Widgets/NavigationWidget.cs.md) — mouse-to-cell/subpixel conversion and pan state.


## Viewport and utility controls

- [MapPanel](MapPanel.cs.md) — camera transforms, zoom anchoring, incremental invalidation, render lifecycle, and Windows redraw suspension.
- [MapPanel designer/resources](MapPanel.Designer.cs.md) — component-only designer and the single Boolean tray metadata entry.
- [ImageTooltip](Controls/ImageTooltip.cs.md) — owner-drawn image tooltip and aspect-ratio behavior.
- [MenuButton](Controls/MenuButton.cs.md) — split-button menu dispatch and rendering.


## Map aggregate and spatial model contracts

- [Map model contracts](MapModel-Contracts.md) — Map aggregate ownership, cell indexing, occupancy masks, overlap closure, update batching, derived resource/wall icons, building bibs, and clone semantics.
- [Map.cs](Map.cs.md) — full local Map aggregate method bodies and event wiring.
- [MapSection.cs](MapSection.cs.md) — theatre conversion, map bounds fields, and property notifications.
- [CellMetrics.cs](CellMetrics.cs.md) — row-major coordinates, adjacency, and clipping.
- [CellGrid.cs](CellGrid.cs.md) — generic evented cell storage and enumeration.
- [OccupierSet.cs](OccupierSet.cs.md) — mask-based object occupancy and event order.
- [OverlapperSet.cs](OverlapperSet.cs.md) — rectangle overlap index and transitive closure.
- [Template.cs](Template.cs.md) — per-cell template instance; distinct from multi-cell TemplateType.IconMask.

The current Rust map-editor-model crate covers only the document/layer/waypoint foundation and part of CellMetrics. It does not yet reproduce the C# aggregate's occupancy, overlap, trigger, bib, batching, invalidation, or typed catalog behavior. Source-body review does not establish parity.


## Remaining map/scenario model records

- [CellTrigger](CellTrigger.cs.md) — per-cell trigger-name reference and its separate cleanup behavior.
- [Overlay](Overlay.cs.md) and [Smudge](Smudge.cs.md) — placed overlay/smudge records, icon/data/tint fields, and type-catalog boundary.
- [Waypoint](Waypoint.cs.md) — named waypoint, player-start flag, nullable cell, and partial Rust coordinate mapping.
- [Trigger](Trigger.cs.md) — trigger event/action pairs, defaults, deep clone, and Building reference cleanup.
- [TeamType](TeamType.cs.md) — team composition, byte-sized limits, mission sequence, and mixed clone semantics.

See [Map model contracts](MapModel-Contracts.md) for how these records participate in the Map aggregate. Their source bodies are reviewed, but no Rust parity is claimed.


## Type definitions and variant catalogs

The model's definition/catalog layer is now documented separately from placed map instances:

- [Type catalog contracts](TypeCatalog-Contracts.md) — ten definition classes, exact IDs/flags, mask and overlap differences, image initialization, equality behavior, and Rust gaps.
- [Type catalog interfaces](TypeCatalog-Interfaces.md) — exact IBrowsableType, ICellOccupier, ICellOverlapper and ITechnoType declarations.
- Per-file guides: [BuildingType](BuildingType.cs.md), [DirectionType](DirectionType.cs.md), [HouseType](HouseType.cs.md), [InfantryType](InfantryType.cs.md), [OverlayType](OverlayType.cs.md), [SmudgeType](SmudgeType.cs.md), [TemplateType](TemplateType.cs.md), [TerrainType](TerrainType.cs.md), [TheaterType](TheaterType.cs.md), and [UnitType](UnitType.cs.md).

### TD/RA static registries — Tranche 6R

- [Registry comparison](Variant-Registry-Catalogs.md) — catalog sizes, category IDs, house mappings, alias details and initialization behavior.
- RA: [DirectionTypes](RedAlert/DirectionTypes.cs.md), [HouseTypes](RedAlert/HouseTypes.cs.md), [InfantryTypes](RedAlert/InfantryTypes.cs.md), [UnitTypes](RedAlert/UnitTypes.cs.md).
- TD: [DirectionTypes](TiberianDawn/DirectionTypes.cs.md), [HouseTypes](TiberianDawn/HouseTypes.cs.md), [InfantryTypes](TiberianDawn/InfantryTypes.cs.md), [UnitTypes](TiberianDawn/UnitTypes.cs.md).

All eight registry source bodies are Body reviewed. The six other registry classes per variant remain open; initialization and parity are not claimed.


The ten definition bodies are Body reviewed; the four small interfaces are Header reviewed. The TD/RA static registry data, ordering, all text keys, theatre filters and Init lifecycle are not yet exhaustively reviewed. No typed Rust catalog mapping or parity tests are claimed.
