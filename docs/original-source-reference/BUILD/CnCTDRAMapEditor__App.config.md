# App.config — build/configuration/resource reference

> **Audit status: file inspected.** This page documents the file's role and salient declarations. Full compiler-flag normalization and reproducible-build verification remain open.

- Upstream: [CnCTDRAMapEditor/App.config](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/App.config)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- File size: 1873 characters.

## Purpose

Runtime configuration for the editor's .NET Framework application. It declares application/user settings sections, targets .NET Framework 4.6.2, and sets defaults including Quality, tool-dialog position, and invite-warning behavior.

## Extracted configuration facts

No ProjectConfiguration elements.

No matching configuration properties extracted.





## Build/port implications

Keep runtime settings/resource metadata distinct from code-level defaults; test locale, framework, resource language, and embedded version behavior.

## Outstanding checks
- [ ] Enumerate every property group, item group, import, custom build step, and dependency.
- [ ] Cross-check project source/resource membership against the 1,331-file inventory.
- [ ] Record exact compiler, linker, preprocessor, include, library, and resource compiler flags.
- [ ] Verify output artifact names, architecture, runtime linkage, and configuration equivalence.
- [ ] Map any behavior needed by Rust and mark obsolete Windows-only integration explicitly.
