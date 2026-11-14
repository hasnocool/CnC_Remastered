# BasicSection.cs — editor model source guide

> **Audit status: source inspected; explicit declarations and methods documented. Rust parity remains pending.**

- Upstream: [CnCTDRAMapEditor/Model/BasicSection.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/BasicSection.cs)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 6857 characters.
- Namespace: MobiusEditor.Model

## Purpose
Scenario [Basic] metadata model with change notifications and legacy INI text conversion.

## Types and exact members
BooleanTypeConverter : TypeConverter
- CanConvertFrom/CanConvertTo permit string conversion only when context is MapContext.
- ConvertTo(bool) emits "1" or "0"; unsupported type/context returns null.
- ConvertFrom(string) examines first uppercased character; T, Y, or 1 means true; empty or other text means false.

PercentageTypeConverter : TypeConverter
- Conversion is restricted to MapContext and string.
- ConvertTo(int) emits percent/100M when FractionalPercentages is enabled, otherwise integer text.
- ConvertFrom(string) parses decimal fraction and multiplies by 100 when fractional mode and dot present; otherwise parses integer. Invalid text returns null.

BasicSection : INotifyPropertyChanged
Properties/backing fields: Name/name (default null); CarryOverCap/carryOverCap (int, -1, percentage converter); CarryOverMoney/carryOverMoney (int, 100, percentage converter); Intro/intro (null); Theme/theme ("No Theme"); Percent/percent (100, percentage converter); Player/public field player (null); Action/action ("x"); Lose/lose ("x"); Win/win ("x"); Brief/brief ("x"); Author/author (null); BasePlayer/basePlayer (null, NonSerializedINIKey); SoloMission/soloMission (false). PropertyChanged event.
- SetField<T> suppresses equal assignments, otherwise assigns and raises PropertyChanged using CallerMemberName.
- OnPropertyChanged invokes subscribers.
Dependencies: MobiusEditor.Utility attributes; .NET component-model, CultureInfo, generic collections. Locale-sensitive parsing/formatting and fractional-percentage compatibility require tests.

## Port status
No Rust equivalent is asserted until the current Rust workspace is inspected and behavior is mapped to implementation paths and tests.
