# TriggersDialog.cs — editor UI source guide

> **Audit status: partially reviewed.** The pinned C# source has been inspected for its principal responsibilities, imports, type names, and method declaration lines. Designer-generated control definitions and complete method-by-method explanations remain separate work.

- Upstream: [CnCTDRAMapEditor/Dialogs/TriggersDialog.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Dialogs/TriggersDialog.cs)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 28334 characters.

## Plain-English purpose

Edits a cloned working set of map triggers. The dialog adapts its controls to TD versus RA trigger schemas, including persistence labels and event/action layout differences.

## Imports and dependencies

- MobiusEditor.Interface
- MobiusEditor.Model
- MobiusEditor.Utility
- System
- System.Collections.Generic
- System.Data
- System.Linq
- System.Windows.Forms

## Type declarations found

- TriggersDialog

## Method declaration lines found

- private void triggersListView_SelectedIndexChanged(object sender, EventArgs e)
- private void teamTypesListView_MouseDown(object sender, MouseEventArgs e)
- private void teamTypesListView_KeyDown(object sender, KeyEventArgs e)
- private void addTriggerToolStripMenuItem_Click(object sender, EventArgs e)
- private void removeTriggerToolStripMenuItem_Click(object sender, EventArgs e)
- private void teamTypesListView_AfterLabelEdit(object sender, LabelEditEventArgs e)
- private void typeComboBox_SelectedValueChanged(object sender, EventArgs e)
- private void trigger1ComboBox_SelectedIndexChanged(object sender, EventArgs e)
- private void trigger2ComboBox_SelectedIndexChanged(object sender, EventArgs e)
- private void UpdateTriggerControls(Trigger trigger, TriggerEvent triggerEvent, TriggerAction triggerAction, ComboBox eventComboBox, NumericUpDown eventNud, ComboBox eventValueComboBox, ComboBox actionComboBox, NumericUpDown actionNud, ComboBox actionValueComboBox)

## Important integration behavior

The dialog edits cloned collections rather than mutating the map's live list during editing; callers must apply the returned collection on acceptance and discard it on cancellation. TD and RA expose different schemas.

## Rust mapping
No Rust implementation path is claimed here. Map UI behavior to a future editor frontend separately from game-format model semantics; verify serialization and undo/redo contracts before declaring parity.

## Outstanding audit
- [ ] Exact private fields, event handlers, method signatures, and all branches.
- [ ] Designer controls and resource dependencies.
- [ ] Complete user interaction sequence and cancellation/commit semantics.
- [ ] Disposal, event unsubscription, timers, native/Steam resource lifetime.
- [ ] TD/RA schema comparison and serialization mapping.
- [ ] Rust module and behavioral-test evidence.
