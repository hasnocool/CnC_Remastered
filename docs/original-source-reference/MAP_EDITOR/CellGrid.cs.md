# CellGrid.cs — map editor source guide

> **Audit status: partially reviewed.** This page inspects the pinned source file directly. The symbol list is a first-pass scan; exact member completeness and all call sites still require verification.

- Upstream: [`CnCTDRAMapEditor/Model/CellGrid.cs`](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/CellGrid.cs)
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source size: 5,701 characters.

## Plain-English purpose

A generic two-dimensional grid indexed by x/y, Point, or flattened cell number. It raises change/clear events, supports adjacency queries through CellMetrics, copies between compatible grids, and enumerates non-null cells or intersections.

## Imports

- `System`
- `System.Collections`
- `System.Collections.Generic`
- `System.Drawing`

## Types found

- `FacingType`
- `CellChangedEventArgs`
- `CellGrid`

## Member and method declarations found

- `public CellChangedEventArgs(CellMetrics metrics, int cell, T oldValue, T value)`
- `public CellChangedEventArgs(CellMetrics metrics, Point location, T oldValue, T value)`
- `public class CellGrid<T> : IEnumerable<(int Cell, T Value)>, IEnumerable`
- `public T this[Point location] { get => this[location.X, location.Y]; set => this[location.X, location.Y] = value; }`
- `public T this[int cell] { get => this[cell % metrics.Width, cell / metrics.Width]; set => this[cell % metrics.Width, cell / metrics.Width] = value; }`
- `public Size Size => metrics.Size;`
- `public int Length => metrics.Length;`
- `public CellGrid(CellMetrics metrics)`
- `public void Clear()`
- `public T Adjacent(Point location, FacingType facing)`
- `public T Adjacent(int cell, FacingType facing)`
- `public bool CopyTo(CellGrid<T> other)`
- `protected virtual void OnCellChanged(CellChangedEventArgs<T> e)`
- `protected virtual void OnCleared()`
- `public IEnumerable<(int Cell, T Value)> IntersectsWith(ISet<int> cells)`
- `public IEnumerable<(int Cell, T Value)> IntersectsWith(ISet<Point> locations)`
- `public IEnumerator<(int Cell, T Value)> GetEnumerator()`

## Behavior that matters to the port

The model is event-driven: cell writes compare old/new values and notify subscribers only when the value changes; Clear emits a separate notification. The flattened index is row-major (`y * Width + x`). Out-of-bounds adjacency returns default rather than throwing.

## Outstanding audit

- [ ] Exact members, property accessors, and interface implementation sites.
- [ ] All call sites and event subscription/unsubscription paths.
- [ ] Error behavior, nullability, and edge cases.
- [ ] Rust mapping and behavioral tests.
