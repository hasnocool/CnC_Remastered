# REDALERT/INI.CPP

**Review status:** Inventory-level summary; detailed function audit pending.  
**Upstream:** [REDALERT/INI.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/INI.CPP)

## Purpose

Handles Red Alert scenario INI parsing and scenario setup. It converts text configuration into the live scenario state, coordinating map, house, unit, terrain, and other object initialization.

## Relationship to map loading

Keep the scenario INI parser distinct from the binary map reader. The map binary provides the base map-cell/template data; INI sections add or configure scenario objects and rules. The exact load order and per-section behavior need to be documented from the implementation.

## Related files

- [MAP.CPP](MAP.CPP.md): Red Alert map grid and binary map operations.
- [CDATA.CPP](CDATA.CPP.md): registered object/template types.
- [TEMPLATE.CPP](TEMPLATE.CPP.md): placed template objects.
- `REDALERT/SCENARIO.CPP`: scenario lifecycle and setup.
