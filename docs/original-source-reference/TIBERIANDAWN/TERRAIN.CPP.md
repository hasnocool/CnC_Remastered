# TIBERIANDAWN/TERRAIN.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [TERRAIN.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/TERRAIN.CPP)

## Purpose
Implements individual terrain objects placed on the map, such as trees and other destructible/interactive terrain features. This is distinct from the base ground template that fills map cells.

## Main class
`TerrainClass` represents a placed terrain object with its own health, animation/effects, map footprint, and scenario serialization.

## Functions
`AI`, `As_Target`, `Can_Enter_Cell`, `Catch_Fire`, `Center_Coord`, `Debug_Dump`, `Draw_It`, `Fire_Out`, `Health_Ratio`, `Init`, `Limbo`, `Mark`, `Radar_Icon`, `Read_INI`, `Start_To_Crumble`, `Take_Damage`, constructors, `Unlimbo`, `Write_INI`, pool `new/delete`, destructor, and `Validate` implement its lifecycle and interaction.

## Terrain terminology
Do not conflate:
- **Template ground:** the base terrain tiles and their theatre/template composition.
- **Terrain objects:** placed objects such as trees, which may burn, crumble, take damage, and be serialized as scenario objects.
- **Overlays:** cell-associated objects such as walls or resource overlays.

## Rust mapping
Related foundations: `crates/rts-core/src/map.rs`, `abstract_object.rs`, `tracker.rs`. Status: no complete destructible terrain-object lifecycle is established.
