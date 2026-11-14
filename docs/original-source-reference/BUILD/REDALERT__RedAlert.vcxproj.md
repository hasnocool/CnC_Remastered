# RedAlert.vcxproj — build/configuration/resource reference

> **Audit status: file inspected.** This page documents the file's role and salient declarations. Full compiler-flag normalization and reproducible-build verification remain open.

- Upstream: [REDALERT/RedAlert.vcxproj](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/RedAlert.vcxproj)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- File size: 30992 characters.

## Purpose

MSBuild C++ project for the Red Alert game DLL. Defines Debug/Release Win32 configurations, Visual Studio v141 toolset, Windows 8.1 SDK target, MultiByte character set, and project-level compiler/linker/resource/source inputs. Release enables whole-program optimization; inspect individual property groups for precise flags.

## Extracted configuration facts

Project configurations:
- Debug|Win32
- Release|Win32

Relevant properties/settings:
- `WindowsTargetPlatformVersion=8.1`
- `ConfigurationType=DynamicLibrary`
- `PlatformToolset=v141`
- `CharacterSet=MultiByte`
- `WholeProgramOptimization=true`
- `TargetName=$(ProjectName)I`
- `WarningLevel=Level3`
- `Optimization=MaxSpeed`
- `WholeProgramOptimization=false`
- `RuntimeLibrary=MultiThreaded`
- `AdditionalDependencies=winmm.lib;Ws2_32.lib;%(AdditionalDependencies)`
- `Optimization=Disabled`
- `RuntimeLibrary=MultiThreadedDebug`
- `ImportLibrary=$(OutDir)$(TargetName).lib`





## Build/port implications

This project is a Windows x86 native DLL build, not a portable Rust build description. A Rust port must independently define target triples, linker/import-library requirements, resource compilation, ABI boundaries, and feature/configuration mapping. The original MSBuild flags are compatibility evidence, not defaults to copy blindly.

## Outstanding checks
- [ ] Enumerate every property group, item group, import, custom build step, and dependency.
- [ ] Cross-check project source/resource membership against the 1,331-file inventory.
- [ ] Record exact compiler, linker, preprocessor, include, library, and resource compiler flags.
- [ ] Verify output artifact names, architecture, runtime linkage, and configuration equivalence.
- [ ] Map any behavior needed by Rust and mark obsolete Windows-only integration explicitly.
