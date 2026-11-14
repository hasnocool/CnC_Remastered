# CellTrigger.cs — editor model source guide

> **Audit status: partially reviewed.** This pass inventories local types, explicit fields, method declaration lines, and dependencies. Full body explanations and Rust parity are still required.

- Upstream: [CnCTDRAMapEditor/Model/CellTrigger.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/CellTrigger.cs)
- Pinned source commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 983 characters.

## Purpose

Cell-level trigger assignment record used by the map model.

## Types

- `CellTrigger`

## Fields detected

- No explicit field declarations detected by first-pass scan.

## Method signatures detected

- No methods matched the signature scan.

## Direct namespace dependencies



## Behavior/port concerns

Definition objects are configuration metadata, not placed runtime entities. Keep identity, theatre/game initialization, and render/occupancy metadata separate from instance state.

## Rust mapping
No Rust equivalent path is asserted without inspecting the implementation tree. Candidate responsibilities should be mapped to editor model/configuration crates, with deterministic occupancy/overlap tests and fixture-based INI compatibility tests.

## Outstanding audit
- [ ] Exact member types, visibility, initializers, and property accessors.
- [ ] Every function body's branches, loops, side effects, and error behavior.
- [ ] Callers, serializers, and game-specific data sources.
- [ ] TD/RA differences and test fixtures.
- [ ] Rust implementation paths and parity tests.
