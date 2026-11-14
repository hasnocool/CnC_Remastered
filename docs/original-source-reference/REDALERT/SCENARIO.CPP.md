# REDALERT/SCENARIO.CPP

**Review status:** Partially reviewed — function index inspected.
**Upstream:** [SCENARIO.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/SCENARIO.CPP)

## Purpose
Controls Red Alert scenario lifecycle and the setup/teardown sequence. It reads the scenario and its INI data, clears or reconstructs state, starts missions, and handles multiplayer house/unit setup and win/loss/restart behavior.

## Main class
`ScenarioClass` is the scenario control object. The file also contains global/helper routines used by scenario loading and multiplayer initialization.

## Function index highlights
- **Load and restore:** `Clear_Scenario`, `Read_Scenario`, `Read_Scenario_INI`, `Post_Load_Game`, `Fill_In_Data`.
- **Start and finish:** `Start_Scenario`, `Do_Win`, `Do_Lose`, `Do_Restart`, `Restate_Mission`.
- **Multiplayer setup:** `Assign_Houses`, `Create_Units`, `Remove_AI_Players`, flag placement/cleanup.
- **Placement helpers:** `Clip_Move`, `Clip_Scatter`, `Scan_Place_Object`.
- **Scenario metadata and effects:** `Set_Scenario_Name`, `Set_Global_To`, palette fade helpers.

## Difference from TD
Red Alert has an explicit `ScenarioClass`, `Read_Scenario_INI`, `Post_Load_Game`, multiplayer house assignment, and palette-fade processing in this file. Its load pipeline must be audited separately.

## Dependencies
Map binary and INI parsers, theatre/type registries, map/cell placement, houses, units/infantry/buildings, triggers/events, palette and game logic.

## Rust mapping
Related foundations: `crates/red-alert/src/lib.rs`, `crates/rts-core/src/map.rs`, `house.rs`, `logic_runtime.rs`, `rgb_palette.rs`. Status: incomplete; no full RA scenario staging/load/start pipeline.
