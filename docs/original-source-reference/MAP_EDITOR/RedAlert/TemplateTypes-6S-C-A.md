# Red Alert TemplateTypes — Tranche 6S-C-A (IDs 0–31)

> **Partially reviewed.** First 32 entries, static constructor and GetTypes inspected. IDs32–400 remain open; not Body reviewed/Rust-mapped/Tested/Complete.

Upstream: `CnCTDRAMapEditor/RedAlert/TemplateTypes.cs` · pin `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae` · blob `027bb5658e2297cf2ade84f9a01567acc8238db8` · [source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/TemplateTypes.cs). Full file 442 lines; registry total 401 definitions (IDs0–400).

| Field | ID | Name token | Icon W×H | Theatre(s) | Flag |
|---|---:|---|---:|---|---|
| `Clear` | 0 | `clear1` | 1×1 | Temperate, Snow, Interior | Clear |
| `Water` | 1 | `w1` | 1×1 | Temperate, Snow | Water |
| `Water2` | 2 | `w2` | 2×2 | Temperate, Snow | Water |
| `Shore01` | 3 | `sh01` | 4×5 | Temperate, Snow | None |
| `Shore02` | 4 | `sh02` | 5×5 | Temperate, Snow | None |
| `Shore03` | 5 | `sh03` | 3×5 | Temperate, Snow | None |
| `Shore04` | 6 | `sh04` | 3×3 | Temperate, Snow | None |
| `Shore05` | 7 | `sh05` | 3×3 | Temperate, Snow | None |
| `Shore06` | 8 | `sh06` | 3×3 | Temperate, Snow | None |
| `Shore07` | 9 | `sh07` | 3×3 | Temperate, Snow | None |
| `Shore08` | 10 | `sh08` | 1×2 | Temperate, Snow | None |
| `Shore09` | 11 | `sh09` | 3×3 | Temperate, Snow | None |
| `Shore10` | 12 | `sh10` | 5×6 | Temperate, Snow | None |
| `Shore11` | 13 | `sh11` | 4×5 | Temperate, Snow | None |
| `Shore12` | 14 | `sh12` | 3×5 | Temperate, Snow | None |
| `Shore13` | 15 | `sh13` | 6×5 | Temperate, Snow | None |
| `Shore14` | 16 | `sh14` | 4×4 | Temperate, Snow | None |
| `Shore15` | 17 | `sh15` | 5×3 | Temperate, Snow | None |
| `Shore16` | 18 | `sh16` | 3×3 | Temperate, Snow | None |
| `Shore17` | 19 | `sh17` | 2×1 | Temperate, Snow | None |
| `Shore18` | 20 | `sh18` | 3×3 | Temperate, Snow | None |
| `Shore19` | 21 | `sh19` | 4×5 | Temperate, Snow | None |
| `Shore20` | 22 | `sh20` | 5×4 | Temperate, Snow | None |
| `Shore21` | 23 | `sh21` | 5×3 | Temperate, Snow | None |
| `Shore22` | 24 | `sh22` | 6×5 | Temperate, Snow | None |
| `Shore23` | 25 | `sh23` | 5×5 | Temperate, Snow | None |
| `Shore24` | 26 | `sh24` | 3×4 | Temperate, Snow | None |
| `Shore25` | 27 | `sh25` | 3×3 | Temperate, Snow | None |
| `Shore26` | 28 | `sh26` | 3×3 | Temperate, Snow | None |
| `Shore27` | 29 | `sh27` | 3×3 | Temperate, Snow | None |
| `Shore28` | 30 | `sh28` | 3×3 | Temperate, Snow | None |
| `Shore29` | 31 | `sh29` | 1×2 | Temperate, Snow | None |

Clear is ID0/token clear1/Clear flag. Water and Water2 IDs1/2 carry Water flag (unlike TD). Shore01–29 occupy IDs3–31 and all support Temperate/Snow. No shore flag in this slice.

Static constructor reflects public static readonly TemplateType fields and ToArray()s; no explicit sort or catalog validation. GetTypes returns backing array enumerable. IDs32–400 remain unreviewed.

RA plugin supplies the catalog to Map; Map copies it. Map.InitTheater calls Init for all definitions regardless of theatre restriction. See [comparison](../Variant-TemplateTypes-6S-C-A.md).

Next bounded batch: IDs32–63. Keep Partially reviewed until all 401 entries and dependencies are covered.