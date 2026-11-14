# PlayerSettings.cs — editor UI source guide

> **Audit status: partially reviewed.** The pinned C# source has been inspected for its principal responsibilities, imports, type names, and method declaration lines. Designer-generated control definitions and complete method-by-method explanations remain separate work.

- Upstream: [CnCTDRAMapEditor/Controls/PlayerSettings.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/PlayerSettings.cs)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 3788 characters.

## Plain-English purpose

Binds a selected House's credits, caps, edge, allies, and RA-only tech/IQ/player-control properties to a user control. Ally selection updates the tracked House state.

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

- PlayerSettings

## Method declaration lines found

- private void playersListBox_SelectedIndexChanged(object sender, EventArgs e)

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
