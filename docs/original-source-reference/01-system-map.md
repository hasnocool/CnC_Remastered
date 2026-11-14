# Original game source: system map

The source tree contains two game implementations and a separate map editor. The games share historical code and concepts, but each variant has its own definitions and behavior.

## Main areas

| Original path | Plain-English responsibility |
|---|---|
| `TIBERIANDAWN/` | Tiberian Dawn game logic, map, scenario, units, buildings, UI, audio, networking, and platform integration. |
| `REDALERT/` | Red Alert game logic and its additional/changed systems, including bridges, zones, expanded map behavior, and Red Alert-specific rules. |
| `TIBERIANDAWN/WIN32LIB/` | Low-level graphics, surfaces, palette, icon-set, file, input, and Windows support used by Tiberian Dawn. |
| `REDALERT/WIN32LIB/` | Red Alert's corresponding low-level support library. Do not assume binary layouts match Tiberian Dawn. |
| `CnCTDRAMapEditor/` | C# Windows Forms map editor, including map model, controls, rendering, tools, dialogs, and resources. |
| Root solution/project files | Define how Visual Studio builds the games and editor. |
| `SCRIPTS/` | Repository utility scripts; document separately from game runtime code. |

## Terrain and map loading: end-to-end mental model

1. The game chooses a game variant and theatre.
2. The theatre/type registry determines which template graphics are valid and which theatre-specific archive names to load.
3. MIX archives locate the individual packed files.
4. The icon-set loader decodes the IFF container and keeps image frames, transparency information, and template map metadata distinct.
5. Template type definitions connect a logical template ID to its theatre-specific icon-set data and dimensions.
6. A map cell stores a template identity and a tile/icon index. It does not mean “take the next random rectangle from a flattened atlas.”
7. Template metadata determines how a multi-cell template is composed, including empty positions.
8. The map class builds and validates the world grid, tracks objects and visibility, and reads/writes the variant's map representation.
9. Rendering turns the resolved terrain cells and their layers into pixels.

The critical distinction is between **asset decoding**, **template interpretation**, **map placement**, and **rendering**. Combining these into one atlas operation loses the relationships the original game relies on.

## Important original terrain files

- [Tiberian Dawn MAP.CPP](TIBERIANDAWN/MAP.CPP.md)
- [Tiberian Dawn TEMPLATE.CPP](TIBERIANDAWN/TEMPLATE.CPP.md)
- [Tiberian Dawn CDATA.CPP](TIBERIANDAWN/CDATA.CPP.md)
- [Tiberian Dawn INI.CPP](TIBERIANDAWN/INI.CPP.md)
- [Tiberian Dawn MIXFILE.CPP](TIBERIANDAWN/MIXFILE.CPP.md)
- [Red Alert MAP.CPP](REDALERT/MAP.CPP.md)
- [Red Alert CDATA.CPP](REDALERT/CDATA.CPP.md)
- [Icon-set decoder](SHARED/ICONSET.CPP.md)
- [Tile/icon-set definitions](SHARED/TILE.H.md)

## Variant boundaries that matter

- Tiberian Dawn and Red Alert have different map extents and map data behavior.
- The two `TILE.H` definitions have different active control structures.
- Tiberian Dawn's template type registry supplies template dimensions from its definitions; Red Alert obtains template width/height from icon-set map metadata.
- Red Alert adds map behaviors such as bridge destruction/counting and zone recalculation.
- The map editor has its own data model and serialization assumptions; it is not the game runtime's map class.

These differences must remain explicit in the Rust format and loading layers.
