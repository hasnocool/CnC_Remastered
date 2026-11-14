# TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP — source guide

> **Audit status: partially reviewed.** Pinned source, direct includes, and original function index inspected. Header/member inventory, complete body explanations, native dependency graph, and Rust parity remain pending.

- Upstream: [TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP)
- Pinned source: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation unit size: 19409 characters.

## Purpose

Keyboard input queue and polling facade, including key-message insertion, buffered retrieval, checking, and mouse-coordinate accessors.

## Direct includes

- keyboard.h
- timer.h
- mono.h

## Original source function index

- WWKeyboardClass::Put — Logic to insert a key into the keybuffer]
- WWKeyboardClass::Get — Logic to get a metakey from the buffer
- WWKeyboardClass::Check — Checks to see if a key is in the buffer
- WWKeyboardClass::Put_Key_Message — Translates and inserts wParam into Keyboard Buffer
- WWKeyboardClass::Buff_Get — Lowlevel function to get a key from key buffer
- WWKeyboardClass::Get_Mouse_X — Returns the mouses current x position in pixels
- WWKeyboardClass::Get_Mouse_Y — returns the mouses current y position in pixels
- WWKeyboardClass::Get_Mouse_XY — Returns the mouses x,y position via reference vars
- Check_Key — compatability routine for old 32 bit library

## Porting concerns

Input buffering and event order affect gameplay responsiveness. Separate platform input collection from deterministic simulation consumption; ensure thread-safe queues and avoid blocking polling.

## Outstanding audit
- [ ] Paired headers and exact types, members, macros, globals, and signatures.
- [ ] Every method's branch/loop/side-effect/error path.
- [ ] Native API interactions, allocation ownership, and call sites.
- [ ] Compare TD/RA versions.
- [ ] Rust modules and compatibility tests.
