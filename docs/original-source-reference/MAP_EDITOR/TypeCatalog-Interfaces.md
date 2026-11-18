# Type catalog interface contracts

> **Audit status: Header reviewed.** Exact interface declarations at the pinned upstream commit were inspected. These interfaces contain no method bodies.

- Upstream baseline: [electronicarts/CnC_Remastered_Collection](https://github.com/electronicarts/CnC_Remastered_Collection/tree/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae)
- Pinned source blobs are listed per interface below.

## IBrowsableType

Source: [IBrowsableType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Interface/IBrowsableType.cs) — blob `219db6826a49b1cc4f458d5b034ce181a07f4574`.

```csharp
public interface IBrowsableType
{
    string DisplayName { get; }
    Image Thumbnail { get; }
}
```

The interface imports System.Drawing for Image. It exposes display text and an image getter only; ownership, disposal, localization and cache lifetime are not specified by the contract.

## ICellOccupier

Source: [ICellOccupier.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Interface/ICellOccupier.cs) — blob `80c11ac1a29b4cfd2a642d1b42def7440ebcb09e`.

```csharp
public interface ICellOccupier
{
    bool[,] OccupyMask { get; }
}
```

The contract provides only a two-dimensional mask. It does not encode an anchor, mask origin, x/y dimension convention, mutability, or caching policy. OccupierSet supplies the interpretation.

## ICellOverlapper

Source: [ICellOverlapper.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Interface/ICellOverlapper.cs) — blob `ebd5b575167f262c9ee56cf9c2cadbbb48a70402`.

```csharp
public interface ICellOverlapper
{
    Rectangle OverlapBounds { get; }
}
```

The contract uses System.Drawing.Rectangle. It does not state whether bounds are clipped to the map or how origin relates to an object's anchor; OverlapperSet and concrete model types supply those semantics.

## ITechnoType

Source: [ITechnoType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Interface/ITechnoType.cs) — blob `c62c2c523cc40d403fccee48aaf9cf5e9ed468a4`.

```csharp
public interface ITechnoType
{
    sbyte ID { get; }
    string Name { get; }
}
```

This is the minimal typed-techno identity contract. It does not include owner, display name, rendering, occupancy, or type-specific flags.

## Implementers inspected in this tranche

- BuildingType: all four interfaces.
- InfantryType: ITechnoType and IBrowsableType.
- OverlayType: ICellOccupier and IBrowsableType.
- TerrainType: ICellOverlapper, ICellOccupier and IBrowsableType.
- UnitType: all four interfaces.
- SmudgeType and TemplateType: IBrowsableType only.

DirectionType, HouseType and TheaterType implement none of these interfaces.

## Rust mapping

No matching Rust trait set or typed catalog was found in `crates/map-editor-model`. Candidate traits should avoid System.Drawing types in the model crate. Thumbnail access belongs behind a renderer/cache interface; occupancy should use a typed mask or cell-footprint abstraction; overlap bounds should use an integer map-space rectangle; techno identity should use a variant-aware ID newtype.

## Verification boundary

The four interface declarations were inspected. This is Header reviewed, not a claim that all implementations or all consumers have been audited. No Rust code changed and no tests were executed.
