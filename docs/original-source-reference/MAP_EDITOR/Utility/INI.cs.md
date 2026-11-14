# Utility/INI.cs — editor INI parser, section model, reflection codec, and diff

> **Audit status: partially reviewed.** Parser/collection structure and reflection-based ParseSection/WriteSection behavior were inspected. Full method-by-method auditing of collection mutation, diff semantics, converters, and all call sites remains open.

- Upstream: https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/Utility/INI.cs
- Pinned source blob: 644962ee1633100acb4e1c9e5733b480f8bdaa9d
- Source size: 18,330 characters
- Namespace: MobiusEditor.Utility

## Types and responsibilities

- INIHelpers: compiled regular expressions for section headers, key/value lines, and comments, plus a textual diff-prefix selector.
- INIKeyValueCollection: ordered, case-insensitive key/value storage; generic Get/Set use TypeDescriptor converters.
- INISection: named section containing an INIKeyValueCollection; parses key/value lines and formats its contents.
- INISectionCollection: ordered, case-insensitive named sections; Add, AddRange, Remove, and Extract operations.
- INI: owns Sections, parses TextReader/string input, enumerates sections, and formats a complete document.
- INIDiffType: flags None=0, Added=1, Removed=2, Updated=4, AddedOrUpdated=5.
- INISectionDiff and INIDiff: record key-level and section-level differences.
- NonSerializedINIKeyAttribute: property marker used by the reflection codec to omit a property.

## Lexical and collection behavior

The regexes are:

- Section: ^\s*\[([^\]]*)\]
- Key/value: ^\s*(.*?)\s*=([^;]*)
- Comment: ^\s*(#|;)

Both key/value collections and section collections use OrderedDictionary with StringComparer.OrdinalIgnoreCase. Insertion order is retained while lookups are case-insensitive.

INI.Parse(TextReader) reads one line at a time. A matching section header selects/creates the current section. Before the first section, lines are ignored. Lines beginning with # or ; after optional whitespace are skipped. Other lines are passed to the current section parser. The key/value regex captures the value only up to the first semicolon, so an inline semicolon suffix is not retained as value data.

INISectionCollection.Extract(name) returns the named section and removes it from the collection. The game plugins use this destructively: after recognized sections are consumed, the remaining collection is assigned to extraSections and re-added during save. This is the mechanism that preserves unknown/unrecognized sections across an editor round trip.

INI.ToString emits each section header, any non-empty key/value body, blank lines between sections, and a final newline.

## Reflection-based model conversion

ParseSection<T>(context, section, data):

1. Gets TypeDescriptor property descriptors for data.
2. Enumerates public instance properties with a setter.
3. Skips properties marked NonSerializedINIKey.
4. Only processes a property when the INI section contains a key matching its property name.
5. Uses the descriptor converter, falling back to TypeDescriptor.GetConverter(property.PropertyType), and converts from string.
6. Catches FormatException only for bool properties and recognizes yes/no as a fallback. Other format errors are rethrown.

WriteSection<T>(context, section, data):

1. Enumerates public instance properties with a getter.
2. Skips NonSerializedINIKey properties.
3. Serializes every value-type property and every non-null reference-type property.
4. Uses the TypeDescriptor converter to convert to string and writes the property name as the INI key.

MapContext is passed by the plugins so property converters can depend on map/game context, including editor-specific percentage conversion behavior. DefaultValue attributes alone do not suppress serialization in this helper; inclusion is based on property type/value and converter support.

## Porting consequences

- Preserve case-insensitive lookups and insertion order.
- Preserve semicolon truncation and comment handling if behavior parity is required.
- Keep section extraction distinct from lookup: Extract mutates the collection.
- Do not replace reflection conversion with naive string parsing; property converters and NonSerializedINIKey are part of the format contract.
- Boolean yes/no fallback is limited to FormatException and bool properties.
- Unknown-section preservation is implemented by the plugins plus this collection API, not by ParseSection alone.

## Rust mapping and evidence

No Rust INI parser/writer corresponding to this editor utility was identified in the current target tree. crates/rts-core/src/inibin.rs is a marker for the intentionally empty legacy REDALERT/INIBIN.CPP translation unit; it is not the editor's INI serializer. crates/map-editor-model currently provides a document model but no INI reflection codec. No parity tests were run in this source-audit batch.

## Remaining audit

- [ ] Inventory every public/private member and complete collection/diff behavior.
- [ ] Audit all custom TypeConverter implementations and MapContext branches.
- [ ] Trace every INI.ParseSection/WriteSection call site.
- [ ] Add tests for case folding, duplicate keys, inline semicolons, comments, converters, and unknown-section round trips.
