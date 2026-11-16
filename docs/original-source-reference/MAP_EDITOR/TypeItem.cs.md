# TypeItem<T> — display-name/value adapter for property selectors

Pinned upstream: [TypeItem.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/TypeItem.cs).  
Pinned blob: e2bb0fab754e601a97101d9b22e45984c0366908. Source length: 30 lines.  
Audit status: **Body reviewed**. The complete generic type and constructor were inspected. No Rust mapping or test is claimed.

## Exact type

TypeItem<T> is a public generic class in MobiusEditor.Model with no generic constraints.

Members:
- public string Name { get; private set; }
- public T Type { get; private set; }
- public TypeItem(string name, T type) assigns Name and Type directly.

There is no null validation, equality override, display formatting override, conversion method, serialization contract, or notification event. Equality therefore remains reference equality for TypeItem instances unless the generic T value is compared by consumer code.

## Consumers and binding contract

ObjectProperties.Initialize creates TypeItem<HouseType> values from Map.Houses using the HouseType.Name and HouseType instance. Rebind creates TypeItem<DirectionType> values from Map.DirectionTypes, sometimes filtering out Facing.None for Infantry.

ObjectProperties.Designer configures the relevant PropertiesComboBox controls with DisplayMember=Name and ValueMember=Type. The UI displays the string while SelectedValue binds the underlying strongly typed HouseType or DirectionType to the model.

TypeComboBox uses the same wrapper pattern for editor catalogs. The wrapper is therefore a UI adapter, not a game-domain type and not a serialized map record.

## Rust port requirements

Use an explicit selector item such as { display_name, value_id } or a typed pair. Keep display text separate from stable model identity. Do not serialize TypeItem itself, and do not rely on object reference identity across reloads.

No Rust source or tests were changed in this tranche.
