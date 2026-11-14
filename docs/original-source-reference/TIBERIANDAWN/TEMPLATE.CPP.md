# TIBERIANDAWN/TEMPLATE.CPP

**Review status:** Reviewed for the class role and source function index.  
**Upstream:** [TIBERIANDAWN/TEMPLATE.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/TEMPLATE.CPP)  
**Related files:** [TEMPLATE.H](TEMPLATE.H.md), [CDATA.CPP](CDATA.CPP.md), [MAP.CPP](MAP.CPP.md)

## Purpose

This file implements runtime template objects. A template object is a map object representing a piece of static terrain artwork placed at a particular cell or group of cells. It is distinct from a template *type*: the type describes the reusable graphic/metadata definition; the object is one placement in a scenario.

## Main class

### `TemplateClass`

Represents one placed template instance. It participates in the normal map-object lifecycle: creation, placement into the world, selection/marking, removal, and scenario serialization.

## Functions

| Function | Plain-English responsibility |
|---|---|
| `As_Target` | Converts the template instance into the game's generic target identifier format. |
| `Init` | Resets the template-object system/pool before use. |
| `Mark` | Marks or unmarks the template object's map footprint for selection/interaction feedback. |
| `Read_INI` | Reconstructs template placements from the scenario INI data. |
| `Select` | Handles selection of a template object. |
| `TemplateClass` constructors | Initialize a default or fully specified template instance. |
| `Unlimbo` | Moves the newly created template object into the active map/world. |
| `Write_INI` | Serializes template placements back into scenario INI data. |
| `delete` | Returns an instance to the template-object pool. |
| `new` | Allocates an instance from the template-object pool. |
| `Validate` | Checks that a template pointer/object is valid. |

## Why this matters to terrain loading

The template object is not the atlas entry. It is the placed game-world object that connects scenario data to the map. The actual image set and type metadata are managed by the template type registry in `CDATA.CPP`; cell-level map behavior is handled by `MAP.CPP`.

## Rust port mapping

No complete `TemplateClass` runtime equivalent is present yet. The editor's cell model is not a replacement for the original template-object lifecycle, selection, INI read/write, and pool behavior.
