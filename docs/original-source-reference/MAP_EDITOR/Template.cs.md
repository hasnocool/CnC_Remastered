# Template.cs — one-cell template instance

> **Source status: Body reviewed. Rust mapping: partial identifier storage only.** This file is a small, fully inspected model type.

- Upstream source: [CnCTDRAMapEditor/Model/Template.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Template.cs)
- Upstream blob: 8fd901a31859ed14a88834feb7c0aed2faefa2c6
- Namespace: MobiusEditor.Model
- Imports: MobiusEditor.Interface; System.Drawing.

## Exact contract

Template is a public class implementing ICellOccupier.

- Type : TemplateType { get; set; } — auto-property, no explicit initializer.
- Icon : int { get; set; } — auto-property, no explicit initializer.
- OverlapSize : Size returns a new 1x1 Size.
- OccupyMask : bool[,] returns a newly allocated 1x1 array containing true each time the property is read.

There are no constructors, events, equality overrides, serialization methods, or other methods in this file. Template does not implement ICellOverlapper.

## Important distinction

The Template instance occupies exactly one cell in OccupierSet terms. The multi-icon footprint and missing-icon layout belong to TemplateType.IconWidth/IconHeight/IconMask and TemplateTool placement logic, not to this class's OccupyMask. Do not conflate the per-cell Template object with the complete multi-cell template definition.

## Rust comparison

MapDocument currently stores only CellLayers.template: Option<u16>; this is a template identifier slot, not a typed Template instance with an icon index and occupancy interface. A Rust map model needs to preserve both template type ID and icon index per cell, with theatre-specific IconMask/footprint behavior represented by the catalog/placement layer.

## Verification

Complete source body inspected. No Rust parity test was run.
