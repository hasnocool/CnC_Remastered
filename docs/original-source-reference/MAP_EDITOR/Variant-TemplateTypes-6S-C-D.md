# Tranche 6S-C-D — TD/RA TemplateTypes catalog slice (IDs 96–127)

32 declarations per variant parsed from the exact pinned source. Both inventory rows remain **Partially reviewed**. Cumulative declaration coverage is IDs0–127: 128/216 TD and 128/401 RA. No Rust editor-catalog mapping or parity tests are claimed.

- [TD guide](TiberianDawn/TemplateTypes-6S-C-D.md) · [RA guide](RedAlert/TemplateTypes-6S-C-D.md) · previous [slice64–95](Variant-TemplateTypes-6S-C-C.md).

## Segment comparison

| Variant | IDs96–127 | Families and token pattern | Theatres |
|---|---|---|---|
| TD | 96–127 | Road4–Road35, tokens `d04`–`d35`; footprints 2×2 through 3×4/4×3 | All Winter, Desert, Temperate |
| RA | 96–127 | ShoreCliff38; Boulder1–6; selected patches; River01–13; Falls1/Falls1a/Falls2 | All Temperate, Snow |

All 64 declarations use the overload defaulting to `TemplateTypeFlag.None`.

## Findings

- TD's range is one continuous road family. All entries support all three TD theatres.
- RA switches families within the same ID range. Preserve source order; do not sort or renumber by family.
- RA Boulder4 (ID100), Boulder5 (ID101), Boulder6 (ID102) and Patch15 (ID111) declare literal 0×0 dimensions.
- The source call path reaches every entry: Red Alert GamePlugin's Theater case calls Map.InitTheater; Map.InitTheater iterates every TemplateType and calls Init. TemplateType.Init constructs a Bitmap from max(width,height) before its icon loops. The zero-sized Bitmap request is source-confirmed; the exact runtime exception was not executed.
- TD has no zero-sized entries in this slice. Road5 is 3×4; Road9 and Road13 are 4×3.

## Rust comparison

- `crates/map-editor-model/src/lib.rs` stores `CellLayers.template: Option<u16>`; it has no editor TemplateType catalog, token, theatre list, footprint, flag, thumbnail or icon mask.
- `crates/rts-core/src/red_alert_template.rs` contains a partial engine-runtime `TemplateClass`/mark lifecycle port. Its `TemplateTypeInfo` is runtime type metadata, not the C# map-editor static registry.
- Existing runtime TemplateClass unit-test source is not evidence of editor catalog parity. No Rust code changed and no tests ran in this tranche.

## Review boundary and provenance

The registry static constructor/GetTypes, shared TemplateType model, Map.InitTheater call path and TemplateTool consumer were inspected in earlier bounded passes. This tranche extends declaration coverage only; both source files remain Partially reviewed until all declarations and full dependency/consumer/serializer behavior are audited.

- TD blob `97fdfd2109d241da8b51139d974028ed1198b140`: [declarations lines120–151](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/TemplateTypes.cs#L120-L151).
- RA blob `027bb5658e2297cf2ade84f9a01567acc8238db8`: [declarations lines120–151](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L120-L151).
- Shared model: [TemplateType.cs lines102–130](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/TemplateType.cs#L102-L130).
- All-type initialization: [Map.cs lines267–272](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Map.cs#L267-L272).
- RA INI call site: [GamePlugin.cs lines1676–1679](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/GamePlugin.cs#L1676-L1679).

Next: IDs128–159 for both variants.
