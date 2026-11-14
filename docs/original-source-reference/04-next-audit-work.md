# Documentation completion plan

## Current tranche

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
