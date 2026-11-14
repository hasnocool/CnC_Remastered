# TIBERIANDAWN/EVENT.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [EVENT.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/EVENT.CPP)

## Purpose

Implements the game's queued event/command representation. Events are small messages that describe something the simulation should execute, such as a target action, cell change, mission update, sidebar production action, or animation step.

The event layer separates “an action was requested” from “the simulation executes that action.” This is important for deterministic game logic and network/multiplayer synchronization.

## Main class: `EventClass`

An event carries a type plus the information needed to execute it. The source has multiple constructors because different event categories require different payloads.

## Functions

| Function | Plain-English job |
|---|---|
| `EventClass` constructors | Build events for IDs/cells, target references, mission changes, navigation-computer changes, cell-affecting object types, sidebar build actions, special flags, animation sequencing, and mega-missions. |
| `Execute` | Applies the queued event/command to the game state. |

## Dependencies

Events are consumed by the game logic/event queue and can invoke object missions, navigation, production/sidebar systems, animation, and cell/map behavior. The exact event enum and payload union/struct must be documented from the header in the next pass.

## Rust port mapping

Related foundations include `crates/rts-core/src/queue.rs`, `logic_runtime.rs`, `connection.rs`, and game-specific logic modules. No complete typed equivalent of the original event payloads and execution semantics is established. Use a typed enum rather than an unvalidated byte payload; preserve stable event ordering and deterministic execution.
