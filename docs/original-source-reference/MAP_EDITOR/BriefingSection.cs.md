# BriefingSection.cs — editor model source guide

> **Audit status: source inspected; explicit declarations and methods documented. Rust parity remains pending.**

- Upstream: [CnCTDRAMapEditor/Model/BriefingSection.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/BriefingSection.cs)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 1025 characters.
- Namespace: MobiusEditor.Model

## Purpose and members
Minimal briefing section model.
- Briefing: string auto-property, default null.
No explicit methods, constructors, fields, or events. Uses DefaultValueAttribute. Serialization/UI handling is elsewhere.
Rust mapping: no equivalent verified; preserve absent/null versus empty-string semantics if format distinguishes them.

## Port status
No Rust equivalent is asserted until the current Rust workspace is inspected and behavior is mapped to implementation paths and tests.
