# BuildingType.cs — body-reviewed editor definition

> **Audit status: Body reviewed.** Complete 167-line source body inspected at the pinned upstream commit. Rust mapping and behavioral tests remain separate.

- Upstream: [CnCTDRAMapEditor/Model/BuildingType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/BuildingType.cs)
- Pinned source blob: `39a7a0546b6820a145080911c96cfdc8e47b7d7a`
- Namespace/type: `MobiusEditor.Model.BuildingType`
- Interfaces: `ICellOverlapper`, `ICellOccupier`, `ITechnoType`, `IBrowsableType`
- Direct imports: `MobiusEditor.Interface`, `MobiusEditor.Render`, `System`, `System.Drawing`

## Exact public contract

All properties are public. Unless noted, setters are private:

| Property | Type | Accessor / role |
|---|---|---|
| ID | `sbyte` | private set; game catalog identifier |
| Name | `string` | private set; normalized runtime name |
| DisplayName | `string` | private set; localized game text |
| Tilename | `string` | private set; original tile/catalog name |
| OverlapBounds | `Rectangle` | computed from `OccupyMask` dimensions |
| OccupyMask | `bool[,]` | private set; full placement/overlap mask |
| BaseOccupyMask | `bool[,]` | private set; original base footprint |
| Size | `Size` | private set; width=mask columns, height=mask rows |
| HasBib | `bool` | private set |
| OwnerHouse | `string` | private set |
| Theaters | `TheaterType[]` | private set; nullable in forwarding constructors |
| IsFake | `bool` | private set |
| HasTurret | `bool` | private set |
| FactoryOverlay | `string` | private set |
| Thumbnail | `Image` | public get/set; generated preview |

The primary constructor is:

```csharp
BuildingType(sbyte id, string name, string textId, bool[,] occupyMask,
    bool hasBib, string ownerHouse, TheaterType[] theaters,
    bool isFake, bool hasTurret, string factoryOverlay)
```

Four forwarding overloads omit, respectively, theatre metadata; theatre plus fake/turret/factory metadata; fake/turret/factory metadata; or turret/factory metadata. They delegate to the primary constructor and supply null/false defaults.

## Constructor behavior

1. Stores ID, owner, theatre array, fake/turret flags, factory overlay, and the supplied mask reference as `BaseOccupyMask`.
2. Sets `Name` to `name.Substring(0, name.Length - 1) + "f"` when `isFake`; otherwise it retains `name`. `Tilename` always retains the original `name`.
3. Resolves `DisplayName` immediately through `Globals.TheGameTextManager[textId]`.
4. Sets `Size` from the original mask dimensions.
5. Without a bib, `OccupyMask` aliases `BaseOccupyMask` (same array).
6. With a bib, allocates a mask one row taller. It copies original rows except the last, then fills the final two rows completely with `true`. Thus the full mask deliberately replaces the original bottom-row shape with two full-width rows; `BaseOccupyMask` remains unchanged.

No constructor validates null names/masks, non-empty dimensions, mask shape, duplicate IDs, or text-key existence. A fake type with an empty name reaches a negative substring length; malformed/empty masks can fail later during bib expansion.

## Method bodies

- `Equals(object obj)`: same-type comparison uses `this == obj` (reference equality; no operator overload is declared here). A boxed `sbyte` compares to ID. A string compares to Name using `StringComparison.OrdinalIgnoreCase`. Other values delegate to `object.Equals`.
- `GetHashCode()`: hashes ID, not object identity or Name.
- `ToString()`: returns Name.
- `Init(GameType gameType, TheaterType theater, HouseType house, DirectionType direction)`: constructs a temporary `Building` with this type, supplied house/direction, and Strength=256; calls `MapRenderer.Render(gameType, theater, Point.Empty, Globals.TileSize, Globals.TileScale, mockBuilding)`. If the returned size is non-empty, allocates a bitmap of that size, obtains Graphics in a `using` block, invokes the returned draw callback, and assigns Thumbnail. If the returned size is empty, Thumbnail is not cleared; reinitialization can therefore retain an earlier image.

## Cross-file dependencies and catalog callers

- TD and RA static catalogs construct these definitions in `TiberianDawn/BuildingTypes.cs` and `RedAlert/BuildingTypes.cs`. The two catalogs supply different IDs, names, owner-house strings, masks, theatre restrictions, fake variants, and factory overlay metadata.
- `Building` instances consume the type's base/full masks and metadata. `Map` maintains separate Buildings and Technos occupancy registrations; bib smudges are a separate map relationship.
- `MapRenderer.Render` is the thumbnail dependency. `Globals.TheGameTextManager` resolves display strings.

The registry files are sampled here only to establish construction provenance; their complete static data, ordering, and initialization lifecycle remain a separate audit tranche.

## Rust port requirements

No typed Rust BuildingType equivalent was found in `crates/map-editor-model`; that crate currently stores numeric layer IDs. Model ID/name/display name, theatre eligibility, owner, turret/fake/factory metadata, base mask, full mask, and preview metadata as distinct fields. Preserve the bib mask rule explicitly and avoid aliasing mutable mask buffers accidentally. Thumbnail generation should remain in a renderer/UI layer, not the deterministic map document.

## Verification and open work

- [x] Complete source body and constructor overloads inspected.
- [x] Occupancy mask construction and equality/hash behavior recorded.
- [x] Renderer/text/catalog dependencies identified.
- [ ] Exhaustive TD/RA catalog data audit and all call sites.
- [ ] Rust type/catalog mapping.
- [ ] Golden mask and fake-name tests; renderer fixture comparison.
- [ ] Runtime reproduction of malformed-input and stale-thumbnail edge cases.

No Rust source was changed and no test was executed in this documentation pass.
