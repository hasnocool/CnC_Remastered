# Tiberian Dawn POWER.CPP — source guide

> **Audit status: partially reviewed.** Based on the pinned upstream translation unit and its original function index. Header/member inventory, complete body explanations, call graph, and verified Rust parity remain open.

- Source: [TIBERIANDAWN/POWER.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/POWER.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 22,270 characters.

## Plain-English purpose

Implements house power accounting and power-state behavior used by production buildings and power-dependent gameplay systems.

## Direct compile-time dependencies

- `function.h`

## Original source function index

- PowerClass::AI — Process the power bar logic.
- PowerClass::Draw_It — Renders the power bar graphic.
- PowerClass::Init_Clear — Clears all the power bar variables.
- PowerClass::One_Time — One time processing for the power bar.
- PowerClass::PowerButtonClass::Action — Handles the mouse over the power bar area.
- PowerClass::PowerClass — Default constructor for the power bar class.
- PowerClass::Refresh_Cells — Intercepts the redraw logic to see if sidebar to redraw too.
- Power_Height — Given a value figure where it falls on bar

The list preserves original source descriptions; it is not evidence of every branch, state mutation, return contract, or side effect.

## Outstanding audit

- [ ] All classes/structs/enums/aliases/macros/globals/constants.
- [ ] All members and fields, including types, visibility, initialization, and purpose.
- [ ] Exact signatures, parameters, return values, overloads.
- [ ] Function-by-function body walkthrough and side effects.
- [ ] Callers/callees and shared-state dependencies.
- [ ] Red Alert differences and Rust/test mapping.
