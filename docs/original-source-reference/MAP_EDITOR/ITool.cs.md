# ITool.cs — map editor source guide

> **Audit status: partially reviewed.** This page inspects the pinned source file directly. The symbol list is a first-pass scan; exact member completeness and all call sites still require verification.

- Upstream: [`CnCTDRAMapEditor/Interface/ITool.cs`](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Interface/ITool.cs)
- Pinned commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source size: 1,140 characters.

## Plain-English purpose

Defines the minimal disposable editor-tool contract. Each tool exposes the map layers it operates on, allowing MainForm to synchronize active layer visibility with the selected tool.

## Imports

- `MobiusEditor.Model`
- `System`
- `System.Collections.Generic`
- `System.Drawing`
- `System.Linq`
- `System.Text`
- `System.Threading.Tasks`

## Types found

- `ITool`

## Member and method declarations found

- No matching declarations found.

## Behavior that matters to the port

Tool lifetime is explicitly disposable; the host communicates active layer selection through a mutable Layers property.

## Outstanding audit

- [ ] Exact members, property accessors, and interface implementation sites.
- [ ] All call sites and event subscription/unsubscription paths.
- [ ] Error behavior, nullability, and edge cases.
- [ ] Rust mapping and behavioral tests.
