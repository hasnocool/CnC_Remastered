# TIBERIANDAWN/TEMPLATE.H

**Review status:** Partially reviewed.  
**Upstream:** [TIBERIANDAWN/TEMPLATE.H](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/TEMPLATE.H)

## Purpose

Declares the Tiberian Dawn `TemplateClass` interface used for an individual template object placed on the map. Its implementation is in `TEMPLATE.CPP`.

## Main class

### `TemplateClass`

An instance of a terrain template in the game world. It is a placed object, not the static catalogue entry that describes every available terrain template. That catalogue is primarily in `CDATA.CPP` through `TemplateTypeClass`.

## Related files

- [TEMPLATE.CPP](TEMPLATE.CPP.md): object lifecycle and scenario serialization.
- [CDATA.CPP](CDATA.CPP.md): static template-type registry and theatre-specific icon sets.
- [MAP.CPP](MAP.CPP.md): world grid and cell occupancy.

## Review note

A follow-up pass should document every member and signature from the header, including inherited object relationships and any pointer/pool ownership assumptions.
