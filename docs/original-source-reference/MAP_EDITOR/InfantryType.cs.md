# InfantryType.cs — editor model source guide

> **Audit status: partially reviewed.** This pass inventories local types, explicit fields, method declaration lines, and dependencies. Full body explanations and Rust parity are still required.

- Upstream: [CnCTDRAMapEditor/Model/InfantryType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/InfantryType.cs)
- Pinned source commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 3342 characters.

## Purpose

Game/theatre/house-specific infantry definition with initialization and identity semantics.

## Types

- `InfantryType`

## Fields detected

- No explicit field declarations detected by first-pass scan.

## Method signatures detected

- `public override bool Equals(object obj)`
- `public override int GetHashCode()`
- `public override string ToString()`
- `public void Init(GameType gameType, TheaterType theater, HouseType house, DirectionType direction)`

## Direct namespace dependencies

- MobiusEditor.Interface
- MobiusEditor.Render
- MobiusEditor.Utility
- System
- System.Drawing

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
