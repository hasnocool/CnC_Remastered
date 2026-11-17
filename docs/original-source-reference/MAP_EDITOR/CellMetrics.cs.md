# CellMetrics.cs — rectangular cell geometry

> **Source status: Body reviewed. Rust mapping: partial, concrete implementation exists.** The complete 4,694-byte source body was inspected. This status does not claim API or edge-case parity.

- Upstream source: [CnCTDRAMapEditor/Model/CellMetrics.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/CellMetrics.cs)
- Upstream blob: c426886dced5a8d0080614771125fa8a11360682
- Namespace: MobiusEditor.Model
- Imports: System, System.Collections.Generic, System.Drawing, System.Linq, System.Text, System.Threading.Tasks. The last three imports are unused in this file.

## Complete member contract

- Static readonly AdjacentFacings : FacingType[] contains North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest in that order; None is not included.
- Width:int and Height:int have public getters and private setters.
- TopLeft:Point always returns Point.Empty.
- Size:Size returns Width/Height.
- Bounds:Rectangle is constructed from TopLeft and Size.
- Length:int is Width * Height.
- Contains(Point):bool checks 0 <= X < Width and 0 <= Y < Height.
- Contains(int):bool checks 0 <= cell < Length.
- CellMetrics(int width, int height) stores both values without validation.
- CellMetrics(Size size) delegates to the integer constructor.
- GetCell(Point location, out int cell):bool computes cell = location.Y * Width + location.X first, then returns Contains(location). The out value is populated even for an invalid point.
- GetLocation(int cell, out Point location):bool computes (cell % Width, cell / Width) first, then returns Contains(cell). Width zero therefore throws DivideByZeroException before the bounds result can be returned.
- Adjacent(Point location, FacingType facing, out Point adjacent):bool starts with adjacent = location, applies one of eight cardinal/diagonal offsets, and finally checks Contains(adjacent). FacingType.None has no switch arm, so it leaves the location unchanged and succeeds when that original point is in bounds.
- Adjacent(int cell, FacingType facing, out int adjacent):bool first calls GetLocation; on either invalid input or invalid neighbor it writes -1 and returns false; otherwise it converts the neighbor back with GetCell.
- Clip(ref Point location):void clamps X/Y independently to [0, Width-1] and [0, Height-1].
- Clip(ref Point location, Size margin):void delegates to the asymmetric overload using the same margin for both sides.
- Clip(ref Point location, Size topLeftMargin, Size bottomRightMargin):void clamps X to [left margin, Width - right margin - 1] and Y to [top margin, Height - bottom margin - 1].

## Edge behavior and risks

There is no constructor validation for zero/negative dimensions. Reverse index conversion divides by Width. Clip assumes a non-empty grid and sensible margins; if the requested lower bound exceeds the upper bound, the nested Math.Max/Math.Min expression does not report an error and may return a point outside the intended grid. GetCell still returns a flattened integer for an invalid point, so callers must honor its bool result. Row-major indexing is Y * Width + X.

## Rust implementation comparison

- Concrete path: [crates/map-editor-model/src/cell_metrics.rs](https://github.com/hasnocool/CnC_Remastered/blob/main/crates/map-editor-model/src/cell_metrics.rs), using rts_core::CellCoord, Facing, MapDimensions, and MapError.
- The Rust implementation provides dimensions, length, contains, get_cell/get_location, eight-direction adjacency, and non-margin clipping. Its adjacent(None) explicitly returns the same coordinate when in bounds, matching the C# None fall-through.
- Rust construction returns Result and delegates dimension validity to MapDimensions::new; C# accepts raw integers.
- The Rust implementation does not expose TopLeft/Bounds or either margin-aware Clip overload.
- Existing Rust source contains tests for row-major conversion, all eight directions plus None, edge rejection, and clipping. Their presence is not evidence that they were executed in this audit run.

## Verification

Full C# body and current Rust implementation were read. No tests were executed; no Complete or Tested status is claimed.
