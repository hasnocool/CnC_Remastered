# ImageTooltip — owner-drawn image-only ToolTip

Pinned upstream:
- [ImageTooltip.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/ImageTooltip.cs) — blob b916e6e06669686c76e7b6a58ac9425b03309ffc, 65 lines.
- [ImageTooltip.Designer.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/ImageTooltip.Designer.cs) — blob 50eeddb4047b2ad9ff8aaec3525dfd4cafb7d96e, 51 lines.

Audit statuses: code-behind **Body reviewed**; designer **Header reviewed**. The source tree contains no ImageTooltip.resx.

## State and behavior

ImageTooltip derives from ToolTip and exposes Size MaxSize, default Size.Empty.

Constructor:
- calls InitializeComponent;
- sets OwnerDraw=true;
- subscribes Popup to ImageTooltip_Popup;
- subscribes Draw to ImageTooltip_Draw.

ImageTooltip_Popup(object sender, PopupEventArgs e):
- checks whether e.AssociatedControl.Tag is Image;
- if so, starts from image.Size and independently clamps Width and Height to MaxSize dimensions when those dimensions are positive;
- assigns the resulting size to e.ToolTipSize;
- otherwise sets e.Cancel=true.

ImageTooltip_Draw(object sender, DrawToolTipEventArgs e):
- if AssociatedControl.Tag is Image, draws that image into e.Bounds.

The generated designer creates only a Component container and disposes it from Dispose(bool). It creates no child controls, default properties, event wiring, or resource accessor.

## Review findings

- **Aspect ratio is not preserved:** Width and Height are clamped independently. A 200×100 image with MaxSize 100×100 becomes a 100×100 tooltip and DrawImage stretches the source into that square.
- **Tag is the image contract:** any non-Image Tag cancels the tooltip. The control does not accept an Image directly or validate a null AssociatedControl before accessing Tag.
- **Image ownership remains external:** the tooltip draws the Tag image but does not clone or dispose it.
- **No resource payload:** there is no .resx file for this control.

A Rust replacement should compute one uniform scale factor from MaxSize, preserve the source aspect ratio, and keep image lifetime with the control that owns the source image.

No Rust code or tests were changed in this tranche.
