# Tranche 6S-C-A — TemplateTypes partial catalogs and model/tool/serialization

**Scope:** TD/RA TemplateTypes registry bodies are partially reviewed for IDs0–31, plus static construction/GetTypes. Full catalogs (TD216, RA401) remain incomplete. TemplateType model and TemplateTool were previously Body reviewed; this batch traces their contracts and template serialization call sites without claiming full GamePlugin audits.

- [TD IDs0–31](TiberianDawn/TemplateTypes-6S-C-A.md) · [RA IDs0–31](RedAlert/TemplateTypes-6S-C-A.md) · [TemplateType model](TemplateType.cs.md) · [TemplateTool](Tools/TemplateTool.cs.md)

## Catalog differences

| Variant | Total | Slice IDs | Clear | Water flags | Shore IDs in slice |

|---|---:|---|---|---|---|

| TD | 216 (0–215) | 0–31 | clear1, ID0, Clear | Water/Water2: None | Shore1–5,11–15: 3–12 |

| RA | 401 (0–400) | 0–31 | clear1, ID0, Clear | Water/Water2: Water | Shore01–29: 3–31 |

TD Water ID1 is 1×1 and Water2 ID2 2×2, neither Water-flagged. RA marks both Water. TD shores use legacy gaps (1–5 and 11–15) in this range; RA Shore01–29 are sequential. Exact fields/dimensions/theatres/flags are in batch tables.

## Model flags and Init

`TemplateTypeFlag`: None=0, Clear=2, Water=4, OreMine=8 (bit0 unused). TemplateType stores ushort ID, Name, IconWidth/Height, TheaterType[] Theaters and Flag; IconSize/NumIcons computed; IconMask is bool[IconWidth,IconHeight], Thumbnail mutable Image.

Init uses square cell size OriginalTileWidth/4, a canvas with max(IconWidth,IconHeight) cells, clears the mask, loops y then x with row-major icon index, and looks up Name/icon across theater.Tilesets. Found tiles draw and set mask[x,y]=true. Missing icons are transparent/false; if any found, thumbnail is partial, else null. It assigns IconMask either way and does not filter by Theaters itself. The unused/no-result bitmap and previous Thumbnail are not explicitly disposed (possible leak, not runtime-tested).

## TemplateTool

Catalog filter: Thumbnail!=null, Theaters.Contains(active theatre), and Clear flag absent. Category is leading lowercase run (`^([a-z]*)`); groups sorted by key, within-group order follows source enumerable. ImageList uses max dimensions and Depth24Bit. Max()/First() have no empty guard; Theaters dereferenced without null guard.

Preview left-click selects only IconMask-true cells; right-click clears icon selection. Normal map left-click picks whole type, right-click individual icon; empty cell picks clear1. Shift placement; Ctrl map-bounds editing.

Single-icon SetTemplate writes index y*IconWidth+x into one cell. Whole-template placement records undo for the full in-bounds footprint, then writes only mask-true cells (holes untouched). Whole removal records footprint and clears all in-bounds cells including mask holes; this asymmetry is source behavior. Per-cell undo/redo dictionaries are committed on mouse-up.

## Serialization call sites traced (plugins not full-body audited)

| Variant | Load shape | Type ID | Icon | Null sentinel |

|---|---|---|---|---|

| TD binary | 2 bytes per cell, y-major/x-minor | byte | byte | 255/255 |

| RA MapPack | LCW section type 3; first all types, then all icons | ushort | byte | ushort 65535, byte 255 |

TD resolves ID, theatre-filter, and range-checks icons only for non-Clear; stores type+icon or null. RA resolves ushort types and theatre-filters first; second pass nulls non-Clear icons >= NumIcons, else assigns icon. Clear bypasses icon range validation. RA save writes ushort IDs then icon bytes and LCW-compresses MapPack; TD save writes byte type/icon pairs. These are call-site traces, not full GamePlugin body review.

## Rust parity gap

Rust map-editor-model stores numeric CellLayers.template only; no typed catalog, masks, theatre filters, flags, thumbnails or tool. Catalog slice source documented; not Rust-mapped/Tested. No Rust source/build/tests changed. Next batch IDs32–63.