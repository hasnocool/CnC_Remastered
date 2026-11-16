# ObjectProperties — typed property binding, dirty tracking, and popup lifecycle

Pinned upstream:
- [ObjectProperties.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/ObjectProperties.cs) — blob 27adc9d6d87822168e620e834bd344919c773ffd, 273 lines.
- [ObjectProperties.Designer.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/ObjectProperties.Designer.cs) — blob f4116474b379424faeff3401273d5f5df89cb72d, 346 lines.
- [ObjectProperties.resx](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/ObjectProperties.resx) — blob 1af7de150c99c12dd67a509fe57c10d63e4eeb04, 120 lines.

Audit statuses: code-behind **Body reviewed**; generated designer **Header reviewed**; resource file **Resource reviewed**. No Rust mapping, build, UI test, or parity test is claimed.

## State and public contract

ObjectProperties derives from UserControl. It stores:
- bool isMockObject;
- IGamePlugin Plugin with private setter;
- INotifyPropertyChanged obj;
- public INotifyPropertyChanged Object property.

Object setter compares references, unsubscribes Obj_PropertyChanged from the previous object, stores the new object, subscribes to its PropertyChanged event, and calls Rebind. Setting Object to null therefore detaches the previous model event and clears all bindings through Rebind.

## Exact method behavior

| Method / exact signature | Behavior |
|---|---|
| ObjectProperties() | Calls InitializeComponent. |
| Initialize(IGamePlugin plugin, bool isMockObject) | Stores mock status and plugin, subscribes plugin.Map.Triggers.CollectionChanged, binds House choices as TypeItem<HouseType> wrappers, binds MissionTypes, populates Trigger choices, and attaches a Disposed handler that sets Object=null and unsubscribes from the captured map's trigger collection. |
| Triggers_CollectionChanged(object sender, NotifyCollectionChangedEventArgs e) | Calls UpdateDataSource. |
| UpdateDataSource() | Replaces triggerComboBox.DataSource with Trigger.None followed by distinct trigger names from Plugin.Map.Triggers. Names retain source enumeration order; they are not sorted. |
| Rebind() | Clears DataBindings from House, Strength, Direction, Mission, Trigger, BasePriority, Prebuilt, Sellable, and Rebuild controls. Returns for null Object. For Infantry, Unit, or Building, configures type-specific data sources, visibility, enabled state, and bindings, then adds common House/Strength/Direction/Trigger bindings. |
| Obj_PropertyChanged(object sender, PropertyChangedEventArgs e) | On Type, reruns Rebind. On BasePriority, enables Prebuilt only when BasePriority>=0. On IsPrebuilt, when false, selects BasicSection.BasePlayer or falls back to the first HouseType, writes that House to the Building, and enables/disables the House selector. Finally marks Plugin.Dirty unless editing a mock object. |
| comboBox_SelectedValueChanged(object sender, EventArgs e) | Writes every Binding on the sender ComboBox immediately. |
| nud_ValueChanged(object sender, EventArgs e) | Writes every Binding on the sender NumericUpDown immediately. |
| checkBox_CheckedChanged(object sender, EventArgs e) | Writes every Binding on the sender CheckBox immediately. |

## Type-specific binding matrix

| Model type | House | Direction | Mission | Base priority / prebuilt | Sellable / rebuild |
|---|---|---|---|---|---|
| Infantry | Enabled; SelectedValue bound to House | DataSource excludes Facing.None | Visible; SelectedItem bound | Hidden | Hidden |
| Unit | Enabled; SelectedValue bound to House | All DirectionTypes | Visible; SelectedItem bound | Hidden | Hidden |
| Building | Enabled only when IsPrebuilt | All DirectionTypes; visible only when Type exists and HasTurret | Hidden | Visible; BasePriority bound; Prebuilt bound and enabled only when BasePriority>=0 | Hidden for Tiberian Dawn; visible/bound for Red Alert |

Common bindings are House.SelectedValue, Strength.Value, Direction.SelectedValue, and Trigger.SelectedItem. Mission uses SelectedItem. Building-specific bindings are BasePriority.Value and IsPrebuilt.Checked, plus Sellable.Checked and Rebuild.Checked for Red Alert.

