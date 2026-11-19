# UnitTypes.cs

**Status: Body reviewed.** Full file body read at pinned upstream commit f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae. Rust mapping, tests and parity are separate statuses.

[Exact upstream source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/UnitTypes.cs)  
Blob SHA: 184cf7b18354d06499726577a8f81a36e89a4c45

27 entries: 22 ground IDs 0–21 and 5 aircraft IDs 32–36; no vessel entries.

### Ground

| ID | Field | Name | Text key | Owner | Turret |
|---|---|---|---|---|---|
| 0 | HTank | htnk | TEXT_UNIT_TITLE_GDI_MAMMOTH_TANK | Goodguy | 1 |
| 1 | MTank | mtnk | TEXT_UNIT_TITLE_GDI_MED_TANK | Goodguy | 1 |
| 2 | LTank | ltnk | TEXT_UNIT_TITLE_NOD_LIGHT_TANK | Badguy | 1 |
| 3 | STank | stnk | TEXT_UNIT_TITLE_NOD_STEALTH_TANK | Badguy | 0 |
| 4 | FTank | ftnk | TEXT_UNIT_TITLE_NOD_FLAME_TANK | Badguy | 0 |
| 5 | Visceroid | vice | TEXT_UNIT_TITLE_VICE | Special | 0 |
| 6 | APC | apc | TEXT_UNIT_TITLE_GDI_APC | Goodguy | 0 |
| 7 | MLRS | msam | TEXT_UNIT_TITLE_GDI_MRLS | Goodguy | 1 |
| 8 | Jeep | jeep | TEXT_UNIT_TITLE_GDI_HUMVEE | Goodguy | 1 |
| 9 | Buggy | bggy | TEXT_UNIT_TITLE_NOD_NOD_BUGGY | Badguy | 1 |
| 10 | Harvester | harv | TEXT_UNIT_TITLE_GDI_HARVESTER | Goodguy | 0 |
| 11 | Arty | arty | TEXT_UNIT_TITLE_NOD_ARTILLERY | Badguy | 0 |
| 12 | SAM | mlrs | TEXT_UNIT_TITLE_GDI_MLRS | Goodguy | 1 |
| 13 | Hover | lst | TEXT_UNIT_TITLE_LST | Goodguy | 0 |
| 14 | MHQ | mhq | TEXT_UNIT_TITLE_MHQ | Goodguy | 0 |
| 15 | GunBoat | boat | TEXT_UNIT_TITLE_WAKE | Goodguy | 1 |
| 16 | MCV | mcv | TEXT_UNIT_TITLE_GDI_MCV | Goodguy | 0 |
| 17 | Bike | bike | TEXT_UNIT_TITLE_NOD_RECON_BIKE | Badguy | 0 |
| 18 | Tric | tric | TEXT_UNIT_TITLE_TRIC | Special | 0 |
| 19 | Trex | trex | TEXT_UNIT_TITLE_TREX | Special | 0 |
| 20 | Rapt | rapt | TEXT_UNIT_TITLE_RAPT | Special | 0 |
| 21 | Steg | steg | TEXT_UNIT_TITLE_STEG | Special | 0 |

Turret and Fixed-wing columns are Boolean constructor arguments shown as 1=true and 0=false.

### Aircraft

| ID | Field | Name | Text key | Owner | Fixed wing |
|---|---|---|---|---|---|
| 32 | Tran | tran | TEXT_UNIT_TITLE_GDI_TRANSPORT | Goodguy | 0 |
| 33 | A10 | a10 | TEXT_UNIT_TITLE_A10 | Goodguy | 1 |
| 34 | Heli | heli | TEXT_UNIT_TITLE_NOD_HELICOPTER | Badguy | 0 |
| 35 | C17 | c17 | TEXT_UNIT_TITLE_C17 | Badguy | 1 |
| 36 | Orca | orca | TEXT_UNIT_TITLE_GDI_ORCA | Goodguy | 0 |

Private readonly Types is reflection-populated without explicit sorting; GetTypes returns its backing array. UnitType constructors eagerly resolve text keys. Preserve exact ID/name/owner/turret/fixed-wing data, Aircraft mask 32 and the absence of vessels. No Rust TD catalog exists.
