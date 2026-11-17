# Overlay.cs — placed overlay instance

> **Source status: Body reviewed. Rust mapping: only a numeric layer slot exists; typed parity is absent.**

- Upstream: [CnCTDRAMapEditor/Model/Overlay.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Overlay.cs)
- Upstream blob: 17cf6e75e4c20a9c9497026e7c9ddf456f969e93
- Namespace: MobiusEditor.Model.
- Imports: MobiusEditor.Interface; System.Drawing.

Overlay is a public class implementing ICellOccupier. Its public auto-properties are Type:OverlayType and Icon:int. OverlapSize returns a new 1x1 Size. OccupyMask delegates directly to Type.OccupyMask; this dereference throws if Type is null. Tint:Color is initialized to Color.White. There is no explicit constructor, INotifyPropertyChanged event, equality override, clone, or serialization method.

The instance is distinct from OverlayType: Overlay carries the selected type, per-cell icon index, and tint; OverlayType supplies flags, theatre metadata, thumbnail and occupancy mask. Map stores Overlay instances in CellGrid<Overlay>. The grid's CellChanged event fires when the cell reference changes, not when Overlay.Icon or Tint is mutated. Map's resource/wall derived-icon routines therefore mutate the live object without replacing the cell reference.

Rust MapDocument currently has CellLayers.overlay:Option<u16>, which stores a numeric slot only. It does not represent OverlayType flags, icon index, tint, resource/wall semantics, or occupancy. No Rust parity test was run.
