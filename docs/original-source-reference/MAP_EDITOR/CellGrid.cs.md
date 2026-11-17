# CellGrid.cs — evented generic cell storage

> **Source status: Body reviewed. Rust mapping: partial at the document-storage level only.** Complete 5,703-byte C# body inspected at the pinned source commit.

- Upstream source: [CnCTDRAMapEditor/Model/CellGrid.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/CellGrid.cs)
- Upstream blob: e4097ee09d41865369add855f06645f84cc6d09e
- Namespace: MobiusEditor.Model
- Imports: System, System.Collections, System.Collections.Generic, System.Drawing.

## Types and fields

FacingType enum order/value: None=0, North=1, NorthEast=2, East=3, SouthEast=4, South=5, SouthWest=6, West=7, NorthWest=8.

CellChangedEventArgs<T> : EventArgs exposes four public readonly fields: Cell:int, Location:Point, OldValue:T, Value:T. Its (CellMetrics,int,T,T) constructor stores the supplied cell and derives Location via GetLocation; its (CellMetrics,Point,T,T) constructor stores the supplied location and derives Cell via GetCell. Both ignore the bool result from the conversion, so invalid input still produces an event-args object with the conversion's out value.

CellGrid<T> implements IEnumerable<(int Cell,T Value)> and non-generic IEnumerable. Private readonly fields are metrics:CellMetrics and cells:T[,]. Public indexers:
- this[int x,int y] reads cells[y,x]. Setter compares with EqualityComparer<T>.Default; if unequal, saves old value, writes the new value, then raises CellChanged with point-based event args.
- this[Point location] delegates to the x/y indexer.
- this[int cell] maps to x=cell % metrics.Width and y=cell / metrics.Width, then delegates to x/y.
- Size and Length delegate to metrics.Size and metrics.Length.
- Events: CellChanged : EventHandler<CellChangedEventArgs<T>> and Cleared : EventHandler<EventArgs>.

## Constructors and methods

- CellGrid(CellMetrics metrics) stores the metrics reference and allocates a rectangular T[metrics.Height, metrics.Width].
- Clear() calls Array.Clear over the complete backing array and always raises Cleared, including when already empty. It does not raise one CellChanged event per former element.
- Adjacent(Point,FacingType):T returns the neighbor's value if metrics.Adjacent succeeds, otherwise default(T).
- Adjacent(int,FacingType):T first resolves the cell; invalid cells return default(T), otherwise it performs point adjacency and returns default(T) for an invalid neighbor.
- CopyTo(CellGrid<T> other):bool compares only Length. Different width/height shapes with equal element counts are accepted. It copies every flattened index through the destination indexer, so destination CellChanged events may fire. It returns false only when lengths differ.
- OnCellChanged and OnCleared are protected virtual event dispatchers.
- IntersectsWith(ISet<int>) iterates the supplied set, keeps in-bounds cell IDs, reads each value, and yields only non-null values as (cell,value).
- IntersectsWith(ISet<Point>) does the same for in-bounds points, deriving the flattened cell ID for each yielded item.
- GetEnumerator() scans flattened indices from zero to Length-1 and yields only non-null values. The explicit non-generic enumerator delegates to it.

## Behavioral details

The grid is row-major through CellMetrics. Direct indexers do not perform an explicit bounds check; the backing array/index arithmetic supplies the eventual exception behavior. A flattened index with Width zero fails during modulo/division. Event handlers run synchronously after the stored cell has changed. The event args constructors do not reject failed coordinate conversions. Enumeration and intersection filtering use value != null; empty/default cells are omitted.

## Rust comparison

The current MapDocument stores Vec<CellLayers> and accesses it through MapDimensions.index. That provides flattened storage and bounds-aware cell access, but there is no generic CellGrid<T>, synchronous CellChanged/Cleared event contract, adjacency-by-value helper, intersection iterator, or shape-only CopyTo behavior. Rust MapDocument::set_cell is a direct mutation returning Result; it is not an evented grid.

## Port requirements

Keep the data store independent of UI callbacks. If editor observers are required, emit explicit change records after a successful mutation and define batch/clear semantics. Decide whether shape compatibility must be width-and-height equality rather than reproducing the C# same-Length-only copy rule. Add tests for redundant writes, clear-on-empty, invalid event coordinates, and same-area/different-shape copies.

## Verification

Complete source body inspected. No runtime tests or Rust parity tests were executed.
