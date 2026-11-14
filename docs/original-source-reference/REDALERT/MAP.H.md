# REDALERT/MAP.H

**Review status:** Partially reviewed.  
**Upstream:** [REDALERT/MAP.H](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/MAP.H)

## Purpose

Declares Red Alert's `MapClass` and the types/state used by the Red Alert map runtime. Its behavior is implemented in [MAP.CPP](MAP.CPP.md).

## Main class

### `MapClass`

Represents the Red Alert world grid and exposes the operations needed to initialize it, query cells and regions, place/remove objects, update visibility, serialize the map, and maintain connected zones and bridges.

## Variant note

This is not simply a copy of `TIBERIANDAWN/MAP.H`. Red Alert's map dimensions, bridge behavior, zone logic, and map serialization must be audited independently.

## Review note

A declaration-by-declaration member list is still required. This page records the class-level responsibility without inventing member semantics.
