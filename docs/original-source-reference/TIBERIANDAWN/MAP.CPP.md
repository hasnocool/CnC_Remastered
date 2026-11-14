# Tiberian Dawn MAP.CPP — source guide

> **Audit status: partially reviewed.** Grounded in the pinned upstream translation unit and its original function index. Exact header members, signatures, full function-body explanations, call graph, and Rust parity remain pending.

- Source: [TIBERIANDAWN/MAP.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/MAP.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 76,464 characters.

## Plain-English purpose

Implements tactical-map runtime operations. This first pass records the original function index and direct dependencies; each map operation's state changes and interactions must be expanded from its body.

## Direct compile-time dependencies

- `function.h`

## Original source function index

- MapClass::Cell_Distance — Determines the distance between two cells.
- MapClass::Cell_Region — Determines the region from a specified cell number.
- MapClass::Cell_Threat — Gets a houses threat value for a cell
- MapClass::Close_Object — Finds a clickable close object to the specified coordinate.
- MapClass::In_Radar — Is specified cell in the radar map?
- MapClass::Init — clears all cells
- MapClass::Logic — Handles map related logic functions.
- MapClass::One_Time — Performs special one time initializations for the map.
- MapClass::Overlap_Down — computes & marks object's overlap cells
- MapClass::Overlap_Up — Computes & clears object's overlap cells
- MapClass::Overpass — Performs any final cleanup to a freshly constructed map.
- MapClass::Pick_Up — Removes specified object from the map.
- MapClass::Place_Down — Places the specified object onto the map.
- MapClass::Place_Random_Crate — Places a crate at random location on map.
- MapClass::Read_Binary — reads the map's binary image file
- MapClass::Set_Map_Dimensions — Initialize the map.
- MapClass::Sight_From — Mark as visible the cells within a specified radius.
- MapClass::Validate — validates every cell on the map
- MapClass::Write_Binary — writes the map's binary image file

These are original source index descriptions, not a substitute for reading each body. Do not infer complete branch behavior, return contracts, or side effects from names alone.

## Outstanding symbol and implementation audit

- [ ] Classes, structs, enums, aliases, macros, globals, constants.
- [ ] Every field/member with type, visibility, initialization, and purpose.
- [ ] Exact function signatures, overloads, parameters, and return values.
- [ ] Every function body: branches, loops, state changes, side effects, and failure paths.
- [ ] Important callers, callees, shared state, and ownership assumptions.
- [ ] Red Alert counterpart differences.
- [ ] Rust path and test mapping, with implemented/partial/absent status.
