# Tranche 6S-C-B — TD/RA TemplateTypes catalog slice (IDs 32–63)

Pinned-source catalog declaration audit: 32 additional entries per variant. Both TemplateTypes registries remain **Partially reviewed**. Cumulative catalog coverage IDs0–63 (64 entries each); TD IDs64–215 and RA IDs64–400 remain open. No Rust mapping/tests.

- [TD guide](TiberianDawn/TemplateTypes-6S-C-B.md) · [RA guide](RedAlert/TemplateTypes-6S-C-B.md) · previous [slice0–31](Variant-TemplateTypes-6S-C-A.md).

## Segment summary

| Variant | IDs32–63 | Theatre availability |
|---|---|---|
| TD | Slope20–38 (32–50); Shore32/33 (51–52); Desert Shore20–23 (53–56); Brush1–7 (57–63) | slopes Winter/Desert/Temperate; Shore32/33 Temperate/Winter; desert shore/brush Desert only |
| RA | Shore30–56 (32–58); ShoreCliff01–05 (59–63) | all Temperate/Snow |

## Source-derived findings

- Exactly 32 declarations per variant; IDs contiguous 32–63.
- Every entry has `TemplateTypeFlag.None`.
- TD shore naming/order is non-monotonic: Shore32/33 IDs51–52 precede Desert Shore20–23 IDs53–56; retain source order.
- TD Brush1–5 are 1×1; Brush6/7 are 2×2; Brush8+ lies beyond this slice.
- RA cliff tokens use `wc`, not `sh`. Cliff01/02 are 2×2 and 2×3; 03–05 are 2×2.
- Exact fields/tokens/dimensions/theatre arrays are in per-variant tables. Pinned blobs: TD `97fdfd2109d241da8b51139d974028ed1198b140`, RA `027bb5658e2297cf2ade84f9a01567acc8238db8`.

Prior review inspected reflection registry construction, TemplateType.Init mask generation and TemplateTool theatre/thumbnail/Clear filtering. Plugin serialization was call-site traced, not a complete GamePlugin body review.

## Rust and tests

Rust map-editor-model currently stores numeric template IDs only; typed per-variant catalogs, theatre arrays, flags, icon dimensions and masks are absent. Documentation only; no Rust code, tests, runtime, render or serialization parity executed.

Next batch **IDs64–95**. Keep both source inventory rows Partially reviewed.
