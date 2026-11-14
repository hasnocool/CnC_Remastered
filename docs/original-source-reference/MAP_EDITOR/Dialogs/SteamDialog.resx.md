# Dialogs/SteamDialog.resx — Workshop dialog resources

> **Audit status: resource reviewed.** XML data entries and their use by the generated SteamDialog designer were inspected.

- Upstream: https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Dialogs/SteamDialog.resx
- Pinned source blob: 06dd6806a7b16b15d596ec617e68ef835d29b505
- File size: 6,768 characters
- XML data entries: 5

## Resource inventory

| Name | Declared type / encoding | Value summary |
|---|---|---|
| Name1 | string | 22-character string: this is my long string |
| Color1 | System.Drawing.Color, System.Drawing | empty value element |
| Bitmap1 | application/x-microsoft.net.object.binary.base64 | 54-character placeholder payload |
| Icon1 | System.Drawing.Icon, System.Drawing; bytearray/base64 MIME | 88-character placeholder payload |
| label5.Text | string | 585-character Workshop content/rights notice |

The generated SteamDialog.Designer.cs reads label5.Text using ComponentResourceManager.GetString. No GetObject/GetString lookup for Name1, Color1, Bitmap1, or Icon1 was found in that designer file; their presence is recorded rather than assigning them an assumed runtime role.

## Port notes

The rights notice is UI content, not map data. Keep it in the dialog/resource layer. If the dialog is recreated outside WinForms, preserve the text and intended display without treating the generic sample entries as required map-format fields.
