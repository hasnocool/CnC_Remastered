# ErrorMessageBox.cs — editor source reference

> **Audit status: partially reviewed.** Source declarations and method signatures were mechanically enumerated from the pinned file; generated UI details and full behavioral explanation are incomplete.

- Upstream: [CnCTDRAMapEditor/Dialogs/ErrorMessageBox.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Dialogs/ErrorMessageBox.cs)
- Pinned source commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 1689 characters.

## Purpose

Displays error details and provides a copy-to-clipboard action.

## Types

- ErrorMessageBox

## Fields detected

- No explicit field declarations detected by the first-pass scan.

## Method declarations detected

- `private void btnCopy_Click(object sender, EventArgs e)`

## Implementation notes

btnCopy_Click is the explicit user action; verify clipboard format and error handling. Designer defines controls and event wiring.

## Rust mapping
No equivalent path is asserted. MapPanel behavior should be mapped to the Rust editor viewport/render crate only after transform, invalidation, zoom, and input semantics are fully characterized. Dialogs and WinForms layout are UI-specific and should be reimplemented at the presentation layer.

## Remaining audit
- [ ] Exact signatures and full method-body explanations.
- [ ] Designer controls, layout dimensions, tab order, event hookup, and resource keys.
- [ ] Disposal and native handle lifetime.
- [ ] Settings persistence and interaction with App.config.
- [ ] Rust implementation paths and behavioral tests.
