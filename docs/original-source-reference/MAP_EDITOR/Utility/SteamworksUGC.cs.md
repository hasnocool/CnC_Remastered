# Utility/SteamworksUGC.cs — Steamworks initialization and Workshop publish state machine

> **Audit status: partially reviewed.** The full 10.6 KB file was inspected for operation states, API calls, callbacks, error branches, and lifecycle. Steamworks.NET binding internals, callback thread guarantees, native library packaging, and failure-injection tests remain open.

- Upstream: https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Utility/SteamworksUGC.cs
- Pinned source blob: 378bdb56877e2202fcdafaff2934da90988c8286
- Source size: 10,654 characters
- Namespace: MobiusEditor.Utility

## Types and state

ISteamworksOperation : IDisposable declares Done, Failed, Status, OnSuccess(), and OnFailed().

SteamworksUGCPublishOperation stores the package path, tags, success/failure delegates, CreateItem and SubmitItemUpdate CallResult handles, and the SteamSection being updated. Done is true when neither callback handle is active. Failed and Status are privately set.

SteamworksUGC is a static coordinator with IsInit, CurrentOperation, WorkshopURL, IsSteamBuild, a GameLobbyJoinRequested callback, and Init/Shutdown/Service/PublishUGC methods.

## Publish operation

The constructor chooses CreateUGCItem when PublishedFileId equals the Steam invalid ID; otherwise it chooses UpdateUGCItem. It then sets Status to Publishing UGC.

CreateUGCItem calls SteamUGC.CreateItem with the current AppID and Community file type. An invalid API-call handle immediately marks the operation failed. Otherwise it creates and attaches a CallResult<CreateItemResult_t> callback.

UpdateUGCItem starts an item update for the existing published ID. An invalid update handle fails immediately. It then applies content path, preview path, English update language, title, description, visibility, and tags in sequence. The boolean chain short-circuits after the first failed SteamUGC setter. A failed setter or invalid SubmitItemUpdate call marks failure. Otherwise a SubmitItemUpdate callback is attached.

OnCreateItemResult treats I/O failure as publish failure. An OK result copies the new PublishedFileId into SteamSection and immediately begins UpdateUGCItem. FileNotFound and NotLoggedOn receive distinct status strings; other results become generic failure.

OnSubmitItemUpdateResult handles I/O failure, success (sets Done status and stores the published ID), FileNotFound, LimitExceeded, and generic EResult failure. OnSuccess and OnFailed delegate to callbacks supplied by the caller. Dispose releases both CallResult objects.

## Static Steamworks lifecycle

IsSteamBuild is determined by the presence of steam_appid.txt in the current working directory. Init is idempotent when already initialized. Otherwise it requires the Steam build marker, Packsize.Test, DllCheck.Test, and SteamAPI.Init to succeed. It installs the warning-message hook, registers the lobby-join callback, and sets IsInit.

Shutdown disposes the lobby callback and current operation, clears CurrentOperation, calls SteamAPI.Shutdown, and clears IsInit.

Service calls SteamAPI.RunCallbacks. When CurrentOperation.Done becomes true, it invokes OnFailed or OnSuccess according to Failed, disposes the operation, and clears CurrentOperation. PublishUGC refuses to start when an operation is already present; otherwise it creates one operation and returns true.

WorkshopURL returns empty when the Steam AppID is invalid; otherwise it constructs the app's Workshop URL. A lobby-join request displays a modal message that multiplayer invitations cannot be accepted in the map editor.

## Host scheduling and UI interaction

Program initializes Steamworks only for a Steam build and shuts it down after Application.Run returns. MainForm owns a 500 ms timer that calls SteamworksUGC.Service while initialized. SteamDialog owns a separate 500 ms timer for displaying CurrentOperation.Status; its timer is stopped/disposed when the dialog closes.

This split matters for a port: callback servicing is owned by the main editor loop, not by the publish dialog. A replacement must preserve one serialized operation at a time and keep callback pumping alive while the dialog is open.

## Rust mapping and risks

No Steamworks coordinator or Workshop publishing implementation was found in the current Rust target tree. This source inspection does not prove the native Steamworks callback thread or reentrancy guarantees. A Rust port should isolate the SDK callback pump from UI state, transfer results through a thread-safe non-blocking channel, and ensure UI controls are only changed on the UI thread. It should also make cancellation and shutdown behavior explicit.

## Remaining audit

- [ ] Inspect Steamworks.NET callback wrappers and generated native method bindings.
- [ ] Verify native DLL architecture, deployment, and Packsize/DllCheck behavior.
- [ ] Test every EResult, I/O failure, invalid handle, and shutdown-during-operation path.
- [ ] Confirm callback thread and reentrancy guarantees.
- [ ] Define an async, non-blocking Rust operation state machine with deterministic cancellation and teardown.
