# Original source to Rust port cross-reference

This page maps the first audited original files to the current Rust workspace. The mapping is a navigation aid, not proof of complete parity. Consult [PORTING_STATUS.md](../../PORTING_STATUS.md) and [CPP_PORT_INVENTORY.md](../CPP_PORT_INVENTORY.md) for the authoritative implementation status.

| Original file | Original responsibility | Current Rust location / status |
|---|---|---|
| `TIBERIANDAWN/MAP.CPP` | Map grid lifecycle, map binary read/write, placement/overlap, visibility, validation, map logic. | `crates/map-editor-model` only contains an initial editor model; runtime map behavior is not ported. |
| `TIBERIANDAWN/MAP.H` | Declares map class and its public state/operations. | No complete runtime `MapClass` equivalent yet. |
| `TIBERIANDAWN/TEMPLATE.CPP` | Runtime template objects placed on map cells and serialized through scenario INI. | Not fully ported. |
| `TIBERIANDAWN/CDATA.CPP` | Static template type definitions and theatre-specific graphics initialization. | No complete theatre/template registry yet. |
| `TIBERIANDAWN/INI.CPP` | Scenario INI loading and scenario object placement. | Scenario parsing/assembly remains incomplete. |
| `TIBERIANDAWN/MIXFILE.CPP/H` | Legacy MIX archive caching, lookup, retrieval, and offsets. | `crates/rts-formats/src/mix.rs` (partial; see port status). |
| `REDALERT/MAP.CPP` | Red Alert map runtime, map binary read/write, zones, bridge operations, placement and visibility. | No complete Red Alert runtime map implementation yet. |
| `REDALERT/CDATA.CPP` | Red Alert static type tables, template definitions, theatre and template initialization. | No complete theatre/template registry yet. |
| `TIBERIANDAWN/WIN32LIB/ICONSET.CPP` | Reads IFF icon sets and exposes image/map/transparency data. | `crates/rts-formats/src/iconset.rs` (initial implementation; variant layout audit required). |
| `TIBERIANDAWN/WIN32LIB/TILE.H` | Tiberian Dawn icon-set control structure and loader declarations. | `crates/rts-formats/src/iconset.rs`; do not serialize a native C struct as a universal cross-variant header. |
| `REDALERT/WIN32LIB/TILE.H` | Red Alert icon-set control structure, including map width/height fields. | `crates/rts-formats/src/iconset.rs`; Red Alert metadata needs explicit support. |
| `CnCTDRAMapEditor/Model/` | Editor-side map representation and related models. | `crates/map-editor-model/` is only an initial subset. |

## Required architecture direction

Keep these as separate responsibilities in Rust:

- `rts-formats`: byte-level parsing of MIX, IFF, icon sets, maps, palettes, and related legacy formats.
- Theatre/template catalog: theatre identity, suffixes, supported template types, template dimensions, and icon-set associations.
- Map document: decoded map cells and object/layer references, with TD and RA decoders kept distinct.
- Terrain resolver: converts a map's template ID + icon index into the actual tile image and occupancy behavior.
- Renderer: draws already-resolved terrain and layers; it should not guess template identity from atlas coordinates.
