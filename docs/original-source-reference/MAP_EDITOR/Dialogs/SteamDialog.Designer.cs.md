# Dialogs/SteamDialog.Designer.cs — generated Workshop dialog controls

> **Audit status: header reviewed.** Generated field inventory, event wiring, and resource lookup were inspected. Pixel-level layout and every generated property assignment are not yet documented.

- Upstream: https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Dialogs/SteamDialog.Designer.cs
- Pinned source blob: 70b31a05a04f92b54637218aba1f88e34cd3761f
- Source size: 21,177 characters

## Generated controls

The partial form declares:

- TableLayoutPanel: tableLayoutPanel1, tableLayoutPanel2
- Labels: label1, label2, label3, label4, label5, statusLbl
- TextBoxes: descriptionTxt, titleTxt, previewTxt
- ComboBox: visibilityComboBox
- Panels: panel1, panel2
- Buttons: previewBtn, btnClose, btnGoToSteam
- MenuButton: btnPublishMap
- ContextMenuStrip: publicMapContextMenuStrip
- ToolStripMenuItem: publishAsNewToolStripMenuItem
- ImageTooltip: imageTooltip

InitializeComponent wires TextChanged for descriptionTxt and previewTxt, Click for previewBtn, btnGoToSteam, publishAsNewToolStripMenuItem, and btnPublishMap. The close button and remaining layout are generated in the same file; this page does not claim a complete geometry/property audit.

## Resource binding

The designer creates a ComponentResourceManager for SteamDialog and reads label5.Text through resources.GetString. No other GetObject/GetString lookup was found in this designer source.

## Port notes

A replacement UI must preserve control semantics and event routing, not the WinForms table-layout implementation. The split-button behavior matters because it exposes publish-as-new only for an existing PublishedFileId.

## Remaining audit

- [ ] Record all control property values, dimensions, anchors, tab order, and accessibility settings.
- [ ] Verify generated Dispose behavior and component-container ownership.
- [ ] Compare every designer event hookup with the code-behind handler inventory.
