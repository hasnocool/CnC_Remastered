# OccupierSet.cs — editor model source guide

> **Audit status: partially reviewed.** This pass inventories local types, explicit fields, method declaration lines, and dependencies. Full body explanations and Rust parity are still required.

- Upstream: [CnCTDRAMapEditor/Model/OccupierSet.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OccupierSet.cs)
- Pinned source commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 9207 characters.

## Purpose

Generic spatial occupancy index. Tracks occupiers by cells and masks, rejects conflicts, and raises add/remove/clear events.

## Types

- `OccupierAddedEventArgs`
- `OccupierRemovedEventArgs`
- `OccupierSet`

## Fields detected

- `Cell`
- `Location`
- `Occupier`
- `metrics`
- `occupierCells`
- `Occupiers`

## Method signatures detected

- `public bool CanAdd(Point location, T occupier, bool[,] occupyMask)`
- `public bool CanAdd(int x, int y, T occupier, bool[,] occupyMask) => CanAdd(new Point(x, y), occupier, occupyMask);`
- `public bool CanAdd(int cell, T occupier, bool[,] occupyMask) => metrics.GetLocation(cell, out Point location) ? CanAdd(location, occupier, occupyMask) : false;`
- `public bool CanAdd(Point location, T occupier) => (occupier != null) ? CanAdd(location, occupier, occupier.OccupyMask) : false;`
- `public bool CanAdd(int x, int y, T occupier) => (occupier != null) ? CanAdd(x, y, occupier, occupier.OccupyMask) : false;`
- `public bool CanAdd(int cell, T occupier) => (occupier != null) ? CanAdd(cell, occupier, occupier.OccupyMask) : false;`
- `public bool Add(Point location, T occupier, bool[,] occupyMask)`
- `public bool Add(int x, int y, T occupier, bool[,] occupyMask) => Add(new Point(x, y), occupier, occupyMask);`
- `public bool Add(int cell, T occupier, bool[,] occupyMask) => metrics.GetLocation(cell, out Point location) ? Add(location, occupier, occupyMask) : false;`
- `public bool Add(Point location, T occupier) => (occupier != null) ? Add(location, occupier, occupier.OccupyMask) : false;`
- `public bool Add(int x, int y, T occupier) => (occupier != null) ? Add(x, y, occupier, occupier.OccupyMask) : false;`
- `public bool Add(int cell, T occupier) => (occupier != null) ? Add(cell, occupier, occupier.OccupyMask) : false;`
- `public void Clear()`
- `public bool Contains(int x, int y) => ((x >= 0) && (x < occupierCells.GetLength(1)) && (y >= 0) && (y < occupierCells.GetLength(0)));`
- `public bool Contains(Point location) => Contains(location.X, location.Y);`
- `public bool Contains(int cell) => metrics.GetLocation(cell, out Point location) ? Contains(location) : false;`
- `public bool Contains(T occupier) => occupiers.ContainsKey(occupier);`
- `public bool Remove(T occupier)`
- `public bool Remove(Point location) => Remove(this[location]);`
- `public bool Remove(int x, int y) => Remove(new Point(x, y));`
- `public bool Remove(int cell) => metrics.GetLocation(cell, out Point location) ? Remove(location) : false;`
- `protected virtual void OnOccupierAdded(OccupierAddedEventArgs<T> e)`
- `protected virtual void OnOccupierRemoved(OccupierRemovedEventArgs<T> e)`
- `protected virtual void OnCleared()`
- `private bool DoAdd(Point location, T occupier, bool[,] occupyMask)`
- `private bool DoRemove(T occupier)`
- `private static IEnumerable<Point> GetOccupyPoints(Point location, bool[,] occupyMask)`
- `private static IEnumerable<Point> GetOccupyPoints(Point location, T occupier) => GetOccupyPoints(location, occupier.OccupyMask);`

## Direct namespace dependencies

- MobiusEditor.Interface
- System
- System.Collections
- System.Collections.Generic
- System.Drawing
- System.Linq

## Behavior/port concerns

This type is a central spatial invariant: occupancy masks expand an object into occupied points; Add/CanAdd must agree on bounds and conflict semantics; Remove must clear exactly the cells owned by the occupier; event ordering and clear behavior must be preserved.

## Rust mapping
No Rust equivalent path is asserted without inspecting the implementation tree. Candidate responsibilities should be mapped to editor model/configuration crates, with deterministic occupancy/overlap tests and fixture-based INI compatibility tests.

## Outstanding audit
- [ ] Exact member types, visibility, initializers, and property accessors.
- [ ] Every function body's branches, loops, side effects, and error behavior.
- [ ] Callers, serializers, and game-specific data sources.
- [ ] TD/RA differences and test fixtures.
- [ ] Rust implementation paths and parity tests.
