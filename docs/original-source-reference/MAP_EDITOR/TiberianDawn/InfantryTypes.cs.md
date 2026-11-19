# InfantryTypes.cs

**Status: Body reviewed.** Full file body read at pinned upstream commit f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae. Rust mapping, tests and parity are separate statuses.

[Exact upstream source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/InfantryTypes.cs)  
Blob SHA: cf55aa69fc0a1be2aefe050590425466285bd23b

## Catalog

| ID | Field | Name token | Display key | Owner |
|---|---|---|---|---|
| 0 | E1 | e1 | TEXT_UNIT_TITLE_GDI_MINIGUNNER | Goodguy |
| 1 | E2 | e2 | TEXT_UNIT_TITLE_GDI_GRENADIER | Goodguy |
| 2 | E3 | e3 | TEXT_UNIT_TITLE_GDI_ROCKET_SOLDIER | Badguy |
| 3 | E4 | e4 | TEXT_UNIT_TITLE_NOD_FLAMETHROWER | Badguy |
| 4 | E5 | e5 | TEXT_UNIT_TITLE_NOD_CHEM_WARRIOR | Badguy |
| 5 | E7 | e6 | TEXT_UNIT_TITLE_GDI_ENGINEER | Goodguy |
| 6 | Commando | rmbo | TEXT_UNIT_TITLE_GDI_COMMANDO | Goodguy |
| 7 | C1 | c1 | TEXT_UNIT_TITLE_CIV1 | Neutral |
| 8 | C2 | c2 | TEXT_UNIT_TITLE_CIV2 | Neutral |
| 9 | C3 | c3 | TEXT_UNIT_TITLE_CIV3 | Neutral |
| 10 | C4 | c4 | TEXT_UNIT_TITLE_CIV4 | Neutral |
| 11 | C5 | c5 | TEXT_UNIT_TITLE_CIV5 | Neutral |
| 12 | C6 | c6 | TEXT_UNIT_TITLE_CIV6 | Neutral |
| 13 | C7 | c7 | TEXT_UNIT_TITLE_CIV7 | Neutral |
| 14 | C8 | c8 | TEXT_UNIT_TITLE_CIV8 | Neutral |
| 15 | C9 | c9 | TEXT_UNIT_TITLE_CIV9 | Neutral |
| 16 | C10 | c10 | TEXT_UNIT_TITLE_C10 | Neutral |
| 17 | Moebius | moebius | TEXT_UNIT_TITLE_MOEBIUS | Neutral |
| 18 | Delphi | delphi | TEXT_UNIT_TITLE_DELPHI | Neutral |
| 19 | DrChan | chan | TEXT_UNIT_TITLE_CHAN | Neutral |

20 entries, sbyte IDs 0–19. Field E7 has ID 5 but Name token e6 (GDI Engineer); E5 is ID4/name e5 and there is no E6 field. Constructors eagerly resolve localization keys. Reflection builds the private readonly Types array from public readonly static fields without explicit sorting; GetTypes returns it. Preserve the field alias separately from serialized Name and test exact lookup.
