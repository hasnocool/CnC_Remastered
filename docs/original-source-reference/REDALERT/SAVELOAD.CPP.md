# REDALERT SAVELOAD.CPP — source guide

> **Audit status: partially reviewed.** This first pass inspects the pinned translation unit, its direct includes, and original source function index. Paired headers, exhaustive symbols, full body explanations, call graphs, and Rust parity remain outstanding.

- Upstream: [REDALERT/SAVELOAD.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/SAVELOAD.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation-unit size: 53991 characters.

## Plain-English system purpose

Game-state serialization and restoration. It saves/loads miscellaneous state and objects, encodes/decodes pointer references, exposes savefile metadata, and restores global runtime state. RA additionally handles multiplayer values/messages and player reconciliation.

## Direct compile-time dependencies

- function.h
- vortex.h
- tcpip.h
- ccdde.h

## Original function index

- Code_All_Pointers — Code all pointers.
- Decode_All_Pointers — Decodes all pointers.
- Get_Savefile_Info — gets description, scenario #, house
- Load_Game — loads a saved game
- Load_MPlayer_Values — Loads multiplayer-specific values
- Load_Misc_Values — loads miscellaneous variables
- MPlayer_Save_Message — pops up a "saving..." message
- Put_All — Store all save game data to the pipe.
- Reconcile_Players — Reconciles loaded data with the 'Players' vector
- Save_Game — saves a game to disk
- Save_MPlayer_Values — Saves multiplayer-specific values
- Save_Misc_Values — saves miscellaneous variables

These entries preserve the source's own responsibility index. They do not establish all branch behavior, state changes, error paths, return contracts, or resource ownership.

## TD/RA comparison
The matching game variant exists in the other game directory. A true parity comparison requires inspecting both complete implementations and paired headers; do not assume the variants are identical.

## Rust mapping
No equivalent is asserted without inspecting the current Rust implementation and tests. Candidate port concerns: stable entity identity, pointer relocation, versioned serialization, and atomic load failure behavior.

## Outstanding audit checklist
- [ ] Inspect paired header and enumerate all classes, structs, enums, fields, globals, constants, macros, and exact signatures.
- [ ] Explain each function body: parameters, return values, branches, loops, side effects, and failure paths.
- [ ] Trace callers/callees and shared global state.
- [ ] Compare TD/RA implementations where both exist.
- [ ] Map every behavior to Rust path and tests; distinguish implemented, partial, absent, and divergent.
