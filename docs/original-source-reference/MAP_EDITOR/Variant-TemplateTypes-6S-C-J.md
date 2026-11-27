# Tranche 6S-C-J — TD/RA TemplateTypes comparison

Pinned upstream: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.

## Declaration coverage

Tiberian Dawn ends at ID215; it has no declarations in this range. Red Alert IDs288–319 contain 32 declarations, all 1×1, Interior-only and Flag=None.

| RA IDs | Fields | Tile keys | Footprint | Theatre |
|---|---|---|---:|---|
| 288–290 | GSTR0009–GSTR0011 | gstr0009–gstr0011 | 1×1 each | Interior |
| 291–317 | LWAL0001–LWAL0027 | lwal0001–lwal0027 | 1×1 each | Interior |
| 318–319 | STRP0001–STRP0002 | strp0001–strp0002 | 1×1 each | Interior |

Every declaration uses the five-argument constructor, which forwards with `TemplateTypeFlag.None`.

## Cumulative declaration coverage

- TD: 216/216 declarations (IDs0–215).
- RA: 320/401 declarations (IDs0–319); 81 remain (IDs320–400).
- Both `TemplateTypes.cs` inventory rows remain Partially reviewed. Declaration enumeration is not a full source body, dependency, consumer, initializer or serializer audit.

## Variant serialization and Rust mapping

- TD editor BIN cells are interleaved `u8 type, u8 icon` pairs; null cells are `0xFF,0xFF`.
- RA MapPack is plane-oriented: all little-endian `u16 type IDs`, then all `u8 icon IDs`; null cells use `0xFFFF,0xFF`.
- Rust has distinct TD and RA raw decoders (`rts-formats::td_map`, `rts-formats::ra_map_pack`) and a partial source-derived asset catalog/resolver (`rts-core::template_catalog`, `renderer::template_assets`). These are not full GamePlugin serializers or a C#-equivalent editor type catalog.
- Rust metadata entries for RA IDs288–319 match all 32 C# keys and Interior theatre masks. The Rust RA catalog has IDs0–399 only; C# editor ID400 AntHill is absent and requires build-macro reconciliation.
- Both Rust atlas builders currently pass raw empty-cell sentinels (TD255, RA65535) into catalog lookup; the provider then reports UnknownTemplateType. This static call-path issue requires sentinel normalization and regression tests.

## Verification

- Exact upstream RA source blob: `027bb5658e2297cf2ade84f9a01567acc8238db8`; declarations IDs288–319 are lines312–343.
- Rust catalog cross-check: IDs288–319, matching stems and mask=4 for all 32; no mismatches.
- No Rust code changed; no Rust tests, C# runtime tests, or retail asset checks were executed.
- Next tranche: 6S-C-K, Red Alert IDs320–351.
