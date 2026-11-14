# IGamePlugin.cs — map editor plugin contract

> **Audit status: header reviewed.** The complete interface and enum declarations were inspected at the pinned upstream commit. Concrete game implementations and host call sites were traced in this tranche; exhaustive behavior/parity testing remains open.

- Upstream: https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Interface/IGamePlugin.cs
- Pinned source blob: 6d0915207de1e59f30f69e8eae5ef9e42d202f9c
- Source size: 1,557 characters
- Namespace: MobiusEditor.Interface

## Exact enums

**FileType** declares None, INI, BIN, MEG, PGM in that order.

**GameType** declares None, TiberianDawn, RedAlert in that order.

Neither enum assigns explicit numeric values, so the ordinary zero-based C# values apply.

## Interface contract

IGamePlugin inherits IDisposable and declares:

| Member | Exact contract |
|---|---|
| GameType | read-only GameType property |
| Map | read-only MobiusEditor.Model.Map property |
| MapImage | read-only System.Drawing.Image property |
| Dirty | bool property with get and set |
| New(string theater) | initialize a new map for the selected theatre |
| Load(string path, FileType fileType) | returns IEnumerable<string> diagnostics; implementation may also throw |
| Save(string path, FileType fileType) | returns bool success/failure |
| Dispose() | inherited from IDisposable |

The interface has no method bodies, default implementations, cancellation token, async contract, or explicit error type. Load diagnostics are strings; Save failure is only a bool at this boundary.

## Concrete implementations and host call sites

- MobiusEditor.RedAlert.GamePlugin implements the contract for Red Alert.
- MobiusEditor.TiberianDawn.GamePlugin implements it for Tiberian Dawn.
- MainForm selects the concrete implementation from GameType for new/load operations, invokes Load and displays returned diagnostics, and invokes Save for the selected FileType.
- MainForm's publish command opens SteamDialog with the current plugin, then triggers the normal save command after the dialog closes.
- The preview-generation path constructs either plugin with mapImage=false, loads an INI, renders a preview, and disposes the plugin.

The host catches exceptions around load; in non-DEVELOPER builds it converts them to a failed load result, while developer builds rethrow. The string sequence is therefore not a complete exception contract.

## Port boundary

The Rust workspace currently has an initial crates/map-editor-model document model, not a one-for-one IGamePlugin abstraction. Do not treat MapDocument as proof that the C# plugin lifecycle, file-type dispatch, warning collection, preview ownership, or disposal contract has been ported.

## Remaining audit

- [ ] Enumerate every plugin construction, replacement, and disposal path in MainForm.
- [ ] Confirm Dirty transitions and map-image ownership for both implementations.
- [ ] Define typed Rust load diagnostics and errors before mapping the string/bool API.
- [ ] Add TD/RA fixture tests for every FileType, including round-trip and malformed-input cases.
- [ ] Verify plugin disposal on every host replacement and failure path.
