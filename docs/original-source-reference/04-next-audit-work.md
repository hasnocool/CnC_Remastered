# Documentation completion plan

## Current tranche

### Tranche 6L — template and decorative overlay tools

- Audited the complete pinned TemplateTool.cs (1,009 lines) and OverlaysTool.cs (342 lines) bodies, including catalog construction, preview selection, placement/removal, mouse/keyboard modes, map-bounds resizing, undo/redo, render overlays, and disposal.
- Inspected Template/TemplateType, Overlay/OverlayType, CellGrid, NavigationWidget, Map.Bounds, MainForm tool filters, and the pinned Globals pixel constants.
- Documented TemplateType IconMask semantics, row-major icon selection, partial edge placement, mask-hole behavior, and the difference between whole-template placement (mask-true cells only) and whole-template removal (the full rectangular footprint).
- Documented Ctrl-driven eight-edge/corner map-bound editing, buffered template-cell undo, and the separate map-bounds command.
- Found a map-bounds dirty-state gap, an undisposed template preview NavigationWidget, a Clear fallback assumption (clear1), modifier-release and bounds-mode picking edge cases, and multiple undisposed GDI drawing objects.
- Documented OverlaysTool's placeable category filter, crate-only inclusion, click-only behavior, row-zero/final-row add exclusion, non-wall picking mismatch, and placement-ghost/commit row mismatch.
- Added three source-reference documents and updated the inventory, Rust cross-reference, placement-tool index, and roadmap. Inventory now records 21 legacy Reviewed, 132 Partially reviewed, 11 Header reviewed, 19 Body reviewed, 9 Resource reviewed, and 1,139 Inventory only. No source is claimed Rust-mapped, Tested, or Complete.
- No Rust code was changed and no build, UI test, or parity test was executed.

### Tranche 6K — terrain and resource editing tools

- Audited the complete pinned TerrainTool.cs (456 lines) and ResourcesTool.cs (365 lines) bodies, including constructors, event routing, placement/movement, brush mutation, undo/redo closures, status updates, rendering, and disposal.
- Inspected Terrain/TerrainType, Overlay/OverlayType, TerrainProperties, TerrainToolDialog, ResourcesToolDialog designer constraints, Map resource-adjacency recalculation, and OccupierSet behavior.
- Documented Terrain's use of Map.Technos and OccupyMask, theatre-specific type initialization, transformable icon 22, pivot-preserving drag, and per-operation add/remove undo.
- Found Terrain drag movement does not set plugin.Dirty or register undo/redo; TerrainProperties defines a dirty handler but the inspected code-behind/designer do not subscribe it; Terrain preview dereferences Type.Thumbnail without a null guard.
- Documented ResourcesTool's 1/3/5/7/9 brush sizes, row-zero/final-row add exclusion, resource-only removal, grouped per-stroke undo, first-matching type selection, adjacent-resource icon tables, and Map.TotalResources formula.
- Recorded repeated full resource-total recalculation during brush input, undisposed render Pens, and the difference between Terrain per-operation undo and ResourcesTool per-stroke undo.
- Added three source-reference documents and updated the 1,331-file inventory, Rust cross-reference, placement-tool index, and roadmap. Inventory now records 21 legacy Reviewed, 132 Partially reviewed, 11 Header reviewed, 17 Body reviewed, 9 Resource reviewed, and 1,141 Inventory only. No source is claimed Rust-mapped, Tested, or Complete.
- No Rust code was changed and no build, UI test, or parity test was executed.

### Tranche 6J — shared ViewTool lifecycle and map overlay renderer

