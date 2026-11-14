# TIBERIANDAWN ANIM.CPP — source guide

> **Audit status: partially reviewed.** This first pass inspects the pinned translation unit, its direct includes, and original source function index. Paired headers, exhaustive symbols, full body explanations, call graphs, and Rust parity remain outstanding.

- Upstream: [TIBERIANDAWN/ANIM.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/ANIM.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Translation-unit size: 62589 characters.

## Plain-English system purpose

Animation runtime object: owns animation lifecycle, attachment to parent objects, render-layer placement, coordinate adjustment, marking/overlap, and frame AI. Red Alert adds atom-damage and name lookup helpers.

## Direct compile-time dependencies

- function.h

## Original function index

- AnimClass::AI — This is the low level anim processor.
- AnimClass::Adjust_Coord — Adjusts anim coordinates
- AnimClass::AnimClass — The constructor for animation objects.
- AnimClass::As_Target — Converts the animation into a target value.
- AnimClass::Attach_To — Attaches animation to object specified.
- AnimClass::Sort_Above — Sorts the animation above the target specified.
- AnimClass::Center_Coord — Determine center of animation.
- AnimClass::Detach — Remove animation if attached to target.
- AnimClass::Draw_It — Draws the animation at the location specified.
- AnimClass::In_Which_Layer — Determines what render layer the anim should be in.
- AnimClass::Init — Performs pre-scenario initialization.
- AnimClass::Mark — Signals to map that redrawing is necessary.
- AnimClass::Middle — Processes any middle events.
- AnimClass::Occupy_List — Determines the occupy list for the animation.
- AnimClass::Overlap_List — Determines the overlap list for the animation.
- AnimClass::Render — Draws an animation object.
- AnimClass::Sort_Y — Returns with the sorting coordinate for the animation.
- AnimClass::Start — Processes initial animation side effects.
- AnimClass::delete — Returns an anim object back to the free pool.
- AnimClass::new — Allocates an anim object from the pool.
- AnimClass::~AnimClass — Destructor for anim objects.
- AnimClass::Validate — validates anim pointer
- Shorten_Attached_Anims — Reduces attached animation durations.

These entries preserve the source's own responsibility index. They do not establish all branch behavior, state changes, error paths, return contracts, or resource ownership.

## TD/RA comparison
The matching game variant exists in the other game directory. A true parity comparison requires inspecting both complete implementations and paired headers; do not assume the variants are identical.

## Rust mapping
No equivalent is asserted without inspecting the current Rust implementation and tests. Candidate port concerns: deterministic animation timing, attachment ownership, render-layer ordering, and animation lifetime.

## Outstanding audit checklist
- [ ] Inspect paired header and enumerate all classes, structs, enums, fields, globals, constants, macros, and exact signatures.
- [ ] Explain each function body: parameters, return values, branches, loops, side effects, and failure paths.
- [ ] Trace callers/callees and shared global state.
- [ ] Compare TD/RA implementations where both exist.
- [ ] Map every behavior to Rust path and tests; distinguish implemented, partial, absent, and divergent.
