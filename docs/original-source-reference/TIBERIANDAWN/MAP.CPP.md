# TIBERIANDAWN/MAP.CPP

**Review status:** Reviewed for the map-class responsibility and the functions listed in the source file's own function index.  
**Upstream:** [TIBERIANDAWN/MAP.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/MAP.CPP)  
**Related declaration:** [MAP.H](MAP.H.md)

## Purpose in plain English

This is the Tiberian Dawn world-map manager. It owns the map's cell grid and performs the operations that make cells behave like part of a playable world. It is not merely a file decoder and it is not the template graphics database.

It reads and writes the map's binary image, initializes map dimensions, validates cells, tracks which objects occupy cells, computes overlap, and updates visibility/radar-related state. The scenario loader and template/type code feed information into this system.

## Main class

### `MapClass`

Think of `MapClass` as the manager for the board the RTS simulation runs on. It answers questions about cells, performs world-grid maintenance, and places/removes game objects while keeping occupancy information consistent.

## Functions in the original source index

| Function | Plain-English responsibility |
|---|---|
| `Cell_Distance` | Measures the distance between two map cells using the game's cell-coordinate rules. |
| `Cell_Region` | Converts a cell number into the region it belongs to. |
| `Cell_Threat` | Reads a house's threat value for a cell; used by AI or tactical decisions. |
| `Close_Object` | Finds a nearby object that can be selected/clicked at a screen coordinate. |
| `In_Radar` | Checks whether a cell is inside the radar/minimap coverage area. |
| `Init` | Clears and initializes map cell state. |
| `Logic` | Runs map-specific periodic logic. |
| `One_Time` | Performs map initialization that should happen only once. |
| `Overlap_Down` | Adds an object's footprint to the map's overlap/occupancy tracking. |
| `Overlap_Up` | Removes an object's footprint from overlap tracking. |
| `Overpass` | Performs final cleanup/normalization after map construction. |
| `Pick_Up` | Removes an object from the map's cell occupancy. |
| `Place_Down` | Places an object and updates the cells it occupies. |
| `Place_Random_Crate` | Finds a suitable random cell and places a crate there. |
| `Read_Binary` | Reads the map's binary map image into the map system. |
| `Set_Map_Dimensions` | Initializes the playable/map dimensions and related cell boundaries. |
| `Sight_From` | Marks cells visible within a given radius from a source cell. |
| `Validate` | Checks map cells for invalid or inconsistent state. |
| `Write_Binary` | Writes the map's binary image representation. |

## How it relates to terrain

The map grid stores per-cell terrain/template identity and associated tile selection. `MapClass` provides the world-cell framework and binary map persistence. It does not independently know how every theatre's multi-cell graphic is assembled.

For a correct terrain reconstruction, follow this chain:

1. `MIXFILE.CPP` finds the packed map and graphics resources.
2. `CDATA.CPP` defines the template types available to this game and theatre.
3. `ICONSET.CPP` decodes the template's icon-set file.
4. `TEMPLATE.CPP` handles a template object placed into the scenario/map.
5. `MAP.CPP` owns the world grid and map-cell operations.
6. The renderer uses the resolved map cell and template metadata to draw the correct image.

## Important format distinction

The TD map format must be interpreted using TD's own decoder and source definitions. Do not infer Red Alert's map layout from this file. Red Alert has its own `MAP.CPP` and its own dimensions and additional map behaviors.

## Rust port mapping

The current `crates/map-editor-model` contains initial cell metrics and editor-side model structures, not a full runtime `MapClass`. The full map lifecycle, object occupancy, map binary load/save, visibility, and scenario integration remain separate porting work. See [cross-reference](../03-rust-port-cross-reference.md).

## Evidence

The responsibilities above are based on the source file's function index and implementation in the pinned EA source. The function table is an index-level explanation; individual internal branches and call sites should be expanded in follow-up deep dives.
