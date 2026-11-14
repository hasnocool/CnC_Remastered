# TerrainProperties.cs — editor UI source guide

> **Audit status: partially reviewed.** The pinned C# source has been inspected for its principal responsibilities, imports, type names, and method declaration lines. Designer-generated control definitions and complete method-by-method explanations remain separate work.

- Upstream: [CnCTDRAMapEditor/Controls/TerrainProperties.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/TerrainProperties.cs)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 4898 characters.

## Plain-English purpose

Edits a selected Terrain instance's trigger and type-related properties. It tracks trigger-collection changes, rebuilds the trigger list, rebinds controls, and marks non-mock maps dirty.

## Imports and dependencies

- MobiusEditor.Interface
- MobiusEditor.Model
- MobiusEditor.Utility
- System
- System.Collections.Generic
- System.ComponentModel
- System.Data
- System.Linq
- System.Windows.Forms

## Type declarations found

- TerrainProperties
- TerrainPropertiesPopup

## Method declaration lines found

- public void Initialize(IGamePlugin plugin, bool isMockObject)
- private void Triggers_CollectionChanged(object sender, System.Collections.Specialized.NotifyCollectionChangedEventArgs e)
- private void UpdateDataSource()
- private void Rebind()
- private void Obj_PropertyChanged(object sender, PropertyChangedEventArgs e)
- private void comboBox_SelectedValueChanged(object sender, EventArgs e)
- private void nud_ValueChanged(object sender, EventArgs e)
- protected override void OnClosed(ToolStripDropDownClosedEventArgs e)

## Important integration behavior

The control uses WinForms data binding and game-specific branches; the model remains the source of truth, and event handlers can trigger map dirty-state changes.

## Rust mapping
No Rust implementation path is claimed here. Map UI behavior to a future editor frontend separately from game-format model semantics; verify serialization and undo/redo contracts before declaring parity.

## Outstanding audit
- [ ] Exact private fields, event handlers, method signatures, and all branches.
- [ ] Designer controls and resource dependencies.
- [ ] Complete user interaction sequence and cancellation/commit semantics.
- [ ] Disposal, event unsubscription, timers, native/Steam resource lifetime.
- [ ] TD/RA schema comparison and serialization mapping.
- [ ] Rust module and behavioral-test evidence.
