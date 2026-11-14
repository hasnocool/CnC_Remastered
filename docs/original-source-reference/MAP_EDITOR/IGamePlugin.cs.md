# IGamePlugin.cs — map editor source guide

> **Audit status: partially reviewed.** This page inspects the pinned source file directly. The symbol list is a first-pass scan; exact member completeness and all call sites still require verification.

- Upstream: [`CnCTDRAMapEditor/Interface/IGamePlugin.cs`](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Interface/IGamePlugin.cs)
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source size: 1,557 characters.

## Plain-English purpose

Defines the editor's game-specific plugin boundary. A plugin supplies its game identity, current Map and rendered image, dirty state, new-map creation, load/save operations, and disposal lifecycle; file and game enums define supported values.

## Imports

- `MobiusEditor.Model`
- `MobiusEditor.Utility`
- `System`
- `System.Collections.Generic`
- `System.Drawing`
- `System.IO`

## Types found

- `FileType`
- `GameType`
- `IGamePlugin`

## Member and method declarations found

- No matching declarations found.

## Behavior that matters to the port

The plugin owns format-specific map creation and serialization while the host consumes a common Map and image. Load returns a sequence of strings (suitable for warnings/errors); Save reports success as bool.

## Outstanding audit

- [ ] Exact members, property accessors, and interface implementation sites.
- [ ] All call sites and event subscription/unsubscription paths.
- [ ] Error behavior, nullability, and edge cases.
- [ ] Rust mapping and behavioral tests.
