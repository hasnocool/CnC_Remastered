# SteamDialog.cs — editor UI source guide

> **Audit status: partially reviewed.** The pinned C# source has been inspected for its principal responsibilities, imports, type names, and method declaration lines. Designer-generated control definitions and complete method-by-method explanations remain separate work.

- Upstream: [CnCTDRAMapEditor/Dialogs/SteamDialog.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Dialogs/SteamDialog.cs)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 8945 characters.

## Plain-English purpose

Manages Steam Workshop publishing metadata and preview generation. It builds preview images in a temp directory, selects visibility, displays upload status, and interacts with Steamworks UGC APIs.

## Imports and dependencies

- MobiusEditor.Controls
- MobiusEditor.Interface
- MobiusEditor.Utility
- Steamworks
- System
- System.Collections.Generic
- System.ComponentModel
- System.Diagnostics
- System.Drawing
- System.Drawing.Imaging
- System.IO
- System.Windows.Forms

## Type declarations found

- SteamDialog

## Method declaration lines found

- protected override void OnLoad(EventArgs e)
- private void StatusUpdateTimer_Tick(object sender, EventArgs e)
- protected override void OnClosed(EventArgs e)
- protected virtual void OnPublishSuccess()
- protected virtual void OnOperationFailed(string status)
- private void EnableControls(bool enable)
- private void btnGoToSteam_Click(object sender, EventArgs e)
- private void btnPublishMap_Click(object sender, EventArgs e)
- private void previewBtn_Click(object sender, EventArgs e)
- private void publishAsNewToolStripMenuItem_Click(object sender, EventArgs e)
- private void previewTxt_TextChanged(object sender, EventArgs e)
- private void descriptionTxt_TextChanged(object sender, EventArgs e)
- private void UpdateControls()

## Important integration behavior

Steam integration is asynchronous/callback-driven in parts and owns temporary preview image resources. Audit callback registration, upload state transitions, timer shutdown, image disposal, and native Steam API lifetime before porting.

## Rust mapping
No Rust implementation path is claimed here. Map UI behavior to a future editor frontend separately from game-format model semantics; verify serialization and undo/redo contracts before declaring parity.

## Outstanding audit
- [ ] Exact private fields, event handlers, method signatures, and all branches.
- [ ] Designer controls and resource dependencies.
- [ ] Complete user interaction sequence and cancellation/commit semantics.
- [ ] Disposal, event unsubscription, timers, native/Steam resource lifetime.
- [ ] TD/RA schema comparison and serialization mapping.
- [ ] Rust module and behavioral-test evidence.
