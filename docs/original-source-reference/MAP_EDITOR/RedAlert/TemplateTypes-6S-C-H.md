# Tranche 6S-C-H — Red Alert TemplateTypes IDs 224–255

Pinned upstream `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`; source `CnCTDRAMapEditor/RedAlert/TemplateTypes.cs`, blob `027bb5658e2297cf2ade84f9a01567acc8238db8`. The full 442-line file contains 401 contiguous explicit declarations, IDs0–400. This slice is IDs224–255 at source lines248–279. Inventory status remains **Partially reviewed**; declaration enumeration is not complete dependency, consumer, serializer or Rust parity review.

## Exact declarations

The five-argument `TemplateType` constructor delegates to the six-argument overload with `TemplateTypeFlag.None`. Every row therefore has Flag=None. IDs224–252 support Temperate and Snow; IDs253–255 are Interior-only.

| Field | ID | Tile key | Footprint | Theatres | Flag | Source |
|---|---:|---|---:|---|---|---:|
| Rough09 | 224 | `rf09` | 1×2 | Temperate/Snow | None | [248](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L248) |
| Rough10 | 225 | `rf10` | 2×1 | Temperate/Snow | None | [249](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L249) |
| Rough11 | 226 | `rf11` | 2×1 | Temperate/Snow | None | [250](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L250) |
| Road44 | 227 | `d44` | 1×1 | Temperate/Snow | None | [251](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L251) |
| Road45 | 228 | `d45` | 1×1 | Temperate/Snow | None | [252](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L252) |
| River14 | 229 | `rv14` | 1×2 | Temperate/Snow | None | [253](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L253) |
| River15 | 230 | `rv15` | 2×1 | Temperate/Snow | None | [254](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L254) |
| RiverCliff01 | 231 | `rc01` | 2×2 | Temperate/Snow | None | [255](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L255) |
| RiverCliff02 | 232 | `rc02` | 2×2 | Temperate/Snow | None | [256](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L256) |
| RiverCliff03 | 233 | `rc03` | 2×2 | Temperate/Snow | None | [257](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L257) |
| RiverCliff04 | 234 | `rc04` | 2×2 | Temperate/Snow | None | [258](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L258) |
| Bridge1a | 235 | `br1a` | 4×3 | Temperate/Snow | None | [259](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L259) |
| Bridge1b | 236 | `br1b` | 4×3 | Temperate/Snow | None | [260](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L260) |
| Bridge1c | 237 | `br1c` | 4×3 | Temperate/Snow | None | [261](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L261) |
| Bridge2a | 238 | `br2a` | 5×3 | Temperate/Snow | None | [262](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L262) |
| Bridge2b | 239 | `br2b` | 5×3 | Temperate/Snow | None | [263](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L263) |
| Bridge2c | 240 | `br2c` | 5×3 | Temperate/Snow | None | [264](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L264) |
| Bridge3a | 241 | `br3a` | 4×2 | Temperate/Snow | None | [265](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L265) |
| Bridge3b | 242 | `br3b` | 4×2 | Temperate/Snow | None | [266](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L266) |
| Bridge3c | 243 | `br3c` | 4×2 | Temperate/Snow | None | [267](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L267) |
| Bridge3d | 244 | `br3d` | 4×2 | Temperate/Snow | None | [268](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L268) |
| Bridge3e | 245 | `br3e` | 4×2 | Temperate/Snow | None | [269](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L269) |
| Bridge3f | 246 | `br3f` | 4×2 | Temperate/Snow | None | [270](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L270) |
| F01 | 247 | `f01` | 3×3 | Temperate/Snow | None | [271](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L271) |
| F02 | 248 | `f02` | 3×3 | Temperate/Snow | None | [272](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L272) |
| F03 | 249 | `f03` | 3×3 | Temperate/Snow | None | [273](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L273) |
| F04 | 250 | `f04` | 3×3 | Temperate/Snow | None | [274](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L274) |
| F05 | 251 | `f05` | 3×3 | Temperate/Snow | None | [275](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L275) |
| F06 | 252 | `f06` | 3×3 | Temperate/Snow | None | [276](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L276) |
| ARRO0001 | 253 | `arro0001` | 1×1 | Interior | None | [277](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L277) |
| ARRO0002 | 254 | `arro0002` | 1×1 | Interior | None | [278](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L278) |
| ARRO0003 | 255 | `arro0003` | 1×1 | Interior | None | [279](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs#L279) |

## Contracts and limits

- IDs are explicit registry values. Consumers should use the stored ID rather than infer it from a field name or tile key.
- The first 29 declarations are explicitly whitelisted for Temperate and Snow. The final three are explicitly Interior-only. A registry declaration does not guarantee a matching tile exists in every listed theatre.
- `TemplateType.Init(TheaterType)` resolves footprint cells through the theatre tileset list and template key/icon index. Missing lookups leave the corresponding mask cell false; `Thumbnail` is null if no icon was found.
- Bridge dimensions describe the declared editor template footprint; this audit does not infer physical connectivity, collision geometry or map serialization from dimensions.
- F01–F06 are 3×3 declarations; this catalogue alone does not establish their in-game semantic meaning.

## Dependencies and Rust mapping

The static declarations are materialized by the plugin's reflected type registry and consumed by the shared editor model. Full RA GamePlugin serializer and all consumers remain open. Rust `CellLayers.template: Option<u16>` stores a numeric slot only; it does not encode the RA name, dimensions, theatre whitelist, mask or thumbnail. No concrete typed RA catalogue mapping is established. No Rust parity test was run.

## Evidence and status

- Source blob: `027bb5658e2297cf2ade84f9a01567acc8238db8`; complete source fetched at the pinned commit.
- Slice: exactly 32 contiguous declarations, IDs224–255, lines248–279.
- Registry total: 401 IDs0–400. Cumulative RA declaration coverage: 256/401.
- Status: **Partially reviewed**. IDs256–400 and full implementation/consumer/serializer review remain open. No Rust-mapped, Tested or Complete claim is made.

Previous slice: [IDs192–223](TemplateTypes-6S-C-G.md) · [TD/RA comparison](../Variant-TemplateTypes-6S-C-H.md)
