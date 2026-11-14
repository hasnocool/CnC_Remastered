# REDALERT WEAPON.CPP — source guide

> **Audit status: partially reviewed.** This first pass inspects the pinned translation unit, its direct includes, and original source function index. Paired headers, exhaustive symbols, full body explanations, call graphs, and Rust parity remain outstanding.

- Upstream: [REDALERT/WEAPON.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/WEAPON.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation-unit size: 20474 characters.

## Plain-English system purpose

Red Alert weapon-type definitions, INI loading, name lookup, allocation, pointer conversion, and threat compatibility. WeaponTypeClass is data/configuration, distinct from projectile/weapon firing behavior.

## Direct compile-time dependencies

- function.h

## Original function index

- Armor_From_Name — Convert ASCII name into armor type number.
- WeaponTypeClass::As_Pointer — Give a weapon type ID, fetch pointer to weapon type object.
- WeaponTypeClass::Read_INI — Fetch the weapon data from the INI database.
- WeaponTypeClass::WeaponTypeClass — Default constructor for weapon type objects.
- WeaponTypeClass::operator delete — Returns weapon type object back to special heap.
- WeaponTypeClass::operator new — Allocates a weapon type object form the special heap.
- WeaponTypeClass::~WeaponTypeClass — Destructor for weapon type class objects.
- Weapon_From_Name — Conver ASCII name to weapon type ID number.
- WeaponTypeClass::Allowed_Threats — Determine what threats this weapon can address.

These entries preserve the source's own responsibility index. They do not establish all branch behavior, state changes, error paths, return contracts, or resource ownership.

## TD/RA comparison
This translation unit is present in Red Alert; no corresponding Tiberian Dawn file with this exact name was found in the pinned source tree.

## Rust mapping
No equivalent is asserted without inspecting the current Rust implementation and tests. Candidate port concerns: INI defaults, threat masks, weapon identity, and serialization compatibility.

## Outstanding audit checklist
- [ ] Inspect paired header and enumerate all classes, structs, enums, fields, globals, constants, macros, and exact signatures.
- [ ] Explain each function body: parameters, return values, branches, loops, side effects, and failure paths.
- [ ] Trace callers/callees and shared global state.
- [ ] Compare TD/RA implementations where both exist.
- [ ] Map every behavior to Rust path and tests; distinguish implemented, partial, absent, and divergent.
