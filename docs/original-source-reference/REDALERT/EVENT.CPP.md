# REDALERT/EVENT.CPP

**Review status:** Partially reviewed — function index inspected.
**Upstream:** [EVENT.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/EVENT.CPP)

## Purpose
Defines and executes queued simulation events. Events encode commands such as target actions, mission changes, navigation updates, cell-affecting object changes, sidebar production, special flags, and animation sequencing.

## Main class
`EventClass` stores a typed event and its payload. Multiple constructors create the appropriate payload shape for each event family; `Execute` dispatches the command.

## Function inventory
The source index lists constructors for ID/cell events, target events, mission changes, navigation-computer changes, cell/object-type changes, sidebar build events, special flags, animation sequencing, mega-missions, and variable-sized events. `Execute` performs the queued command.

## Difference from TD
Red Alert includes a variable-sized event constructor. Preserve payload length and ownership rules rather than forcing every event into a fixed-size structure.

## Dependencies
Event queue/game loop, object and mission systems, navigation, map/cell state, sidebar/factory production, and animation sequencing.

## Rust mapping
Related foundations: `crates/rts-core/src/queue.rs`, `logic_runtime.rs`, `connection.rs`, and `crates/red-alert/src/lib.rs`. Status: no complete RA event enum/payload/executor parity established.
