# TIBERIANDAWN/MAP.H

**Review status:** Partially reviewed.  
**Upstream:** [TIBERIANDAWN/MAP.H](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/MAP.H)

## Purpose

This header declares the Tiberian Dawn `MapClass` interface and the map-related types/state that other game systems use. The implementation lives primarily in `MAP.CPP`.

## Main class

### `MapClass`

The class is the public contract for the game's world grid. Other systems use it to ask about cells, place or remove objects, check visibility, and load/save the map image. The header is where those operations and their data relationships are declared; the .CPP file contains their behavior.

## Important relationship

Read this file with [MAP.CPP](MAP.CPP.md). A port that copies only the binary reader but omits the map class's occupancy and lifecycle rules will not reproduce the original map behavior.

## Rust port mapping

There is not yet a complete runtime map class in the Rust workspace. The editor model is a foundation, not a behaviorally complete replacement.

## Review note

This page intentionally avoids guessing at every member's semantics before a full declaration-by-declaration pass. The next audit should enumerate all fields, constants, helper types, and method signatures in this header.
