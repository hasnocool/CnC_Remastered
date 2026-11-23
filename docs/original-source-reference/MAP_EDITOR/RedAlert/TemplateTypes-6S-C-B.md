# Red Alert TemplateTypes — Tranche 6S-C-B (IDs 32–63)

> **Partially reviewed.** This batch documents catalog declarations IDs32–63. Static constructor/GetTypes were inspected previously; IDs64–400 and complete consumer/runtime parity remain open. Not Body reviewed, Rust-mapped, Tested, or Complete.

Pinned upstream: `CnCTDRAMapEditor/RedAlert/TemplateTypes.cs`, commit `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`, blob `027bb5658e2297cf2ade84f9a01567acc8238db8` ([source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs)). Total 401 entries, IDs0–400.

## IDs 32–63

| Field | ID | Token | Dimensions | Theatres | Flags |
|---|---:|---|---:|---|---|
| `Shore30` | 32 | `sh30` | 3×2 | Temperate, Snow | None |
| `Shore31` | 33 | `sh31` | 6×5 | Temperate, Snow | None |
| `Shore32` | 34 | `sh32` | 4×4 | Temperate, Snow | None |
| `Shore33` | 35 | `sh33` | 3×4 | Temperate, Snow | None |
| `Shore34` | 36 | `sh34` | 6×5 | Temperate, Snow | None |
| `Shore35` | 37 | `sh35` | 4×4 | Temperate, Snow | None |
| `Shore36` | 38 | `sh36` | 4×3 | Temperate, Snow | None |
| `Shore37` | 39 | `sh37` | 3×3 | Temperate, Snow | None |
| `Shore38` | 40 | `sh38` | 2×1 | Temperate, Snow | None |
| `Shore39` | 41 | `sh39` | 3×3 | Temperate, Snow | None |
| `Shore40` | 42 | `sh40` | 5×5 | Temperate, Snow | None |
| `Shore41` | 43 | `sh41` | 4×4 | Temperate, Snow | None |
| `Shore42` | 44 | `sh42` | 4×3 | Temperate, Snow | None |
| `Shore43` | 45 | `sh43` | 3×3 | Temperate, Snow | None |
| `Shore44` | 46 | `sh44` | 2×2 | Temperate, Snow | None |
| `Shore45` | 47 | `sh45` | 3×3 | Temperate, Snow | None |
| `Shore46` | 48 | `sh46` | 2×2 | Temperate, Snow | None |
| `Shore47` | 49 | `sh47` | 3×3 | Temperate, Snow | None |
| `Shore48` | 50 | `sh48` | 2×2 | Temperate, Snow | None |
| `Shore49` | 51 | `sh49` | 3×3 | Temperate, Snow | None |
| `Shore50` | 52 | `sh50` | 2×2 | Temperate, Snow | None |
| `Shore51` | 53 | `sh51` | 3×3 | Temperate, Snow | None |
| `Shore52` | 54 | `sh52` | 3×3 | Temperate, Snow | None |
| `Shore53` | 55 | `sh53` | 3×3 | Temperate, Snow | None |
| `Shore54` | 56 | `sh54` | 3×3 | Temperate, Snow | None |
| `Shore55` | 57 | `sh55` | 1×1 | Temperate, Snow | None |
| `Shore56` | 58 | `sh56` | 2×1 | Temperate, Snow | None |
| `ShoreCliff01` | 59 | `wc01` | 2×2 | Temperate, Snow | None |
| `ShoreCliff02` | 60 | `wc02` | 2×3 | Temperate, Snow | None |
| `ShoreCliff03` | 61 | `wc03` | 2×2 | Temperate, Snow | None |
| `ShoreCliff04` | 62 | `wc04` | 2×2 | Temperate, Snow | None |
| `ShoreCliff05` | 63 | `wc05` | 2×2 | Temperate, Snow | None |

All entries use `TemplateTypeFlag.None`. Shore30–56 (IDs32–58) and ShoreCliff01–05 (IDs59–63) are Temperate/Snow. Cliff tokens use `wc01`–`wc05`, distinct from `sh30`–`sh56`.

## Port notes

- Public static readonly catalog definitions use the previously inspected reflection registry.
- IDs continue sequentially; five cliff entries follow the 27 shore entries.
- No Clear/Water/OreMine flags in this segment.
- IDs64–400 and complete catalog dependency/consumer review remain open.

Previous [slice 0–31](TemplateTypes-6S-C-A.md). [Comparison](../Variant-TemplateTypes-6S-C-B.md). Next slice IDs96–127. [6S-C-C](TemplateTypes-6S-C-C.md).
