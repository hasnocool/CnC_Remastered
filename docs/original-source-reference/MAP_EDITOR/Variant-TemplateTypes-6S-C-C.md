# Tranche 6S-C-C — TD/RA TemplateTypes catalog slice (IDs 64–95)

32 declarations per variant. Both inventory rows remain **Partially reviewed**. Cumulative catalog coverage IDs0–95 (96 each); TD IDs96–215 and RA IDs96–400 open. No Rust mapping/tests.

- [TD guide](TiberianDawn/TemplateTypes-6S-C-C.md) · [RA guide](RedAlert/TemplateTypes-6S-C-C.md) · previous [slice32–63](Variant-TemplateTypes-6S-C-B.md).

## Segment summary

| Variant | IDs64–95 | Theatre availability |
|---|---|---|
| TD | Brush8–10 (64–66); Patch1–8 (67–74); Shore16–19 (75–78); Patch13–15 (79–81); Boulder1–6 (82–87); Shore6–10 (88–92); Road1–3 (93–95) | Brush8–10 Desert; Patch1–4 Temperate/Desert, 5/6 Desert, 7/8 all three; Shore16 Winter/Temperate, 17/18 all three, 19 Desert; Patch13–15 Winter/Temperate; Boulder1/2 all three, 3 Winter/Temperate, 4–6 Temperate; Shore6–10 Winter/Temperate; roads all three |
| RA | ShoreCliff06–37 (64–95) | Temperate/Snow |

## Findings

- Both files contain 32 contiguous declarations in this slice; full catalogs remain sequential TD0–215/RA0–400.
- All entries use the overload defaulting to `TemplateTypeFlag.None`.
- TD source order interleaves brush, patches, shore, boulder, shore and road groups; theatre availability varies inside Patch/Boulder families.
- RA continues cliff tokens `wc`; most are 2×2, with 2×3, 3×2 and 1×2 exceptions.
- Exact declaration tables and source anchors are in the per-variant guides. Blobs TD `97fdfd2109d241da8b51139d974028ed1198b140`, RA `027bb5658e2297cf2ade84f9a01567acc8238db8`.

TemplateType.Init and TemplateTool behavior were audited earlier. GamePlugin source was fetched but not fully body-audited; serializer findings remain call-site-level.

## Rust/parity status

Rust stores template IDs without these typed catalogs, theatre lists, flags, dimensions or masks. No Rust changes, tests, rendering or serialization parity executed. Next IDs96–127.
