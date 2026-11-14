# TIBERIANDAWN/INI.CPP

**Review status:** Reviewed at the scenario-loading responsibility/function-index level.  
**Upstream:** [TIBERIANDAWN/INI.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/INI.CPP)

## Purpose

This file reads and writes scenario INI data and performs scenario setup tasks. It is a bridge between the text-based scenario description and the live game world. It is not the low-level binary map-image decoder.

## Functions listed by the original source

| Function | Plain-English responsibility |
|---|---|
| `Assign_Houses` | Assigns houses/factions to players in multiplayer scenarios. |
| `Clear_Flag_Spots` | Removes flag overlays from the map. |
| `Clip_Move` | Moves from a cell in a direction while keeping the result inside the map. |
| `Clip_Scatter` | Chooses a scattered nearby cell but prevents leaving the map. |
| `Create_Units` | Creates starting infantry and vehicles for the relevant multiplayer setup. |
| `Furthest_Cell` | Finds the cell furthest from a set of other cells. |
| `Place_Flags` | Places multiplayer flag objects/overlays. |
| `Read_Scenario_Ini` | Reads scenario settings and reconstructs scenario state. |
| `Remove_AI_Players` | Removes AI houses and their units for the applicable game setup. |
| `Scan_Place_Object` | Searches around a requested cell for a legal nearby placement. |
| `Set_Scenario_Name` | Builds the scenario's display/name string. |
| `Sort_Cells` | Orders cells by distance for placement/search behavior. |
| `Write_Scenario_Ini` | Writes scenario state back to INI format. |

## Relationship to map terrain

Scenario loading coordinates several separate tasks: the binary map image establishes the map's terrain/cell data; template/object INI sections create placed objects; template type metadata resolves names and art; and map operations validate and place those objects. A correct implementation must preserve this order and not treat the INI file as a flat terrain image.

## Rust port mapping

Full scenario INI parsing, object reconstruction, and map integration remain incomplete. Keep this separate from the byte-level TD map decoder.
