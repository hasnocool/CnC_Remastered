# Trigger.cs — scenario trigger, event, and action records

> **Source status: Body reviewed. Rust mapping: none.** Complete source body inspected at the pinned upstream commit.

- Upstream: [CnCTDRAMapEditor/Model/Trigger.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/Trigger.cs)
- Upstream blob: 90f8ddce8c087a8c24ff111602540bade1d0a60c
- Namespace: MobiusEditor.Model.
- Imports: MobiusEditor.Interface; System.

## Enumerations

- TriggerPersistantType: Volatile=0, SemiPersistant=1, Persistant=2. The misspelling is part of the source identifier contract.
- TriggerMultiStyleType: Only=0, And=1, Or=2, Linked=3.

## TriggerEvent

Public class implementing ICloneable. Static readonly string None is "None". Public auto-properties EventType:string, Team:string, Data:long have no explicit initializers. Clone creates a new TriggerEvent and copies all three values. Explicit ICloneable.Clone delegates to Clone.

## TriggerAction

Public class implementing ICloneable. Static readonly string None is "None". Public auto-properties ActionType:string, Trigger:string, Team:string, Data:long. Clone creates a new TriggerAction and copies all four values. Explicit ICloneable.Clone delegates to Clone.

## Trigger

Public class implementing INamedType and ICloneable. Static readonly None string is "None". Public auto-properties Name:string and House:string have no explicit initializers. PersistantType defaults to Volatile; EventControl defaults to Only. Event1 and Event2 are public getters/private setters initialized to distinct TriggerEvent objects whose EventType is TriggerEvent.None. Action1 and Action2 are public getters/private setters initialized to distinct TriggerAction objects whose ActionType is TriggerEvent.None (the value is the same "None" string as TriggerAction.None).

Clone constructs a new Trigger, copies Name/PersistantType/House/EventControl, and deep-clones each of the four nested event/action objects. Equals(object) uses reference equality for another Trigger (no operator overload is declared here), compares Name to a string case-insensitively, and otherwise delegates to base.Equals. GetHashCode returns Name.GetHashCode(); ToString returns Name. Explicit ICloneable.Clone delegates to Clone.

## Integration and edge cases

Map stores triggers in ObservableCollection<Trigger>. On any collection change, Map scans every Building and resets a non-empty Building.Trigger to Trigger.None when no current Trigger.Name matches it. That handler does not scan CellTrigger cells or TeamType.Trigger references. Trigger names are mutable, and cleanup is collection-change driven rather than property-change driven.

Name is nullable at the type level in this source and has no initializer; GetHashCode throws if it is null. Nested event/action references are initialized and privately set, so callers mutate their public properties rather than replace the nested objects through the Trigger API. The current Rust map-editor-model has no Trigger/TriggerEvent/TriggerAction model, collection, name-reference reconciliation, or clone contract.

TD and RA serializers use their own trigger schemas; this shared editor object is not proof that both games serialize identical fields. No runtime tests or Rust parity tests were executed.
