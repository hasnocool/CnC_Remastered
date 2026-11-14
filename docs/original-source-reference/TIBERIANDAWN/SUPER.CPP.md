# Tiberian Dawn SUPER.CPP — source guide

> **Audit status: partially reviewed.** Based on the pinned source translation unit and original function index. Exact class/member inventory, full implementation walkthrough, call graph, and verified Rust parity remain outstanding.

- Source: [TIBERIANDAWN/SUPER.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/SUPER.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 23,220 characters.

## Plain-English purpose

Implements superweapon availability and recharge state, including charging, suspension, enable/remove, and discharge operations.

## Direct compile-time dependencies

- `function.h`

## Original source function index

- SuperClass::AI — Process the super weapon AI.                                            *
- SuperClass::Anim_Stage — Fetches the animation stage for this super weapon.              *
- SuperClass::Discharged — Handles discharged action for special super weapon.             *
- SuperClass::Enable — Enable this super special weapon.                                   *
- SuperClass::Forced_Charge — Force the super weapon to full charge state.                 *
- SuperClass::Impatient_Click — Called when player clicks on unfinished super weapon.      *
- SuperClass::Recharge — Starts the special super weapon recharging.                       *
- SuperClass::Remove — Removes super weapon availability.                                  *
- SuperClass::SuperClass — Constructor for special super weapon objects.                   *
- SuperClass::Suspend — Suspend the charging of the super weapon.                          *

This is the original source's index, not a complete semantic description of each implementation.

## Outstanding audit

- [ ] Every declaration, field, global, constant, macro, and exact signature.
- [ ] Every function's control flow, state mutation, return contract, and side effects.
- [ ] Callers, callees, data dependencies, and invariants.
- [ ] TD/RA differences where applicable.
- [ ] Rust implementation path and parity-test evidence.
