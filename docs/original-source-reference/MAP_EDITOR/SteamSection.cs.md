# Model/SteamSection.cs — Workshop metadata persisted with a map

> **Audit status: body reviewed.** The complete class body and its serializer interaction were inspected. Rust mapping and behavior tests remain pending.

- Upstream: https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/SteamSection.cs
- Pinned source blob: c264012f2a441a06b8e157d91d68b2f10eaf2822
- Source size: 1,475 characters
- Namespace/type: MobiusEditor.Model.SteamSection

## Exact public properties

| Property | Type | DefaultValue metadata |
|---|---|---|
| Title | string | null |
| Description | string | null |
| PreviewFile | string | null |
| Visibility | ERemoteStoragePublishedFileVisibility | k_ERemoteStoragePublishedFileVisibilityPublic |
| PublishedFileId | ulong | 0 |

All five properties are public auto-properties. The class has no explicit constructor, validation, events, methods, or custom equality behavior. The compiler supplies the default constructor; property values therefore follow CLR defaults unless assigned.

## Serialization and consumers

Both game plugins parse and write the map's Steam section through INI.ParseSection/WriteSection, except SaveINI omits the Steam section for FileType.PGM. The reflection writer emits value types even at their default value and emits reference properties only when non-null; DefaultValue attributes are metadata, not an omission rule in that writer.

SteamDialog reads and edits Title, Description, PreviewFile, and Visibility. SteamworksUGCPublishOperation uses PublishedFileId to choose create-versus-update and stores the returned published ID back into the same SteamSection instance.

## Rust mapping

No Rust Workshop metadata model or Steam UGC integration was identified in the current target tree. Keep this metadata separate from the core map-cell document so offline map editing does not require Steamworks.

## Remaining work

- [ ] Define typed visibility values and a stable serialized representation in Rust.
- [ ] Test default and missing-key behavior for both game variants.
- [ ] Verify PGM omission and map-save persistence after successful creation of a new Workshop item.
