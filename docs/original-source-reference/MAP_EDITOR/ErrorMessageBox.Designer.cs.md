# ErrorMessageBox.Designer.cs — editor source reference

> **Audit status: partially reviewed.** Source declarations and method signatures were mechanically enumerated from the pinned file; generated UI details and full behavioral explanation are incomplete.

- Upstream: [CnCTDRAMapEditor/Dialogs/ErrorMessageBox.Designer.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Dialogs/ErrorMessageBox.Designer.cs)
- Pinned source commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 7613 characters.

## Purpose

Displays error details and provides a copy-to-clipboard action.

## Types

- ErrorMessageBox

## Fields detected

- `components`
- `tableLayoutPanel1`
- `txtErrors`
- `lblMessage`
- `flowLayoutPanel1`
- `btnOK`
- `btnCopy`

## Method declarations detected

- `protected override void Dispose(bool disposing)`
- `private void InitializeComponent()`

## Implementation notes

Designer-generated WinForms code owns control construction, layout, component-container disposal, and event hookup. Keep the paired code-behind and .resx resources synchronized; hand edits can be overwritten by the designer.

## Rust mapping
No equivalent path is asserted. MapPanel behavior should be mapped to the Rust editor viewport/render crate only after transform, invalidation, zoom, and input semantics are fully characterized. Dialogs and WinForms layout are UI-specific and should be reimplemented at the presentation layer.

## Remaining audit
- [ ] Exact signatures and full method-body explanations.
- [ ] Designer controls, layout dimensions, tab order, event hookup, and resource keys.
- [ ] Disposal and native handle lifetime.
- [ ] Settings persistence and interaction with App.config.
- [ ] Rust implementation paths and behavioral tests.
