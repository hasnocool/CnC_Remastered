# REDALERT/BUILDING.CPP

**Review status:** Partially reviewed — function index inspected.
**Upstream:** [BUILDING.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/BUILDING.CPP)

## Purpose
Implements Red Alert buildings: construction/placement, production, animation, weapons, power, capture, destruction, docking and unit exits, and building-specific AI.

## Main class
`BuildingClass` is a placed strategic object. Unlike mobile units, it occupies a fixed multi-cell footprint and often acts as a factory, defensive weapon, service point, or power provider.

## Function index highlights
The source index includes AI, click handling, animation modes, target assignment, construction/placement legality, firing, player movement/demolition permissions, capture, Tesla-coil charging, aircraft landing checkpoints, crew generation, docking/exit coordinates, debris creation, idle state, production cancellation/abandonment, and drawing. This list is a responsibility summary; a complete method table still needs to be extracted.

## Dependencies
Building type definitions and theatre graphics, map/cell occupancy, house ownership, factories/production, power, weapons/warheads, animation, aircraft docking, and scenario INI.

## Difference from TD
Red Alert adds behaviors including Tesla charging and aircraft landing checkpoints. Confirm each override and type-specific branch from RA source; do not reuse TD building behavior blindly.

## Rust mapping
Related foundations: `crates/rts-core/src/house.rs`, `weapon.rs`, `warhead.rs`, `combat.rs`, `map.rs`, `turret.rs`; no complete RA BuildingClass equivalent.