- Audited the complete pinned ViewTool.cs body, including ITool implementation, stored fields, Layers property, constructor subscriptions, invalidation, BasicSection change handling, pre/post-render callbacks, every shared overlay branch, and disposal.
- Documented exact rendering behavior for waypoint labels, techno trigger labels (Terrain, Building, Unit, and each InfantryGroup stop), cell-trigger labels, and map boundaries.
- Confirmed MapPanel_PreRender treats a null cell set as a full render, skips a non-null empty set, filters supplied cells through Map.Metrics.Contains, applies nearest-neighbor/high-speed graphics settings only when quality is above one, and invokes MapRenderer synchronously.
- Found shared GDI lifetime gaps: the base post-render overlay branches allocate brushes, pens, and StringFormat objects without deterministic disposal. Derived object tools also allocate pens on each render.
- Recorded that ViewTool captures plugin.Map at construction, stores but does not consume the UndoRedoList, and requires derived tools to unsubscribe their own event handlers.
- Added the ViewTool source guide and updated the 1,331-file inventory, Rust cross-reference, and audit roadmap. Inventory now records 21 legacy Reviewed, 132 Partially reviewed, 11 Header reviewed, 15 Body reviewed, 9 Resource reviewed, and 1,143 Inventory only. No source is claimed Rust-mapped, Tested, or Complete.
- No Rust source was changed and no build, GUI test, or parity test was run.

### Tranche 6I — building, infantry, and unit placement tools

- Audited the complete pinned bodies of BuildingTool.cs (503 lines), InfantryTool.cs (540 lines), and UnitTool.cs (427 lines), including all constructors, state properties, event handlers, placement/removal/movement helpers, preview rendering, post-render overlays, and disposal.
- Traced MainForm.RefreshActiveTool catalog filters and the ViewTool/ITool lifecycle. Inspected the supporting Building, Infantry/InfantryGroup, Unit, Map, and OccupierSet contracts needed to explain occupancy and collection side effects.
- Documented Building's dual Buildings/Technos registration, BaseOccupyMask versus OccupyMask checks, bib smudge lifecycle, and contiguous BasePriority normalization.
- Documented InfantryGroup's five stopping slots, nearest-stop ordering, single-nearest pick/select behavior versus nearest-free add/remove/drag behavior, and member/group back-reference updates.
- Documented Unit placement through OccupierSet.Add and move rollback when the destination is invalid or occupied.
- Found cross-tool gaps: these three classes do not enqueue undo/redo commands despite receiving the shared UndoRedoList; per-object PropertyChanged handlers are not detached; BuildingTool and InfantryTool do not close their property popup during disposal; and each derived PostRenderMap allocates an undisposed Pen per render.
- Recorded unchecked First() constructor assumptions, synchronous full-map cloning in the render path, and the existing Rust model's lack of typed Building/Unit/InfantryGroup/occupancy/command support.
- Added four source-reference documents and updated the cross-reference, tool-dialog index, placement-tool index, inventory, and roadmap. Inventory now records 21 legacy Reviewed, 132 Partially reviewed, 11 Header reviewed, 14 Body reviewed, 9 Resource reviewed, and 1,144 Inventory only. No source is claimed Rust-mapped, Tested, or Complete.
- No Rust code was changed and no build, UI test, or parity test was executed.

### Tranche 6H — tool dialogs, type selector, and placement tools

- Audited all seven tool-dialog code-behind files, all seven generated designers, and all seven linked .resx files. Documented exact exposed controls, constructor/plugin initialization, designer layouts, selector bindings, and resource payload names/types.
- Traced MainForm.RefreshActiveTool call sites for Map, Smudge, Overlay, Resources, Terrain, Infantry, Unit, Building, Wall, Waypoint, and CellTrigger tools, including source-type filters and control handoff.
- Audited TypeComboBox data binding, item measurement, owner-draw, thumbnail fallback, and designer container lifecycle.
- Audited complete CellTriggersTool and WaypointsTool method bodies, including mouse/keyboard event flow, map mutations, dirty-state changes, undo/redo granularity, and event unsubscription.
- Found a concrete GenericToolDialog resource-manager mismatch: its designer requests templateTypeComboBox.MissingThumbnail through TemplateToolDialog resources even though the bitmap is stored in GenericToolDialog.resx. The failed lookup overwrites TypeComboBox's non-null default and can trigger null dereferences for entries without thumbnails.
- Found edge conditions to address in a port: empty type sequences throw in TypeComboBox.Types; CellTriggersTool can construct a trigger with a null name when no selection exists; WaypointsTool indexes the waypoint array without validating SelectedIndex.
- Updated the 1,331-file inventory: 21 legacy Reviewed, 132 Partially reviewed, 11 Header reviewed, 11 Body reviewed, 9 Resource reviewed, 1,147 Inventory only. No files are claimed Rust-mapped, Tested, or Complete.
- Added ten reference documents and updated the editor guide, MainForm guide, Rust cross-reference, and next-work plan. No Rust source was changed and no build/UI/parity test was run.


