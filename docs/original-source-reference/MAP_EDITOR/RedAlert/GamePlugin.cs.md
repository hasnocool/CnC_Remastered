# RedAlert/GamePlugin.cs — Red Alert editor plugin and scenario serializer

> **Audit status: partially reviewed.** This tranche inspected the constructor, New, Load, LoadINI, Save, SaveINI, Validate, compression helpers, and disposal path. The 79 KB source still needs a complete declaration/body pass, exhaustive call-site review, and fixture-based round-trip testing.

- Upstream: https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/RedAlert/GamePlugin.cs
- Pinned source blob: c7e4f0ef32ceb50c7e31676f98cf01ca62179e57
- Source size: 79,464 characters
- Type: MobiusEditor.RedAlert.GamePlugin, internal IGamePlugin implementation

## State and initialization

The implementation declares static readonly movieTypes (an ordered movie identifier list, including x and comments for missing assets) and static readonly technoTypes. The static constructor builds technoTypes from InfantryTypes.GetTypes() concatenated with UnitTypes.GetTypes(), cast to ITechnoType. Instance state includes Map, MapImage, Dirty, extraSections, and disposedValue.

The constructor creates player-start waypoints P0 through P7, general numbered waypoints 8 through 97, and Home, Reinf., and Special. It initializes BasicSection defaults, selects the first house type as player, and constructs Map with Red Alert house/theatre/template/terrain/overlay/smudge/event/action/mission/direction/infantry/unit/building/team-mission catalogs. It sets TiberiumOrGoldValue=35 and GemValue=110, subscribes to BasicSection and MapSection property changes, and creates a map-sized Bitmap only when mapImage is true. The parameterless constructor delegates to the image-producing overload.

New(string theater) selects a matching theatre or falls back to Temperate, sets the playable rectangle inset by one cell on each side, updates the base-player house, and marks Dirty=true.

## Load dispatch and INI parsing

Load(string path, FileType fileType):

- INI and BIN: parse the file at path as text INI.
- MEG and PGM: open a Megafile and select the first entry whose extension lowercases to .mpr; parse it if found.
- Other values: throw NotSupportedException.

The INI/BIN branch does not read a separate binary sidecar. This differs from Tiberian Dawn.

LoadINI begins a Map update and extracts sections in this order: Basic, Map, Steam, TeamTypes, Trigs, MapPack, Terrain, OverlayPack, Smudge, Units, Aircraft, Ships, Infantry, Structures, Base, Waypoints, CellTriggers, Briefing. Basic and Map use INI.ParseSection with MapContext(Map, true). Player is normalized to a known house, falling back to the first house, and BasePlayer is derived from it. Steam is parsed into Map.SteamSection.

TeamTypes values are comma-tokenized. The parser reads house ID, packed flags, recruit priority, initial/max counts, origin (file value plus one), trigger, class count/classes, and mission count/missions. Flag bits are 0x01 roundabout, 0x02 suicide, 0x04 autocreate, 0x08 prebuilt, 0x10 reinforcable. Class entries use TYPE:COUNT; mission entries use INDEX:ARGUMENT. Counts are bounded by Globals.MaxTeamClasses and Globals.MaxTeamMissions. Unknown classes and malformed class/mission token shapes add diagnostics. ArgumentOutOfRangeException while consuming a team record is swallowed.

Trigs records must contain exactly 18 comma-separated tokens. The parser maps persistence, house, event-control, two events, and two actions. It repairs legacy union payloads: selected event/action data is masked to the low byte; Text Trigger action data is clamped to 1..209. Other token counts produce a diagnostic.

MapPack is LCW-decompressed as three bytes per cell. The stream is plane-oriented: a UInt16 template ID for every cell, then one icon byte for every cell. Template IDs are resolved against the current theatre; unsupported theatre types are discarded. Non-Clear icons outside NumIcons are discarded. OverlayPack is a separate LCW-compressed one-byte-per-cell overlay ID stream; the save path writes signed -1 for empty cells. Terrain, Smudge, Units, Aircraft, Ships, Infantry, Structures, Base, Waypoints, CellTriggers, and Briefing are reconstructed from their named sections. Placement collisions and unknown type references are reported.

After recognized sections are extracted, the remaining INI sections are assigned to extraSections. Map.EndUpdate is then called. The update is not protected by a finally block, so an exception during parsing can bypass EndUpdate; malformed numeric conversions can throw instead of becoming returned diagnostics.

## Save and serialized output

Save first calls Validate; failed validation returns false. INI/BIN writes an .mpr text file plus .tga preview and .json metadata sidecars. MEG/PGM builds a megafile containing uppercase .MPR, .TGA, and .JSON entries. Unsupported values throw NotSupportedException.

SaveINI re-adds extraSections, then serializes Basic and Map. Steam is emitted unless the type is PGM. It writes SMUDGE, TERRAIN, CellTriggers, TeamTypes, INFANTRY, STRUCTURES, Base, UNITS, AIRCRAFT, SHIPS, Trigs, Waypoints, enabled house sections, and Briefing. Basic.NewINIFormat is set to 3.

TeamTypes writes house ID, packed flag bits, recruit priority, counts, origin minus one, trigger, class count and TYPE:COUNT pairs, then mission count and index:argument pairs. Infantry groups are flattened into one numbered record per non-null infantry slot. Structures include only prebuilt buildings. Units, aircraft, and ships use distinct sections. Base contains Player, Count, and numbered structure/cell pairs ordered by BasePriority. Trigs writes 18 fields matching the parser's positional contract. Waypoints are emitted only when Cell has a value. Briefing newlines are replaced by @.

MapPack writes a UInt16 template-ID plane followed by an icon-byte plane; empty cells use UInt16.MaxValue and byte.MaxValue. OverlayPack writes one signed byte per cell, with -1 for empty cells.

The LCW compressor splits input into chunks of at most 8,192 bytes. Each chunk is encoded as little-endian UInt16 compressed length, UInt16 decompressed length, then compressed bytes. The resulting stream is Base64-encoded and split into 70-character INI values with one-based numeric keys. The decompressor concatenates values, Base64-decodes them, reads chunk headers, and calls WWCompression.LcwDecompress into a buffer sized from map dimensions and bytesPerCell.

## Validation and lifetime

Validate checks maximum counts for aircraft, prebuilt buildings, infantry members, terrain objects, units, team types, and triggers. Non-solo maps require at least two placed waypoints; solo maps require the Home waypoint. Failure displays a Validation Error message and returns false.

Dispose(bool) disposes MapImage when disposing is true and only once; Dispose delegates to it.

## Rust mapping and evidence

The current crates/map-editor-model contains generic CellLayers, MapDocument, dimensions, and waypoints. It does not implement the RA plugin, INI schema, team/trigger codecs, LCW MapPack/OverlayPack codec, UGC metadata, previews/JSON sidecars, or RA object placement. No Rust parity or fixture test is claimed.

## Remaining audit

- [ ] Complete every remaining field, nested type, and method body in this 79 KB file.
- [ ] Inspect decompressor bounds/error behavior and LCW edge cases.
- [ ] Trace every Load/Save caller and exception/dirty-state transition.
- [ ] Build golden RA INI/PGM fixtures and compare exact bytes/section order.
- [ ] Test unknown-section preservation and malformed/overlapping object records.
- [ ] Map each serialized section to a typed Rust codec and test it independently.
