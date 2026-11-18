# HouseType.cs — body-reviewed editor definition

> **Audit status: Body reviewed.** Complete 83-line source body inspected. Variant catalogs, serialization and Rust mapping remain open.

- Upstream: [CnCTDRAMapEditor/Model/HouseType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/HouseType.cs)
- Pinned source blob: `75a4a14ab3c2ef27a46a1b9ae7a95df4bcc94ce5`
- Namespace/type: `MobiusEditor.Model.HouseType`
- Imports: `System`, `System.Collections.Generic`, `System.Linq`, `System.Text`, `System.Threading.Tasks` (the last two are unused in this body).

## Exact members

| Property | Type | Access |
|---|---|---|
| ID | `sbyte` | public get, private set |
| Name | `string` | public get, private set |
| UnitTeamColor | `string` | public get, private set |
| BuildingTeamColor | `string` | public get, private set |
| OverrideTeamColors | `IDictionary<string,string>` | public get, private set; assigned a Dictionary |

Primary constructor:

```csharp
HouseType(sbyte id, string name, string unitTeamColor,
    string buildingTeamColor, params (string type, string teamColor)[] overrideTeamColors)
```

It stores the scalar values and materializes the tuple array using LINQ `ToDictionary(x => x.type, x => x.teamColor)`. The result is a new dictionary using the default string equality comparer (case-sensitive). Duplicate keys throw during construction; a null key is rejected by Dictionary. No validation or recovery is present.

Forwarding constructors:

- `HouseType(sbyte id, string name, string teamColor)` uses the same color for units and buildings.
- `HouseType(sbyte id, string name)` delegates with a null team color.

## Method bodies

- `Equals(object obj)`: same-type branch uses `this == obj` (reference equality); boxed sbyte compares ID; string compares Name using OrdinalIgnoreCase; otherwise calls base.
- `GetHashCode()`: hashes ID.
- `ToString()`: returns Name.

The type contains no display-name localization, image, rendering, event, or serialization method. It is configuration metadata for ownership and team-color selection.

## Catalog and consumer relationships

TD and RA provide separate `HouseTypes` catalogs. The TD catalog includes Good/Bad/Neutral/Special and multiplayer entries; RA adds its country/faction house definitions and a larger multiplayer set. `House` is the placed/scenario house record that refers to a HouseType. BuildingType, InfantryType and UnitType carry owner-house strings; HouseType's override map allows per-unit-type team-color exceptions.

The catalog declarations were sampled to confirm construction use; complete catalog order, ID coverage, and all color override entries are still pending.

## Rust port requirements

No Rust HouseType equivalent was found in `crates/map-editor-model`. Keep stable house IDs, canonical names, unit/building color values, and per-type overrides in variant-specific immutable catalogs. Choose and document a deterministic case policy for override keys; the C# dictionary is case-sensitive even though other definition-name comparisons are case-insensitive. Validate duplicate and null keys at catalog-load time with explicit errors rather than relying on a constructor exception.

## Verification

- [x] All properties, three constructors and method bodies inspected.
- [x] Dictionary materialization and failure behavior recorded.
- [ ] Full TD/RA catalog and serialization audit.
- [ ] Rust catalog mapping and tests.
- [ ] Color-selection fixture parity.

No Rust code was changed and no test was executed.
