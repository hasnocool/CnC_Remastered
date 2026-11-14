# Smudge.cs — editor model source guide

> **Audit status: source inspected; explicit declarations and methods documented. Rust parity remains pending.**

- Upstream: [CnCTDRAMapEditor/Model/Smudge.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Smudge.cs)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 1144 characters.
- Namespace: MobiusEditor.Model

## Purpose and members
Lightweight placed ground-mark record.
Type:SmudgeType; Icon:int; Data:int; Tint:Color default White. All are public auto-properties.
No explicit methods, constructors, events, or interfaces. Data is distinct from Icon and should not be collapsed during porting.

## Port status
No Rust equivalent is asserted until the current Rust workspace is inspected and behavior is mapped to implementation paths and tests.
