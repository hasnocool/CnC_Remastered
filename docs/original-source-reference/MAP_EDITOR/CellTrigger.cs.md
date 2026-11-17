# CellTrigger.cs — per-cell trigger reference

> **Source status: Body reviewed. Rust mapping: none.** Complete source body inspected at the pinned upstream commit.

- Upstream: [CnCTDRAMapEditor/Model/CellTrigger.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/CellTrigger.cs)
- Upstream blob: 54299cb22a93c2f32bb0c7a8757cfa9ac5f9365c
- Namespace: MobiusEditor.Model.

CellTrigger is a one-property reference object. Its public auto-property Trigger:string is initialized to Model.Trigger.None. It has no explicit constructor, methods, events, validation, interfaces, or serialization logic in this file. The property remains publicly mutable and can be assigned null or an arbitrary trigger name.

Map stores CellTrigger values in CellGrid<CellTrigger>. Trigger names are reconciled by the Map trigger-collection change handler only for Building.Trigger; the source handler does not scan CellTriggers. Therefore deleting/renaming a Trigger does not get an equivalent automatic cleanup for cell-trigger cells from this Map handler.

The Rust MapDocument currently has no cell-trigger layer or typed trigger-name cell value. Its CellLayers contains only template, terrain, overlay, and smudge IDs. Preserve the distinction between a cell-level trigger reference and a placed object's Trigger property.

No C# runtime test or Rust parity test was run.
