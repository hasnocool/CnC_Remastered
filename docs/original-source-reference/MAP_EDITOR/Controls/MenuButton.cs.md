# MenuButton — split button and ContextMenuStrip dispatch

Pinned upstream:
- [MenuButton.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/MenuButton.cs) — blob 0838fefb96c1bc49fd1f5eba2ce4459dae1e0cf0, 75 lines.
- [MenuButton.Designer.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/MenuButton.Designer.cs) — blob eb789bb89bfc7c4c0e64beb139e70b16265fbbee, 51 lines.

Audit statuses: code-behind **Body reviewed**; designer **Header reviewed**. The source tree contains no MenuButton.resx.

## Public state

MenuButton derives from Button.
- const int DefaultSplitWidth = 20.
- ContextMenuStrip Menu with DefaultValue(null), Browsable(true), and DesignerSerializationVisibility.Visible.
- int SplitWidth, default 20, with the same designer/browsing visibility metadata.

Constructor calls InitializeComponent.

## Exact override behavior

OnMouseDown(MouseEventArgs mevent):
1. Creates splitRect at x = Width - SplitWidth, y=0, width=SplitWidth, height=Height.
2. If Menu is non-null, the button is left-clicked, and the point lies in splitRect, calls Menu.Show(this, 0, Height).
3. Otherwise delegates to base.OnMouseDown.

A click in the split region therefore opens the menu and does not invoke the base button mouse-down path. The main button area retains normal Button behavior.

OnPaint(PaintEventArgs pevent):
1. Calls base.OnPaint.
2. When Menu is non-null and SplitWidth>0, computes a fixed triangle near the right edge.
3. Uses ControlText brush when Enabled and ButtonShadow when disabled.
4. Fills the triangle and draws a dotted vertical separator at ClientRectangle.Width-SplitWidth with a Pen disposed by a using scope.

The generated designer creates only a Component container and its Dispose(bool) path; no visual children, default settings, event wiring, or resource accessor are present.

## Review findings

- SplitWidth is not clamped to [0, Width]. A value larger than Width makes the split rectangle extend left over the entire button; a negative value creates an invalid/empty split rectangle while suppressing the painted arrow.
- The menu opens at (0, Height), aligned to the button's left edge rather than the split region.
- Menu is externally supplied and is not disposed by MenuButton; ownership must remain with the caller.
- The arrow and separator are painted only when Menu is non-null and SplitWidth>0.
- The separator Pen is correctly disposed; unlike several map tools, this control does not leak a Pen per paint.

A Rust UI replacement should clamp the split width to the actual client width, keep menu ownership explicit, and preserve the distinction between menu clicks and normal Button clicks.

No Rust code or tests were changed in this tranche.
