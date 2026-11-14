# MapSection.cs — editor model source guide

> **Audit status: source inspected; explicit declarations and methods documented. Rust parity remains pending.**

- Upstream: [CnCTDRAMapEditor/Model/MapSection.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/MapSection.cs)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 3581 characters.
- Namespace: MobiusEditor.Model

## Types and members
TheaterTypeConverter : TypeConverter
- CanConvertFrom/CanConvertTo allow string conversion only in MapContext.
- ConvertTo(TheaterType) returns its Name.
- ConvertFrom(string) searches MapContext.Map.TheaterTypes for an equal theatre; if none matches, returns the first theatre. Empty theatre lists cause First() to throw.

MapSection : INotifyPropertyChanged
- X:int, Y:int, Width:int, Height:int; each has private backing field and DefaultValue(0).
- Theater:TheaterType; private backing field, TheaterTypeConverter, DefaultValue(null).
- PropertyChanged event.
- SetField<T> equality-checks, assigns and notifies.
- OnPropertyChanged invokes subscribers.
Dependencies: LINQ, component-model, MapContext. Rust mapping unverified. Preserve the fallback/empty-list contract deliberately.

## Port status
No Rust equivalent is asserted until the current Rust workspace is inspected and behavior is mapped to implementation paths and tests.
