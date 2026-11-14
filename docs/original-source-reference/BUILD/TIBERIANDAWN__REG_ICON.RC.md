# REG_ICON.RC — build/configuration/resource reference

> **Audit status: file inspected.** This page documents the file's role and salient declarations. Full compiler-flag normalization and reproducible-build verification remain open.

- Upstream: [TIBERIANDAWN/REG_ICON.RC](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/REG_ICON.RC)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- File size: 266 characters.

## Purpose

Legacy Borland resource script binding ICON_1 to conquer.ico through reg_icon.rh.

## Extracted configuration facts

No ProjectConfiguration elements.

No matching configuration properties extracted.



## Resource declarations
The script includes its companion .rh identifier header and binds ICON_1 to conquer.ico. The referenced icon and .rh file must be tracked as build inputs.

## Build/port implications

Keep runtime settings/resource metadata distinct from code-level defaults; test locale, framework, resource language, and embedded version behavior.

## Outstanding checks
- [ ] Enumerate every property group, item group, import, custom build step, and dependency.
- [ ] Cross-check project source/resource membership against the 1,331-file inventory.
- [ ] Record exact compiler, linker, preprocessor, include, library, and resource compiler flags.
- [ ] Verify output artifact names, architecture, runtime linkage, and configuration equivalence.
- [ ] Map any behavior needed by Rust and mark obsolete Windows-only integration explicitly.
