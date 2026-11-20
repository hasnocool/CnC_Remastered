# Tranche 6S-A — TD/RA Building, Overlay and Smudge catalogs

**Status:** Catalog body review complete for six static registry files. Declarations were captured from the pinned upstream sources and checked against model constructors. Documentation only; Rust mapping and tests remain open.

- Upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Guides: [TD BuildingTypes](TiberianDawn/BuildingTypes.cs.md), [RA BuildingTypes](RedAlert/BuildingTypes.cs.md), [TD OverlayTypes](TiberianDawn/OverlayTypes.cs.md), [RA OverlayTypes](RedAlert/OverlayTypes.cs.md), [TD SmudgeTypes](TiberianDawn/SmudgeTypes.cs.md), [RA SmudgeTypes](RedAlert/SmudgeTypes.cs.md).
- Models: [BuildingType](BuildingType.cs.md), [OverlayType](OverlayType.cs.md), [SmudgeType](SmudgeType.cs.md), [TheaterType](TheaterType.cs.md).

## Catalog counts and ID domains

| Catalog | TD entries | TD IDs | RA entries | RA IDs |
|---|---:|---|---:|---|
| Building | 60 | 0–59 | 63 | 0–36, 43–63, 82–86 |
| Overlay | 27 | 1–17, 20–29 | 25 | 0–24 |
| Smudge | 15 | 0–14 | 15 | 0–14 |

RA BuildingTypes has gaps 37–42 and 64–81; TD OverlayTypes has gaps 0, 18, and 19.

## Building differences

- TD building IDs are contiguous 0–59. RA has 63 definitions, including mines 43–44, civilians 45–63, and special entries 82–86.
- TD civilian V01–V19 have Temperate/Winter restrictions in groups; V12 and V13 both use `TEXT_STRUCTURE_TITLE_CIV12`; V19 is unrestricted. V20–V37 are Desert-only. V25 repeats CIV1 and V13 repeats CIV12; these source keys are preserved.
- RA V01–V19 are Temperate/Snow. V12, V13 and V19 share `TEXT_STRUCTURE_RA_CIVILIAN`. RA fake entries IDs 32–36 are FakeWeapon, FakeConst, FakeShipYard, FakeSubPen and FakeCommand. BuildingType derives effective Name by replacing the last input-token character with `f`; Tilename retains the original token.
- TD `Const` uses `TEXT_STRUCTURE_RA_FACT`; intent is not determined.
- TD Weapon ID 0 and RA Weapon ID 2 use FactoryOverlay `weap2`; RA FakeWeapon also uses it.
- Geometry differs: TD Storage 1×2 bibbed vs RA 1×1 non-bib; TD AirStrip 2×4 bibbed vs RA 2×3 non-bib; TD Repair bibbed vs RA non-bib; TD Power non-full 2×2 vs RA full; TD AdvancedPower 2×2 vs RA 3×3. Per-file tables contain every mask, key, owner, flag and restriction.

## Overlay differences

| Category | TD | RA |
|---|---|---|
| Walls | IDs 1–5: sandbag, cyclone, brick, barbwire, wood | IDs 0–4 same; Fence ID 23 adds sixth |
| Resource stages | IDs 6–17: 12 Tiberium stages | IDs 5–8: 4 gold stages |
| Gems | none | IDs 9–12: 4 gem stages |
| Civilians | IDs 20–26, v12–v18 | IDs 13–19, v12–v18 |
| Flag spot | ID 27 | ID 20 |
| Crates | IDs 28–29 wood/steel | IDs 21–22 wood/steel, ID 24 water |
| Theatre restrictions | V12/V13 Temperate; V14–V18 Temperate+Winter | V12–V18 Temperate+Snow |

OverlayTypeFlag values: None=0, TiberiumOrGold=1, Gems=2, Wall=4, Crate=8, Flag=16. IsPlaceable passes only None or Crate-only; resource/wall/flag entries use specialized tool predicates.

## Smudge catalogs

TD and RA are identical: Crater IDs 0–5 (`cr1`–`cr6`), Scorch IDs 6–11 (`sc1`–`sc6`), Bib1 ID12 logical 4×2, Bib2 ID13 3×2, Bib3 ID14 2×2. Bib flags are 3, 5 and 9, each containing shared Bib bit 1. Use bit tests for bib classification and exact equality for a specific bib type.

## Registry initialization and consumers

All six registries use reflection over public static fields to create array-backed catalogs without explicit sort or validation. BuildingTypes has a private readonly array; OverlayTypes and SmudgeTypes have non-readonly private arrays. `GetTypes()` exposes the array rather than a copy.

- TD/RA GamePlugin constructors pass these catalogs into Map, which copies the enumerables into lists.
- `Map.InitTheater(gameType)` iterates template, smudge, overlay, terrain, infantry, unit and building definitions and calls their Init for the selected theatre.
- Overlay/Smudge lookup requests tile 0 and assigns render metadata only on success; BuildingType.Init renders a mock building using owner-house lookup and north direction.
- MainForm filters Overlay by IsPlaceable and null-or-current-theatre; Resources by IsResource; Wall by IsWall; Smudge excludes bibs; Structures theatre-filters and sorts IsFake then Name. Wall selector sorts by Name but does not currently theatre-filter.

## Porting risks

**Source facts:** catalog instances are shared static objects per variant; Map.InitTheater mutates Thumbnail and SmudgeType.RenderSize; failed lookups do not clear prior metadata; display-text lookup occurs in relevant constructors; GetTypes exposes array-backed data.

**Runtime hypotheses:** concurrent map initialization may race on shared render metadata; a failed later tile lookup may preserve stale thumbnails; early static construction may access an uninitialized text manager. None has been reproduced.

## Rust mapping and verification

No typed catalog implementation exists in `crates/map-editor-model`; numeric cell-layer IDs remain. No source is Rust-mapped or Tested. Port exact catalogs as typed data, validate IDs/masks/keys/theatre restrictions, and key asset caches by game+theatre+definition. Golden catalog/render fixtures remain outstanding.
