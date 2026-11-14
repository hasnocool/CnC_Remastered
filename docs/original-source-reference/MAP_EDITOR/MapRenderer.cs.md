# MapRenderer.cs — map editor source guide

> **Audit status: partially reviewed.** This is a first-pass source inspection against the pinned upstream C# file. The extracted declarations are not a complete parser-backed symbol inventory; nested, multiline, generated, and conditional declarations need verification.

- Upstream: [`CnCTDRAMapEditor/Render/MapRenderer.cs`](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Render/MapRenderer.cs)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source size: 37,087 characters.

## Plain-English purpose

Static map drawing pipeline. It converts editor map/model state into a WinForms Graphics surface, using directional lookup tables and per-layer rendering for templates, terrain, overlays/resources, smudges, units, infantry, buildings, and other map annotations.

## Namespaces and imports

- `MobiusEditor.Interface`
- `MobiusEditor.Model`
- `MobiusEditor.Utility`
- `System`
- `System.Collections.Generic`
- `System.Diagnostics`
- `System.Drawing`
- `System.Drawing.Imaging`
- `System.Linq`

## Top-level and nested type names found

- `MapRenderer`

## Method/declaration lines found

- `public static void Render(GameType gameType, Map map, Graphics graphics, ISet<Point> locations, MapLayerFlag layers, int tileScale)`
- `public static void Render(GameType gameType, Map map, Graphics graphics, ISet<Point> locations, MapLayerFlag layers)`



## Dependencies and Rust-port considerations

- The editor is a separate C# WinForms application, not part of the native C++ game runtime.
- The map model and renderer encode editor semantics that should be compared against Rust map/theatre/render crates, not blindly translated as UI code.
- UI event handlers, undo/redo, tool/plugin lifecycle, file serialization, and Steam Workshop/UGC behavior are distinct responsibilities.
- No Rust parity is claimed by this page; equivalent paths and behavioral tests must be identified explicitly.

## Outstanding audit

- [ ] Exact class/member/field/enum inventories from full declarations.
- [ ] Every method's full signature and implementation explanation.
- [ ] UI event flows, model mutation, invalidation, undo/redo, and serialization side effects.
- [ ] Cross-file dependencies and plugin/tool contracts.
- [ ] Rust mapping and test evidence.
