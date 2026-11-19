# HouseTypes.cs

**Status: Body reviewed.** Full file body read at pinned upstream commit f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae. Rust mapping, tests and parity are separate statuses.

[Exact upstream source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/HouseTypes.cs)  
Blob SHA: 9c9477bbae056736a85a0bd198fa8602cf8bc0b0

## Catalog

| ID | Field | Name | Unit color | Building color |
|---|---|---|---|---|
| 0 | Good | GoodGuy | GOOD | GOOD |
| 1 | Bad | BadGuy | BAD_UNIT | BAD_STRUCTURE |
| 2 | Neutral | Neutral | null | null |
| 3 | Special | Special | null | null |
| 4 | Multi1 | Multi1 | MULTI1 | MULTI1 |
| 5 | Multi2 | Multi2 | MULTI2 | MULTI2 |
| 6 | Multi3 | Multi3 | MULTI3 | MULTI3 |
| 7 | Multi4 | Multi4 | MULTI4 | MULTI4 |
| 8 | Multi5 | Multi5 | MULTI5 | MULTI5 |
| 9 | Multi6 | Multi6 | MULTI6 | MULTI6 |

BadGuy has per-type overrides harv → BAD_STRUCTURE and mcv → BAD_STRUCTURE. HouseType creates OverrideTeamColors with ToDictionary/default case-sensitive comparer; duplicate keys throw. The 10 public static readonly entries are reflected into a private readonly Types array without explicit sorting; GetTypes returns that backing array.

GetBasePlayer maps BadGuy to GoodGuy and all other inputs to BadGuy (Name string comparison is case-insensitive). Preserve IDs, split colors and overrides; don't collapse to a two-faction enum.