### Tranche 6G — game plugin contracts, TD/RA serializers, INI codec, and Workshop publishing

- Corrected the IGamePlugin reference with exact FileType/GameType values and every interface member; traced concrete plugin selection and host load/save call sites.
- Added bounded source guides for both game-specific GamePlugin implementations. Recorded variant-specific FileType dispatch, section inventories, team/trigger record differences, TD's interleaved two-byte-per-cell BIN format, and RA's LCW-compressed MapPack/OverlayPack format.
- Added the shared editor INI utility guide: case-insensitive ordered sections, lexical rules, reflection converters, NonSerializedINIKey handling, destructive section extraction, and unknown-section preservation.
- Audited SteamSection, SteamworksUGC, SteamDialog code-behind, generated controls, and linked resx files. Traced the 500 ms callback pump through MainForm and Init/Shutdown through Program.
- Recorded a publish-path risk: SteamDialog ignores the bool returned by plugin.Save before requesting a UGC upload. Its OnLoad also replaces the persisted preview path with a generated minimap or solo banner.
- Updated the 1,331-file inventory with Header reviewed, Body reviewed, and Resource reviewed states. Counts: 21 legacy Reviewed, 132 Partially reviewed, 3 Header reviewed, 1 Body reviewed, 2 Resource reviewed, 1,172 Inventory only. No file is claimed Rust-mapped, tested, or complete by this tranche.
- Added a cross-variant serialization contract page and updated the Rust cross-reference. The current Rust map-editor-model crate remains a generic initial model; no equivalent plugin, INI codec, TD/RA scenario serializer, or Steam UGC service was found.



### Tranche 7E — assembly and low-level ABI inventory

- Added 11 assembly source references covering facing/remap/buffer/XOR-delta/LCW routines and platform support modules.
- Captured visible PROC labels and assembler directives where present; documented required calling-convention, register, bounds, and differential-test audits.
- Marked the paths partially reviewed. This is not a substitute for instruction-level or ABI review.


### Tranche 7D — DirectDraw, buffers, IFF, resource loading, fonts, and input

- Added 12 first-pass guides across both game trees for DDRAW, BUFFER, IFF, LOAD, FONT, KEYBOARD, and MOUSEWW.
- Recorded source function indexes and direct includes; highlighted platform renderer boundaries, resource validation, buffer ownership, and non-blocking input requirements.
- Inventory statuses updated to partially reviewed. Header-level ABI details, function bodies, and Rust parity remain open.


### Tranche 6F — editor type definitions and spatial indexes

- Added ten source guides for BuildingType, CellTrigger, DirectionType, House, HouseType, InfantryType, OccupierSet, OverlapperSet, OverlayType, and SmudgeType.
- Captured detected types, fields, method declarations, namespaces, and critical occupancy/overlap/alliance-mask invariants.
- Marked ten paths partially reviewed; exact full member types, function bodies, serializer contracts, and Rust parity remain open.


### Tranche 6E — editor project and control/dialog implementation

- Added first-pass pages for the editor .csproj, BasicSettings, BriefingSettings, MapPanel, ErrorMessageBox, and InviteMessageBox code-behind/designer files.
- Recorded detected class names, fields, methods, and key viewport/WinForms lifecycle responsibilities.
- Marked ten inventory paths partially reviewed. Designer/resource synchronization, full handler bodies, and Rust UI mapping remain open.


### Tranche 8A — solution, project, runtime configuration, and native resource scripts

- Added 11 reference pages for both Visual Studio solutions, both native .vcxproj files and filters, editor App.config, both game .rc files, and legacy icon resource scripts.
- Captured target architecture/configuration, v141 toolset, Windows SDK target, .NET Framework runtime target, language/resource metadata, and IDE-filter distinction.
- Inventory statuses updated to Reviewed at file-role/configuration declaration level. Full flags/dependency normalization and reproducible-build parity remain open.


### Tranche 7C — Win32 graphics, palette, icon-set, and timer primitives

