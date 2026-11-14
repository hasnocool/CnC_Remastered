# REDALERT/WIN32LIB/TILE.H — variant comparison

**Upstream:** [REDALERT/WIN32LIB/TILE.H](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/WIN32LIB/TILE.H)

## Purpose

Declares Red Alert's icon-set loader interface and active icon-set control structure.

## Important difference

The active Red Alert control structure includes explicit `MapWidth` and `MapHeight` fields in addition to the image dimensions/count/allocation/size and data offsets. This makes its active structure larger than the Tiberian Dawn active structure.

Red Alert's `TemplateTypeClass::Init` reads the template width and height from the icon-set map metadata. Tiberian Dawn instead uses the dimensions in its template type definitions. This difference is directly relevant to correct theatre decoding.

## Porting implications

- Keep the TD and RA raw layouts separate.
- Parse the IFF chunks into semantic fields rather than trusting native structure packing.
- Preserve MAP metadata and dimensions.
- Validate MAP dimensions against the number of entries and the template registry.
- Never replace unknown map entries with invented tile indices.
