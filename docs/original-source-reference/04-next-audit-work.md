# Documentation completion plan

## Current tranche

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

1. Map, template, theatre, icon-set, palette, MIX, IFF, and scenario loading.
2. Core object model: abstract object, techno, cell, unit, infantry, aircraft, vessel, building, bullet, weapon, warhead.
3. Simulation loop, events, triggers, team types, missions, AI, pathfinding, combat, damage, and power.
4. Rendering, animation, sprites, palettes, UI, audio, movies, and input.
5. Save/load, networking, multiplayer, platform APIs, DDE, WOL, and legacy Windows support.
6. C# map editor, its data model, tools, dialogs, resources, and serialization.
7. Build/project configuration, scripts, assembly, and remaining resources.

## Quality gate for a completed page

A page is not complete merely because it names the class. It should explain all important symbols and methods, have links to related files, identify the current Rust mapping, and state what behavior is still missing. When a file has no active behavior in the pinned build, say so and cite the build condition that disables it.