- Added eight source guides for GBUFFER, ICONSET, PALETTE, and TIMER in both game trees.
- Recorded translation-unit dependencies and source function indexes, with porting concerns for buffer ownership, IFF bounds validation, palette determinism, and timer semantics.
- Updated inventory statuses to partially reviewed; paired-header/member/body audit and Rust parity remain open.


### Tranche 7B — multiplayer, online-service, and settings dialogs

- Added first-pass guides for INTERNET, NETDLG, LOADDLG, and SOUNDDLG in both games.
- Captured direct includes and original function-index entries; called out WOL/legacy Windows integration and separation of UI from serialization/playback.
- Updated inventory statuses to partially reviewed; exhaustive symbols, function bodies, and Rust mappings remain open.


### Tranche 7A — animation, combat, pathfinding, persistence, audio, and MIX archives

- Added 14 first-pass guides across Tiberian Dawn and Red Alert for ANIM, COMBAT, FINDPATH, SAVELOAD, AUDIO, MIXFILE, plus Red Alert WEAPON and WARHEAD.
- Captured each translation unit's direct includes and original function index, and called out important cross-game differences and Rust-port risks.
- Updated the inventory paths to partially reviewed. Paired headers, full method bodies, dependency graphs, and Rust parity remain open.


### Tranche 6D — editor settings, team/trigger dialogs, and Steam publishing UI

- Added first-pass guides for MapSettingsDialog, NewMapDialog, TeamTypesDialog, TriggersDialog, SteamDialog, BasicSettings, PlayerSettings, and TerrainProperties.
- Captured key TD/RA UI differences, cloned working-set semantics, property binding, dirty-state handling, Win32 TreeView interop, and Steam preview/UGC responsibilities.
- Marked eight files partially reviewed; full handlers, designer resources, lifecycle/callback audits, and Rust mappings remain open.


### Tranche 6C — editor scenario and placed-object models

- Added detailed guides for BasicSection, BriefingSection, MapSection, Building, Infantry/InfantryGroup, Unit, Terrain, Overlay, Smudge, and Waypoint.
- Documented concrete fields/properties, converters, constructors, methods, occupancy delegation, notifications, and clone/equality edge cases from source.
- Updated ten inventory entries to Reviewed at source-documentation level; Rust parity remains a separate open task.


### Tranche 5E — Red Alert simulation orchestration

- Added first-pass source guides for Red Alert LOGIC, MISSION, and TEAM.
- Captured pinned source links, direct include dependencies, and original function-index entries.
- Marked the three files partially reviewed; full member/body/call-graph and TD-difference audits remain open.


### Tranche 6B — editor grid primitives and plugin/tool contracts

- Added source guides for CellGrid, CellMetrics, IGamePlugin, and ITool.
- Documented concrete grid indexing, change notifications, adjacency/bounds behavior, and game-plugin/tool contracts from source.
- Marked four files partially reviewed; exhaustive call-site and Rust parity checks remain open.


### Tranche 6A — map editor application/model/rendering entry points

- Added first-pass guides for Program.cs, MainForm.cs, Model/Map.cs, and Render/MapRenderer.cs.
- Recorded startup/resource initialization, main-window orchestration, map model ownership/layer flags, and rendering responsibilities from the actual C# source.
- Marked these four files partially reviewed; full method-body explanations, exhaustive fields, event flow, serialization, and Rust parity remain open.


### Tranche 5D — Tiberian Dawn special actions, superweapons, and targets

- Added first-pass guides for SPECIAL, SUPER, and TARGET with pinned source links, direct includes, and original function-index entries.
- Updated inventory status; complete member inventories, body explanations, call graphs, and Rust parity remain open.


### Tranche 5C — Tiberian Dawn radar, reinforcement, and rules

- Added first-pass guides for RADAR, REINF, and RULES, with source links, direct includes, and function-index entries.
- Updated inventory status; detailed declarations, function-body analysis, call graphs, and Rust mapping remain pending.


### Tranche 5B — Tiberian Dawn team and economy systems

- Added first-pass source guides for TEAM, TEAMTYPE, and POWER, including direct includes and original function-index entries.
- Updated the three source inventory entries to partially reviewed; full declarations, body semantics, call graphs, and Rust mappings remain open.


### Tranche 5A — Tiberian Dawn simulation foundations

