# Upstream C++ translation-unit port inventory

Pinned source: [electronicarts/CnC_Remastered_Collection](https://github.com/electronicarts/CnC_Remastered_Collection)
Upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`

- Total C++ translation units: **494**
- Fully ported: **91**
- Not yet fully ported: **403**
- A translation unit is complete only when its active behavior has a Rust implementation. Intentionally empty source files are identified explicitly; placeholders do not count.

## Coverage by directory

| Upstream directory | C++ files | Complete | Remaining |
|---|---:|---:|---:|
| `REDALERT/` | 275 | 73 | 202 |
| `REDALERT/WIN32LIB/` | 26 | 24 | 2 |
| `TIBERIANDAWN/` | 161 | 14 | 147 |
| `TIBERIANDAWN/WIN32LIB/` | 32 | 28 | 4 |

## File-by-file coverage

### `REDALERT/`

| Status | Upstream C++ file | Rust port / notes |
|---|---|---|
| Not started | `REDALERT/2KEYFRAM.CPP` | — |
| Not started | `REDALERT/AADATA.CPP` | — |
| Not started | `REDALERT/ABSTRACT.CPP` | — |
| Not started | `REDALERT/ADATA.CPP` | — |
| Complete | `REDALERT/ADPCM.CPP` | rts-formats/src/adpcm.rs — SOS ADPCM type-1 decoder. |
| Not started | `REDALERT/AIRCRAFT.CPP` | — |
| Not started | `REDALERT/ANIM.CPP` | — |
| Not started | `REDALERT/AUDIO.CPP` | — |
| Complete | `REDALERT/B64PIPE.CPP` | rts-formats/src/base64_streams.rs — Base64 push adapter. |
| Complete | `REDALERT/B64STRAW.CPP` | rts-formats/src/base64_streams.rs — Base64 pull adapter. |
| Not started | `REDALERT/BAR.CPP` | — |
| Not started | `REDALERT/BASE.CPP` | — |
| Complete | `REDALERT/BASE64.CPP` | rts-formats/src/base64.rs — Base64 packet encoder/decoder. |
| Not started | `REDALERT/BBDATA.CPP` | — |
| Not started | `REDALERT/BDATA.CPP` | — |
| Not started | `REDALERT/BENCH.CPP` | — |
| Not started | `REDALERT/BFIOFILE.CPP` | — |
| Complete | `REDALERT/BIGCHECK.CPP` | No active implementation in the pinned RedAlert.vcxproj configuration; the file is entirely guarded by WOLAPI_INTEGRATION, which is not defined by the project. |
| Complete | `REDALERT/BLOWFISH.CPP` | rts-formats/src/blowfish.rs — Blowfish key schedule, P/S tables, encrypt/decrypt. |
| Complete | `REDALERT/BLOWPIPE.CPP` | rts-formats/src/blowfish_streams.rs — Blowfish push adapter. |
| Complete | `REDALERT/BLWSTRAW.CPP` | rts-formats/src/blowfish_streams.rs — Blowfish demand-driven adapter. |
| Complete | `REDALERT/BMP8.CPP` | rts-formats/src/bmp8.rs — 8-bit BMP headers, 256-entry palette conversion, pixel payload, RAII resource ownership and COLORONCOLOR stretch-draw plan; stale GDI member names are normalized. |
| Not started | `REDALERT/BUFF.CPP` | — |
| Not started | `REDALERT/BUILDING.CPP` | — |
| Not started | `REDALERT/BULLET.CPP` | — |
| Not started | `REDALERT/CARGO.CPP` | — |
| Not started | `REDALERT/CARRY.CPP` | — |
| Not started | `REDALERT/CCDDE.CPP` | — |
| Not started | `REDALERT/CCFILE.CPP` | — |
| Not started | `REDALERT/CCINI.CPP` | — |
| Not started | `REDALERT/CCMPATH.CPP` | — |
| Complete | `REDALERT/CCPTR.CPP` | rts-core/src/ccptr.rs — stable heap-ID pointer resolution, case-insensitive Name comparison, 31 explicit template instantiations and exact relational-operator expressions. |
| Not started | `REDALERT/CCTEN.CPP` | — |
| Not started | `REDALERT/CDATA.CPP` | — |
| Not started | `REDALERT/CDFILE.CPP` | — |
| Not started | `REDALERT/CELL.CPP` | — |
| Complete | `REDALERT/CHECKBOX.CPP` | rts-core/src/checkbox.rs — ToggleClass draw gate, depressed frame/fill, green checked fill, and LEFTRELEASE toggle before base action. |
| Not started | `REDALERT/CHEKLIST.CPP` | — |
| Complete | `REDALERT/CLASS.CPP` | Intentionally empty translation unit; no executable definitions. |
| Not started | `REDALERT/COLRLIST.CPP` | — |
| Not started | `REDALERT/COMBAT.CPP` | — |
| Not started | `REDALERT/COMBUF.CPP` | — |
| Complete | `REDALERT/COMINIT.CPP` | rts-core/src/com_init.rs — RAII OLE lifecycle adapter; host supplies platform calls. |
| Not started | `REDALERT/COMQUEUE.CPP` | — |
| Not started | `REDALERT/CONFDLG.CPP` | — |
| Not started | `REDALERT/CONNECT.CPP` | — |
| Not started | `REDALERT/CONQUER.CPP` | — |
| Not started | `REDALERT/CONST.CPP` | — |
| Not started | `REDALERT/CONTROL.CPP` | — |
| Not started | `REDALERT/COORD.CPP` | — |
| Not started | `REDALERT/CRATE.CPP` | — |
| Complete | `REDALERT/CRC.CPP` | rts-formats/src/legacy_crc.rs — rotate-add checksum engine. |
| Complete | `REDALERT/CRCPIPE.CPP` | rts-formats/src/crc_streams.rs — checksum-aware push adapter. |
| Complete | `REDALERT/CRCSTRAW.CPP` | rts-formats/src/crc_streams.rs — checksum-aware pull adapter. |
| Not started | `REDALERT/CREDITS.CPP` | — |
| Complete | `REDALERT/CREW.CPP` | Intentionally empty translation unit; contains only the function.h include and no executable definitions. |
| Complete | `REDALERT/CSTRAW.CPP` | rts-formats/src/cache_straw.rs — buffered read-ahead adapter |
| Not started | `REDALERT/DDE.CPP` | — |
| Not started | `REDALERT/DEBUG.CPP` | — |
| Complete | `REDALERT/DESCDLG.CPP` | rts-core/src/description_dialog.rs — dialog geometry, gadget setup, redraw/callback loop, OK/cancel handling and text trimming. |
| Not started | `REDALERT/DIAL8.CPP` | — |
| Not started | `REDALERT/DIALOG.CPP` | — |
| Not started | `REDALERT/DIBFILE.CPP` | — |
| Not started | `REDALERT/DIBUTIL.CPP` | — |
| Not started | `REDALERT/DISPLAY.CPP` | — |
| Not started | `REDALERT/DLLInterface.cpp` | — |
| Not started | `REDALERT/DLLInterfaceEditor.cpp` | — |
| Not started | `REDALERT/DOOR.CPP` | — |
| Complete | `REDALERT/DPMI.CPP` | No active implementation in the pinned source; the complete translation unit is enclosed by #if (0). |
| Not started | `REDALERT/DRIVE.CPP` | — |
| Not started | `REDALERT/DROP.CPP` | — |
| Complete | `REDALERT/DTABLE.CPP` | rts-formats/src/adpcm.rs — exact 1,424-entry ADPCM difference table. |
| Not started | `REDALERT/DYNAVEC.CPP` | — |
| Not started | `REDALERT/EDIT.CPP` | — |
| Not started | `REDALERT/EGOS.CPP` | — |
| Complete | `REDALERT/ENDING.CPP` | No active implementation in the pinned source; both GDI_Ending and Nod_Ending bodies are enclosed by #ifdef NEVER. |
| Not started | `REDALERT/EVENT.CPP` | — |
| Not started | `REDALERT/EXPAND.CPP` | — |
| Not started | `REDALERT/FACE.CPP` | — |
| Complete | `REDALERT/FACING.CPP` | rts-core/src/facing.rs — DirType and FacingClass current/desired headings, signed shortest-path rotation, capped rotation rate, sprite rotation zones. |
| Not started | `REDALERT/FACTORY.CPP` | — |
| Complete | `REDALERT/FIELD.CPP` | rts-core/src/field.rs — all scalar/string/chunk constructors, four-byte IDs, eight-byte header, and Host_To_Net/Net_To_Host payload conversion. |
| Not started | `REDALERT/FINDPATH.CPP` | — |
| Complete | `REDALERT/FIXED.CPP` | rts-core/src/fixed16.rs — Q16.16 constructors, arithmetic helpers, ASCII conversion. |
| Complete | `REDALERT/FLASHER.CPP` | rts-core/src/flasher.rs — global countdown, IsBlushing parity, per-player countdowns, flashing bitmask and optional debug text. |
| Complete | `REDALERT/FLY.CPP` | rts-core/src/fly.rs — speed scaling via fixed(speed,256), pixel-quantized movement accumulation, Coord_Move, and edge rollback. |
| Not started | `REDALERT/FOOT.CPP` | — |
| Not started | `REDALERT/FUSE.CPP` | — |
| Not started | `REDALERT/GADGET.CPP` | — |
| Not started | `REDALERT/GAMEDLG.CPP` | — |
| Not started | `REDALERT/GAUGE.CPP` | — |
| Complete | `REDALERT/GETCPU.CPP` | rts-core/src/cpu_info.rs — MMX/CPU-type/vendor-ID wrapper with assembly probe injected through CpuProbe and source-compatible strncpy output behavior. |
| Not started | `REDALERT/GLOBALS.CPP` | — |
| Not started | `REDALERT/GOPTIONS.CPP` | — |
| Not started | `REDALERT/GSCREEN.CPP` | — |
| Not started | `REDALERT/HDATA.CPP` | — |
| Not started | `REDALERT/HEAP.CPP` | — |
| Not started | `REDALERT/HELP.CPP` | — |
| Not started | `REDALERT/HOUSE.CPP` | — |
| Not started | `REDALERT/HSV.CPP` | — |
| Not started | `REDALERT/ICONLIST.CPP` | — |
| Not started | `REDALERT/IDATA.CPP` | — |
| Not started | `REDALERT/INFANTRY.CPP` | — |
| Not started | `REDALERT/INI.CPP` | — |
| Complete | `REDALERT/INIBIN.CPP` | rts-core/src/inibin.rs — intentionally empty upstream translation unit. |
| Complete | `REDALERT/INICODE.CPP` | No active implementation in the pinned source; the entire scenario digest and INI-to-INB implementation is enclosed by #ifdef TOFIX. |
| Not started | `REDALERT/INIT.CPP` | — |
| Complete | `REDALERT/INT.CPP` | rts-core/src/bignum_globals.rs — zero-initialized Error, Carry, Borrow, and precision-specific remainder state. |
| Not started | `REDALERT/INTERNET.CPP` | — |
| Not started | `REDALERT/INTERPAL.CPP` | — |
| Not started | `REDALERT/INTRO.CPP` | — |
| Not started | `REDALERT/IOMAP.CPP` | — |
| Not started | `REDALERT/IOOBJ.CPP` | — |
| Not started | `REDALERT/IPX.CPP` | — |
| Complete | `REDALERT/IPX95.CPP` | rts-core/src/ipx95.rs — disabled THIPX32 loader, fixed legacy socket/connection return values, and all exported IPX95 stubs. |
| Not started | `REDALERT/IPXADDR.CPP` | — |
| Not started | `REDALERT/IPXCONN.CPP` | — |
| Not started | `REDALERT/IPXGCONN.CPP` | — |
| Not started | `REDALERT/IPXMGR.CPP` | — |
| Complete | `REDALERT/ITABLE.CPP` | rts-formats/src/adpcm.rs — exact 1,424-entry ADPCM index table. |
| Not started | `REDALERT/JSHELL.CPP` | — |
| Not started | `REDALERT/KEY.CPP` | — |
| Complete | `REDALERT/KEYBOARD.CPP` | rts-core/src/keyboard.rs — 256-slot ring buffer, 255-entry capacity, modifier/release capture, ASCII translation, mouse coordinate triples, double-click expansion, async key state and message pumping. |
| Not started | `REDALERT/LAYER.CPP` | — |
| Complete | `REDALERT/LCW.CPP` | rts-formats/src/lcw.rs — LCW block decompression. |
| Complete | `REDALERT/LCWPIPE.CPP` | rts-formats/src/lcw_blocks.rs — framed push compression/decompression and partial-block flush. |
| Complete | `REDALERT/LCWSTRAW.CPP` | rts-formats/src/lcw_blocks.rs — framed pull compression/decompression and buffered reads. |
| Complete | `REDALERT/LCWUNCMP.CPP` | rts-formats/src/lcw.rs — second LCW decompression entry point. |
| Complete | `REDALERT/LINK.CPP` | rts-core/src/link.rs — arena-backed LinkClass insertion, traversal, head/tail, unlink, and list-position assignment. |
| Not started | `REDALERT/LIST.CPP` | — |
| Not started | `REDALERT/LOADDLG.CPP` | — |
| Not started | `REDALERT/LOGIC.CPP` | — |
| Complete | `REDALERT/LZO1X_C.CPP` | rts-formats/src/lzo1x.rs — deterministic LZO1X-1 compressor. |
| Complete | `REDALERT/LZO1X_D.CPP` | rts-formats/src/lzo1x.rs — bounds-checked LZO1X decoder. |
| Complete | `REDALERT/LZOPIPE.CPP` | rts-formats/src/lzo_blocks.rs — framed push block stream. |
| Complete | `REDALERT/LZOSTRAW.CPP` | rts-formats/src/lzo_blocks.rs — framed pull block stream. |
| Complete | `REDALERT/LZW.CPP` | rts-formats/src/lzw.rs — 12-bit dictionary engine and 16-bit little-endian codewords. |
| Not started | `REDALERT/LZWOTRAW.CPP` | — |
| Complete | `REDALERT/LZWPIPE.CPP` | rts-formats/src/lzw_blocks.rs — framed push compression/decompression. |
| Complete | `REDALERT/LZWSTRAW.CPP` | rts-formats/src/lzw_blocks.rs — framed pull compression/decompression. |
| Not started | `REDALERT/MAP.CPP` | — |
| Not started | `REDALERT/MAPEDDLG.CPP` | — |
| Not started | `REDALERT/MAPEDIT.CPP` | — |
| Not started | `REDALERT/MAPEDPLC.CPP` | — |
| Not started | `REDALERT/MAPEDSEL.CPP` | — |
| Not started | `REDALERT/MAPEDTM.CPP` | — |
| Not started | `REDALERT/MAPSEL.CPP` | — |
| Not started | `REDALERT/MCI.CPP` | — |
| Not started | `REDALERT/MCIMOVIE.CPP` | — |
| Not started | `REDALERT/MENUS.CPP` | — |
| Not started | `REDALERT/MISSION.CPP` | — |
| Not started | `REDALERT/MIXFILE.CPP` | — |
| Not started | `REDALERT/MONOC.CPP` | — |
| Not started | `REDALERT/MOUSE.CPP` | — |
| Not started | `REDALERT/MP.CPP` | — |
| Not started | `REDALERT/MPGSET.CPP` | — |
| Not started | `REDALERT/MPLAYER.CPP` | — |
| Complete | `REDALERT/MPLIB.CPP` | rts-core/src/mgen.rs — DOS yield interrupt, chunnel interrupt register packing, MGen queue/node/master operations and EAX/ECX result handling; external ordinal/device constants are injected. |
| Complete | `REDALERT/MPMGRD.CPP` | rts-core/src/player_manager.rs — MGen queue IDs, private/global packet routing, connection table, receive counts, response timing, game-player count and queue flush. |
| Complete | `REDALERT/MPMGRW.CPP` | No active implementation in the pinned source; the entire Windows DLL-backed multiplayer manager is enclosed by #if (0)//PG. |
| Complete | `REDALERT/MPU.CPP` | No active implementation for the pinned MSVC Win32 project; Get_CPU_Clock exists only under __BORLANDC__. |
| Not started | `REDALERT/MSGBOX.CPP` | — |
| Not started | `REDALERT/MSGLIST.CPP` | — |
| Not started | `REDALERT/MiscAsm.cpp` | — |
| Not started | `REDALERT/NETDLG.CPP` | — |
| Not started | `REDALERT/NOSEQCON.CPP` | — |
| Not started | `REDALERT/NULLCONN.CPP` | — |
| Not started | `REDALERT/NULLDLG.CPP` | — |
| Not started | `REDALERT/NULLMGR.CPP` | — |
| Not started | `REDALERT/OBJECT.CPP` | — |
| Not started | `REDALERT/ODATA.CPP` | — |
| Not started | `REDALERT/OPTIONS.CPP` | — |
| Not started | `REDALERT/OVERLAY.CPP` | — |
| Not started | `REDALERT/PACKET.CPP` | — |
| Not started | `REDALERT/PALETTEC.CPP` | — |
| Complete | `REDALERT/PIPE.CPP` | rts-formats/src/pipe.rs — push chain forwarding, relink, flush, unlink. |
| Not started | `REDALERT/PK.CPP` | — |
| Not started | `REDALERT/PKPIPE.CPP` | — |
| Not started | `REDALERT/PKSTRAW.CPP` | — |
| Not started | `REDALERT/POWER.CPP` | — |
| Not started | `REDALERT/PROFILE.CPP` | — |
| Not started | `REDALERT/QUEUE.CPP` | — |
| Not started | `REDALERT/RADAR.CPP` | — |
| Not started | `REDALERT/RADIO.CPP` | — |
| Not started | `REDALERT/RAMFILE.CPP` | — |
| Complete | `REDALERT/RAND.CPP` | rts-core/src/random.rs — Sim_Random and Sim_IRandom. |
| Complete | `REDALERT/RANDOM.CPP` | rts-core/src/random_class.rs — seeded LCG and rejection-sampled ranges. |
| Not started | `REDALERT/RAWFILE.CPP` | — |
| Complete | `REDALERT/RAWOLAPI.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/READLINE.CPP` | rts-formats/src/readline.rs — CR/LF line reader and whitespace trimming. |
| Complete | `REDALERT/RECT.CPP` | rts-core/src/rect.rs — validity, intersection, anchor adjustment, union. |
| Not started | `REDALERT/REINF.CPP` | — |
| Not started | `REDALERT/RGB.CPP` | — |
| Complete | `REDALERT/RNDSTRAW.CPP` | rts-formats/src/random_straw.rs — 32 independent LCGs, 1,024-bit seed collection, iterative SHA-1 scramble. |
| Not started | `REDALERT/ROTBMP.CPP` | — |
| Not started | `REDALERT/RULES.CPP` | — |
| Not started | `REDALERT/SAVELOAD.CPP` | — |
| Not started | `REDALERT/SCENARIO.CPP` | — |
| Not started | `REDALERT/SCORE.CPP` | — |
| Not started | `REDALERT/SCROLL.CPP` | — |
| Not started | `REDALERT/SDATA.CPP` | — |
| Not started | `REDALERT/SEDITDLG.CPP` | — |
| Not started | `REDALERT/SENDFILE.CPP` | — |
| Not started | `REDALERT/SEQCONN.CPP` | — |
| Not started | `REDALERT/SESSION.CPP` | — |
| Complete | `REDALERT/SHA.CPP` | rts-formats/src/sha1.rs — streaming SHA-1 engine and non-destructive result snapshots. |
| Not started | `REDALERT/SHAPEBTN.CPP` | — |
| Complete | `REDALERT/SHAPIPE.CPP` | rts-formats/src/sha_streams.rs — non-transforming SHA push adapter |
| Complete | `REDALERT/SHASTRAW.CPP` | rts-formats/src/sha_streams.rs — SHA pull adapter hashing returned bytes |
| Not started | `REDALERT/SIDEBAR.CPP` | — |
| Not started | `REDALERT/SIDEBARGlyphx.CPP` | — |
| Not started | `REDALERT/SLIDER.CPP` | — |
| Not started | `REDALERT/SMUDGE.CPP` | — |
| Not started | `REDALERT/SOUNDDLG.CPP` | — |
| Not started | `REDALERT/SPECIAL.CPP` | — |
| Not started | `REDALERT/SPRITE.CPP` | — |
| Not started | `REDALERT/STARTUP.CPP` | — |
| Not started | `REDALERT/STATBTN.CPP` | — |
| Not started | `REDALERT/STATS.CPP` | — |
| Complete | `REDALERT/STRAW.CPP` | rts-formats/src/straw.rs — demand-driven pull chain and reconnect behavior. |
| Complete | `REDALERT/STUB.CPP` | rts-core/src/legacy_stub.rs — exact launcher message. |
| Not started | `REDALERT/SUPER.CPP` | — |
| Complete | `REDALERT/SURFACE.CPP` | No active implementation in the pinned source; the complete constructor, sub-surface and copy implementation is enclosed by #if (0)//PG. |
| Complete | `REDALERT/Shape.cpp` | rts-core/src/shape_buffer.rs — shape buffer capacity/configuration; caller retains memory ownership. |
| Not started | `REDALERT/TAB.CPP` | — |
| Not started | `REDALERT/TACTION.CPP` | — |
| Not started | `REDALERT/TARGET.CPP` | — |
| Not started | `REDALERT/TCPIP.CPP` | — |
| Not started | `REDALERT/TDATA.CPP` | — |
| Not started | `REDALERT/TEAM.CPP` | — |
| Not started | `REDALERT/TEAMTYPE.CPP` | — |
| Not started | `REDALERT/TECHNO.CPP` | — |
| Not started | `REDALERT/TEMPLATE.CPP` | — |
| Not started | `REDALERT/TENMGR.CPP` | — |
| Not started | `REDALERT/TERRAIN.CPP` | — |
| Not started | `REDALERT/TEVENT.CPP` | — |
| Not started | `REDALERT/TEXTBTN.CPP` | — |
| Not started | `REDALERT/THEME.CPP` | — |
| Not started | `REDALERT/TOGGLE.CPP` | — |
| Not started | `REDALERT/TOOLTIP.CPP` | — |
| Complete | `REDALERT/TRACKER.CPP` | rts-core/src/tracker.rs — legal-target guard, ordered global registry/singleton detachment, dependent-trigger recursion and deletion. |
| Not started | `REDALERT/TRIGGER.CPP` | — |
| Not started | `REDALERT/TRIGTYPE.CPP` | — |
| Not started | `REDALERT/TURRET.CPP` | — |
| Complete | `REDALERT/TXTLABEL.CPP` | rts-core/src/text_label.rs — constructor state, Set_Text, Gadget draw gate, Simple_Text_Print and clipped text dispatch. |
| Not started | `REDALERT/UDATA.CPP` | — |
| Not started | `REDALERT/UDPADDR.CPP` | — |
| Not started | `REDALERT/UNIT.CPP` | — |
| Not started | `REDALERT/UTRACKER.CPP` | — |
| Not started | `REDALERT/VDATA.CPP` | — |
| Complete | `REDALERT/VECTOR.CPP` | rts-core/src/vector.rs and bool_vector.rs — VectorClass, DynamicVectorClass, DynamicVectorArrayClass, and packed BooleanVectorClass. |
| Not started | `REDALERT/VERSION.CPP` | — |
| Not started | `REDALERT/VESSEL.CPP` | — |
| Not started | `REDALERT/VISUDLG.CPP` | — |
| Not started | `REDALERT/VORTEX.CPP` | — |
| Complete | `REDALERT/W95TRACE.CPP` | rts-core/src/w95_trace.rs — active implementation is disabled under #if 0. |
| Not started | `REDALERT/WARHEAD.CPP` | — |
| Not started | `REDALERT/WEAPON.CPP` | — |
| Not started | `REDALERT/WINSTUB.CPP` | — |
| Complete | `REDALERT/WOLAPIOB.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOLEDIT.CPP` | No active implementation in the pinned project; the entire translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOLSTRNG.CPP` | rts-core/src/wol_strings.rs — all 187 English TXT_WOL string constants selected by the pinned ENGLISH project configuration. |
| Complete | `REDALERT/WOL_CGAM.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_CHAT.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_DNLD.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_GSUP.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_LOGN.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_MAIN.CPP` | No active implementation in the pinned project; the translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Complete | `REDALERT/WOL_OPT.CPP` | No active implementation in the pinned project; the entire translation unit is guarded by WOLAPI_INTEGRATION, which is not defined by RedAlert.vcxproj. |
| Not started | `REDALERT/WSPIPX.CPP` | — |
| Not started | `REDALERT/WSPROTO.CPP` | — |
| Not started | `REDALERT/WSPUDP.CPP` | — |
| Complete | `REDALERT/XPIPE.CPP` | rts-formats/src/io_streams.rs — fixed-buffer and file-backed push endpoints |
| Complete | `REDALERT/XSTRAW.CPP` | rts-formats/src/io_streams.rs — fixed-buffer and file-backed pull endpoints |
| Complete | `REDALERT/_WSPROTO.CPP` | rts-core/src/winsock_state.rs — PacketTransport global initialized to null. |

### `REDALERT/WIN32LIB/`

| Status | Upstream C++ file | Rust port / notes |
|---|---|---|
| Complete | `REDALERT/WIN32LIB/ALLOC.CPP` | rts-core/src/legacy_alloc.rs — Windows malloc/free/realloc behavior, allocation flags, counters, DPMI stubs and fixed RAM estimates. |
| Complete | `REDALERT/WIN32LIB/BUFFER.CPP` | rts-core/src/buffer.rs — owned/borrowed storage, size-only allocation, default and destructor semantics. |
| Complete | `REDALERT/WIN32LIB/BUFFGLBL.CPP` | rts-core/src/buffer_globals.rs — callback/viewport null defaults and icon-cache enabled state. |
| Not started | `REDALERT/WIN32LIB/DDRAW.CPP` | — |
| Complete | `REDALERT/WIN32LIB/DELAY.CPP` | rts-core/src/delay.rs — timer threshold and duration loop. |
| Complete | `REDALERT/WIN32LIB/DIPTHONG.CPP` | rts-formats/src/dipthong.rs — dip/undip/fixup transforms and legacy string-block lookup. |
| Complete | `REDALERT/WIN32LIB/DRAWRECT.CPP` | rts-core/src/draw_rect.rs — locked four-edge Draw_Line sequence. |
| Not started | `REDALERT/WIN32LIB/DrawMisc.cpp` | — |
| Complete | `REDALERT/WIN32LIB/FONT.CPP` | rts-core/src/font_metrics.rs — character/string pixel widths and Text_Print cursor coordinate conversion. |
| Complete | `REDALERT/WIN32LIB/GBUFFER.CPP` | rts-core/src/graphic_buffer.rs — system/DirectDraw buffer initialization, viewport attach/clamp/offset math, lock nesting, surface-loss restore, mouse exclusion and blit geometry. |
| Complete | `REDALERT/WIN32LIB/GETSHAPE.CPP` | rts-core/src/shape_access.rs — packed Shape_Type metadata, special prefix words, shape-block indexing, extraction, height mutation and restoration. |
| Complete | `REDALERT/WIN32LIB/ICONSET.CPP` | rts-formats/src/iconset.rs — parses ICON/SINF/SSET/TRNS/MAP IFF chunks, builds packed IControl_Type metadata and exposes bounded icon/map/transparency slices. |
| Complete | `REDALERT/WIN32LIB/IFF.CPP` | rts-formats/src/iff.rs — FORM validation, first-match chunk search, even-byte chunk alignment, bounded reads, chunk writes and FORM-size update. |
| Complete | `REDALERT/WIN32LIB/IRANDOM.CPP` | rts-core/src/legacy_irandom.rs — assembly-compatible Random state and injectable CRT IRandom range sampling. |
| Complete | `REDALERT/WIN32LIB/LOAD.CPP` | rts-formats/src/load.rs — bounded Load_Data/Write_Data/Load_Alloc_Data, legacy compression-header parsing, skip-data extraction, alias-safe tail staging, and NOCOMPRESS/LCW dispatch; HORIZONTAL remains a source-configured no-op. |
| Complete | `REDALERT/WIN32LIB/LOADFONT.CPP` | rts-core/src/load_font.rs — validates the little-endian size prefix, uncompressed marker, five-block font layout, and retains the legacy allocation layout. |
| Complete | `REDALERT/WIN32LIB/MOUSEWW.CPP` | rts-core/src/mouse_ww.rs — live cursor visibility nesting, conditional hide regions, state queries, forced coordinates and buffer-lock coordination; disabled rendering/timer/clip paths remain no-ops. |
| Complete | `REDALERT/WIN32LIB/PALETTE.CPP` | rts-formats/src/palette_fade.rs — byte-identical shared implementation with Tiberian Dawn; IBM palette copy/set, bump-rate calculation, bounded fade steps. |
| Complete | `REDALERT/WIN32LIB/SET_FONT.CPP` | rts-core/src/font.rs — previous font return, little-endian block offsets, width/height metrics. |
| Complete | `REDALERT/WIN32LIB/TIMER.CPP` | rts-core/src/legacy_timer.rs — tick-source selection, accumulated elapsed time, start/stop/set/reset. |
| Complete | `REDALERT/WIN32LIB/TIMERDWN.CPP` | rts-core/src/legacy_timer.rs — countdown timer set/time/start/stop/reset semantics. |
| Complete | `REDALERT/WIN32LIB/TIMERINI.CPP` | rts-core/src/legacy_timer_system.rs — timer-system state, callback tick counters, active flag, frequency and teardown. |
| Complete | `REDALERT/WIN32LIB/WINDOWS.CPP` | rts-formats/src/window_text.rs — text window state, cursor bookkeeping, control codes, dipthong expansion, substitutions, word wrapping, scrolling and draw-operation emission. |
| Complete | `REDALERT/WIN32LIB/WINHIDE.CPP` | rts-core/src/window_mouse.rs — mouse hide/show calls are commented out upstream; observable no-op. |
| Complete | `REDALERT/WIN32LIB/WRITEPCX.CPP` | rts-formats/src/pcx.rs — 128-byte PCX header, source-compatible scanline RLE, 256-colour palette trailer. |
| Complete | `REDALERT/WIN32LIB/_DIPTABL.CPP` | rts-formats/src/dipthong_tables.rs — exact common-character and diphthong lookup tables. |

### `TIBERIANDAWN/`

| Status | Upstream C++ file | Rust port / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/AADATA.CPP` | — |
| Not started | `TIBERIANDAWN/ABSTRACT.CPP` | — |
| Not started | `TIBERIANDAWN/ADATA.CPP` | — |
| Not started | `TIBERIANDAWN/AIRCRAFT.CPP` | — |
| Not started | `TIBERIANDAWN/ALLOC.CPP` | — |
| Not started | `TIBERIANDAWN/ANIM.CPP` | — |
| Not started | `TIBERIANDAWN/AUDIO.CPP` | — |
| Not started | `TIBERIANDAWN/BASE.CPP` | — |
| Not started | `TIBERIANDAWN/BBDATA.CPP` | — |
| Not started | `TIBERIANDAWN/BDATA.CPP` | — |
| Not started | `TIBERIANDAWN/BUILDING.CPP` | — |
| Not started | `TIBERIANDAWN/BULLET.CPP` | — |
| Not started | `TIBERIANDAWN/CARGO.CPP` | — |
| Not started | `TIBERIANDAWN/CCDDE.CPP` | — |
| Not started | `TIBERIANDAWN/CCFILE.CPP` | — |
| Not started | `TIBERIANDAWN/CDATA.CPP` | — |
| Not started | `TIBERIANDAWN/CDFILE.CPP` | — |
| Not started | `TIBERIANDAWN/CELL.CPP` | — |
| Complete | `TIBERIANDAWN/CHECKBOX.CPP` | rts-core/src/checkbox.rs — ToggleClass draw gate, green depressed frame, dark fill, and two diagonal checked-state lines. |
| Not started | `TIBERIANDAWN/CHEKLIST.CPP` | — |
| Not started | `TIBERIANDAWN/COLRLIST.CPP` | — |
| Not started | `TIBERIANDAWN/COMBAT.CPP` | — |
| Not started | `TIBERIANDAWN/COMBUF.CPP` | — |
| Not started | `TIBERIANDAWN/COMQUEUE.CPP` | — |
| Not started | `TIBERIANDAWN/CONFDLG.CPP` | — |
| Not started | `TIBERIANDAWN/CONNECT.CPP` | — |
| Not started | `TIBERIANDAWN/CONQUER.CPP` | — |
| Not started | `TIBERIANDAWN/CONST.CPP` | — |
| Not started | `TIBERIANDAWN/CONTROL.CPP` | — |
| Not started | `TIBERIANDAWN/COORD.CPP` | — |
| Not started | `TIBERIANDAWN/CREDITS.CPP` | — |
| Complete | `TIBERIANDAWN/CREW.CPP` | Intentionally empty translation unit; contains only the function.h include and no executable definitions. |
| Not started | `TIBERIANDAWN/DDE.CPP` | — |
| Not started | `TIBERIANDAWN/DEBUG.CPP` | — |
| Complete | `TIBERIANDAWN/DESCDLG.CPP` | rts-core/src/description_dialog.rs — shared dialog flow with GreenBorder style, SeenBuff-sized background and surface-restore redraw handling. |
| Not started | `TIBERIANDAWN/DIAL8.CPP` | — |
| Not started | `TIBERIANDAWN/DIALOG.CPP` | — |
| Not started | `TIBERIANDAWN/DISPLAY.CPP` | — |
| Not started | `TIBERIANDAWN/DLLInterface.cpp` | — |
| Not started | `TIBERIANDAWN/DLLInterfaceEditor.cpp` | — |
| Not started | `TIBERIANDAWN/DOOR.CPP` | — |
| Complete | `TIBERIANDAWN/DPMI.CPP` | No active implementation in the pinned source; the complete translation unit is enclosed by #if (0). |
| Not started | `TIBERIANDAWN/DRIVE.CPP` | — |
| Not started | `TIBERIANDAWN/EDIT.CPP` | — |
| Complete | `TIBERIANDAWN/ENDING.CPP` | rts-core/src/ending.rs — GDI demo/standard endings, Nod target-selection loop, sound/input gating, cleanup, ending movies, optional trailer and attract-screen sequence. |
| Not started | `TIBERIANDAWN/EVENT.CPP` | — |
| Not started | `TIBERIANDAWN/EXPAND.CPP` | — |
| Complete | `TIBERIANDAWN/FACING.CPP` | rts-core/src/facing.rs — DirType and FacingClass current/desired headings, signed shortest-path rotation, capped rotation rate, sprite rotation zones. |
| Not started | `TIBERIANDAWN/FACTORY.CPP` | — |
| Complete | `TIBERIANDAWN/FIELD.CPP` | rts-core/src/field.rs — shared field serialization and endian conversion behavior. |
| Not started | `TIBERIANDAWN/FINDPATH.CPP` | — |
| Complete | `TIBERIANDAWN/FLASHER.CPP` | rts-core/src/flasher.rs — shared per-player flashing state implementation. |
| Complete | `TIBERIANDAWN/FLY.CPP` | rts-core/src/fly.rs — Fixed_To_Cardinal speed conversion and shared flying-object physics. |
| Not started | `TIBERIANDAWN/FOOT.CPP` | — |
| Not started | `TIBERIANDAWN/FUSE.CPP` | — |
| Not started | `TIBERIANDAWN/GADGET.CPP` | — |
| Not started | `TIBERIANDAWN/GAMEDLG.CPP` | — |
| Not started | `TIBERIANDAWN/GAUGE.CPP` | — |
| Not started | `TIBERIANDAWN/GLOBALS.CPP` | — |
| Not started | `TIBERIANDAWN/GOPTIONS.CPP` | — |
| Not started | `TIBERIANDAWN/GSCREEN.CPP` | — |
| Not started | `TIBERIANDAWN/HDATA.CPP` | — |
| Not started | `TIBERIANDAWN/HEAP.CPP` | — |
| Not started | `TIBERIANDAWN/HELP.CPP` | — |
| Not started | `TIBERIANDAWN/HOUSE.CPP` | — |
| Not started | `TIBERIANDAWN/IDATA.CPP` | — |
| Not started | `TIBERIANDAWN/INFANTRY.CPP` | — |
| Not started | `TIBERIANDAWN/INI.CPP` | — |
| Not started | `TIBERIANDAWN/INIT.CPP` | — |
| Not started | `TIBERIANDAWN/INTERNET.CPP` | — |
| Not started | `TIBERIANDAWN/INTERPAL.CPP` | — |
| Not started | `TIBERIANDAWN/INTRO.CPP` | — |
| Not started | `TIBERIANDAWN/IOMAP.CPP` | — |
| Not started | `TIBERIANDAWN/IOOBJ.CPP` | — |
| Not started | `TIBERIANDAWN/IPX.CPP` | — |
| Complete | `TIBERIANDAWN/IPX95.CPP` | rts-core/src/ipx95.rs — same fixed IPX95 stubs and socket API behavior as Red Alert. |
| Not started | `TIBERIANDAWN/IPXADDR.CPP` | — |
| Not started | `TIBERIANDAWN/IPXCONN.CPP` | — |
| Not started | `TIBERIANDAWN/IPXGCONN.CPP` | — |
| Not started | `TIBERIANDAWN/IPXMGR.CPP` | — |
| Not started | `TIBERIANDAWN/JSHELL.CPP` | — |
| Not started | `TIBERIANDAWN/KEYFRAME.CPP` | — |
| Not started | `TIBERIANDAWN/LAYER.CPP` | — |
| Not started | `TIBERIANDAWN/LINK.CPP` | — |
| Not started | `TIBERIANDAWN/LIST.CPP` | — |
| Not started | `TIBERIANDAWN/LOADDLG.CPP` | — |
| Not started | `TIBERIANDAWN/LOGIC.CPP` | — |
| Not started | `TIBERIANDAWN/MAP.CPP` | — |
| Not started | `TIBERIANDAWN/MAPEDDLG.CPP` | — |
| Not started | `TIBERIANDAWN/MAPEDIT.CPP` | — |
| Not started | `TIBERIANDAWN/MAPEDPLC.CPP` | — |
| Not started | `TIBERIANDAWN/MAPEDSEL.CPP` | — |
| Not started | `TIBERIANDAWN/MAPEDTM.CPP` | — |
| Not started | `TIBERIANDAWN/MAPSEL.CPP` | — |
| Not started | `TIBERIANDAWN/MENUS.CPP` | — |
| Not started | `TIBERIANDAWN/MISSION.CPP` | — |
| Not started | `TIBERIANDAWN/MIXFILE.CPP` | — |
| Not started | `TIBERIANDAWN/MONOC.CPP` | — |
| Not started | `TIBERIANDAWN/MOUSE.CPP` | — |
| Not started | `TIBERIANDAWN/MPLAYER.CPP` | — |
| Not started | `TIBERIANDAWN/MSGBOX.CPP` | — |
| Not started | `TIBERIANDAWN/MSGLIST.CPP` | — |
| Not started | `TIBERIANDAWN/MiscAsm.cpp` | — |
| Not started | `TIBERIANDAWN/NETDLG.CPP` | — |
| Not started | `TIBERIANDAWN/NOSEQCON.CPP` | — |
| Not started | `TIBERIANDAWN/NULLCONN.CPP` | — |
| Not started | `TIBERIANDAWN/NULLDLG.CPP` | — |
| Not started | `TIBERIANDAWN/NULLMGR.CPP` | — |
| Not started | `TIBERIANDAWN/OBJECT.CPP` | — |
| Not started | `TIBERIANDAWN/ODATA.CPP` | — |
| Not started | `TIBERIANDAWN/OPTIONS.CPP` | — |
| Not started | `TIBERIANDAWN/OVERLAY.CPP` | — |
| Not started | `TIBERIANDAWN/PACKET.CPP` | — |
| Not started | `TIBERIANDAWN/POWER.CPP` | — |
| Not started | `TIBERIANDAWN/PROFILE.CPP` | — |
| Not started | `TIBERIANDAWN/QUEUE.CPP` | — |
| Not started | `TIBERIANDAWN/RADAR.CPP` | — |
| Not started | `TIBERIANDAWN/RADIO.CPP` | — |
| Complete | `TIBERIANDAWN/RAND.CPP` | rts-core/src/random.rs — Sim_Random and Sim_IRandom. |
| Not started | `TIBERIANDAWN/RAWFILE.CPP` | — |
| Not started | `TIBERIANDAWN/REINF.CPP` | — |
| Complete | `TIBERIANDAWN/RULES.CPP` | rts-core/src/rules_defaults.rs — exact DifficultyClass and RulesClass constructor defaults. |
| Not started | `TIBERIANDAWN/SAVELOAD.CPP` | — |
| Not started | `TIBERIANDAWN/SCENARIO.CPP` | — |
| Not started | `TIBERIANDAWN/SCORE.CPP` | — |
| Not started | `TIBERIANDAWN/SCROLL.CPP` | — |
| Not started | `TIBERIANDAWN/SDATA.CPP` | — |
| Not started | `TIBERIANDAWN/SEQCONN.CPP` | — |
| Not started | `TIBERIANDAWN/SHAPEBTN.CPP` | — |
| Not started | `TIBERIANDAWN/SIDEBAR.CPP` | — |
| Not started | `TIBERIANDAWN/SIDEBARGlyphx.CPP` | — |
| Not started | `TIBERIANDAWN/SLIDER.CPP` | — |
| Not started | `TIBERIANDAWN/SMUDGE.CPP` | — |
| Not started | `TIBERIANDAWN/SOUNDDLG.CPP` | — |
| Not started | `TIBERIANDAWN/SPECIAL.CPP` | — |
| Not started | `TIBERIANDAWN/STARTUP.CPP` | — |
| Not started | `TIBERIANDAWN/STATS.CPP` | — |
| Not started | `TIBERIANDAWN/SUPER.CPP` | — |
| Complete | `TIBERIANDAWN/Shape.cpp` | rts-core/src/shape_buffer.rs — shared upstream implementation. |
| Not started | `TIBERIANDAWN/TAB.CPP` | — |
| Not started | `TIBERIANDAWN/TARCOM.CPP` | — |
| Not started | `TIBERIANDAWN/TARGET.CPP` | — |
| Not started | `TIBERIANDAWN/TCPIP.CPP` | — |
| Not started | `TIBERIANDAWN/TDATA.CPP` | — |
| Not started | `TIBERIANDAWN/TEAM.CPP` | — |
| Not started | `TIBERIANDAWN/TEAMTYPE.CPP` | — |
| Not started | `TIBERIANDAWN/TECHNO.CPP` | — |
| Not started | `TIBERIANDAWN/TEMPLATE.CPP` | — |
| Not started | `TIBERIANDAWN/TERRAIN.CPP` | — |
| Not started | `TIBERIANDAWN/TEXTBTN.CPP` | — |
| Not started | `TIBERIANDAWN/THEME.CPP` | — |
| Not started | `TIBERIANDAWN/TOGGLE.CPP` | — |
| Not started | `TIBERIANDAWN/TRIGGER.CPP` | — |
| Not started | `TIBERIANDAWN/TURRET.CPP` | — |
| Complete | `TIBERIANDAWN/TXTLABEL.CPP` | rts-core/src/text_label.rs — variant-specific color state, Fancy_Text_Print format and shared clipped text dispatch. |
| Not started | `TIBERIANDAWN/UDATA.CPP` | — |
| Not started | `TIBERIANDAWN/UNIT.CPP` | — |
| Not started | `TIBERIANDAWN/UTRACKER.CPP` | — |
| Not started | `TIBERIANDAWN/VECTOR.CPP` | — |
| Not started | `TIBERIANDAWN/VISUDLG.CPP` | — |
| Not started | `TIBERIANDAWN/WINSTUB.CPP` | — |

### `TIBERIANDAWN/WIN32LIB/`

| Status | Upstream C++ file | Rust port / notes |
|---|---|---|
| Complete | `TIBERIANDAWN/WIN32LIB/ALLOC.CPP` | rts-core/src/legacy_alloc.rs — shared Windows allocator compatibility implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/BUFFER.CPP` | rts-core/src/buffer.rs — shared BufferClass storage and ownership implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/BUFFGLBL.CPP` | rts-core/src/buffer_globals.rs — identical shared global initializers. |
| Not started | `TIBERIANDAWN/WIN32LIB/DDRAW.CPP` | — |
| Complete | `TIBERIANDAWN/WIN32LIB/DIPTHONG.CPP` | rts-formats/src/dipthong.rs — shared implementation; Internet string table and little-endian offset lookup. |
| Complete | `TIBERIANDAWN/WIN32LIB/DRAWRECT.CPP` | rts-core/src/draw_rect.rs — identical shared implementation. |
| Not started | `TIBERIANDAWN/WIN32LIB/DrawMisc.cpp` | — |
| Complete | `TIBERIANDAWN/WIN32LIB/FONT.CPP` | rts-core/src/font_metrics.rs — shared font metrics and print cursor implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/GBUFFER.CPP` | rts-core/src/graphic_buffer.rs — byte-identical shared implementation with Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP` | rts-core/src/shape_access.rs — shared accessor implementation; variant source differs only in non-behavioral formatting. |
| Complete | `TIBERIANDAWN/WIN32LIB/ICONSET.CPP` | rts-formats/src/iconset.rs — shared IFF icon-set loader; variant differences are pointer syntax/include details. |
| Complete | `TIBERIANDAWN/WIN32LIB/IFF.CPP` | rts-formats/src/iff.rs — byte-identical shared implementation with Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/IRANDOM.CPP` | rts-core/src/legacy_irandom.rs — shared implementation; same source routine in both variants. |
| Complete | `TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP` | rts-core/src/td_keyboard.rs — 256-slot ring, locale VK/ASCII remap tables, caps/num toggle table, virtual-key detection, compatibility conversions, mouse triples and source no-op message handler. |
| Complete | `TIBERIANDAWN/WIN32LIB/LOAD.CPP` | rts-formats/src/load.rs — byte-identical shared implementation with Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/LOADFONT.CPP` | rts-core/src/load_font.rs — shared implementation of the same IBM loader and five-block validation used by Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/LOADPAL.CPP` | rts-formats/src/palette.rs — 768-byte IBM palette loading contract. |
| Not started | `TIBERIANDAWN/WIN32LIB/MEM.CPP` | — |
| Complete | `TIBERIANDAWN/WIN32LIB/MORPHPAL.CPP` | rts-formats/src/palette_fade.rs — 768-channel 6-bit palette interpolation and time-progress update. |
| Complete | `TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP` | rts-core/src/mouse_ww.rs — byte-identical shared implementation with Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/NEWDEL.CPP` | rts-core/src/legacy_new_delete.rs — scalar/array new and delete delegation to Alloc(size, MEM_NEW) and Free(pointer). |
| Complete | `TIBERIANDAWN/WIN32LIB/PALETTE.CPP` | rts-formats/src/palette_fade.rs — palette copy/set, single-colour update, fade-rate calculation and bounded channel stepping. |
| Complete | `TIBERIANDAWN/WIN32LIB/REGIONSZ.CPP` | rts-core/src/region_size.rs — signed 32-bit width × height helper with explicit wrapping semantics. |
| Complete | `TIBERIANDAWN/WIN32LIB/SET_FONT.CPP` | rts-core/src/font.rs — identical shared implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/TIMER.CPP` | rts-core/src/legacy_timer.rs — shared TimerClass elapsed-time implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/TIMERDWN.CPP` | rts-core/src/legacy_timer.rs — shared CountDownTimerClass implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/TIMERINI.CPP` | rts-core/src/legacy_timer_system.rs — timer-system state, callback tick counters, active flag, frequency and teardown. |
| Complete | `TIBERIANDAWN/WIN32LIB/WINDOWS.CPP` | rts-formats/src/window_text.rs — byte-identical shared implementation with Red Alert. |
| Complete | `TIBERIANDAWN/WIN32LIB/WINHIDE.CPP` | rts-core/src/window_mouse.rs — shared upstream no-op implementation. |
| Complete | `TIBERIANDAWN/WIN32LIB/WRITEPCX.CPP` | rts-formats/src/pcx.rs — byte-identical shared implementation with Red Alert; same 128-byte header, scanline RLE, and 256-colour palette trailer. |
| Not started | `TIBERIANDAWN/WIN32LIB/WSA.CPP` | — |
| Complete | `TIBERIANDAWN/WIN32LIB/_DIPTABL.CPP` | rts-formats/src/dipthong_tables.rs — identical shared lookup tables. |

## Completion requirements

Port each translation unit with its headers, callers, assembly dependencies, build settings, and relevant data structures. Preserve integer widths, signedness, overflow behavior, byte order, call order, state mutation order, and variant-specific behavior. Platform-specific functionality must be replaced with portable implementations, not placeholders. All file parsing must validate untrusted offsets and lengths.
