# REMAP.ASM — assembly source reference

> **Audit status: partially reviewed.** Source directives and visible PROC labels have been extracted from the pinned assembly file. Instruction-by-instruction control-flow and ABI analysis remains outstanding.

- Upstream: [TIBERIANDAWN/WIN32LIB/REMAP.ASM](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/WIN32LIB/REMAP.ASM)
- Pinned commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 5115 characters.

## Purpose and context

Pixel remapping support used by the Westwood graphics library. The exact palette-index mapping and optimized inner-loop semantics are part of rendering parity.

## PROC labels detected

- `CODESEG`

## Assembly directives detected

- `INCLUDE ".\drawbuff.inc"`
- `INCLUDE ".\gbuffer.inc"`

## ABI and safety requirements
- Identify exported/public symbols and all C/C++ declarations that bind to them.
- Record calling convention, parameter registers/stack offsets, preserved registers, return registers, stack alignment, and clobbers.
- Validate source/destination lengths and pointer arithmetic; document overlap and alignment assumptions.
- Preserve integer overflow, signedness, and bit-exact output where assets or simulation depend on it.
- Provide scalar Rust reference implementations and differential fixtures before replacing optimized assembly.
- Keep architecture-specific assembly behind explicit target cfgs; do not silently execute unsupported instructions.

## Outstanding audit
- [ ] Instruction-by-instruction explanation, branches, loops, and memory effects.
- [ ] All labels/data tables and symbol visibility.
- [ ] Paired headers, inline declarations, call sites, and link inclusion.
- [ ] x86/x64 assumptions and compiler/linker integration.
- [ ] Rust replacement and differential tests.
