# TIBERIANDAWN/BULLET.CPP

**Review status:** Partially reviewed — source function index inspected.
**Upstream:** [BULLET.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/BULLET.CPP)

## Purpose
Implements projectile objects after a weapon has been fired. A bullet has its own update logic and visual position; it is not the weapon definition itself.

## Main class
`BulletClass` represents an in-flight projectile. It is created by combat code, updated by simulation, rendered, and removed when its lifecycle completes.

## Functions
`AI` advances projectile logic; `As_Target` provides target identity; constructors initialize projectile state; `Detach` clears target tracking; `Draw_It` renders the projectile; `Init` resets projectile tracking for scenario setup; `Mark` requests map redraw; `Occupy_List` describes its map footprint; `Unlimbo` activates it; `new/delete` use the game's allocation pool; `Validate` checks the pointer/object.

## Dependencies
Weapon firing and target selection, techno/object tracking, map/display redraw, animation and warhead impact/damage logic.

## Rust mapping
Related foundations: `crates/rts-core/src/weapon.rs`, `warhead.rs`, `combat.rs`, `abstract_object.rs`, `tracker.rs`. Status: no complete projectile simulation and lifecycle is established.
