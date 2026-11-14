# TIBERIANDAWN/HOUSE.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [HOUSE.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/HOUSE.CPP)

## Purpose
Implements a player's faction/house state. It manages money, storage capacity, production permissions, production queues, ownership of objects, threat information, flags, and house-level AI.

## Main class
`HouseClass` represents one house/player faction in a match. It is the strategic accounting and production owner, not the individual unit/building object.

## Function inventory highlights
The index includes house AI; production abandonment/start; nuclear-piece collection; Tiberium storage capacity and threat adjustment; house lookup by number; handicap assignment; base-under-attack alerts; available-money calculation; multiple `Can_Build` overloads for aircraft/buildings/infantry/units and general build checks; destruction/removal of house objects; object detach; enemy-building queries; flag attach/remove; and win/loss/destroy flags. The remaining index contains additional economy, alliance, and production operations to expand in the full symbol pass.

## Dependencies
Object/type registries, building factories, unit/infantry/aircraft production, map threat regions, house flags, combat events, scenario and multiplayer setup.

## Rust mapping
Related foundation: `crates/rts-core/src/house.rs`, `player_manager.rs`, `rules_defaults.rs`. Status: foundational house data exists; complete economy, production legality, alliances, threat and lifecycle behavior remains unverified.
