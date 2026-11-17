# TeamType.cs — editor team composition and mission sequence

> **Source status: Body reviewed. Rust mapping: none.** Complete source body inspected at the pinned upstream commit.

- Upstream: [CnCTDRAMapEditor/Model/TeamType.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/TeamType.cs)
- Upstream blob: 9cbb2cf5d0a574290c3113b06b7bcb12ad3096c4
- Namespace: MobiusEditor.Model.
- Imports: MobiusEditor.Interface; System; System.Collections.Generic; System.Linq.

## TeamTypeClass

Public ICloneable record with Type:ITechnoType and Count:byte auto-properties. Clone returns a new record copying Type by reference and Count by value. Explicit ICloneable.Clone delegates to Clone.

## TeamTypeMission

Public ICloneable record with Mission:string and Argument:int auto-properties. Clone returns a new record copying both values. Explicit ICloneable.Clone delegates to Clone.

## TeamType

Public class implementing INamedType and ICloneable. Static readonly None string is "None". Public auto-properties: Name:string; House:HouseType; IsRoundAbout, IsLearning, IsSuicide, IsAutocreate, IsMercenary:bool; RecruitPriority:int; MaxAllowed, InitNum, Fear:byte; IsReinforcable, IsPrebuilt:bool; Origin:int; Trigger:string initialized to Model.Trigger.None. Classes:List<TeamTypeClass> and Missions:List<TeamTypeMission> are get-only lists initialized to empty lists.

Clone copies all scalar/name/house/trigger values into a new TeamType. House is shared by reference. It appends c.Clone() for every Classes entry and m.Clone() for every Missions entry, producing new list entries and containers. TeamTypeClass.Type references remain shared because TeamTypeClass.Clone copies Type directly.

Equals(object) returns this == obj for another TeamType; no operator overload is declared in this file, so this is reference equality. String comparison is case-insensitive against Name; other values use base.Equals. GetHashCode uses Name.GetHashCode() and ToString returns Name. Null Name therefore makes either method throw. Explicit ICloneable.Clone delegates to Clone.

## Map and serializer boundary

Map owns List<TeamType>. The game-specific GamePlugin implementations serialize team records and mission entries; this shared editor model does not establish identical TD and RA wire/INI schemas. The current Rust map-editor-model has no TeamType, TeamTypeClass, or TeamTypeMission model, no typed House/ITechnoType links, and no team clone behavior.

No C# runtime tests or Rust parity tests were executed.
