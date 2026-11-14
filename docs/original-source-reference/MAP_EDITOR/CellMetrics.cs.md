# CellMetrics.cs — map editor source guide

> **Audit status: partially reviewed.** This page inspects the pinned source file directly. The symbol list is a first-pass scan; exact member completeness and all call sites still require verification.

- Upstream: [`CnCTDRAMapEditor/Model/CellMetrics.cs`](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/CellMetrics.cs)
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source size: 4,692 characters.

## Plain-English purpose

Defines the geometry of a rectangular map grid. It converts between flattened cell IDs and x/y coordinates, checks bounds, finds eight-direction neighbors, and clips points to the map or a margin-constrained interior.

## Imports

- `System`
- `System.Collections.Generic`
- `System.Drawing`
- `System.Linq`
- `System.Text`
- `System.Threading.Tasks`

## Types found

- `CellMetrics`

## Member and method declarations found

- `public Point TopLeft => Point.Empty;`
- `public Size Size => new Size(Width, Height);`
- `public Rectangle Bounds => new Rectangle(TopLeft, Size);`
- `public int Length => Width * Height;`
- `public bool Contains(Point location) => ((location.X >= 0) && (location.X < Width) && (location.Y >= 0) && (location.Y < Height));`
- `public bool Contains(int cell) => ((cell >= 0) && (cell < Length));`
- `public CellMetrics(int width, int height)`
- `public CellMetrics(Size size)`
- `public bool GetCell(Point location, out int cell)`
- `public bool GetLocation(int cell, out Point location)`
- `public bool Adjacent(Point location, FacingType facing, out Point adjacent)`
- `public bool Adjacent(int cell, FacingType facing, out int adjacent)`
- `public void Clip(ref Point location)`
- `public void Clip(ref Point location, Size margin)`
- `public void Clip(ref Point location, Size topLeftMargin, Size bottomRightMargin)`

## Behavior that matters to the port

Flattened cell IDs are row-major (`y * Width + x`); neighbor lookup is bounded; clipping mutates the supplied Point. Width must be nonzero for reverse conversion to be meaningful, and callers must respect the boolean bounds result.

## Outstanding audit

- [ ] Exact members, property accessors, and interface implementation sites.
- [ ] All call sites and event subscription/unsubscription paths.
- [ ] Error behavior, nullability, and edge cases.
- [ ] Rust mapping and behavioral tests.
