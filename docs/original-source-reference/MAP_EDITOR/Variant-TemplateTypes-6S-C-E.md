# Tranche 6S-C-E — TD/RA TemplateTypes IDs 128–159

Both pinned registry files contain exactly 32 declarations in this interval. Full catalogs remain 216 TD entries (IDs0–215) and 401 RA entries (IDs0–400). Cumulative declaration coverage is IDs0–159: 160/216 TD and 160/401 RA. Both source inventory rows remain **Partially reviewed**.

- [TD guide](TiberianDawn/TemplateTypes-6S-C-E.md)
- [RA guide](RedAlert/TemplateTypes-6S-C-E.md)
- Previous tranche: [IDs96–127](Variant-TemplateTypes-6S-C-D.md)

## Variant comparison

| Variant | IDs128–159 | Families | Theatre policy | Footprint observations |
|---|---|---|---|---|
| TD | 128–135 | Road36–Road43, d36–d43 | Winter, Desert, Temperate | All 2×2 |
| TD | 136–148 | River1–River13, rv01–rv13 | Winter, Temperate | 2×2 through 5×4 |
| TD | 149–159 | River14–River24, rv14–rv24 | Desert only | Includes 6×8 River20 and 5×8 River21 |
| RA | 128–134 | Falls2a, Ford1/2, Bridge1/1d/2/2d | Temperate, Snow | Falls 3×2; fords 3×3; bridges 5×2/5×3 |
| RA | 135–159 | Slope01–Slope25, s01–s25 | Temperate, Snow | Mostly 2×2; 1×2, 2×1, 2×3 and 3×2 exceptions |

All 64 declarations use the five-argument constructor, verified to delegate with `TemplateTypeFlag.None`. There are no explicit per-entry flags or 0×0 dimensions in this slice.

## Source-level implications

- TD has a theatre split inside one family: River1–13 are Winter/Temperate, while River14–24 are Desert-only. A Rust resolver must use the explicit whitelist rather than assume river tokens are interchangeable across theatres.
- RA changes family at ID128: falls/ford/bridges precede the slope sequence. Field names omit the leading zero retained in Slope tokens s01–s25.
- TD and RA are independent variant registries. Do not merge IDs by matching field names or infer equivalent geometry from token spelling.
- The shared model's `Init` sizes a square thumbnail using the maximum footprint dimension, then attempts icon lookup in y-outer/x-inner order. The mask is true only for successful theatre-specific tile lookups. `Map.InitTheater` invokes it for every registered template on theatre initialization/change.

## Rust comparison

- `crates/map-editor-model/src/lib.rs` stores `CellLayers.template: Option<u16>`; no typed TD/RA editor catalog, name/token, theatre filter, footprint, flag, icon mask or thumbnail cache is present.
- `crates/rts-core/src/red_alert_template.rs` models the runtime Red Alert `TemplateClass` lifecycle through a host trait. That is not the C# editor's static `TemplateType` catalog.
- No Rust mapping, runtime parity test or image/serialization test is claimed. No Rust code changed.

## Verification boundary and remaining work

The exact upstream source files were fetched at `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`; their blob SHAs matched the complete pinned upstream recursive tree. The parser verified 32 sequential IDs128–159 in each file and recorded every declaration's source line. The shared TemplateType constructor/Init, Map.InitTheater and both plugin theatre-change handlers were fetched. This is declaration coverage, not a complete review of either registry's dependencies or GamePlugin serializer.

Remaining declarations: TD IDs160–215 (56); RA IDs160–400 (241). Next: [Tranche 6S-C-F, IDs160–191](../04-next-audit-work.md). Keep both registry paths Partially reviewed until all declarations, complete bodies, consumers, serialization dependencies and applicable Rust mapping/tests are audited.

## Pinned source references

- [TD TemplateTypes.cs lines152–183](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/TemplateTypes.cs#L152-L183) — blob `97fdfd2109d241da8b51139d974028ed1198b140`.
- [RA TemplateTypes.cs lines152–183](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L152-L183) — blob `027bb5658e2297cf2ade84f9a01567acc8238db8`.
- [TemplateType constructor and Init](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/TemplateType.cs) — blob `f62e53665e9a45ddebba9e1f7d01ca61045fae64`.
- [Map.InitTheater](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Map.cs#L267-L285) — blob `1b478aebee005a8db2b4b7bfb4e044356829bd5c`.