The designer configures Strength to minimum 1, maximum 256, initial 256. BasePriority accepts the full signed 32-bit range, from -2147483648 to 2147483647. Prebuilt, Sellable, and Rebuild are CheckBoxes.

## Designer and resource audit

The generated designer creates a two-column TableLayoutPanel (31.25% / 68.75%) with 11 rows. It contains:
- labels House, Strength, Direction, Mission, Trigger, Base Priority;
- PropertiesComboBox controls for House, Direction, Mission, Trigger;
- NumericUpDown controls for Strength and BasePriority;
- CheckBoxes for Prebuilt, Sellable, Rebuild.

Event wiring:
- House, Direction, Mission, Trigger SelectedIndexChanged → comboBox_SelectedValueChanged.
- Strength and BasePriority ValueChanged → nud_ValueChanged.
- Prebuilt, Sellable, Rebuild CheckedChanged → checkBox_CheckedChanged.

ObjectProperties.resx has four resheaders (resmimetype, version, reader, writer), zero data entries, and zero metadata entries. It contains no image, icon, binary, serialized object, or named string payload.

## Popup lifecycle

ObjectPropertiesPopup derives from ToolStripDropDown. Its constructor creates a new ObjectProperties control, calls Initialize(plugin, false), assigns the selected model object, wraps the control in a ToolStripControlHost, zeroes popup/host padding and margin, and fixes popup min/max/size to the control's size.

When the control is disposed, the popup's handler sets ObjectProperties=null and calls Dispose(true). OnClosed calls base then sets ObjectProperties.Object=null, which detaches the model PropertyChanged subscription. ObjectProperties.Dispose/Disposed also clears Object and removes the Map.Triggers.CollectionChanged subscription.

The object property popup uses PropertiesComboBox so opening a nested ComboBox list does not auto-close the containing ToolStripDropDown.

## Review findings

- **Direction visibility can leak between model types:** Building sets directionComboBox.Visible from Type.HasTurret. Infantry and Unit set a Direction DataSource but do not explicitly restore Visible=true. If the same ObjectProperties instance is rebound from a turretless Building to Infantry or Unit, Direction can remain hidden. The normal popup creates a fresh control per object, but the public Object setter and embedded control support rebinding; test that transition.
- **House fallback assumes a non-empty registry:** when IsPrebuilt becomes false and BasicSection.BasePlayer does not match a HouseType, the handler calls HouseTypes.First(). An empty HouseTypes collection throws.
- **Mock versus live dirty state:** Obj_PropertyChanged marks the plugin dirty for every property change unless isMockObject is true. Preserve this distinction in a Rust editor; changing a mock template must not dirty the live map.
- **Trigger identity:** trigger choices are strings (None plus distinct names), not stable trigger IDs. Duplicate names collapse in the selector. A Rust model should bind stable IDs and retain duplicate-name disambiguation if the underlying format allows it.
- **Map lifetime:** Initialize subscribes to plugin.Map.Triggers and the Disposed handler unsubscribes from that captured collection. The control does not migrate subscriptions if plugin.Map is replaced while it remains alive.
- **Popup lifecycle is stronger than TerrainProperties:** ObjectProperties.Object unsubscribes from the old model and the popup clears Object on close. TerrainProperties defines a dirty handler but its inspected code does not subscribe it.

## Rust port requirements

Use typed property commands for Infantry, Unit, and Building:
- common House, Strength, Direction, Trigger;
- Infantry/Unit Mission;
- Building BasePriority, IsPrebuilt, Sellable, Rebuild;
- type-dependent direction visibility and mission/base fields;
- IsPrebuilt=false house normalization to the active BasePlayer;
- mock-object edits isolated from live document dirty state.

Property changes should be undoable and validated before commit. Bind trigger/house/direction IDs rather than display strings. Add tests for every type transition, turretless-to-turret Building, Building-to-Infantry/Unit visibility reset, BasePriority -1, empty HouseTypes, IsPrebuilt house normalization, mock dirty isolation, trigger-list updates, popup close/unsubscribe, and Red Alert versus Tiberian Dawn controls.

No Rust source or tests were changed in this audit tranche.
