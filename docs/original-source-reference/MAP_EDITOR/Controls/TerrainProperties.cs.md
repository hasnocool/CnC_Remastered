# TerrainProperties — trigger binding, popup lifecycle, and empty resources

Pinned upstream files:
- [TerrainProperties.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/TerrainProperties.cs) — blob ead0e14d9e1d4a23ab293a4cb104633ab2eaf615, 159 lines.
- [TerrainProperties.Designer.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/TerrainProperties.Designer.cs) — blob a5499cadd4eb61423ba6ae04f7908b10ecbdda3f, 113 lines.
- [TerrainProperties.resx](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/TerrainProperties.resx) — blob 1af7de150c99c12dd67a509fe57c10d63e4eeb04, 120 lines.

Audit statuses: code-behind **Body reviewed**; generated designer **Header reviewed**; resource file **Resource reviewed**. No Rust mapping, build, UI test, or save-dirty test is claimed.

## Code-behind state

TerrainProperties derives from UserControl. It stores:
- bool isMockObject;
- IGamePlugin Plugin with private setter;
- Terrain terrain;
- public Terrain property whose setter compares references and calls Rebind when changed.

## Exact method behavior

| Method / exact signature | Behavior |
|---|---|
| TerrainProperties() | Calls InitializeComponent. |
| Initialize(IGamePlugin plugin, bool isMockObject) | Stores the mock flag and plugin, subscribes plugin.Map.Triggers.CollectionChanged, populates the trigger data source, and attaches a Disposed handler that clears Terrain and unsubscribes from the captured map's Triggers collection. |
| Triggers_CollectionChanged(object sender, NotifyCollectionChangedEventArgs e) | Calls UpdateDataSource. |
| UpdateDataSource() | Sets triggerComboBox.DataSource to Trigger.None followed by distinct names from Plugin.Map.Triggers, preserving enumeration order rather than sorting. |
| Rebind() | Clears all triggerComboBox.DataBindings. If Terrain is non-null, adds a SelectedItem binding from the ComboBox to Terrain.Trigger. |
| Obj_PropertyChanged(object sender, PropertyChangedEventArgs e) | Rebinds only when PropertyName is Type. If the control is not editing a mock object, sets Plugin.Dirty=true. |
| comboBox_SelectedValueChanged(object sender, EventArgs e) | Iterates every Binding on the sender ComboBox and calls WriteValue. |
| nud_ValueChanged(object sender, EventArgs e) | Iterates every Binding on the sender NumericUpDown and calls WriteValue. No NumericUpDown or event subscription exists in the inspected designer. |

## Generated designer contract

The designer creates:
- one TableLayoutPanel with two columns (31.25% and 68.75%) and one row;
- one Label named label5 with text Trigger;
- one PropertiesComboBox named triggerComboBox, docked Fill, DropDownList style;
- one SelectedIndexChanged handler wired to comboBox_SelectedValueChanged.

There is no NumericUpDown in this designer, and no Terrain.PropertyChanged subscription is generated here. The generated Dispose(bool) disposes the component container when disposing and calls base.Dispose.

## Resource file audit

TerrainProperties.resx contains:
- 4 resheader entries: resmimetype, version, reader, writer;
- 0 data entries;
- 0 metadata entries;
- no image, icon, binary, serialized object, or named string payload.

The file is schema/header-only. There is no generated resource accessor dependency in the inspected designer.

## Popup lifecycle and call sites

TerrainToolDialog(plugin) calls terrainProperties.Initialize(plugin, true) for the embedded mock-object control. TerrainTool binds that control to mockTerrain and shows it only when GameType is TiberianDawn.

TerrainPropertiesPopup creates a separate TerrainProperties instance, calls Initialize(plugin, false), assigns the existing Terrain, hosts the control inside a ToolStripControlHost, and matches the popup/control size. The control Disposed handler sets the popup's TerrainProperties property to null and calls Dispose(true) on the popup. OnClosed calls base and clears TerrainProperties.Terrain.

Initialize subscribes to the current plugin.Map.Triggers collection and the control's Disposed handler unsubscribes from that captured collection. If a plugin's Map reference is replaced while the control remains alive, the control does not migrate its subscription; the normal editor lifecycle should dispose/recreate the active tool and popup.

## Correctness finding: dirty notification is not wired

Obj_PropertyChanged contains the only Plugin.Dirty=true assignment in this control, but neither TerrainProperties.Initialize, Rebind, nor the generated designer subscribes Terrain.PropertyChanged to Obj_PropertyChanged. The designer wires only triggerComboBox.SelectedIndexChanged to comboBox_SelectedValueChanged. That handler writes DataBindings but does not mark the plugin dirty.

Therefore the intended non-mock dirty-notification path is not reached by the inspected control's own event wiring. TerrainTool's add/remove operations separately set Dirty, but editing Trigger in the property popup has no corresponding dirty assignment in these files. Verify with a save/reopen test before relying on this property editor.

The nud_ValueChanged method is also not wired by the inspected designer and has no NumericUpDown control in this layout.

## Rust port requirements

Represent the terrain trigger editor as an explicit property command:
- bind the selected trigger to a stable trigger identifier, not only a display name;
- update dirty state and undo/redo when the trigger changes;
- refresh the trigger list on trigger collection changes;
- detach subscriptions when the control/popup is disposed;
- keep mock-object editing from dirtying the live map;
- test property changes after popup close, map replacement, and trigger-list updates.

No Rust source or tests were changed in this tranche.
