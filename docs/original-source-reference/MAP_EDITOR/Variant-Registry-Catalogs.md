# TD/RA static registry comparison

**Body reviewed for eight variant registry files** at upstream pin f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.

## Catalog sizes

| Catalog | TD | RA |
|---|---:|---:|
| Direction | 16 | 16 |
| Houses | 10 | 20 |
| Infantry | 20 | 26 |
| Units | 27 (22 ground + 5 aircraft) | 36 (22 ground + 7 aircraft + 7 vessels) |

Direction IDs/facing semantics are identical; other catalogs are variant-specific.

## ID encoding

UnitTypeIDMask uses sbyte constants Aircraft=32 and Vessel=64. Ground IDs are 0–21; aircraft are 32–38 (RA) or 32–36 (TD); RA vessels are 64–70. TD has no vessel entries. Preserve category bits because low indices overlap.

## House differences

TD IDs 0–9: GoodGuy, BadGuy, Neutral, Special, Multi1–Multi6. BadGuy unit color BAD_UNIT, building color BAD_STRUCTURE, overrides harv/mcv to BAD_STRUCTURE. TD GetBasePlayer maps only BadGuy to GoodGuy; all else BadGuy.

RA IDs 0–19: Spain, Greece, USSR, England, Ukraine, Germany, France, Turkey, GoodGuy, BadGuy, Neutral, Special, Multi1–Multi8. RA GetBasePlayer maps USSR, Ukraine and BadGuy to GoodGuy; otherwise BadGuy. String name matching is case-insensitive.

## Infantry aliases

TD field E7 is ID5/name e6 (engineer); E5 is ID4/name e5 and no E6 field. RA E8/E9 use name tokens spy/thf. Field identifier, serialized Name and localization key are distinct. TD owner labels Goodguy/Badguy; RA owner values include Greece/USSR.

## Reflection and initialization

All catalogs scan public static fields, filter readonly instances, and capture arrays without explicit sort. GetTypes returns the backing array. DirectionTypes' Types field is not readonly. InfantryType/UnitType constructors eagerly resolve text keys through Globals.TheGameTextManager, so first catalog access depends on localization initialization. Rust should use explicit immutable ordered tables and defer localization.

## Consumers and Rust gap

Both GamePlugin variants pass catalog enumerables to Map and concatenate InfantryTypes and UnitTypes for techno lookup. Object parser branches classify UnitType by IsUnit/IsAircraft and RA IsVessel. Exact initialization call-chain still needs verification. No typed Rust variant catalogs exist. Required tests: exact entries/cardinality, duplicate ID/name validation, lookup parity, House base-player/color mappings, direction bytes, category masks, and serialization fixtures. No tests executed.