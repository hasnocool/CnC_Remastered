# TIBERIANDAWN SOUNDDLG.CPP — source guide

> **Audit status: partially reviewed.** Based on the pinned source translation unit, direct includes, and original function index. Exact header/member inventory, complete function-body explanations, call graphs, and Rust parity remain pending.

- Source: [TIBERIANDAWN/SOUNDDLG.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/SOUNDDLG.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 13457 characters.

## Plain-English purpose

Audio settings UI; Red Alert also draws music-list entries.

## Direct includes

- function.h
- sounddlg.h

## Original function index

- SoundControlsClass::Process — Handles all the options graphic interface.

## Integration and porting concerns

Settings UI is separate from the audio playback backend; RA includes music-list presentation.

## Outstanding audit
- [ ] Paired header, exact declarations, members, globals, constants, and signatures.
- [ ] Every function body, branch, return contract, side effect, and failure path.
- [ ] Cross-file callers and native/protocol dependencies.
- [ ] TD/RA behavior comparison.
- [ ] Rust paths and tests with explicit parity status.
