# MapSettingsDialog.cs — editor UI source guide

> **Audit status: partially reviewed.** The pinned C# source has been inspected for its principal responsibilities, imports, type names, and method declaration lines. Designer-generated control definitions and complete method-by-method explanations remain separate work.

- Upstream: [CnCTDRAMapEditor/Dialogs/MapSettingsDialog.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Dialogs/MapSettingsDialog.cs)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 6102 characters.

## Plain-English purpose

Hosts editing of Basic, Briefing, and per-player settings in a tree. It binds PropertyTracker instances to controls, tracks player enablement, and uses Win32 TreeView messaging to alter native checkbox state.

## Imports and dependencies

- MobiusEditor.Controls
- MobiusEditor.Interface
- MobiusEditor.Model
- MobiusEditor.Utility
- System
- System.Collections.Generic
- System.Data
- System.Drawing
- System.Linq
- System.Runtime.InteropServices
- System.Windows.Forms

## Type declarations found

- MapSettingsDialog
- TVITEM

## Method declaration lines found

- private void settingsTreeView_AfterSelect(object sender, TreeViewEventArgs e)
- private void settingsTreeView_DrawNode(object sender, DrawTreeNodeEventArgs e)
- private void HideCheckBox(TreeNode node)
- private void settingsTreeView_AfterCheck(object sender, TreeViewEventArgs e)

## Important integration behavior

The dialog crosses managed WinForms and native Win32 TreeView APIs via DllImport/SendMessage. Preserve native struct layout, message constants, and handle lifetime if reproducing this behavior.

## Rust mapping
No Rust implementation path is claimed here. Map UI behavior to a future editor frontend separately from game-format model semantics; verify serialization and undo/redo contracts before declaring parity.

## Outstanding audit
- [ ] Exact private fields, event handlers, method signatures, and all branches.
- [ ] Designer controls and resource dependencies.
- [ ] Complete user interaction sequence and cancellation/commit semantics.
- [ ] Disposal, event unsubscription, timers, native/Steam resource lifetime.
- [ ] TD/RA schema comparison and serialization mapping.
- [ ] Rust module and behavioral-test evidence.
