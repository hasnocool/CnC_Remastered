# DirectionType.cs — body-reviewed editor definition

> **Audit status: Body reviewed.** Complete 72-line source body inspected. Rust mapping and direction parity tests remain open.

- Upstream: [CnCTDRAMapEditor/Model/DirectionType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/DirectionType.cs)
- Pinned source blob: `264bd8b4d2a37fce1511349f32285463ce9735eb`
- Namespace/type: `MobiusEditor.Model.DirectionType`
- Direct import: `System`

## Exact members

| Member | Type | Access |
|---|---|---|
| ID | `byte` | public get, private set |
| Name | `string` | public get, private set |
| Facing | `FacingType` | public get, private set |

Constructors:

- `DirectionType(byte id, string name, FacingType facing)` stores all three arguments.
- `DirectionType(byte id, string name)` delegates with `FacingType.None`.

## Method bodies

- `Equals(object obj)`: for another DirectionType, evaluates `this == obj` (reference equality in this class); for boxed byte, compares ID; for string, compares Name case-insensitively with `StringComparison.OrdinalIgnoreCase`; for FacingType, compares Facing; otherwise delegates to base.
- `GetHashCode()`: returns ID.GetHashCode().
- `ToString()`: returns Name.

No operator overload, mutable state, event, collection, or rendering dependency is declared in this file. Equality is intentionally cross-type: a definition can compare equal to its byte ID, name string, or FacingType value. Two separately constructed DirectionType objects with the same ID/name/facing are not equal by the same-type branch because that branch is reference equality.

## Consumers and dependencies

Object property controls build direction selectors from game-specific direction catalogs; Building/Unit/Infantry instances store a DirectionType. The type is also used by the BuildingType/InfantryType/UnitType preview initialization signatures. The complete TD/RA direction catalog ordering and all serializer consumers remain to be audited in `TiberianDawn/DirectionTypes.cs` and `RedAlert/DirectionTypes.cs`.

## Rust port requirements

No dedicated Rust DirectionType catalog was found in `map-editor-model`. Represent direction ID and facing semantics explicitly; do not serialize the WinForms selector wrapper. Define whether equality is by stable ID rather than accidentally reproducing C# reference equality. Keep display-name lookup and UI selection separate.

## Verification

- [x] Both constructors and all method bodies inspected.
- [ ] TD/RA direction tables and facing conversion parity.
- [ ] Rust mapping and tests.
- [ ] Runtime validation of invalid/duplicate IDs.

No Rust code was changed and no test was executed.
