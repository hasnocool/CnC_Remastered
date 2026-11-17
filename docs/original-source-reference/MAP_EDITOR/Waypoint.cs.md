# Waypoint.cs — named scenario waypoint descriptor

> **Source status: Body reviewed. Rust mapping: partial number-to-coordinate storage only.**

- Upstream: [CnCTDRAMapEditor/Model/Waypoint.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Waypoint.cs)
- Upstream blob: fd2945014259e15a420defcbb81511983485e927
- Namespace: MobiusEditor.Model.
- Imports: MobiusEditor.Interface; System.

WaypointFlag enum contains None=0 and PlayerStart=1. Waypoint is a public class implementing INamedType. Name:string and Flag:WaypointFlag have private setters; Cell:int? has a public getter/setter. Constructors are Waypoint(string name, WaypointFlag flag) and Waypoint(string name), with the latter delegating using None. There is no validation of name or cell bounds.

Equals(object) returns this == obj for another Waypoint; this file declares no operator overload and the class is not partial, so that branch is reference equality. For a string it compares Name with StringComparison.OrdinalIgnoreCase; other values delegate to base.Equals. GetHashCode returns Name.GetHashCode(), and ToString returns Name. A null Name therefore makes GetHashCode throw. Do not use cross-type Equals behavior as a general hash-key contract.

Map owns a Waypoint[] catalog and the waypoint editor tool assigns Cell. The Rust MapDocument uses BTreeMap<u16,CellCoord>, which captures number-to-coordinate association but not Name, WaypointFlag, nullable unplaced state, or the original array/catalog identity. Existing Rust waypoint uniqueness/bounds tests are present in source but were not executed in this run.
