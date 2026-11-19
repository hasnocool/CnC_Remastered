# InfantryTypes.cs

**Status: Body reviewed.** Full file body read at pinned upstream commit f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae. Rust mapping, tests and parity are separate statuses.

[Exact upstream source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/InfantryTypes.cs)  
Blob SHA: 63f9f891356f50a24a91d6d3278d5c9abfe6c2ca

## Catalog

| ID | Field | Name token | Display key | Owner |
|---|---|---|---|---|
| 0 | E1 | e1 | TEXT_UNIT_RA_E1 | Greece |
| 1 | E2 | e2 | TEXT_UNIT_RA_E2 | USSR |
| 2 | E3 | e3 | TEXT_UNIT_RA_E3 | Greece |
| 3 | E4 | e4 | TEXT_UNIT_RA_E4 | USSR |
| 4 | E6 | e6 | TEXT_UNIT_RA_E6 | USSR |
| 5 | E7 | e7 | TEXT_UNIT_RA_E7 | USSR |
| 6 | E8 | spy | TEXT_UNIT_RA_SPY | Greece |
| 7 | E9 | thf | TEXT_UNIT_RA_THF | Greece |
| 8 | Medic | medi | TEXT_UNIT_RA_MEDI | Greece |
| 9 | General | gnrl | TEXT_UNIT_RA_GNRL | Greece |
| 10 | Dog | dog | TEXT_UNIT_RA_DOG | USSR |
| 11 | C1 | c1 | TEXT_UNIT_TITLE_CIV1 | Neutral |
| 12 | C2 | c2 | TEXT_UNIT_TITLE_CIV2 | Neutral |
| 13 | C3 | c3 | TEXT_UNIT_TITLE_CIV3 | Neutral |
| 14 | C4 | c4 | TEXT_UNIT_TITLE_CIV4 | Neutral |
| 15 | C5 | c5 | TEXT_UNIT_TITLE_CIV5 | Neutral |
| 16 | C6 | c6 | TEXT_UNIT_TITLE_CIV6 | Neutral |
| 17 | C7 | c7 | TEXT_UNIT_TITLE_CIV7 | Neutral |
| 18 | C8 | c8 | TEXT_UNIT_TITLE_CIV8 | Neutral |
| 19 | C9 | c9 | TEXT_UNIT_TITLE_CIV9 | Neutral |
| 20 | C10 | c10 | TEXT_UNIT_RA_SCIENTIST | Neutral |
| 21 | Einstein | einstein | TEXT_UNIT_RA_EINSTEIN | Neutral |
| 22 | Delphi | delphi | TEXT_UNIT_RA_DELPHI | Neutral |
| 23 | DrChan | chan | TEXT_UNIT_TITLE_CHAN | Neutral |
| 24 | ShockTrooper | shok | TEXT_UNIT_RA_SHOK | USSR |
| 25 | Mechanic | mech | TEXT_UNIT_RA_MECH | Greece |

26 entries with contiguous sbyte IDs 0–25. Constructors eagerly resolve DisplayName via Globals.TheGameTextManager[textId]. The private readonly Types array is reflection-populated from public readonly static InfantryType fields without explicit sort; GetTypes returns the backing array. Preserve tokens, localization keys and owner labels. Rust should resolve keys after localization initialization and fixture-test every entry.
