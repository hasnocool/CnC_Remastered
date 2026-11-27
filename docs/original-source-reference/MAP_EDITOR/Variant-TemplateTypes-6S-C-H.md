# Tranche 6S-C-H — TD/RA TemplateTypes comparison

Pinned upstream: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.

Tiberian Dawn has no declarations after ID215 (216 total). This tranche advances only Red Alert, from ID224 through ID255. Cumulative declaration coverage: TD216/216 and RA256/401.

## Red Alert IDs224–255

| ID range | Fields | Dimensions | Theatre |
|---|---|---|---|
| 224–226 | Rough09–Rough11 | 1×2, 2×1, 2×1 | Temperate/Snow |
| 227–228 | Road44–Road45 | 1×1 each | Temperate/Snow |
| 229–230 | River14–River15 | 1×2, 2×1 | Temperate/Snow |
| 231–234 | RiverCliff01–04 | 2×2 each | Temperate/Snow |
| 235–237 | Bridge1a–1c | 4×3 each | Temperate/Snow |
| 238–240 | Bridge2a–2c | 5×3 each | Temperate/Snow |
| 241–246 | Bridge3a–3f | 4×2 each | Temperate/Snow |
| 247–252 | F01–F06 | 3×3 each | Temperate/Snow |
| 253–255 | ARRO0001–0003 | 1×1 each | Interior only |

All entries use the five-argument constructor and therefore Flag=None. Theatre whitelists are explicit source data; do not broaden the Interior-only entries or assume all keys resolve in every listed tileset.

## Cross-variant interpretation

- TD declaration enumeration is complete, but its registry source remains Partially reviewed pending dependencies, consumers and plugin serialization.
- RA has 145 declarations remaining (IDs256–400).
- The Rust editor model stores only `Option<u16>` for a cell template reference; no theatre-aware typed registry is mapped.
- Bridge footprints are metadata, not proof of collision or connection behavior. Those semantics require tracing placement, renderer and serialization consumers.

## Verification

- RA source blob: `027bb5658e2297cf2ade84f9a01567acc8238db8`; complete source contains 401 declarations.
- This slice: exactly 32 contiguous IDs224–255 at lines248–279.
- TD source blob: `97fdfd2109d241da8b51139d974028ed1198b140`; declarations IDs0–215.
- Both inventory rows remain Partially reviewed. Rust-mapped=0, Tested=0, Complete=0. No Rust source changed and no tests were executed.
