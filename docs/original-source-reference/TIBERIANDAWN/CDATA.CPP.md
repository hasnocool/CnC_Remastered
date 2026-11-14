# TIBERIANDAWN/CDATA.CPP

**Review status:** Reviewed for template type registry and theatre-loading path.  
**Upstream:** [TIBERIANDAWN/CDATA.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/CDATA.CPP)

## Purpose

This file defines the static catalogue of template types and initializes their graphics. It answers questions such as: “What is template type X?”, “Is it available in this theatre?”, and “Which icon-set data belongs to it?”

This is one of the most important files for understanding why terrain graphics must not be flattened into an unordered atlas.

## Main class

### `TemplateTypeClass`

A template type is a reusable definition, not a placed instance. It contains the identity and metadata used to interpret the graphic set: its name, theatre availability, icon-set data, and template dimensions/related properties.

A separate `TemplateClass` instance represents a particular placement on a map.

## Functions

| Function | Plain-English responsibility |
|---|---|
| `Create_And_Place` | Creates a template instance of this type and places it on the map. |
| `Create_One_Of` | Creates an instance without necessarily performing the complete placement path. |
| `Display` | Draws a generic representation of the template type. It uses the icon-set's map of tile indices and skips entries marked `0xFF`, preserving intentional holes. |
| `From_Name` | Converts a text template name into the corresponding registered type. |
| `Init` | Loads the graphics for template types supported by the selected theatre. |
| `Occupy_List` | Determines which map cells the template occupies based on its icon-set map metadata. |
| `One_Time` | Performs one-time registry/type initialization. |
| `Prep_For_Add` | Prepares the template type for addition to a scenario/map. |
| `TemplateTypeClass` constructor | Initializes a type descriptor and its metadata. |

## Theatre-specific loading

The original implementation iterates the registered template types, checks each type's theatre-availability bitmask, builds the graphics filename using the template's base name and the selected theatre suffix, and retrieves that file from the MIX archive.

For Tiberian Dawn, template width and height are part of the template type definitions. Do not apply Red Alert's icon-set-derived dimensions to this variant without checking its own source.

## Why the atlas looked random

The game does not say “all images in this archive form one terrain atlas.” It has a named template registry, theatre applicability, icon-set image frames, and a per-template composition map. `Display` walks the composition map in order and omits `0xFF` cells. Flattening the icon frames discards the relationships that make a road, river, cliff, or other multi-cell terrain feature meaningful.

## Rust port mapping

The Rust port needs an explicit theatre registry and template-type registry, with stable source ordering, theatre masks/suffixes, icon-set association, dimensions, and composition metadata. This is not yet a complete feature in the current workspace.
