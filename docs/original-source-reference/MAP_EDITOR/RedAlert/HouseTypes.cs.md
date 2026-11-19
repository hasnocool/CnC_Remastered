# HouseTypes.cs

**Status: Body reviewed.** Full file body read at pinned upstream commit f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae. Rust mapping, tests and parity are separate statuses.

[Exact upstream source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/HouseTypes.cs)  
Blob SHA: af4f886fd39ea0ad220674df21d55ba07137d8d8

## Catalog

| ID | Field | Name | Unit color | Building color |
|---|---|---|---|---|
| 0 | Spain | Spain | SPAIN | SPAIN |
| 1 | Greece | Greece | GREECE | GREECE |
| 2 | USSR | USSR | USSR | USSR |
| 3 | England | England | ENGLAND | ENGLAND |
| 4 | Ukraine | Ukraine | UKRAINE | UKRAINE |
| 5 | Germany | Germany | GERMANY | GERMANY |
| 6 | France | France | FRANCE | FRANCE |
| 7 | Turkey | Turkey | TURKEY | TURKEY |
| 8 | Good | GoodGuy | GOOD | GOOD |
| 9 | Bad | BadGuy | BAD | BAD |
| 10 | Neutral | Neutral | null | null |
| 11 | Special | Special | null | null |
| 12 | Multi1 | Multi1 | MULTI1 | MULTI1 |
| 13 | Multi2 | Multi2 | MULTI2 | MULTI2 |
| 14 | Multi3 | Multi3 | MULTI3 | MULTI3 |
| 15 | Multi4 | Multi4 | MULTI4 | MULTI4 |
| 16 | Multi5 | Multi5 | MULTI5 | MULTI5 |
| 17 | Multi6 | Multi6 | MULTI6 | MULTI6 |
| 18 | Multi7 | Multi7 | MULTI7 | MULTI7 |
| 19 | Multi8 | Multi8 | MULTI8 | MULTI8 |

20 public static readonly HouseType fields, sbyte IDs. Private readonly Types is reflection-populated from public static readonly HouseType fields without explicit sort; GetTypes returns the backing array as IEnumerable.

GetBasePlayer returns GoodGuy for USSR, Ukraine or BadGuy; otherwise BadGuy. HouseType.Equals(string) compares Name case-insensitively. Preserve faction houses and aliases separately.
