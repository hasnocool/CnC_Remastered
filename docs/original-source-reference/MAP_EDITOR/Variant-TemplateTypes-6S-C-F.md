# Tranche 6S-C-F — TD/RA TemplateTypes IDs 160–191

Exactly 32 contiguous declarations IDs160–191 were verified in each pinned source file. Cumulative declaration coverage is IDs0–191: **192/216 TD** and **192/401 RA**. Both inventory rows remain **Partially reviewed**.

- [TD IDs160–191](TiberianDawn/TemplateTypes-6S-C-F.md)
- [RA IDs160–191](RedAlert/TemplateTypes-6S-C-F.md)
- [Previous IDs128–159](Variant-TemplateTypes-6S-C-E.md)
- [Next batch](../04-next-audit-work.md): TD final 24 IDs192–215; RA IDs192–223

| Variant | IDs | Families | Theatres | Notable footprints |
|---|---|---|---|---|
| TD | 160 | River25 | Desert | 3×3 |
| TD | 161–164 | Ford1/2, Falls1/2 | Winter/Desert/Temperate | Falls2 3×2; others 3×3 |
| TD | 165–168 | Bridge1/1d, Bridge2/2d | Winter/Temperate | 4×4 and 5×5 |
| TD | 169–172 | Bridge3/3d, Bridge4/4d | Desert | 6×5 and 6×4 |
| TD | 173–180 | Shore24–31 | Desert | 4×1, 3×1, 6×2 and other shore shapes |
| TD | 181–185 | Patch16–20 | Winter | 2×2, 4×2, 4×3 |
| TD | 186–187 | Shore34/35 | Temperate/Winter | 3×3 |
| TD | 188–191 | Shore36–39 | Desert | 1×1 |
| RA | 160–172 | Slope26–38 | Temperate/Snow | Slope27 3×2; others 2×2 |
| RA | 173–191 | Road01–19 | Temperate/Snow | 1×2 through 4×3; Road05 3×4 |

## Porting implications

- IDs are variant-local: TD ID160 River25 is not RA ID160 Slope26.
- TD has several theatre-specific families; RA is uniformly Temperate/Snow for this slice.
- TD Shore32/33 were declared earlier at IDs51/52; do not sort by suffix or synthesize IDs.
- All 64 declarations use the five-argument TemplateType constructor, which delegates with TemplateTypeFlag.None. No 0×0 footprint appears in this slice.
- Preserve exact variant, ID, token, width, height, theatre list and flag. Shared TemplateType.Init builds a max-dimension square thumbnail and width×height mask; Map.InitTheater initializes every template. Earlier RA IDs100–102 and 111 remain a separate untested zero-dimension hazard because Init has no positive-dimension guard.

## Rust status

CellLayers.template remains Option<u16>; there is no typed static editor catalog or theatre-aware mask/thumbnail resolver. crates/rts-core/src/red_alert_template.rs models runtime TemplateClass, not editor TemplateType. Rust-mapped=0 and Tested=0. No Rust code/tests changed or executed.

## Verification

Target main at run start: 3fb4ea3965111066fc419cf93ef0ef061aaf856c. Target tree: 493 entries, complete. Pinned upstream tree: 1,375 entries, complete. TD source blob 97fdfd2109d241da8b51139d974028ed1198b140; RA source blob 027bb5658e2297cf2ade84f9a01567acc8238db8; both were matched against the pinned upstream tree. Source-line links in each variant table point to the exact declaration. Remaining declarations: TD 24, RA 209. Full bodies, consumers, serializers, concrete Rust mapping and applicable tests remain open.
