# Tiberian Dawn RADAR.CPP — source guide

> **Audit status: partially reviewed.** This guide uses the pinned upstream translation unit and its original function index. Exact declarations, all members, full function semantics, callers, and Rust parity remain unaudited.

- Source: [TIBERIANDAWN/RADAR.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/RADAR.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 83,699 characters.

## Plain-English purpose

Implements radar/minimap behavior, including display state and the update/conversion work needed to represent the tactical world in the radar view.

## Direct compile-time dependencies

- `function.h`
- `stdlib.h`

## Original source function index

- Get_Multi_Color — Get the multi color offset number
- RadarClass::AI — Processes radar input (non-tactical).
- RadarClass::Cell_On_Radar — Determines if a cell is currently visible on radar.
- RadarClass::Click_Cell_Calc — Determines what cell the pixel coordinate is over.
- RadarClass::Click_In_Radar — Check to see if a click is in radar map
- RadarClass::Click_In_Radar — Converts a radar click into cell X and Y coordinate.
- RadarClass::Draw_It — Displays the radar map of the terrain.
- RadarClass::Draw_Names — draws players' names on the radar map
- RadarClass::Init_Clear — Sets the radar map to a known state
- RadarClass::Map_Cell — Updates radar map when a cell becomes mapped.
- RadarClass::One_Time — Handles one time processing for the radar map.
- RadarClass::Player_Names — toggles the Player-Names mode of the radar map
- RadarClass::Plot_Radar_Pixel — Updates the radar map with a terrain pixel.
- RadarClass::RadarClass — Default constructor for RadarClass object.
- RadarClass::Radar_Activate — Controls radar activation.
- RadarClass::Radar_Anim — Renders current frame of radar animation
- RadarClass::Radar_Cursor — Adjust the position of the radar map cursor.
- RadarClass::Radar_Pixel — Mark a cell to be rerendered on the radar map.
- RadarClass::Radar_Position — Returns with the current position of the radar map.
- RadarClass::Refresh_Cells — Intercepts refresh request and updates radar if needed
- RadarClass::Render_Infantry — Displays objects on the radar map.
- RadarClass::Render_Overlay — Renders an icon for given overlay
- RadarClass::Render_Terrain — Render the terrain over the given cell
- RadarClass::Set_Map_Dimensions — Sets the tactical map dimensions.
- RadarClass::Set_Radar_Position — Sets the radar map coordinates.
- RadarClass::Set_Radar_Position — Sets the radar position to center around specified cell.
- RadarClass::Set_Tactical_Position — Called when setting the tactical display position.
- RadarClass::Set_Tactical_Position — Called when setting the tactical display position.
- RadarClass::TacticalClass::Action — I/O function for the radar map.
- RadarClass::Zoom_Mode(void) — Handles toggling zoom on the map
- RadarClass::Set_Tactical_Position — Sets the map's tactical position and adjusts radar to
- RadarClass::Coord_To_Radar_Pixel — Converts a coordinate to a radar pixel position

Original index text records intended responsibilities, not verified body behavior.

## Outstanding audit

- [ ] Enumerate all declarations, fields, globals, constants, macros, and exact signatures.
- [ ] Explain every function body, parameter, return value, branch, state change, and side effect.
- [ ] Trace call sites, shared state, and cross-system dependencies.
- [ ] Compare Red Alert counterpart where applicable.
- [ ] Map each behavior to Rust code and tests with explicit parity status.
