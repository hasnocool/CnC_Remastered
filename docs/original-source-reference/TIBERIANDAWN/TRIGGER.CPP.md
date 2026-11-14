# TIBERIANDAWN/TRIGGER.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [TRIGGER.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/TRIGGER.CPP)

## Purpose
Implements scenario triggers: named conditions/events that cause actions when something happens to a cell, house, or object. Triggers let mission designers create scripted behavior without hard-coding each mission.

## Main class
`TriggerClass` represents a configured trigger and its activation/action state.

## Function inventory
`Action_From_Name` and `Event_From_Name` convert INI text to trigger enums; `Name_From_Action/Event` convert enums back to text; `Action_Need_Team`, `Event_Need_Data`, `Event_Need_House`, and `Event_Need_Object` describe which parameters are required; `As_Pointer` and `As_Target` resolve trigger identity; `Init` resets trigger state; `Read_INI`/`Write_INI` serialize triggers; `Spring` overloads process cell-, house-, and object-based trigger activation; `Remove` unregisters a trigger; constructors/destructor and pool allocation manage lifetime; `Validate` checks references. `Do_All_To_Hunt` is a global helper that changes computer-controlled units to hunt behavior.

## Dependencies
Scenario INI parsing, map cells, houses, game objects, team types, event queue, and mission/action dispatch.

## Rust mapping
Related foundations: `crates/rts-core/src/queue.rs`, `logic_runtime.rs`, `map.rs`, `house.rs`. Status: no full trigger definition/evaluation/action engine is established. Use typed condition/action enums and deterministic evaluation order.
