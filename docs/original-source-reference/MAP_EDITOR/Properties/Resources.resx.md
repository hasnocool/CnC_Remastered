# Properties/Resources.resx — embedded editor image resource

> **Audit status: resource reviewed.** XML resource names/types, encoded payload, generated accessor, and observed runtime use were inspected.

- Upstream: https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Properties/Resources.resx
- Pinned source blob: a39c5b200269084fda1f28a069bd4e4f5fbf9416
- File size: 838,483 characters
- XML data entries: 5

## Resource inventory

| Name | Declared type / encoding | Value summary |
|---|---|---|
| Name1 | string | 22-character string: this is my long string |
| Color1 | System.Drawing.Color, System.Drawing | empty value element |
| Bitmap1 | application/x-microsoft.net.object.binary.base64 | 54-character placeholder payload |
| Icon1 | System.Drawing.Icon, System.Drawing; bytearray/base64 MIME | 88-character placeholder payload |
| UI_CustomMissionPreviewDefault | System.Drawing.Bitmap, System.Drawing; bytearray/base64 MIME | PNG payload, 832,475 characters including formatting whitespace; PNG header identifies a 512×512 image |

The generated Resources.Designer.cs exposes UI_CustomMissionPreviewDefault as a strongly typed System.Drawing.Bitmap property backed by ResourceManager.GetObject. SteamDialog.OnLoad writes this bitmap to SoloBanner.png for solo missions and uses it as the Workshop preview.

The other four generic entries are present in the resource XML; the generated accessor inspected here exposes only UI_CustomMissionPreviewDefault.

## Port notes

This is an embedded UI asset, not a game map tile or scenario-format resource. A Rust/UI replacement should extract or reproduce the asset through an explicit resource pipeline and should not couple it to the map binary codec.
