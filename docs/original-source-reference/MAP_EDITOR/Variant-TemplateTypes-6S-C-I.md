# Tranche 6S-C-I — TD/RA TemplateTypes comparison

Pinned upstream: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.

Tiberian Dawn has no declarations after ID215 (216 total). This tranche advances only Red Alert, documenting IDs256–287. Cumulative declaration enumeration is TD216/216 and RA288/401.

## Red Alert IDs256–287

| ID range | Fields | Dimensions | Theatre |
|---|---|---|---|
| 256–267 | ARRO0004–ARRO0015 | 1×1 each | Interior only |
| 268–274 | FLOR0001–FLOR0007 | 1×1 each | Interior only |
| 275–279 | GFLR0001–GFLR0005 | 1×1 each | Interior only |
| 280–287 | GSTR0001–GSTR0008 | 1×1 each | Interior only |

All 32 declarations use Flag=None through the five-argument constructor. Their explicit theatre whitelist is `TheaterTypes.Interior`; no Temperate/Snow compatibility should be inferred from adjacent registry entries.

## Cross-variant interpretation

- TD declaration enumeration is complete at 216/216, but its inventory row remains Partially reviewed pending dependencies, consumers, initialization and GamePlugin serialization.
- RA declaration enumeration is 288/401; 113 declarations remain (IDs288–400).
- Both registry rows remain Partially reviewed. Catalogue declaration coverage is not a full source body audit.
- The Rust cell template field `Option<u16>` is an untyped ID slot; no theatre-aware typed registry mapping or parity test is claimed.

## Verification

- RA source blob: `027bb5658e2297cf2ade84f9a01567acc8238db8`; source pinned at `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.
- This slice: exactly 32 contiguous declarations IDs256–287 at lines280–311.
- TD source declaration count remains 216 (IDs0–215).
- Rust-mapped=0, Tested=0, Complete=0 for these sources.
