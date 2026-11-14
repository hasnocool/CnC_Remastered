# OverlapperSet.cs — editor model source guide

> **Audit status: partially reviewed.** This pass inventories local types, explicit fields, method declaration lines, and dependencies. Full body explanations and Rust parity are still required.

- Upstream: [CnCTDRAMapEditor/Model/OverlapperSet.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OverlapperSet.cs)
- Pinned source commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 4103 characters.

## Purpose

Generic spatial overlap index for map objects. Supports add/remove, copying, and queries against points/rectangles.

## Types

- `OverlapperSet`

## Fields detected

- `metrics`
- `Overlappers`

## Method signatures detected

- `public bool Add(Point location, T overlapper)`
- `public bool Add(int x, int y, T occupier) => Add(new Point(x, y), occupier);`
- `public bool Add(int cell, T overlapper) => metrics.GetLocation(cell, out Point location) ? Add(location, overlapper) : false;`
- `public void Clear() => overlappers.Clear();`
- `public bool Contains(T occupier) => overlappers.ContainsKey(occupier);`
- `public void CopyTo(OverlapperSet<T> other)`
- `public bool Remove(T overlapper)`
- `public ISet<Point> Overlaps(IEnumerable<Rectangle> rectangles)`
- `public ISet<Point> Overlaps(Rectangle rectangle) => Overlaps(rectangle.Yield());`
- `public ISet<Point> Overlaps(IEnumerable<Point> points) => Overlaps(points.Select(p => new Rectangle(p, new Size(1, 1))));`
- `public ISet<Point> Overlaps(Point point) => Overlaps(point.Yield());`

## Direct namespace dependencies

- MobiusEditor.Interface
- MobiusEditor.Utility
- System.Collections
- System.Collections.Generic
- System.Drawing
- System.Linq

## Behavior/port concerns

Overlap queries convert point/rectangle inputs to sets of map cells. Validate clipping and boundary behavior, especially for negative or off-map rectangles; CopyTo must preserve expected index semantics.

## Rust mapping
No Rust equivalent path is asserted without inspecting the implementation tree. Candidate responsibilities should be mapped to editor model/configuration crates, with deterministic occupancy/overlap tests and fixture-based INI compatibility tests.

## Outstanding audit
- [ ] Exact member types, visibility, initializers, and property accessors.
- [ ] Every function body's branches, loops, side effects, and error behavior.
- [ ] Callers, serializers, and game-specific data sources.
- [ ] TD/RA differences and test fixtures.
- [ ] Rust implementation paths and parity tests.
