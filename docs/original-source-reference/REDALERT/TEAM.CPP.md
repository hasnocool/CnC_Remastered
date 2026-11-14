# Red Alert TEAM.CPP — source guide

> **Audit status: partially reviewed.** Grounded in the pinned Red Alert translation unit and its original function index. Exact paired-header members, complete body explanations, call graphs, and Rust parity are still pending.

- Source: [REDALERT/TEAM.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/TEAM.CPP)
- Pinned upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`
- Translation-unit size: 127,793 characters.

## Plain-English purpose

Implements live Red Alert team instances, including group membership, AI, team mission progress, recruitment, and coordination of the units assigned to a team.

## Direct compile-time dependencies

- `function.h`
- `mission.h`

## Original source function index

- TeamClass::AI — Process team logic.
- TeamClass::Add — Adds specified object to team.
- TeamClass::Assign_Mission_Target — Sets teams mission target and clears old target
- TeamClass::Calc_Center — Determines average location of team members.
- TeamClass::Can_Add — Determines if the specified object can be added to team.
- TeamClass::Control — Updates control on a member unit.
- TeamClass::Coordinate_Attack — Handles coordinating a team attack.
- TeamClass::Coordinate_Conscript — Gives orders to new recruit.
- TeamClass::Coordinate_Do — Handles the team performing specified mission.
- TeamClass::Coordinate_Move — Handles team movement coordination.
- TeamClass::Coordinate_Regroup — Handles team idling (regrouping).
- TeamClass::Debug_Dump — Displays debug information about the team.
- TeamClass::Detach — Removes specified target from team tracking.
- TeamClass::Fetch_A_Leader — Looks for a suitable leader member of the team.
- TeamClass::Has_Entered_Map — Determines if the entire team has entered the map.
- TeamClass::Init — Initializes the team objects for scenario preparation.
- TeamClass::Is_A_Member — Tests if a unit is a member of a team
- TeamClass::Is_Leaving_Map — Checks if team is in process of leaving the map
- TeamClass::Lagging_Units — Finds and orders any lagging units to catch up.
- TeamClass::Recruit — Attempts to recruit members to the team for the given index ID.
- TeamClass::Remove — Removes the specified object from the team.
- TeamClass::Scan_Limit — Force all members of the team to have limited scan range.
- TeamClass::Suspend_Teams — Suspends activity for low priority teams
- TeamClass::TMision_Patrol — Handles patrolling from one location to another.
- TeamClass::TMission_Attack — Perform the team attack mission command.
- TeamClass::TMission_Follow — Perform the "follow friendlies" team command.
- TeamClass::TMission_Formation — Process team formation change command.
- TeamClass::TMission_Invulnerable — Makes the entire team invulnerable for a period of tim
- TeamClass::TMission_Load — Tells the team to load onto the transport now.
- TeamClass::TMission_Loop — Causes the team mission processor to jump to new location.
- TeamClass::TMission_Set_Global — Performs a set global flag operation.
- TeamClass::TMission_Spy — Perform the team spy mission.
- TeamClass::TMission_Unload — Tells the team to unload passengers now.
- TeamClass::TeamClass — Constructor for the team object type.
- TeamClass::Took_Damage — Informs the team when the team member takes damage.
- TeamClass::operator delete — Deallocates a team object.
- TeamClass::operator new — Allocates a team object.
- TeamClass::~TeamClass — Team object destructor.
- _Is_It_Breathing — Checks to see if unit is an active team member.
- _Is_It_Playing — Determines if unit is active and an initiated team member.

The original index is a responsibility map, not a semantic substitute for reading the full function body.

## Outstanding audit

- [ ] Complete declarations, members, globals, constants, and exact signatures.
- [ ] Explain each function body, including branches, state changes, return behavior, and side effects.
- [ ] Trace call sites and shared state.
- [ ] Compare exact behavior against the Tiberian Dawn counterpart.
- [ ] Map to Rust modules and tests with explicit parity status.
