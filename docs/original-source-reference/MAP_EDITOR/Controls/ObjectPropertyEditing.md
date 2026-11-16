# Object property editing — model binding and nested popup contracts

Pinned upstream: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae. Detailed guides: [ObjectProperties](ObjectProperties.cs.md), [PropertiesComboBox](PropertiesComboBox.cs.md), and [TypeItem<T>](../TypeItem.cs.md).

## Control relationship

ObjectProperties is the typed property editor for Infantry, Unit, and Building. ObjectPropertiesPopup creates a fresh ObjectProperties control for a selected live model object and hosts it in a ToolStripDropDown. PropertiesComboBox temporarily disables the outer popup's AutoClose while a nested selector list is open. TypeItem<T> adapts display names to strongly typed HouseType and DirectionType values.

ObjectProperties subscribes to the current model's PropertyChanged event and removes that subscription when Object changes or the popup closes. It separately subscribes to Map.Triggers.CollectionChanged and removes that subscription when the control is disposed.

## Type-specific fields

- Infantry: House, Strength, Direction (Facing.None excluded), Mission, Trigger.
- Unit: House, Strength, Direction, Mission, Trigger.
- Building: House, Strength, Direction only when turret-bearing, Trigger, BasePriority, IsPrebuilt; Sellable and Rebuild are visible only in Red Alert.

The control binds selector values directly to model properties. House and Direction use TypeItem<T> wrappers with DisplayMember=Name and ValueMember=Type. Mission and Trigger use selected items.

## Dirty and lifecycle behavior

ObjectProperties marks Plugin.Dirty from Obj_PropertyChanged unless editing a mock object. Setting IsPrebuilt=false also normalizes the Building.House to BasicSection.BasePlayer, falling back to the first HouseType. Popup close clears Object and unsubscribes from the model.

TerrainProperties is a separate control with a similarly named Obj_PropertyChanged method, but its inspected designer/code-behind do not subscribe Terrain.PropertyChanged. That contrast is documented in [TerrainProperties.cs.md](TerrainProperties.cs.md).

## Findings

- Rebinding the same ObjectProperties instance from a turretless Building to Infantry or Unit can leave Direction hidden because those branches do not explicitly set Visible=true.
- The IsPrebuilt=false fallback throws if HouseTypes is empty.
- Trigger selectors use distinct display names rather than stable IDs.
- PropertiesComboBox does not restore AutoClose if its nested dropdown is disposed without OnDropDownClosed.
- ObjectProperties.resx has four headers and no data/metadata entries. PropertiesComboBox has no .resx file.

No Rust source, build, or UI tests were changed in this tranche.
