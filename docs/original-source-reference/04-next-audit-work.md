## Tranche 6S-C-C — TemplateTypes IDs64–95 (completed)

- Refreshed main at `7cd4ae5f0fc40781a4c735968bb24e7d6df51541`, inventory, roadmap, cross-reference, editor README and TypeCatalog-Contracts; exact TD/RA TemplateTypes sources fetched at pinned `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.
- Target recursive tree 484 entries complete; root tree `b818147c9c3e36e08eed365f804f8a903f0177ae`. Upstream recursive tree 1375 entries complete. Source blobs TD `97fdfd2109d241da8b51139d974028ed1198b140`, RA `027bb5658e2297cf2ade84f9a01567acc8238db8`.
- Parsed actual static declarations; verified 32 contiguous IDs64–95 per variant and full sequential catalog totals TD216/RA401.
- TD: Brush8–10, Patch1–8, Shore16–19, Patch13–15, Boulder1–6, Shore6–10, Road1–3. RA: ShoreCliff06–37. Exact fields/tokens/dimensions/theatres/flags/line anchors documented.
- Added two guides and comparison; updated inventory rows/snapshot, cross-reference, TypeCatalog-Contracts, MAP_EDITOR README, previous-slice navigation and this roadmap.
- Reconciled inventory: 17 legacy Reviewed, 121 Partially reviewed, 20 Header reviewed, 67 Body reviewed, 12 Resource reviewed, 1,094 Inventory only. Prior snapshot was stale. Total 1,331; Rust-mapped/Tested/Complete=0. Reference docs 257.
- No Rust/tests executed. Plugin bodies fetched for context but not reviewed; serialization remains call-site-level.

## Next batch — Tranche 6S-C-D (TemplateTypes IDs96–127)

1. Parse IDs96–127 in both pinned catalogs and validate IDs, tokens, dimensions, theatre arrays, flags and source anchors.
2. Add per-variant guides/comparison; update inventory, cross-reference, contracts and editor README. Keep both source rows Partially reviewed.
3. Continue 32-entry ranges until catalogs are enumerated; then separately audit initialization, tool consumers and serializers.
4. Prioritize full GamePlugin bodies, resources/designers, typed Rust catalog parity, shared libraries, remaining runtime systems and second-pass reviews. Rust-mapped/Tested/Complete are separate evidence states.