# Tiberian Dawn TEAM.CPP — source guide

> **Audit status: partially reviewed.** Based on the pinned upstream translation unit and its original function index. Header/member inventory, complete body explanations, call graph, and verified Rust parity remain open.

- Source: [TIBERIANDAWN/TEAM.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/TEAM.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 60,382 characters.

## Plain-English purpose

Implements live team instances: grouped combat units executing team-level AI, membership, mission progression, and coordination.

## Direct compile-time dependencies

- `function.h`
- `mission.h`

## Original source function index

- Assign_Mission_Target — Sets teams mission target and clears old target
- TeamClass::Add — Adds specified object to team.                                          *
- TeamClass::AI — Process team logic.                                                      *
- TeamClass::As_Target — Converts this team object into a target number.                   *
- TeamClass::Calc_Center — Determines average location of team members.                    *
- TeamClass::Control — Updates control on a member unit.                                   *
- TeamClass::Coordinate_Attack — Handles coordinating a team attack.                       *
- TeamClass::Coordinate_Conscript — Gives orders to new recruit.                           *
- TeamClass::Coordinate_Move — Handles team movement coordination.                         *
- TeamClass::Coordinate_Regroup — Handles team idling (regrouping).                        *
- TeamClass::Coordinate_Unload — Tells the team to unload passengers now.                  *
- TeamClass::Detach — Removes specified target from team tracking.                         *
- TeamClass::Init — Initializes the team objects for scenario preparation.                 *
- TeamClass::Is_A_Member — Tests if a unit is a member of a team
- TeamClass::Recruit — Attempts to recruit members to the team for the given index ID.     *
- TeamClass::Remove — Removes the specified object from the team.                          *
- TeamClass::Suspend_Teams — Suspends activity for low priority teams
- TeamClass::Took_Damage — Informs the team when the team member takes damage.             *
- TeamClass::Validate — validates team pointer

The list preserves original source descriptions; it is not evidence of every branch, state mutation, return contract, or side effect.

## Outstanding audit

- [ ] All classes/structs/enums/aliases/macros/globals/constants.
- [ ] All members and fields, including types, visibility, initialization, and purpose.
- [ ] Exact signatures, parameters, return values, overloads.
- [ ] Function-by-function body walkthrough and side effects.
- [ ] Callers/callees and shared-state dependencies.
- [ ] Red Alert differences and Rust/test mapping.
