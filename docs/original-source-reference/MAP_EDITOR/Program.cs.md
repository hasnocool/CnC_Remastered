# Program.cs — map editor source guide

> **Audit status: partially reviewed.** This is a first-pass source inspection against the pinned upstream C# file. The extracted declarations are not a complete parser-backed symbol inventory; nested, multiline, generated, and conditional declarations need verification.

- Upstream: [`CnCTDRAMapEditor/Program.cs`](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Program.cs)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Source size: 4,547 characters.

## Plain-English purpose

Application bootstrap. Main() pins the process culture to en-US, creates and loads the game megafile manager, initializes texture/tileset/team-colour/text managers, conditionally initializes Steam, starts WinForms, and disposes Steam and megafile resources on normal shutdown.

## Namespaces and imports

- `MobiusEditor.Dialogs`
- `MobiusEditor.Utility`
- `System`
- `System.Globalization`
- `System.IO`
- `System.Threading`
- `System.Windows.Forms`

## Top-level and nested type names found

- `Program`

## Method/declaration lines found

- `static void Main()`



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
