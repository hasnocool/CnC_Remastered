# Overlay.cs — editor model source guide

> **Audit status: source inspected; explicit declarations and methods documented. Rust parity remains pending.**

- Upstream: [CnCTDRAMapEditor/Model/Overlay.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Overlay.cs)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 1230 characters.
- Namespace: MobiusEditor.Model

## Purpose and members
Lightweight placed overlay record implementing ICellOccupier.
Type:OverlayType auto-property. Icon:int auto-property. OverlapSize always returns Size(1,1). OccupyMask delegates to Type.OccupyMask. Tint:Color auto-property default White.
No explicit constructors or methods. Type must be non-null before occupancy access. Rendering and serialization live elsewhere.

## Port status
No Rust equivalent is asserted until the current Rust workspace is inspected and behavior is mapped to implementation paths and tests.
