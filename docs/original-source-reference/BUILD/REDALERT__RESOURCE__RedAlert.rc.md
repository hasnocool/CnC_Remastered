# RedAlert.rc — build/configuration/resource reference

> **Audit status: file inspected.** This page documents the file's role and salient declarations. Full compiler-flag normalization and reproducible-build verification remain open.

- Upstream: [REDALERT/RESOURCE/RedAlert.rc](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/RESOURCE/RedAlert.rc)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- File size: 2359 characters.

## Purpose

Windows resource script compiled into the Red Alert DLL. Includes resource.h/afxres.h, declares English (US) resource language and code page 1252, and embeds version metadata identifying the Red Alert DLL.

## Extracted configuration facts

No ProjectConfiguration elements.

No matching configuration properties extracted.


## Resource-script declarations
The script compiles version/resource metadata into the native binary. Language and code-page declarations affect resource localization/encoding.


## Build/port implications

Keep runtime settings/resource metadata distinct from code-level defaults; test locale, framework, resource language, and embedded version behavior.

## Outstanding checks
- [ ] Enumerate every property group, item group, import, custom build step, and dependency.
- [ ] Cross-check project source/resource membership against the 1,331-file inventory.
- [ ] Record exact compiler, linker, preprocessor, include, library, and resource compiler flags.
- [ ] Verify output artifact names, architecture, runtime linkage, and configuration equivalence.
- [ ] Map any behavior needed by Rust and mark obsolete Windows-only integration explicitly.
