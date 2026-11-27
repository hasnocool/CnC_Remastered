# Map editor serialization contract — Tiberian Dawn and Red Alert

> **Audit status: cross-file contract partially reviewed.** This page consolidates observed behavior from both GamePlugin implementations, Utility/INI.cs, SteamSection, and the MainForm call path. It is not a substitute for fixture tests or a complete audit of each source file.

Pinned upstream: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.

## Shared plugin boundary

The host uses IGamePlugin for both variants. It exposes GameType, Map, MapImage, Dirty, New(theater), Load(path, FileType), Save(path, FileType), and IDisposable. MainForm selects the concrete plugin by GameType and owns normal load/save interaction.

## FileType behavior is variant-specific

| FileType | Tiberian Dawn | Red Alert |
|---|---|---|
| INI | Reads/writes paired .ini text and .bin map data | Reads/writes .mpr text plus .tga preview and .json metadata |
| BIN | Same paired .ini/.bin path as INI | Aliased to the same .mpr/.tga/.json path as INI; no separate .bin sidecar in this plugin |
| MEG | Reads .ini and .bin entries; writes .INI, .BIN, .TGA, .JSON package entries | Reads the first .mpr entry; writes .MPR, .TGA, .JSON package entries |
| PGM | Same package path, but Steam section is omitted from the INI | Same package path, but Steam section is omitted from the INI |

Unsupported values throw NotSupportedException. FileType is not a universal extension mapping: the two game plugins implement different container and map-data contracts.

## INI lexical and reflection rules

The shared parser stores sections and keys in insertion-ordered, case-insensitive OrderedDictionary instances. Section headers use a bracket regex; comment lines beginning with # or ; are ignored; key/value parsing stops at the first semicolon. INISectionCollection.Extract removes a section from the source collection.

ParseSection and WriteSection use reflection plus TypeDescriptor converters. Public writable properties are parsed, public readable properties are written, NonSerializedINIKey properties are excluded, and converters are selected from descriptors or the property type. ParseSection has a bool-only yes/no fallback when a converter throws FormatException. WriteSection emits value types and non-null reference types. MapContext supplies game/map context to converters.

Each plugin extracts recognized sections and assigns the remaining collection to extraSections. Save re-adds those sections, preserving unknown sections through an editor round trip.

## Tiberian Dawn format contract

The standalone/package map payload is a separate binary stream. Each map cell is row-major and contains two interleaved bytes: template ID then icon ID. Empty cells write 0xFF for both.

The INI sections consumed by the editor are Basic, Map, Briefing, Steam, TeamTypes, Triggers, Terrain, Overlay, Smudge, Infantry, Units, Aircraft, Structures, Base, Waypoints, and CellTriggers.

TD TeamTypes uses separate comma-delimited boolean fields for roundabout, learning, suicide, autocreate, and mercenary, followed by recruit priority, max/initial counts, fear, class count and TYPE:COUNT entries, mission count and mission/argument entries, and reinforcable/prebuilt flags. TD Triggers uses a six-field record in the inspected writer. Do not reuse the RA packed flags or 18-token trigger parser.

## Red Alert format contract

RA's MapPack is Base64-wrapped LCW data. The decompressed layout is plane-oriented: a UInt16 template ID for every cell, followed by one icon byte for every cell. OverlayPack is a separate one-byte-per-cell stream with -1 as the empty sentinel. Compression chunks contain UInt16 compressed and decompressed lengths followed by LCW data; input chunks are at most 8,192 decompressed bytes and encoded Base64 text is split into 70-character values.

The INI sections consumed by the editor are Basic, Map, Steam, TeamTypes, Trigs, MapPack, Terrain, OverlayPack, Smudge, Units, Aircraft, Ships, Infantry, Structures, Base, Waypoints, CellTriggers, and Briefing.

RA TeamTypes uses a packed flag integer: 0x01 roundabout, 0x02 suicide, 0x04 autocreate, 0x08 prebuilt, and 0x10 reinforcable. Its Trigs section has 18 positional tokens. The parser repairs legacy union payloads by masking selected event/action data to one byte and clamps Text Trigger action data to 1..209.

