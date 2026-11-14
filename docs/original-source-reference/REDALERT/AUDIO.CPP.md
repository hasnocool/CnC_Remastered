# REDALERT AUDIO.CPP — source guide

> **Audit status: partially reviewed.** This first pass inspects the pinned translation unit, its direct includes, and original source function index. Paired headers, exhaustive symbols, full body explanations, call graphs, and Rust parity remain outstanding.

- Upstream: [REDALERT/AUDIO.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/AUDIO.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation-unit size: 43498 characters.

## Plain-English system purpose

Gameplay sound and speech facade. Provides positional/non-positional sound effects, speech start/stop/status, and in RA name/id conversion for voice assets and variants.

## Direct compile-time dependencies

- function.h

## Original function index

- Is_Speaking — Checks to see if the eva voice is still playing.
- Sound_Effect — General purpose sound player.
- Sound_Effect — Plays a sound effect in the tactical map.
- Speak — Computer speaks to the player.
- Speak_AI — Handles starting the EVA voices.
- Speech_Name — Fetches the name for the voice specified.
- Stop_Speaking — Forces the EVA voice to stop talking.
- Voc_From_Name — Fetch VocType from ASCII name specified.
- Voc_Name — Fetches the name for the sound effect.

These entries preserve the source's own responsibility index. They do not establish all branch behavior, state changes, error paths, return contracts, or resource ownership.

## TD/RA comparison
The matching game variant exists in the other game directory. A true parity comparison requires inspecting both complete implementations and paired headers; do not assume the variants are identical.

## Rust mapping
No equivalent is asserted without inspecting the current Rust implementation and tests. Candidate port concerns: non-blocking audio command submission, positional attenuation, voice channel ownership, and graceful missing-asset behavior.

## Outstanding audit checklist
- [ ] Inspect paired header and enumerate all classes, structs, enums, fields, globals, constants, macros, and exact signatures.
- [ ] Explain each function body: parameters, return values, branches, loops, side effects, and failure paths.
- [ ] Trace callers/callees and shared global state.
- [ ] Compare TD/RA implementations where both exist.
- [ ] Map every behavior to Rust path and tests; distinguish implemented, partial, absent, and divergent.
