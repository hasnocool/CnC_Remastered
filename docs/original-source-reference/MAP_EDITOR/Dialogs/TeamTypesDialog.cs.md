# TeamTypesDialog.cs — editor UI source guide

> **Audit status: partially reviewed.** The pinned C# source has been inspected for its principal responsibilities, imports, type names, and method declaration lines. Designer-generated control definitions and complete method-by-method explanations remain separate work.

- Upstream: [CnCTDRAMapEditor/Dialogs/TeamTypesDialog.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Dialogs/TeamTypesDialog.cs)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 19494 characters.

## Plain-English purpose

Edits a working copy of map team-type definitions. It builds available techno types from infantry and units, configures game-specific controls, and edits team classes and mission rows before exposing the result.

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

- TeamTypesDialog

## Method declaration lines found

- private void teamTypesListView_SelectedIndexChanged(object sender, EventArgs e)
- private void teamTypesListView_MouseDown(object sender, MouseEventArgs e)
- private void teamTypesListView_KeyDown(object sender, KeyEventArgs e)
- private void addTeamTypeToolStripMenuItem_Click(object sender, EventArgs e)
- private void removeTeamTypeToolStripMenuItem_Click(object sender, EventArgs e)
- private void teamTypesListView_AfterLabelEdit(object sender, LabelEditEventArgs e)
- private void teamsDataGridView_CellValueNeeded(object sender, DataGridViewCellValueEventArgs e)
- private void teamsDataGridView_CellValuePushed(object sender, DataGridViewCellValueEventArgs e)
- private void teamsDataGridView_NewRowNeeded(object sender, DataGridViewRowEventArgs e)
- private void teamsDataGridView_RowValidated(object sender, DataGridViewCellEventArgs e)
- private void teamsDataGridView_RowDirtyStateNeeded(object sender, QuestionEventArgs e)
- private void teamsDataGridView_CancelRowEdit(object sender, QuestionEventArgs e)
- private void teamsDataGridView_UserDeletingRow(object sender, DataGridViewRowCancelEventArgs e)
- private void teamsDataGridView_UserAddedRow(object sender, DataGridViewRowEventArgs e)
- private void teamsDataGridView_UserDeletedRow(object sender, DataGridViewRowEventArgs e)
- private void missionsDataGridView_CellValueNeeded(object sender, DataGridViewCellValueEventArgs e)
- private void missionsDataGridView_CellValuePushed(object sender, DataGridViewCellValueEventArgs e)
- private void missionsDataGridView_NewRowNeeded(object sender, DataGridViewRowEventArgs e)
- private void missionsDataGridView_RowValidated(object sender, DataGridViewCellEventArgs e)
- private void missionsDataGridView_RowDirtyStateNeeded(object sender, QuestionEventArgs e)
- private void missionsDataGridView_CancelRowEdit(object sender, QuestionEventArgs e)
- private void missionsDataGridView_UserDeletingRow(object sender, DataGridViewRowCancelEventArgs e)
- private void missionsDataGridView_UserAddedRow(object sender, DataGridViewRowEventArgs e)
- private void missionsDataGridView_UserDeletedRow(object sender, DataGridViewRowEventArgs e)
- private void updateDataGridViewAddRows(DataGridView dataGridView, int maxItems)

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
