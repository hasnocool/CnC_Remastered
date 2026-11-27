# Tranche 6S-C-G — TD/RA TemplateTypes IDs 192–215 / 192–223

Pinned upstream `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`. Both source files were fetched in full and their blob SHAs matched the pinned recursive upstream tree.

- [TD IDs192–215](TiberianDawn/TemplateTypes-6S-C-G.md)
- [RA IDs192–223](RedAlert/TemplateTypes-6S-C-G.md)
- [Previous tranche IDs160–191](Variant-TemplateTypes-6S-C-F.md)
- [Next work](../04-next-audit-work.md): RA IDs224–255. TD has no further declarations.

## Coverage

| Variant | New IDs | Count | Catalogue total | Cumulative declarations documented |
|---|---:|---:|---:|---:|
| Tiberian Dawn | 192–215 | 24 | 216 | 216/216 |
| Red Alert | 192–223 | 32 | 401 | 224/401 |

## Catalogue differences

| Variant / IDs | Fields and keys | Theatre whitelist | Footprints |
|---|---|---|---|
| TD 192–215 | Shore40–Shore63; sh40–sh63 | Desert only | 1×2 through 6×1; exact dimensions in per-entry table |
| RA 192–215 | Road20–Road43; d20–d43 | Temperate/Snow | 2×2, 3×2, 3×3 |
| RA 216–223 | Rough01–Rough08; rf01–rf08 | Temperate/Snow | 1×1 for Rough01–07; Rough08 is 1×2 |

All 56 declarations use the five-argument constructor and receive TemplateTypeFlag.None. IDs are variant-local.

## Shared initialization contract

TemplateType stores ushort ID, name/key, icon width/height, theatre array, flag, icon mask and thumbnail. Init(TheaterType) creates a max-footprint square bitmap, loops y-major/x-minor, and marks mask cells only when the theatre-specific tile lookup succeeds. Map.InitTheater invokes every template initializer before smudge, overlay, terrain and object registries. No positive-dimension guard precedes bitmap allocation. Earlier RA zero-sized declarations IDs100–102 and 111 remain outside this slice and need runtime reproduction.

Both registry static constructors reflect public static readonly TemplateType fields and materialize with ToArray; no explicit sorting is applied. GetTypes returns the stored array as IEnumerable, not a defensive copy.

## Rust status and outstanding work

Rust CellLayers.template remains Option<u16>; there is no typed TD/RA catalogue, theatre resolver, icon-mask generation or thumbnail cache. Rust Red Alert TemplateClass models runtime placed templates, not the editor registry. TD declaration enumeration is complete, but both rows remain Partially reviewed until full bodies, consumers, GamePlugin serialization and initialization dependencies are audited. Rust-mapped=0, Tested=0, Complete=0; no Rust code/tests changed.

## Verification

- TD blob `97fdfd2109d241da8b51139d974028ed1198b140`: exactly 24 IDs192–215, lines216–239.
- RA blob `027bb5658e2297cf2ade84f9a01567acc8238db8`: exactly 32 IDs192–223, lines216–247.
