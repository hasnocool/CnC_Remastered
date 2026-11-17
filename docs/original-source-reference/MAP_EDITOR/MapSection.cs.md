# MapSection.cs — map bounds, theatre selection, and property conversion

> **Source status: Body reviewed. Rust mapping: not yet implemented.** The complete 3,583-byte source body was inspected at the pinned upstream commit. Call-site coverage beyond the direct Map/TypeDescriptor contract remains open.

- Upstream source: [CnCTDRAMapEditor/Model/MapSection.cs](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Model/MapSection.cs)
- Upstream blob: 26859015b37286681ea444f1371131afc7e295ee
- Namespace: MobiusEditor.Model
- Imports: System; System.Collections.Generic; System.ComponentModel; System.Globalization; System.Linq; System.Runtime.CompilerServices.

## TheaterTypeConverter

Public class deriving from TypeConverter.

| Method | Exact behavior |
|---|---|
| CanConvertFrom(ITypeDescriptorContext context, Type sourceType) : bool | Returns true only when context is a MapContext and sourceType is exactly string. |
| CanConvertTo(ITypeDescriptorContext context, Type destinationType) : bool | Returns true only for MapContext and destinationType exactly string. |
| ConvertTo(ITypeDescriptorContext context, CultureInfo culture, object value, Type destinationType) : object | Returns null unless value is TheaterType and CanConvertTo succeeds; otherwise returns TheaterType.Name. Culture is unused. |
| ConvertFrom(ITypeDescriptorContext context, CultureInfo culture, object value) : object | Returns null unless context/value satisfy CanConvertFrom. Casts context to MapContext, reads Map.TheaterTypes, selects the first t for which t.Equals(value), otherwise falls back to TheaterTypes.First(). Culture is unused. |

The fallback depends on TheaterType.Equals(object), which accepts strings and compares Name with StringComparison.OrdinalIgnoreCase. A syntactically valid but unknown theatre string therefore silently resolves to the first configured theatre. An empty TheaterTypes list throws from First(); this is not converted into a validation error.

## MapSection

Public class implementing INotifyPropertyChanged.

- Event: PropertyChanged : PropertyChangedEventHandler.
- Private backing fields: x:int, y:int, width:int, height:int, theater:TheaterType. No explicit initializers are present, so CLR defaults are 0 and null.
- Public X, Y, Width, Height properties each expose get/set around their corresponding backing field and carry DefaultValue(0).
- Public Theater property carries TypeConverter(typeof(TheaterTypeConverter)) and DefaultValue(null).
- SetField<T>(ref T field, T value, [CallerMemberName] string propertyName = null) : bool is protected. EqualityComparer<T>.Default.Equals suppresses redundant assignments. Otherwise it stores the new value, raises PropertyChanged, and returns true.
- OnPropertyChanged(string propertyName) is protected and invokes PropertyChanged only when a subscriber exists.

The section is only a data/notification object: it does not validate non-negative coordinates, positive dimensions, map bounds, or a non-null theatre. Those constraints must be enforced by the owning editor/serializer if required.

## Direct integration

Map owns a readonly MapSection instance. Map.Theater delegates to MapSection.Theater; Map.TopLeft, Size, and Bounds delegate to X/Y/Width/Height. MapContext implements ITypeDescriptorContext and exposes its Map through Instance. MapContext.GetService returns null, OnComponentChanged is empty, and OnComponentChanging returns true.

## Rust comparison

The current crates/map-editor-model/src/lib.rs MapDocument stores GameVariant, MapDimensions, cell layers, and waypoint coordinates. It has no MapSection equivalent, theatre catalog/selection, property-grid converter, change notification, or map-origin/bounds state. A future Rust model should separate validated map bounds/theatre identity from any UI-specific string conversion. Do not copy the unknown-name-to-first-theatre fallback without an explicit compatibility decision.

## Verification

Source body inspected; no C# runtime test, Rust test, or serialization parity test was run in this audit batch.
