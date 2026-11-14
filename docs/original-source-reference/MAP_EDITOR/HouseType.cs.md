# HouseType.cs — editor model source guide

> **Audit status: partially reviewed.** This pass inventories local types, explicit fields, method declaration lines, and dependencies. Full body explanations and Rust parity are still required.

- Upstream: [CnCTDRAMapEditor/Model/HouseType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/HouseType.cs)
- Pinned source commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 2648 characters.

## Purpose

Static/configured house identity and metadata used by placed objects and scenario properties.

## Types

- `HouseType`

## Fields detected

- No explicit field declarations detected by first-pass scan.

## Method signatures detected

- `public override bool Equals(object obj)`
- `public override int GetHashCode()`
- `public override string ToString()`

## Direct namespace dependencies

- System
- System.Collections.Generic
- System.Linq
- System.Text
- System.Threading.Tasks

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
