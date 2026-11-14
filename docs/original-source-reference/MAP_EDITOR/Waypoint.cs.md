# Waypoint.cs — editor model source guide

> **Audit status: source inspected; explicit declarations and methods documented. Rust parity remains pending.**

- Upstream: [CnCTDRAMapEditor/Model/Waypoint.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Waypoint.cs)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 1975 characters.
- Namespace: MobiusEditor.Model

## Types and members
WaypointFlag: None, PlayerStart.
Waypoint implements INamedType. Name:string and Flag:WaypointFlag have private setters; Cell:int? has public getter/setter.
Constructors: Waypoint(name, flag); Waypoint(name) defaults flag to None.

## Methods
Equals(object): for Waypoint delegates to overloaded equality operator; for string compares Name case-insensitively; otherwise base Equals.
GetHashCode returns Name.GetHashCode() (case-sensitive).
ToString returns Name.
Important caveat: case-insensitive string equality and case-sensitive hash code may be inconsistent for hash-based collections; operator definition must be found elsewhere before fully asserting Waypoint equality.

## Port status
No Rust equivalent is asserted until the current Rust workspace is inspected and behavior is mapped to implementation paths and tests.
