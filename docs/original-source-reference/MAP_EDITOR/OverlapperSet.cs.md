# OverlapperSet.cs — transitive rectangle-overlap index

> **Source status: Body reviewed. Rust mapping: none found in map-editor-model.** Full 4,105-byte source body inspected at the pinned upstream commit.

- Upstream source: [CnCTDRAMapEditor/Model/OverlapperSet.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/OverlapperSet.cs)
- Upstream blob: 492f545bbd9d1c4cbb427f7eca4d726a72cb554d
- Direct interface dependency: ICellOverlapper.OverlapBounds : Rectangle.
- Utility dependencies: Rectangle.Points() and single-item Yield() from ExtensionMethods.

## State and API

OverlapperSet<T> is constrained to T : class, ICellOverlapper and implements IEnumerable<(Point Location,T Overlapper)> plus non-generic IEnumerable.

- Private readonly metrics:CellMetrics.
- Private readonly overlappers:Dictionary<T,Rectangle>, storing each object's absolute overlap rectangle.
- Indexer this[T] returns nullable Rectangle; Overlappers exposes dictionary keys.
- Constructor stores metrics.
- Add(Point,T) rejects null or duplicate objects, reads overlapper.OverlapBounds, offsets that rectangle by the supplied anchor, and stores it. It does not clip to map bounds, test collisions, or require the anchor to be valid.
- Add(x,y,T) delegates to Point. Add(cell,T) first resolves the cell through CellMetrics and returns false for an invalid cell.
- Clear empties the dictionary. Contains tests dictionary membership.
- CopyTo iterates this set and calls Add on the destination. The tuple local names are Location and Occupier in source, but the second value is the overlapper.
- GetEnumerator projects each stored Rectangle.Location and object.
- Remove rejects null/unregistered objects, then removes the dictionary entry. The retrieved Rectangle local is not otherwise used.
- OfType<U>() filters by runtime type and casts.

## Overlap query algorithm

Overlaps(IEnumerable<Rectangle>) first creates a HashSet<Rectangle> from the supplied rectangles. It then repeats:
1. Capture the current set count.
2. Find every stored overlap rectangle that intersects any rectangle already in the set.
3. Union those stored rectangles into the set.
4. Stop when the set count no longer changes.

This computes the transitive closure of intersecting stored rectangles: a rectangle that does not intersect the original query can still be included if it intersects a rectangle discovered in an earlier iteration. The final result expands every rectangle in the closure into individual points via Rectangle.Points() and returns a HashSet<Point>.

Overloads accept one Rectangle, a sequence of Points (converted to 1x1 rectangles), or one Point (converted through Yield). Input rectangles/points and stored overlap rectangles are not clipped to CellMetrics. The result can therefore include negative or off-map coordinates.

## Map integration

Map maintains Overlappers for ICellOverlapper technos. Technos_OccupierAdded/Removed update this index immediately outside a Map.BeginUpdate/EndUpdate batch; during a batch they set invalidateOverlappers and Map.Update rebuilds the set from all Technos after the batch. Building and Terrain implement ICellOverlapper; InfantryGroup reports a 3x3 rectangle offset by (-1,-1).

## Rust comparison and requirements

No equivalent rectangle-overlap index is present in crates/map-editor-model. A Rust port needs an explicit rectangle type, object identity, and deterministic transitive closure. Tests should cover chains A↔B↔C, edge-touch semantics as defined by Rectangle.IntersectsWith, duplicate/null additions, invalid anchors, negative rectangles, and no map clipping.

## Verification

Complete source body inspected. No Rust implementation mapping or tests were executed.
