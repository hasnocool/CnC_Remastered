# BriefingSettings.cs — editor source reference

> **Audit status: partially reviewed.** Source declarations and method signatures were mechanically enumerated from the pinned file; generated UI details and full behavioral explanation are incomplete.

- Upstream: [CnCTDRAMapEditor/Controls/BriefingSettings.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/BriefingSettings.cs)
- Pinned source commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 1218 characters.

## Purpose

Small WinForms control hosting the briefing text editor.

## Types

- BriefingSettings

## Fields detected

- No explicit field declarations detected by the first-pass scan.

## Method declarations detected

- No method declarations detected by the first-pass scan.

## Implementation notes

The csproj is authoritative for compile/resource inclusion and target framework. Its full property/import/reference graph must be extracted before asserting a reproducible build.

## Rust mapping
No equivalent path is asserted. MapPanel behavior should be mapped to the Rust editor viewport/render crate only after transform, invalidation, zoom, and input semantics are fully characterized. Dialogs and WinForms layout are UI-specific and should be reimplemented at the presentation layer.

## Remaining audit
- [ ] Exact signatures and full method-body explanations.
- [ ] Designer controls, layout dimensions, tab order, event hookup, and resource keys.
- [ ] Disposal and native handle lifetime.
- [ ] Settings persistence and interaction with App.config.
- [ ] Rust implementation paths and behavioral tests.
