# Tiberian Dawn source guide

Tiberian Dawn's original game code lives under `TIBERIANDAWN/`. It includes the simulation, world/map classes, scenario parsing, units, buildings, interface, file/archive access, and a `WIN32LIB` support layer.

## Terrain/map files reviewed in this tranche

- [MAP.CPP](MAP.CPP.md) and [MAP.H](MAP.H.md): map grid and map runtime.
- [TEMPLATE.CPP](TEMPLATE.CPP.md) and [TEMPLATE.H](TEMPLATE.H.md): individual template objects on the map.
- [CDATA.CPP](CDATA.CPP.md): static template-type definitions and theatre asset initialization.
- [INI.CPP](INI.CPP.md): scenario INI parsing and object placement.
- [MIXFILE.CPP](MIXFILE.CPP.md) and [MIXFILE.H](MIXFILE.H.md): archive lookup and file retrieval.

The complete path inventory is in [02-original-file-inventory.md](../02-original-file-inventory.md). This guide will grow to cover every source/header pair and all remaining files. The map and template files must be read together; none is a standalone “terrain atlas” implementation.