## Workshop metadata and publishing

SteamSection contains Title, Description, PreviewFile, Visibility, and PublishedFileId. Both serializers include [Steam] except for PGM. SteamDialog writes a PGM package to a temporary directory and requests a UGC create/update operation. SteamworksUGC allows one active operation and requires its callback pump to be serviced by MainForm's 500 ms timer. Program owns Init/Shutdown.

A concrete risk was observed: SteamDialog ignores the bool returned by plugin.Save when producing MAPDATA.PGM, then proceeds to PublishUGC. A port should stop before upload if package validation/save fails.

## Rust parity boundary at the target commit

The current crates/map-editor-model provides CellLayers (template ID plus logical icon byte), MapDocument, cell dimensions, waypoints, CellMetrics, and object-selection plans. Rust also has raw TD BIN and RA Base64/LCW MapPack decoders in `rts-formats::td_map` and `rts-formats::ra_map_pack`, plus partial template asset metadata/ICON-set resolution in `rts-core::template_catalog` and `renderer::template_assets`. These are decoders/resolvers, not full TD/RA GamePlugin serializers: no matching map-payload writers, editor INI parser/writer, complete IGamePlugin lifecycle, SteamSection model, Steamworks UGC state machine, or Workshop UI was found. `crates/rts-core/src/inibin.rs` is only a marker for the intentionally empty legacy REDALERT/INIBIN translation unit and must not be mistaken for the editor INI codec.

## Required parity tests

- Golden INI parse/write tests for case-insensitive names, comments, semicolon truncation, converters, and unknown-section preservation.
- TD paired .ini/.bin byte-for-byte round trips, including empty cells and invalid template/icon values.
- RA MapPack LCW decode tests for chunk boundaries, truncated headers, invalid Base64, and map-size bounds; add matching encoder/decoder round trips and null-sentinel atlas integration tests.
- TD and RA TeamTypes/Triggers schema fixtures, malformed token counts, unknown type names, and numeric overflow.
- PGM omission of Steam metadata, MEG entry naming/casing, preview/JSON sidecar generation, and validation failure.
- Workshop create/update callback success/failure, concurrent publish rejection, callback pump shutdown, and save-after-dialog behavior.


## Template type resolution and Rust decoder boundary — Tranche 6S-C-J

The RA editor serializer resolves MapPack types only after decoding the two planes. For each cell it reads a UInt16 type ID, finds a matching `TemplateType` by `Equals(ushort)`, nulls a type whose `Theaters` excludes the active theatre, then reads the icon plane. For non-Clear templates, `iconValue >= NumIcons` clears the cell. Save writes the UInt16 type plane followed by icon bytes; null cells use `ushort.MaxValue` and `byte.MaxValue`. TD remains interleaved byte pairs with null sentinel `0xFF,0xFF`. Source: [RA load](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/GamePlugin.cs#L478), [RA save](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/GamePlugin.cs#L1486), [TD load](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/GamePlugin.cs#L831), [TD save](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/GamePlugin.cs#L1161).

Rust's `TdTemplateMap` and `RaTemplateMap` preserve raw cell IDs (including TD 255 and RA 65535) rather than normalizing them to empty cells. The current `TemplateAssetProvider` atlas builders collect every raw ID and call `load_icon_sets` before atlas construction; they do not filter those sentinels. Since neither sentinel is in the corresponding source-derived catalog, maps with empty cells reach `UnknownTemplateType`. This is a source-level integration defect; it was not executed in this run. Preserve raw decode fidelity if desired, but normalize sentinels at the renderer/map-model boundary and add regression tests for both variants.

Rust unit tests exist in the two decoder modules and template catalog, but they were not executed here and are not retail scenario validation. The RA asset catalog currently has IDs0–399; the C# editor registry includes ID400 AntHill, while the pinned RA `DEFINES.H` defines `FIXIT_ANTS` at line87 and conditionally includes the enum/CData entry. Reconcile this macro/build contract before claiming full catalog parity.
