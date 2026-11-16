# MapPanel designer and resource audit

Pinned upstream:
- [MapPanel.Designer.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/MapPanel.Designer.cs) — blob 3f90f192cfd29df2602853810eb92b6e3d872e74, 53 lines.
- [MapPanel.resx](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/MapPanel.resx) — blob e5858cc294d2f5ff6f6f6e8e3a82684c29204a69, 123 lines.

Audit statuses: designer **Header reviewed**; resource file **Resource reviewed**.

## Designer contract

MapPanel.Designer.cs declares the components container, overrides Dispose(bool) to dispose it when requested, and InitializeComponent only calls SuspendLayout and ResumeLayout(false). It creates no child controls, sets no visual properties, and wires no events. The camera, rendering, zoom, and input behavior is entirely in MapPanel.cs.

## Resource XML audit

MapPanel.resx contains:
- 4 resheaders: resmimetype, version, reader, writer;
- 0 data entries;
- 1 metadata entry: name=$this.TrayLargeIcon, type=System.Boolean, value=False.

There are no image, icon, binary, serialized object, or named string data payloads. The metadata entry is designer metadata and does not create a runtime resource accessor.

See [MapPanel.cs.md](MapPanel.cs.md) for camera transforms, invalidation, render ordering, and lifecycle findings.
