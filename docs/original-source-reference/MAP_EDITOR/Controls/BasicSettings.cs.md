# BasicSettings.cs — editor UI source guide

> **Audit status: partially reviewed.** The pinned C# source has been inspected for its principal responsibilities, imports, type names, and method declaration lines. Designer-generated control definitions and complete method-by-method explanations remain separate work.

- Upstream: [CnCTDRAMapEditor/Controls/BasicSettings.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/BasicSettings.cs)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 4502 characters.

## Plain-English purpose

Binds scenario Basic-section properties to WinForms controls, populating house/movie choices and switching fields according to TD versus RA game rules.

## Imports and dependencies

- MobiusEditor.Interface
- System
- System.Data
- System.Linq
- System.Windows.Forms

## Type declarations found

- BasicSettings

## Method declaration lines found

- private void isSinglePlayerCheckBox_CheckedChanged(object sender, EventArgs e)

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
