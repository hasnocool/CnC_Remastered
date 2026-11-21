# Tiberian Dawn TemplateTypes — Tranche 6S-C-A (IDs 0–31)

> **Partially reviewed.** First 32 catalog entries, static constructor and GetTypes inspected. IDs32–215 remain open; not Body reviewed/Rust-mapped/Tested/Complete.

Upstream: `CnCTDRAMapEditor/TiberianDawn/TemplateTypes.cs` · pin `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae` · blob `97fdfd2109d241da8b51139d974028ed1198b140` · [source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/TemplateTypes.cs). Full file 257 lines; registry total 216 definitions (IDs0–215).

| Field | ID | Name token | Icon W×H | Theatre(s) | Flag |
|---|---:|---|---:|---|---|
| `Clear` | 0 | `clear1` | 1×1 | Winter, Desert, Temperate | Clear |
| `Water` | 1 | `w1` | 1×1 | Winter, Temperate, Desert | None |
| `Water2` | 2 | `w2` | 2×2 | Winter, Temperate | None |
| `Shore1` | 3 | `sh1` | 3×3 | Winter, Temperate | None |
| `Shore2` | 4 | `sh2` | 3×3 | Winter, Temperate | None |
| `Shore3` | 5 | `sh3` | 1×1 | Winter, Temperate | None |
| `Shore4` | 6 | `sh4` | 2×1 | Winter, Temperate | None |
| `Shore5` | 7 | `sh5` | 3×3 | Winter, Temperate | None |
| `Shore11` | 8 | `sh11` | 3×3 | Winter, Temperate | None |
| `Shore12` | 9 | `sh12` | 3×3 | Winter, Temperate | None |
| `Shore13` | 10 | `sh13` | 3×3 | Winter, Temperate | None |
| `Shore14` | 11 | `sh14` | 3×3 | Winter, Temperate | None |
| `Shore15` | 12 | `sh15` | 3×3 | Winter, Temperate | None |
| `Slope1` | 13 | `s01` | 2×2 | Winter, Desert, Temperate | None |
| `Slope2` | 14 | `s02` | 2×3 | Winter, Desert, Temperate | None |
| `Slope3` | 15 | `s03` | 2×2 | Winter, Desert, Temperate | None |
| `Slope4` | 16 | `s04` | 2×2 | Winter, Desert, Temperate | None |
| `Slope5` | 17 | `s05` | 2×2 | Winter, Desert, Temperate | None |
| `Slope6` | 18 | `s06` | 2×3 | Winter, Desert, Temperate | None |
| `Slope7` | 19 | `s07` | 2×2 | Winter, Desert, Temperate | None |
| `Slope8` | 20 | `s08` | 2×2 | Winter, Desert, Temperate | None |
| `Slope9` | 21 | `s09` | 3×2 | Winter, Desert, Temperate | None |
| `Slope10` | 22 | `s10` | 2×2 | Winter, Desert, Temperate | None |
| `Slope11` | 23 | `s11` | 2×2 | Winter, Desert, Temperate | None |
| `Slope12` | 24 | `s12` | 2×2 | Winter, Desert, Temperate | None |
| `Slope13` | 25 | `s13` | 3×2 | Winter, Desert, Temperate | None |
| `Slope14` | 26 | `s14` | 2×2 | Winter, Desert, Temperate | None |
| `Slope15` | 27 | `s15` | 2×2 | Winter, Desert, Temperate | None |
| `Slope16` | 28 | `s16` | 2×3 | Winter, Desert, Temperate | None |
| `Slope17` | 29 | `s17` | 2×2 | Winter, Desert, Temperate | None |
| `Slope18` | 30 | `s18` | 2×2 | Winter, Desert, Temperate | None |
| `Slope19` | 31 | `s19` | 2×2 | Winter, Desert, Temperate | None |

TD Water (ID1) and Water2 (ID2) are not Water-flagged. Shore1–5 and Shore11–15 occupy IDs3–12 and are Winter/Temperate only. Slopes1–19 occupy IDs13–31 and support Winter/Desert/Temperate. Clear is ID0, token clear1, Clear flag.

Static constructor reflects public static fields, filters IsInitOnly fields assignable to TemplateType, materializes with ToArray(); no explicit sorting, ID/name/dimension/theatre validation. GetTypes returns backing array enumerable. Remaining catalog entries not inspected in this tranche.

Map receives this catalog from the TD plugin and copies it. Map.InitTheater calls Init on all template types regardless of Theaters; consumers filter later. See [comparison](../Variant-TemplateTypes-6S-C-A.md) for model, tool and serialization call-site findings.

Next bounded batch: IDs32–63. Keep Partially reviewed until all 216 entries and dependencies are covered.