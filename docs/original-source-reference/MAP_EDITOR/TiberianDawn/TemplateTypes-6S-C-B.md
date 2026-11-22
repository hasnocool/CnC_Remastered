# Tiberian Dawn TemplateTypes — Tranche 6S-C-B (IDs 32–63)

> **Partially reviewed.** This batch documents catalog declarations IDs32–63. Static constructor/GetTypes were inspected previously; IDs64–215 and complete consumer/runtime parity remain open. Not Body reviewed, Rust-mapped, Tested, or Complete.

Pinned upstream: `CnCTDRAMapEditor/TiberianDawn/TemplateTypes.cs`, commit `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`, blob `97fdfd2109d241da8b51139d974028ed1198b140` ([source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/TemplateTypes.cs)). Total 216 entries, IDs0–215.

## IDs 32–63

| Field | ID | Token | Dimensions | Theatres | Flags |
|---|---:|---|---:|---|---|
| `Slope20` | 32 | `s20` | 2×3 | Winter, Desert, Temperate | None |
| `Slope21` | 33 | `s21` | 1×2 | Winter, Desert, Temperate | None |
| `Slope22` | 34 | `s22` | 2×1 | Winter, Desert, Temperate | None |
| `Slope23` | 35 | `s23` | 3×2 | Winter, Desert, Temperate | None |
| `Slope24` | 36 | `s24` | 2×2 | Winter, Desert, Temperate | None |
| `Slope25` | 37 | `s25` | 2×2 | Winter, Desert, Temperate | None |
| `Slope26` | 38 | `s26` | 2×2 | Winter, Desert, Temperate | None |
| `Slope27` | 39 | `s27` | 3×2 | Winter, Desert, Temperate | None |
| `Slope28` | 40 | `s28` | 2×2 | Winter, Desert, Temperate | None |
| `Slope29` | 41 | `s29` | 2×2 | Winter, Desert, Temperate | None |
| `Slope30` | 42 | `s30` | 2×2 | Winter, Desert, Temperate | None |
| `Slope31` | 43 | `s31` | 2×2 | Winter, Desert, Temperate | None |
| `Slope32` | 44 | `s32` | 2×2 | Winter, Desert, Temperate | None |
| `Slope33` | 45 | `s33` | 2×2 | Winter, Desert, Temperate | None |
| `Slope34` | 46 | `s34` | 2×2 | Winter, Desert, Temperate | None |
| `Slope35` | 47 | `s35` | 2×2 | Winter, Desert, Temperate | None |
| `Slope36` | 48 | `s36` | 2×2 | Winter, Desert, Temperate | None |
| `Slope37` | 49 | `s37` | 2×2 | Winter, Desert, Temperate | None |
| `Slope38` | 50 | `s38` | 2×2 | Winter, Desert, Temperate | None |
| `Shore32` | 51 | `sh32` | 3×3 | Temperate, Winter | None |
| `Shore33` | 52 | `sh33` | 3×3 | Temperate, Winter | None |
| `Shore20` | 53 | `sh20` | 4×1 | Desert | None |
| `Shore21` | 54 | `sh21` | 3×1 | Desert | None |
| `Shore22` | 55 | `sh22` | 6×2 | Desert | None |
| `Shore23` | 56 | `sh23` | 2×2 | Desert | None |
| `Brush1` | 57 | `br1` | 1×1 | Desert | None |
| `Brush2` | 58 | `br2` | 1×1 | Desert | None |
| `Brush3` | 59 | `br3` | 1×1 | Desert | None |
| `Brush4` | 60 | `br4` | 1×1 | Desert | None |
| `Brush5` | 61 | `br5` | 1×1 | Desert | None |
| `Brush6` | 62 | `br6` | 2×2 | Desert | None |
| `Brush7` | 63 | `br7` | 2×2 | Desert | None |

All entries in this segment use `TemplateTypeFlag.None`. Slope20–38 (IDs32–50) support Winter/Desert/Temperate. Shore32/33 (IDs51–52) support Temperate/Winter. Desert-only Shore20–23 occupy IDs53–56, followed by Brush1–7 at IDs57–63, also Desert-only.

## Port notes

- Field order is source order; public static readonly definitions are collected by the previously reviewed reflection registry.
- IDs are contiguous, but the shore field names are non-monotonic: Shore32/33 precede desert Shore20–23.
- No Clear/Water/OreMine flags in this segment. Theatre restrictions are catalog data; the model Init's lookup behavior is documented separately.
- Remaining IDs64–215, full catalog dependencies and consumers remain open.

Previous [slice 0–31](TemplateTypes-6S-C-A.md). [Comparison](../Variant-TemplateTypes-6S-C-B.md). Next slice IDs64–95.
