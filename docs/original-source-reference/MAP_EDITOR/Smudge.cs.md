# Smudge.cs — placed ground-mark instance

> **Source status: Body reviewed. Rust mapping: only a numeric layer slot exists; typed parity is absent.**

- Upstream: [CnCTDRAMapEditor/Model/Smudge.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Smudge.cs)
- Upstream blob: 642bd3f45eb9c52847fe24cbc63eb7365304af2a
- Namespace: MobiusEditor.Model.
- Imports: MobiusEditor.Interface and System.Drawing; the interface import is unused in this file.

Smudge is a public data class with four public auto-properties: Type:SmudgeType, Icon:int, Data:int, and Tint:Color initialized to Color.White. There is no explicit constructor, interface implementation, event, validation, clone, or method. Data is a separate integer and must not be folded into Icon or discarded. SmudgeType supplies multi-cell size, theatre-specific render size, bib flags, and thumbnail; Smudge itself does not expose an occupancy mask.

Map stores Smudge objects in CellGrid<Smudge>. Building bibs are represented as Smudge instances with Data=0 and a tint copied from the Building. Rust CellLayers.smudge:Option<u16> preserves only a numeric layer slot, not Type/Icon/Data/Tint or bib ownership. No Rust parity test was run.
