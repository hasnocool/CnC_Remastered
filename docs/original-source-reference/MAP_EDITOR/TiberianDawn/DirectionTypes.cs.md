# DirectionTypes.cs

**Status: Body reviewed.** Full file body read at pinned upstream commit f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae. Rust mapping, tests and parity are separate statuses.

[Exact upstream source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/DirectionTypes.cs)  
Blob SHA: 748409dfda37066c02437fd7d3f851a38f69adf4

## Direction table

| ID | Field | Name | Facing |
|---|---|---|---|
| 0 | North | North | North |
| 16 | NorthNorthEast | North-NorthEast | None |
| 32 | NorthEast | NorthEast | NorthEast |
| 48 | EastNorthEast | East-NorthEast | None |
| 64 | East | East | East |
| 80 | EastSouthEast | East-SouthEast | None |
| 96 | SouthEast | SouthEast | SouthEast |
| 112 | SouthSouthEast | South-SouthEast | None |
| 128 | South | South | South |
| 144 | SouthSouthWest | South-SouthWest | None |
| 160 | SouthWest | SouthWest | SouthWest |
| 176 | WestSouthWest | West-SouthWest | None |
| 192 | West | West | West |
| 208 | WestNorthWest | West-NorthWest | None |
| 224 | NorthWest | NorthWest | NorthWest |
| 240 | NorthNorthWest | North-NorthWest | None |

## Static construction

16 public static readonly DirectionType fields. Static constructor reflects public static fields, filters readonly DirectionType values, and materializes the private Types array without explicit sorting. GetTypes returns the backing array as IEnumerable. Intermediate entries use FacingType.None. Rust should use an explicit ordered table and test each byte ID/name/facing; TD and RA catalogs are identical.
