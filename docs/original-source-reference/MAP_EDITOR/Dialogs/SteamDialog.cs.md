# SteamDialog.cs — Workshop metadata and publish workflow

> **Audit status: partially reviewed.** The full handwritten code-behind was inspected, and its generated designer/resource references and host callback path were traced. Steamworks.NET internals, native resource behavior, and UI integration tests remain open.

- Upstream: https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Dialogs/SteamDialog.cs
- Pinned source blob: c6c047e7cff3a004765e65c65236bc8a1168aef6
- Source size: 8,947 characters
- Namespace/type: MobiusEditor.Dialogs.SteamDialog : Form

## State and construction

The dialog stores the supplied IGamePlugin, a static temp preview directory under the OS temp path named CnCRCMapEditor, and a WinForms Timer. The constructor initializes controls, binds Public/Friends Only/Private values to the visibility combo box, configures the status timer to 500 ms, subscribes its Tick handler, and disposes the preview Image stored in previewTxt.Tag when the dialog is disposed.

## Lifecycle

OnLoad copies persisted Steam metadata into the controls, then sets the publish split-button width according to whether PublishedFileId is already valid. It creates the preview directory and writes Map.GenerateWorkshopPreview() to Minimap.png. For a solo mission it writes the embedded UI_CustomMissionPreviewDefault resource as SoloBanner.png and uses that as the preview; otherwise it uses Minimap.png. It sets the tooltip and starts the status timer.

StatusUpdateTimer_Tick copies CurrentOperation.Status into the status label when non-empty. OnClosed stops and disposes this dialog timer. The main Steam callback pump is separate: MainForm's own 500 ms timer calls SteamworksUGC.Service while initialized.

OnPublishSuccess and OnOperationFailed update the status label and re-enable controls. EnableControls toggles title, visibility, preview, description, publish, and close controls.

## User actions

- btnGoToSteam_Click opens SteamworksUGC.WorkshopURL when non-empty.
- btnPublishMap_Click fills BasicSection.Name from the title only when the map name is empty, and fills BasicSection.Author from SteamFriends.GetPersonaName() only when author is empty. It copies preview/title/description/visibility into Map.SteamSection.
- The publish handler creates/clears a temp directory named CnCRCMapEditorPublishUGC, asks the plugin to save a PGM package as MAPDATA.PGM, then builds tags: TD or RA, plus SinglePlayer or MultiPlayer. It starts SteamworksUGC.PublishUGC with success/failure callbacks and disables controls only when a new operation was accepted.
- previewBtn_Click opens a PNG-only file picker, initially based on the current preview path.
- publishAsNewToolStripMenuItem_Click resets PublishedFileId to the invalid ID, then reuses the publish button handler.
- previewTxt_TextChanged disposes the previous preview image, attempts to load the selected image into a detached Bitmap, and stores it in Tag. Any exception clears Tag. UpdateControls then enables publishing only when the preview loaded and description is non-empty.
- descriptionTxt_TextChanged recalculates the publish-button state.
- UpdateControls checks only preview availability and non-empty description; it does not validate title, map name, author, or file size.

## Important behavior and risks

The return value from plugin.Save(pgmPath, FileType.PGM) is ignored before PublishUGC is invoked. If validation fails and Save returns false, the handler still proceeds to construct tags and request a Workshop upload. A port should stop before upload if package validation/save fails.

OnLoad initially assigns the persisted PreviewFile to the textbox but then replaces it with the generated minimap or solo banner path. The persisted custom preview is therefore not automatically restored as the active preview during this load sequence.

The preview path is user-controlled. Image loading exceptions are swallowed and represented by a null Tag, so the UI prevents publishing when the file cannot be loaded. The generated preview and copied image lifetimes need explicit ownership in a non-WinForms implementation.

## MainForm integration

MainForm opens SteamDialog with the active IGamePlugin. After the dialog closes, it triggers the ordinary save command, which is how changed Workshop metadata and a newly assigned PublishedFileId are persisted to the map file. MainForm's callback-service timer remains responsible for pumping Steam callbacks while the dialog's status timer only displays progress.

## Rust mapping

No Rust Workshop dialog or publishing UI is currently present. Keep UI behavior separate from the file-format metadata model and from the Steamworks service state machine.

## Remaining audit

- [ ] Test custom preview selection, invalid images, cancellation, and solo/non-solo preview behavior.
- [ ] Check plugin.Save return before starting upload.
- [ ] Verify save-after-dialog behavior when publishing fails, the dialog is cancelled, or PublishUGC rejects a concurrent operation.
- [ ] Audit generated designer layout, resource use, and disposal behavior in detail.
- [ ] Verify Steamworks callback results and native resource ownership.
