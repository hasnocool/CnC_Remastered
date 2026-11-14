# Map.cs — map editor source guide

> **Audit status: partially reviewed.** This is a first-pass source inspection against the pinned upstream C# file. The extracted declarations are not a complete parser-backed symbol inventory; nested, multiline, generated, and conditional declarations need verification.

- Upstream: [`CnCTDRAMapEditor/Model/Map.cs`](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Map.cs)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source size: 23,854 characters.

## Plain-English purpose

Core editor-side map model. It defines layer flags and map context, owns the map sections and typed cell grids/occupancy collections, tracks invalidated layers/locations, and exposes map dimensions/theatre plus editing and change-notification behavior.

## Namespaces and imports

- `MobiusEditor.Interface`
- `MobiusEditor.Render`
- `MobiusEditor.Utility`
- `System`
- `System.Collections.Generic`
- `System.Collections.ObjectModel`
- `System.ComponentModel`
- `System.Drawing`
- `System.Drawing.Drawing2D`
- `System.Linq`
- `TGASharpLib`

## Top-level and nested type names found

- `MapLayerFlag`
- `MapContext`
- `Map`

## Method/declaration lines found

- `public object GetService(Type serviceType) => null;`
- `public void OnComponentChanged() { }`
- `public bool OnComponentChanging() => true;`
- `public void BeginUpdate()`
- `public void EndUpdate()`
- `public void InitTheater(GameType gameType)`
- `private void Update()`
- `private void UpdateResourceOverlays(ISet<Point> locations)`
- `private void UpdateWallOverlays(ISet<Point> locations)`
- `private void RemoveBibs(Building building)`
- `private void AddBibs(Point location, Building building)`
- `public Map Clone()`
- `public TGA GeneratePreview(Size previewSize, bool sharpen)`
- `public TGA GenerateMapPreview()`
- `public TGA GenerateWorkshopPreview()`
- `private void Overlay_CellChanged(object sender, CellChangedEventArgs<Overlay> e)`
- `private void Technos_OccupierAdded(object sender, OccupierAddedEventArgs<ICellOccupier> e)`
- `private void Technos_OccupierRemoved(object sender, OccupierRemovedEventArgs<ICellOccupier> e)`
- `private void Buildings_OccupierAdded(object sender, OccupierAddedEventArgs<ICellOccupier> e)`
- `private void Buildings_OccupierRemoved(object sender, OccupierRemovedEventArgs<ICellOccupier> e)`
- `private void Triggers_CollectionChanged(object sender, System.Collections.Specialized.NotifyCollectionChangedEventArgs e)`



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
