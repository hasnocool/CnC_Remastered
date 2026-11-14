# RedAlert.vcxproj.filters — build/configuration/resource reference

> **Audit status: file inspected.** This page documents the file's role and salient declarations. Full compiler-flag normalization and reproducible-build verification remain open.

- Upstream: [REDALERT/RedAlert.vcxproj.filters](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/RedAlert.vcxproj.filters)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- File size: 56362 characters.

## Purpose

Visual Studio IDE logical grouping metadata for Red Alert source, Win32 library, and resource files. It is not a compiler input list and does not determine build semantics.

## Extracted configuration facts

No ProjectConfiguration elements.

No matching configuration properties extracted.

## IDE filters
Filter groups and file-to-filter assignments affect Solution Explorer organization only; do not treat them as source inclusion or build dependencies.



## Build/port implications

Preserve source organization for maintainability, but derive actual source membership from the .vcxproj, not this filter file.

## Outstanding checks
- [ ] Enumerate every property group, item group, import, custom build step, and dependency.
- [ ] Cross-check project source/resource membership against the 1,331-file inventory.
- [ ] Record exact compiler, linker, preprocessor, include, library, and resource compiler flags.
- [ ] Verify output artifact names, architecture, runtime linkage, and configuration equivalence.
- [ ] Map any behavior needed by Rust and mark obsolete Windows-only integration explicitly.
