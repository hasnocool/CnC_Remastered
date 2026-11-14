# TIBERIANDAWN/SCENARIO.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [SCENARIO.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/SCENARIO.CPP)

## Purpose

Controls the scenario lifecycle: clearing old scenario state, reading a new scenario, filling in derived/default data, starting the mission, and handling win/loss/restart flow.

This is orchestration logic. It coordinates map, house, unit, building, trigger, and scenario configuration systems rather than decoding every resource itself.

## Functions

| Function | Plain-English job |
|---|---|
| `Clear_Scenario` | Clears state so a new scenario can be loaded without stale objects or settings. |
| `Do_Lose` | Runs the mission-loss presentation and related flow. |
| `Do_Restart` | Handles restarting the current mission. |
| `Do_Win` | Runs the mission-win presentation and related flow. |
| `Fill_In_Data` | Reconstructs/defaults data that is not explicitly stored in the scenario file. |
| `Read_Scenario` | Loads the scenario from disk and coordinates its setup. |
| `Restate_Mission` | Re-displays or restates the current objective. |
| `Start_Scenario` | Transitions the loaded scenario into active play. |

## Dependencies

Works with `INI.CPP`, `MAP.CPP`, `CDATA.CPP`, house/type registries, unit/building/infantry INI readers, triggers, events, and game-loop logic.

## Rust port mapping

Current related foundations include `crates/tiberian-dawn/src/lib.rs`, `crates/rts-core/src/map.rs`, `house.rs`, and `logic_runtime.rs`. There is no complete scenario orchestration pipeline established. The implementation should load data into a staging document, validate all references, then atomically activate the scenario rather than partially mutating the live simulation as parsing proceeds.
