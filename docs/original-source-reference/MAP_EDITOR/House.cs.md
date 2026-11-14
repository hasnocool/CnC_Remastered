# House.cs — editor model source guide

> **Audit status: partially reviewed.** This pass inventories local types, explicit fields, method declaration lines, and dependencies. Full body explanations and Rust parity are still required.

- Upstream: [CnCTDRAMapEditor/Model/House.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/House.cs)
- Pinned source commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 4984 characters.

## Purpose

Player/house state and alliance-mask representation, including conversion between editor properties and legacy INI-compatible values.

## Types

- `AlliesMask`
- `AlliesMaskTypeConverter`
- `House`

## Fields detected

- `Type`

## Method signatures detected

- `public void Set(int id)`
- `public void Clear(int id)`
- `public override bool Equals(object obj)`
- `public override int GetHashCode()`
- `public override string ToString()`
- `public IEnumerator<int> GetEnumerator()`
- `public override bool CanConvertFrom(ITypeDescriptorContext context, Type sourceType)`
- `public override bool CanConvertTo(ITypeDescriptorContext context, Type destinationType)`
- `public override object ConvertTo(ITypeDescriptorContext context, CultureInfo culture, object value, Type destinationType)`
- `public override object ConvertFrom(ITypeDescriptorContext context, CultureInfo culture, object value)`

## Direct namespace dependencies

- MobiusEditor.Utility
- System
- System.Collections
- System.Collections.Generic
- System.ComponentModel
- System.Globalization
- System.Linq

## Behavior/port concerns

Alliance masks are bit-level serialized state. Preserve bit width, signedness, and textual conversion exactly; do not replace with an unordered set without compatibility encoding.

## Rust mapping
No Rust equivalent path is asserted without inspecting the implementation tree. Candidate responsibilities should be mapped to editor model/configuration crates, with deterministic occupancy/overlap tests and fixture-based INI compatibility tests.

## Outstanding audit
- [ ] Exact member types, visibility, initializers, and property accessors.
- [ ] Every function body's branches, loops, side effects, and error behavior.
- [ ] Callers, serializers, and game-specific data sources.
- [ ] TD/RA differences and test fixtures.
- [ ] Rust implementation paths and parity tests.
