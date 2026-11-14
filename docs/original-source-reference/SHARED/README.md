# Shared graphics and asset-format source guide

The two game variants each include a `WIN32LIB` tree. The code has shared lineage, but the active structure definitions and compilation behavior can differ. Compare the actual TD and RA files before treating a format as common.

## Reviewed

- [Tiberian Dawn ICONSET.CPP](ICONSET.CPP.md): IFF icon-set loading.
- [Tiberian Dawn TILE.H](TILE.H.md): TD icon-set control structure.
- [Red Alert TILE.H comparison](TILE-H-REDALERT.md): RA control structure differences.

## Decode pipeline

1. MIX archive manager retrieves the packed file.
2. IFF parser identifies the icon-set form and its chunks.
3. Icon-set loader interprets image dimensions/bitplanes and the image payload.
4. Transparency data is retained separately.
5. Template map data is retained separately and interpreted by the game variant's template registry.
6. The theatre/template system resolves the map's logical tile reference to the correct frame.

Do not flatten the data into an atlas before interpreting the metadata.
