# REDALERT/TEMPLATE.CPP

**Review status:** Reviewed for class role and source function index.  
**Upstream:** [REDALERT/TEMPLATE.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/TEMPLATE.CPP)

## Purpose

Implements the runtime `TemplateClass` object: one instance of a template placed into the Red Alert map. It handles the object's lifecycle and its interaction with the map.

## Functions

| Function | Plain-English responsibility |
|---|---|
| `Init` | Resets the template-object system. |
| `Mark` | Marks or unmarks the object's map footprint. |
| `Select` | Handles selection of the template object. |
| `TemplateClass` constructor | Initializes a template instance. |
| `Unlimbo` | Places the instance into the active map/world. |
| `delete` | Returns the instance to its object pool. |
| `new` | Allocates an instance from its pool. |

## Difference from Tiberian Dawn

The Red Alert source function index does not list the TD-specific `Read_INI`, `Write_INI`, `As_Target`, or `Validate` methods in this translation unit. Do not assume identical method sets merely because the class name is the same.

## Related files

- [MAP.CPP](MAP.CPP.md): map placement and occupancy.
- [CDATA.CPP](CDATA.CPP.md): template type definitions and graphics.
- [Tiberian Dawn TEMPLATE.CPP](../TIBERIANDAWN/TEMPLATE.CPP.md): comparison reference.
