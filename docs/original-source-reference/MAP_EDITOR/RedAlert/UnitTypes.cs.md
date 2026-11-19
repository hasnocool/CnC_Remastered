# UnitTypes.cs

**Status: Body reviewed.** Full file body read at pinned upstream commit f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae. Rust mapping, tests and parity are separate statuses.

[Exact upstream source](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/UnitTypes.cs)  
Blob SHA: f19c63378e3488ac1dd18958c3fa78413727b34f

36 entries: 22 ground IDs 0–21, 7 aircraft IDs 32–38, and 7 vessels IDs 64–70. UnitTypeIDMask is sbyte: Aircraft=32 (bit 5), Vessel=64 (bit 6). Category bits distinguish reused raw indices.

### Ground

| ID | Field | Name | Text key | Owner | Turret |
|---|---|---|---|---|---|
| 0 | HTank | 4tnk | TEXT_UNIT_RA_4TNK | USSR | 1 |
| 1 | MTank | 3tnk | TEXT_UNIT_RA_3TNK | USSR | 1 |
| 2 | MTank2 | 2tnk | TEXT_UNIT_RA_2TNK | Greece | 1 |
| 3 | LTank | 1tnk | TEXT_UNIT_RA_1TNK | Greece | 1 |
| 4 | APC | apc | TEXT_UNIT_RA_APC | Greece | 0 |
| 5 | MineLayer | mnly | TEXT_UNIT_RA_MNLY | Greece | 0 |
| 6 | Jeep | jeep | TEXT_UNIT_RA_JEEP | Greece | 1 |
| 7 | Harvester | harv | TEXT_UNIT_RA_HARV | Greece | 0 |
| 8 | Arty | arty | TEXT_UNIT_RA_ARTY | Greece | 0 |
| 9 | MRJammer | mrj | TEXT_UNIT_RA_MRJ | Greece | 0 |
| 10 | MGG | mgg | TEXT_UNIT_RA_MGG | Greece | 0 |
| 11 | MCV | mcv | TEXT_UNIT_RA_MCV | Greece | 0 |
| 12 | V2Launcher | v2rl | TEXT_UNIT_RA_V2RL | USSR | 0 |
| 13 | ConvoyTruck | truk | TEXT_UNIT_RA_TRUK | Greece | 0 |
| 14 | Ant1 | ant1 | TEXT_UNIT_RA_ANT1 | Special | 0 |
| 15 | Ant2 | ant2 | TEXT_UNIT_RA_ANT2 | Special | 0 |
| 16 | Ant3 | ant3 | TEXT_UNIT_RA_ANT3 | Special | 0 |
| 17 | Chrono | ctnk | TEXT_UNIT_RA_CTNK | Greece | 0 |
| 18 | Tesla | ttnk | TEXT_UNIT_RA_TTNK | USSR | 0 |
| 19 | MAD | qtnk | TEXT_UNIT_RA_QTNK | Greece | 0 |
| 20 | DemoTruck | dtrk | TEXT_UNIT_RA_DTRK | USSR | 0 |
| 21 | Phase | stnk | TEXT_UNIT_RA_STNK | Greece | 1 |

Turret and Fixed-wing columns are Boolean constructor arguments shown as 1=true and 0=false.

### Aircraft

| ID | Field | Name | Text key | Owner | Fixed wing |
|---|---|---|---|---|---|
| 32 | Tran | tran | TEXT_UNIT_RA_TRAN | USSR | 0 |
| 33 | Badr | badr | TEXT_UNIT_RA_BADR | USSR | 1 |
| 34 | U2 | u2 | TEXT_UNIT_RA_U2 | USSR | 1 |
| 35 | Mig | mig | TEXT_UNIT_RA_MIG | USSR | 1 |
| 36 | Yak | yak | TEXT_UNIT_RA_YAK | USSR | 1 |
| 37 | Heli | heli | TEXT_UNIT_RA_HELI | Greece | 0 |
| 38 | Hind | hind | TEXT_UNIT_RA_HIND | USSR | 0 |

### Vessels

| ID | Field | Name | Text key | Owner | Turret |
|---|---|---|---|---|---|
| 64 | Submarine | ss | TEXT_UNIT_RA_SS | USSR | 0 |
| 65 | Destroyer | dd | TEXT_UNIT_RA_DD | Greece | 1 |
| 66 | Cruiser | ca | TEXT_UNIT_RA_CA | Greece | 1 |
| 67 | Transport | lst | TEXT_UNIT_RA_LST | Greece | 0 |
| 68 | PTBoat | pt | TEXT_UNIT_RA_PT | Greece | 1 |
| 69 | MissileSubmarine | msub | TEXT_UNIT_RA_MSUB | USSR | 0 |
| 70 | Carrier | carr | TEXT_UNIT_RA_CARR | Greece | 0 |

Static Types is reflection-populated from public readonly fields without explicit sort; GetTypes returns the backing array. Constructors eagerly resolve localization. Keep turret and fixed-wing metadata distinct; model occupancy (1×1) and overlap (3×3) are separate. Rust must fixture all 36 entries.
