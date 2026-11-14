# Tiberian Dawn source guide

Tiberian Dawn's original game code lives under `TIBERIANDAWN/`. It includes the simulation, world/map classes, scenario parsing, units, buildings, interface, file/archive access, and a `WIN32LIB` support layer.

## Runtime and map files reviewed in these tranches

- [Core object hierarchy](CORE-OBJECT-HIERARCHY.md)
- [OBJECT.CPP](OBJECT.CPP.md), [TECHNO.CPP](TECHNO.CPP.md), [FOOT.CPP](FOOT.CPP.md)
- [CELL.CPP](CELL.CPP.md), [BUILDING.CPP](BUILDING.CPP.md), [UNIT.CPP](UNIT.CPP.md), [INFANTRY.CPP](INFANTRY.CPP.md), [AIRCRAFT.CPP](AIRCRAFT.CPP.md), [BULLET.CPP](BULLET.CPP.md)
- [HOUSE.CPP](HOUSE.CPP.md), [TRIGGER.CPP](TRIGGER.CPP.md), [TERRAIN.CPP](TERRAIN.CPP.md), [OVERLAY.CPP](OVERLAY.CPP.md), [DISPLAY.CPP](DISPLAY.CPP.md), [SCENARIO.CPP](SCENARIO.CPP.md), [EVENT.CPP](EVENT.CPP.md)

## Terrain/map files reviewed in this tranche

- [MAP.CPP](MAP.CPP.md) and [MAP.H](MAP.H.md): map grid and map runtime.
- [TEMPLATE.CPP](TEMPLATE.CPP.md) and [TEMPLATE.H](TEMPLATE.H.md): individual template objects on the map.
- [CDATA.CPP](CDATA.CPP.md): static template-type definitions and theatre asset initialization.
- [INI.CPP](INI.CPP.md): scenario INI parsing and object placement.
- [MIXFILE.CPP](MIXFILE.CPP.md) and [MIXFILE.H](MIXFILE.H.md): archive lookup and file retrieval.

The complete path inventory is in [02-original-file-inventory.md](../02-original-file-inventory.md). This guide will grow to cover every source/header pair and all remaining files. The map and template files must be read together; none is a standalone “terrain atlas” implementation.
