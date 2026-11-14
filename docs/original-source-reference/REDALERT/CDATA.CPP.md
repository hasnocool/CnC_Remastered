# REDALERT/CDATA.CPP

**Review status:** Reviewed for template registry and theatre-loading path.  
**Upstream:** [REDALERT/CDATA.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/CDATA.CPP)

## Purpose

Defines Red Alert's static game type catalogues, including template type descriptors, and initializes theatre-specific graphics. It is central to understanding which icon-set file belongs to a template and how its map/composition metadata is interpreted.

## Main class: `TemplateTypeClass`

A template type is the reusable definition of a terrain graphic set. It is different from `TemplateClass`, which is one placement in the live map.

## Functions

| Function | Plain-English responsibility |
|---|---|
| `As_Reference` | Returns the registered template type for a given type identifier. |
| `Create_And_Place` | Creates and places an instance of the selected template type. |
| `Create_One_Of` | Creates an instance of the template type. |
| `Display` | Draws the template as a multi-cell composition using its icon-set map; `0xFF` entries are skipped rather than drawn as tiles. |
| `From_Name` | Finds a template type from its text name. |
| `Init` | Loads graphics for template types supported by the chosen theatre and obtains template width/height from icon-set map metadata. |
| `Land_Type` | Determines the land classification for a particular icon/tile within the template. |
| `Occupy_List` | Determines which cells of the template are occupied. |
| `One_Time` | Performs one-time type initialization. |
| `Prep_For_Add` | Prepares a template type for adding to a scenario. |
| `TemplateTypeClass` constructor | Initializes a template type descriptor. |
| `operator new/delete` | Allocate/free template type descriptors using the game's special heap. |

## Critical Red Alert behavior

The Red Alert `Init` path gets each applicable theatre-specific template asset from the MIX archive and reads the template's width and height from the icon-set map metadata. This differs from Tiberian Dawn, where the dimensions are defined by the template type records.

The display/occupancy routines use the icon-set's map of image indices. Empty entries marked `0xFF` are intentional holes. A renderer must preserve these holes and the template's shape; it must not pack every frame into a generic atlas and assume every frame is used.

## Rust port mapping

The Rust port needs a Red Alert-specific template registry/metadata path. A single shared parser can expose semantic icon-set data, but the TD and RA interpretations of dimensions and template metadata must remain variant-aware.
