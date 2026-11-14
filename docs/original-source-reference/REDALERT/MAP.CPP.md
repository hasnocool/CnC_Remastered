# REDALERT/MAP.CPP

**Review status:** Reviewed for map-class responsibility and the source function index.  
**Upstream:** [REDALERT/MAP.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/MAP.CPP)  
**Related declaration:** [MAP.H](MAP.H.md)

## Purpose

Implements Red Alert's world-map manager. It owns the map's cell grid and maintains object placement, overlap, visibility, map zones, bridge state, and binary map persistence.

This file is related to Tiberian Dawn's `MAP.CPP`, but it is not interchangeable with it. Red Alert adds behaviors and has its own map dimensions and serialization rules.

## Main class

### `MapClass`

The manager for the Red Alert game board. It provides cell/region queries, map initialization, object placement/removal, visibility, zone operations, and bridge-related operations.

## Functions in the original source index

| Function | Plain-English responsibility |
|---|---|
| `Base_Region` | Finds the base owner/zone associated with a cell. |
| `Cell_Region` | Maps a cell number to its region. |
| `Cell_Threat` | Reads a house's threat value at a cell. |
| `Close_Object` | Finds an object near a requested screen coordinate for clicking. |
| `Destroy_Bridge_At` | Finds and destroys the bridge at a specified location. |
| `Detach` | Removes an object's references from map bookkeeping. |
| `In_Radar` | Checks whether a cell lies inside radar coverage. |
| `Init` | Clears/initializes map cell state. |
| `Intact_Bridge_Count` | Counts bridges that remain intact. |
| `Logic` | Runs map-specific periodic logic. |
| `Nearby_Location` | Finds a generally clear cell near a requested cell. |
| `One_Time` | Performs one-time map initialization. |
| `Overlap_Down` / `Overlap_Up` | Adds/removes an object's footprint from map occupancy tracking. |
| `Overpass` | Performs final cleanup after map construction. |
| `Pick_Up` / `Place_Down` | Removes or places an object while updating occupancy. |
| `Place_Random_Crate` / `Remove_Crate` | Places or removes a crate on a map cell. |
| `Read_Binary` / `Write_Binary` | Reads/writes the Red Alert map's binary representation. |
| `Set_Map_Dimensions` | Sets the map's playable dimensions and boundaries. |
| `Sight_From` | Marks cells visible around a source cell. |
| `Validate` | Checks map cells for consistency. |
| `Zone_Reset` | Reinitializes zone labels to match the map. |
| `Zone_Span` | Flood-fills a connected map region and labels its zone. |
| `Pick_Random_Location` | Chooses a random valid map location. |

## Differences from Tiberian Dawn

Red Alert includes explicit bridge operations and zone connectivity logic in this class. Its map grid is also larger in the pinned source configuration (see `REDALERT/DEFINES.H` and the repository's `PORTING_STATUS.md`). Do not parse RA map bytes using the TD decoder.

## Rust port mapping

A complete Red Alert runtime map implementation is not present. The existing map editor model is only an initial editor-side data model, not the original `MapClass`.
