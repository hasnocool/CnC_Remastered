# PropertiesComboBox — nested dropdown auto-close coordination

Pinned upstream:
- [PropertiesComboBox.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/PropertiesComboBox.cs) — blob 46c260987c97c777f9efcdc4da92b14bbba94509, 66 lines.
- [PropertiesComboBox.Designer.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Controls/PropertiesComboBox.Designer.cs) — blob 70afac49de7f6f4322a0f7e7f2b65f95166eaa69, 51 lines.

Audit statuses: code-behind **Body reviewed**; designer **Header reviewed**. The source tree contains no PropertiesComboBox.resx. No Rust mapping, build, UI test, or parity test is claimed.

## Role and state

PropertiesComboBox derives from ComboBox. It is used inside ObjectProperties hosted by ObjectPropertiesPopup, which is a ToolStripDropDown. The custom control temporarily disables the containing ToolStripDropDown.AutoClose while its own dropdown is open, preventing the outer property popup from closing when the user interacts with a nested list.

Fields:
- bool savedAutoClose — prior host AutoClose value.

Private property:
- ToolStripDropDown DropDownHost — starts at Parent and walks the Parent chain until it finds a ToolStripDropDown, returning null if no such ancestor exists.

## Exact method behavior

| Method / exact signature | Behavior |
|---|---|
| PropertiesComboBox() | Calls InitializeComponent. |
| OnDropDown(EventArgs e) | Finds DropDownHost. If found, saves its AutoClose value and sets AutoClose=false. Then calls base.OnDropDown(e). |
| OnDropDownClosed(EventArgs e) | Finds DropDownHost. If found, restores savedAutoClose. Then calls base.OnDropDownClosed(e). |

The generated designer creates only a Component container and overrides Dispose(bool) to dispose it when requested. It contains no visual children, property defaults, event wiring, or resource accessor.

## Review findings

- The host lookup is dynamic, so the same control can operate inside a ToolStripDropDown or as a normal ComboBox outside one.
- The saved AutoClose value is restored before the base OnDropDownClosed callback.
- If the parent popup has already been detached/disposed when OnDropDownClosed runs, DropDownHost returns null and there is no host left to restore.
- savedAutoClose is a single field; the implementation assumes one open dropdown lifecycle per control and does not maintain a nesting stack.
- The code does not override Dispose to restore AutoClose if a nested dropdown is torn down without OnDropDownClosed. The parent popup is normally disposed with its host, but this path should be tested.

## Rust/UI port requirements

A replacement UI should model nested popup ownership explicitly:
- suspend parent auto-close while a child selector is active;
- restore the previous setting on every close/cancel/dispose path;
- support repeated open/close cycles without stale state;
- do not rely on a global window search when the popup parent is known.

No Rust code or UI tests were changed in this tranche.
