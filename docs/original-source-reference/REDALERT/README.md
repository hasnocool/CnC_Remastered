# Red Alert source guide

Red Alert's original game implementation lives under `REDALERT/`. It shares a large amount of lineage with Tiberian Dawn, but its map, terrain, type registry, and gameplay code evolved. Treat its files as an independent source of truth.

## Terrain/map files reviewed in this tranche

- [MAP.CPP](MAP.CPP.md) and [MAP.H](MAP.H.md): Red Alert's map grid, zones, bridge behavior, and binary persistence.
- [TEMPLATE.CPP](TEMPLATE.CPP.md): placed template objects.
- [CDATA.CPP](CDATA.CPP.md): Red Alert template type catalogue and theatre-specific metadata.
- [INI.CPP](INI.CPP.md): scenario setup and INI processing.
- [MIXFILE.CPP](MIXFILE.CPP.md) and [MIXFILE.H](MIXFILE.H.md): archive access.

The complete original path inventory is in [02-original-file-inventory.md](../02-original-file-inventory.md). Differences from Tiberian Dawn are recorded explicitly; similar filenames are not proof of identical behavior.