- Added source-index guides for LOGIC, MISSION, and MAP from the pinned upstream translation units.
- Captured direct include dependencies and original function-index responsibilities.
- Marked these paths partially reviewed; declarations, fields, full function bodies, call graphs, and Rust mappings remain open.


### Tranche 4 — Tiberian Dawn gameplay and presentation

- Added guides for BUILDING, AIRCRAFT, BULLET, HOUSE, TRIGGER, TERRAIN, OVERLAY, and DISPLAY.
- Marked the eight paths partially reviewed in the complete source inventory.
- Captured the distinction between base template terrain, destructible terrain objects, and overlays, plus the tactical display's role.
- Detailed member/field inventories and implementation-body call graphs remain open.


### Tranche 3 — Red Alert core runtime

- Added source guides for OBJECT, CELL, TECHNO, FOOT, UNIT, INFANTRY, BUILDING, SCENARIO, and EVENT.
- Marked these nine original paths as partially reviewed in the inventory.
- Captured Red Alert-specific behavior such as cell Tiberium growth, navigation queues, anti-unit combat ratings, teleport checks, Tesla charging, variable-sized events, and scenario post-load processing.
- These pages remain partial until declarations, fields, complete function bodies, and call sites are audited.

### Tranche 2 — Tiberian Dawn core runtime

- Added source guides for OBJECT, CELL, TECHNO, FOOT, UNIT, INFANTRY, SCENARIO, and EVENT.
- Added a conceptual object-hierarchy page linking the inheritance/behavior layers.
- Updated the file inventory for these eight files to partially reviewed.
- Function inventories are derived from the original source's own function indexes. These pages are deliberately marked partial until headers, fields, signatures, implementations, and call sites have all been audited.


- Created the reference-library structure.
- Captured all 1,331 original file paths in [the complete inventory](02-original-file-inventory.md).
- Added detailed first-pass guides for the core terrain/map pipeline in both game variants.
- Added an initial cross-reference to the current Rust workspace.

## Remaining work

1. Audit every remaining original file, including all 494 C++ translation units, headers, assembly, C# editor files, project files, resources, and scripts.
2. For every source/header file, enumerate all declared and defined classes, structs, enums, globals, and functions.
3. Explain each function's actual behavior from its implementation, including important branches, state changes, and side effects.
4. Compare paired TD/RA files and record exact differences.
5. Inspect call sites and dependencies so pages explain how functionality is used, not only what the file contains.
6. Map every original behavior to Rust modules, tests, and remaining work.
7. Keep page status and inventory synchronized with [CPP_PORT_INVENTORY.md](../CPP_PORT_INVENTORY.md).

## Priority order

1. Finish the editor plugin/serialization second pass: complete TD/RA GamePlugin bodies, per-section layouts, LCW bounds/error behavior, unknown-section edge cases, and host lifecycle.
2. Continue the remaining C# editor Tools implementations after BuildingTool, InfantryTool, UnitTool, ViewTool, TerrainTool, ResourcesTool, TemplateTool, and OverlaysTool: SmudgeTool, WallsTool, remaining controls (ObjectProperties/TerrainProperties/TypeItem), game-specific type registries, and all remaining .resx files with linked designer usage.
3. Add TD/RA golden INI/BIN/MEG/PGM fixtures and byte-for-byte round-trip tests; map codecs and plugin contracts to typed Rust modules.
4. Map, template, theatre, icon-set, palette, MIX, IFF, and native scenario loading.
5. Core object model: abstract object, techno, cell, unit, infantry, aircraft, vessel, building, bullet, weapon, warhead.
6. Simulation loop, events, triggers, team types, missions, AI, pathfinding, combat, damage, power, rendering, animation, audio, input, networking, and platform integration.
7. Shared libraries, build/project configuration, scripts, assembly, and remaining native source/resources.

## Quality gate for a completed page

A page is not complete merely because it names the class. It should explain all important symbols and methods, have links to related files, identify the current Rust mapping, and state what behavior is still missing. When a file has no active behavior in the pinned build, say so and cite the build condition that disables it. Keep audit depth separate from port evidence: Header reviewed, Body reviewed, and Resource reviewed do not imply Rust-mapped, Tested, or Complete. Complete requires verified declarations, bodies, dependencies/call sites, an identified Rust mapping, and executed parity tests.
