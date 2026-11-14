# TiberianDawn/GamePlugin.cs — Tiberian Dawn editor plugin and scenario serializer

> **Audit status: partially reviewed.** This tranche inspected load/save dispatch, INI section inventory, binary map codec, serializer structure, validation entry point, and unknown-section preservation. The 61 KB source still requires a complete member/body/call-site audit and fixture-based round-trip testing.

- Upstream: https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/CnCTDRAMapEditor/TiberianDawn/GamePlugin.cs
- Pinned source blob: ae24699eb1e213ad19ba09f55e50f42aa1a809dc
- Source size: 61,343 characters
- Type: MobiusEditor.TiberianDawn.GamePlugin, internal IGamePlugin implementation

## Load contract

Load(string path, FileType fileType) treats INI and BIN as a paired standalone format: it opens the .ini text file and .bin binary map file, parses the INI, applies LoadINI, then applies LoadBinary. MEG and PGM open a Megafile and locate both .ini and .bin entries; when the required entries are present, both are parsed and applied. Unsupported values throw NotSupportedException.

LoadINI calls Map.BeginUpdate, then consumes Basic, Map, Briefing, Steam, TeamTypes, Triggers, Terrain, Overlay, Smudge, Infantry, Units, Aircraft, Structures, Base, Waypoints, and CellTriggers. It applies Basic/Map/Briefing/Steam models through the shared INI reflection helpers, then decodes game-specific records and placed objects. After known sections are extracted, the remaining sections are retained in extraSections for a subsequent save. Map.EndUpdate is called after this work; it is not protected by a finally block in the inspected method.

## Binary map codec

LoadBinary clears Map.Templates and reads row-major. For every y and x cell it reads exactly two bytes: template type ID, then icon index. The type is resolved against the editor's template catalog and current theatre. A non-Clear template with an icon outside NumIcons is discarded. SaveBinary writes the same interleaved two-byte record per cell; an empty cell is represented by byte.MaxValue for both bytes.

This differs materially from Red Alert's MapPack section: TD uses a separate binary sidecar/megafile entry and a compact interleaved pair, while RA uses Base64-wrapped LCW chunks in MapPack and OverlayPack.

## Save contract

Save validates first and returns false when validation fails. INI/BIN output writes .ini, .bin, .tga preview, and .json metadata. MEG/PGM output packages uppercase .INI, .BIN, .TGA, and .JSON entries in a Megafile.

SaveINI re-adds extraSections, writes Basic and Map, and writes Steam unless the type is PGM. It emits Briefing, CellTriggers, TeamTypes, Triggers, Waypoints, Base, Infantry, Structures, Units, Aircraft, enabled house sections, Overlay, Smudge, and Terrain.

TD TeamTypes is not the RA packed-bitfield schema. Its comma-token sequence contains house name; separate roundabout, learning, suicide, autocreate, and mercenary flags; recruit priority; maximum and initial counts; fear; class count and TYPE:COUNT pairs; mission count and mission/argument pairs; then reinforcable and prebuilt flags.

The TD Triggers section uses a six-field record in the inspected SaveINI path: Event1 type, Action1 type, Event1 data, house, Action1 team, and persistence type. It is not the 18-token RA Trigs structure. Base priorities are emitted in descending priority order with reverse numbering, and the location payload packs y and x into bit fields. Infantry groups are flattened into numbered per-slot records; prebuilt structures, units, aircraft, overlays, smudges, and terrain use their own sections.

## Relevant method surface observed

- New(string theater)
- Load(string path, FileType fileType)
- LoadINI(INI ini)
- LoadBinary(BinaryReader reader)
- Save(string path, FileType fileType)
- SaveINI(INI ini, FileType fileType)
- SaveBinary(BinaryWriter writer)
- SaveMapPreview(Stream stream)
- SaveJSON(JsonTextWriter writer)
- Validate()
- BasicSection_PropertyChanged(object sender, PropertyChangedEventArgs e)
- MapSection_PropertyChanged(object sender, PropertyChangedEventArgs e)
- UpdateBasePlayerHouse()
- Dispose(bool disposing)
- Dispose()

This is the audited method surface for the tranche, not a complete declaration inventory.

## Rust mapping and evidence

The current map-editor-model crate covers only a generic document model, cell layers, dimensions, waypoints, cell metrics, and object-selection plans. It does not implement the TD plugin lifecycle, paired INI/BIN codec, object/trigger/team serializer, preview/JSON output, or unknown-section round trip. No parity test is claimed.

## Remaining audit

- [ ] Inspect all remaining TD plugin members and method bodies.
- [ ] Capture exact TD per-section parser field layouts and error paths.
- [ ] Test paired .ini/.bin and MEG/PGM fixtures, including missing-sidecar behavior.
- [ ] Compare output byte-for-byte with the pinned C# implementation.
- [ ] Trace plugin ownership and dirty-state behavior through MainForm.
