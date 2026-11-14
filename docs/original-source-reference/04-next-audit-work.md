# Documentation completion plan

## Current tranche

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
