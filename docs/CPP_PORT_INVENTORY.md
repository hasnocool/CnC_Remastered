# Upstream C++ translation-unit port inventory

Pinned source: [electronicarts/CnC_Remastered_Collection](https://github.com/electronicarts/CnC_Remastered_Collection)
Upstream commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`

- Total C++ translation units: **494**
- Fully ported: **56**
- Not yet fully ported: **438**
- A translation unit is complete only when its active behavior has a Rust implementation. Intentionally empty source files are identified explicitly; placeholders do not count.

## Coverage by directory

| Upstream directory | C++ files | Complete | Remaining |
|---|---:|---:|---:|
| `REDALERT/` | 275 | 46 | 229 |
| `REDALERT/WIN32LIB/` | 26 | 3 | 23 |
| `TIBERIANDAWN/` | 161 | 3 | 158 |
| `TIBERIANDAWN/WIN32LIB/` | 32 | 4 | 28 |

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
| Not started | `REDALERT/BIGCHECK.CPP` | — |
| Complete | `REDALERT/BLOWFISH.CPP` | rts-formats/src/blowfish.rs — Blowfish key schedule, P/S tables, encrypt/decrypt. |
| Complete | `REDALERT/BLOWPIPE.CPP` | rts-formats/src/blowfish_streams.rs — Blowfish push adapter. |
| Complete | `REDALERT/BLWSTRAW.CPP` | rts-formats/src/blowfish_streams.rs — Blowfish demand-driven adapter. |
| Not started | `REDALERT/BMP8.CPP` | — |
| Not started | `REDALERT/BUFF.CPP` | — |
| Not started | `REDALERT/BUILDING.CPP` | — |
| Not started | `REDALERT/BULLET.CPP` | — |
| Not started | `REDALERT/CARGO.CPP` | — |
| Not started | `REDALERT/CARRY.CPP` | — |
| Not started | `REDALERT/CCDDE.CPP` | — |
| Not started | `REDALERT/CCFILE.CPP` | — |
| Not started | `REDALERT/CCINI.CPP` | — |
| Not started | `REDALERT/CCMPATH.CPP` | — |
| Not started | `REDALERT/CCPTR.CPP` | — |
| Not started | `REDALERT/CCTEN.CPP` | — |
| Not started | `REDALERT/CDATA.CPP` | — |
| Not started | `REDALERT/CDFILE.CPP` | — |
| Not started | `REDALERT/CELL.CPP` | — |
| Not started | `REDALERT/CHECKBOX.CPP` | — |
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
| Not started | `REDALERT/CREW.CPP` | — |
| Complete | `REDALERT/CSTRAW.CPP` | rts-formats/src/cache_straw.rs — buffered read-ahead adapter |
| Not started | `REDALERT/DDE.CPP` | — |
| Not started | `REDALERT/DEBUG.CPP` | — |
| Not started | `REDALERT/DESCDLG.CPP` | — |
| Not started | `REDALERT/DIAL8.CPP` | — |
| Not started | `REDALERT/DIALOG.CPP` | — |
| Not started | `REDALERT/DIBFILE.CPP` | — |
| Not started | `REDALERT/DIBUTIL.CPP` | — |
| Not started | `REDALERT/DISPLAY.CPP` | — |
| Not started | `REDALERT/DLLInterface.cpp` | — |
| Not started | `REDALERT/DLLInterfaceEditor.cpp` | — |
| Not started | `REDALERT/DOOR.CPP` | — |
| Not started | `REDALERT/DPMI.CPP` | — |
| Not started | `REDALERT/DRIVE.CPP` | — |
| Not started | `REDALERT/DROP.CPP` | — |
| Complete | `REDALERT/DTABLE.CPP` | rts-formats/src/adpcm.rs — exact 1,424-entry ADPCM difference table. |
| Not started | `REDALERT/DYNAVEC.CPP` | — |
| Not started | `REDALERT/EDIT.CPP` | — |
| Not started | `REDALERT/EGOS.CPP` | — |
| Not started | `REDALERT/ENDING.CPP` | — |
| Not started | `REDALERT/EVENT.CPP` | — |
| Not started | `REDALERT/EXPAND.CPP` | — |
| Not started | `REDALERT/FACE.CPP` | — |
| Complete | `REDALERT/FACING.CPP` | rts-core/src/facing.rs — DirType and FacingClass current/desired headings, signed shortest-path rotation, capped rotation rate, sprite rotation zones. |
| Not started | `REDALERT/FACTORY.CPP` | — |
| Not started | `REDALERT/FIELD.CPP` | — |
| Not started | `REDALERT/FINDPATH.CPP` | — |
| Complete | `REDALERT/FIXED.CPP` | rts-core/src/fixed16.rs — Q16.16 constructors, arithmetic helpers, ASCII conversion. |
| Not started | `REDALERT/FLASHER.CPP` | — |
| Not started | `REDALERT/FLY.CPP` | — |
| Not started | `REDALERT/FOOT.CPP` | — |
| Not started | `REDALERT/FUSE.CPP` | — |
| Not started | `REDALERT/GADGET.CPP` | — |
| Not started | `REDALERT/GAMEDLG.CPP` | — |
| Not started | `REDALERT/GAUGE.CPP` | — |
| Not started | `REDALERT/GETCPU.CPP` | — |
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
| Not started | `REDALERT/INICODE.CPP` | — |
| Not started | `REDALERT/INIT.CPP` | — |
| Complete | `REDALERT/INT.CPP` | rts-core/src/bignum_globals.rs — zero-initialized Error, Carry, Borrow, and precision-specific remainder state. |
| Not started | `REDALERT/INTERNET.CPP` | — |
| Not started | `REDALERT/INTERPAL.CPP` | — |
| Not started | `REDALERT/INTRO.CPP` | — |
| Not started | `REDALERT/IOMAP.CPP` | — |
| Not started | `REDALERT/IOOBJ.CPP` | — |
| Not started | `REDALERT/IPX.CPP` | — |
| Not started | `REDALERT/IPX95.CPP` | — |
| Not started | `REDALERT/IPXADDR.CPP` | — |
| Not started | `REDALERT/IPXCONN.CPP` | — |
| Not started | `REDALERT/IPXGCONN.CPP` | — |
| Not started | `REDALERT/IPXMGR.CPP` | — |
| Complete | `REDALERT/ITABLE.CPP` | rts-formats/src/adpcm.rs — exact 1,424-entry ADPCM index table. |
| Not started | `REDALERT/JSHELL.CPP` | — |
| Not started | `REDALERT/KEY.CPP` | — |
| Not started | `REDALERT/KEYBOARD.CPP` | — |
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
| Not started | `REDALERT/MPLIB.CPP` | — |
| Not started | `REDALERT/MPMGRD.CPP` | — |
| Not started | `REDALERT/MPMGRW.CPP` | — |
| Not started | `REDALERT/MPU.CPP` | — |
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
| Not started | `REDALERT/RAWOLAPI.CPP` | — |
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
| Not started | `REDALERT/SURFACE.CPP` | — |
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
| Not started | `REDALERT/TRACKER.CPP` | — |
| Not started | `REDALERT/TRIGGER.CPP` | — |
| Not started | `REDALERT/TRIGTYPE.CPP` | — |
| Not started | `REDALERT/TURRET.CPP` | — |
| Not started | `REDALERT/TXTLABEL.CPP` | — |
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
| Not started | `REDALERT/W95TRACE.CPP` | — |
| Not started | `REDALERT/WARHEAD.CPP` | — |
| Not started | `REDALERT/WEAPON.CPP` | — |
| Not started | `REDALERT/WINSTUB.CPP` | — |
| Not started | `REDALERT/WOLAPIOB.CPP` | — |
| Not started | `REDALERT/WOLEDIT.CPP` | — |
| Not started | `REDALERT/WOLSTRNG.CPP` | — |
| Not started | `REDALERT/WOL_CGAM.CPP` | — |
| Not started | `REDALERT/WOL_CHAT.CPP` | — |
| Not started | `REDALERT/WOL_DNLD.CPP` | — |
| Not started | `REDALERT/WOL_GSUP.CPP` | — |
| Not started | `REDALERT/WOL_LOGN.CPP` | — |
| Not started | `REDALERT/WOL_MAIN.CPP` | — |
| Not started | `REDALERT/WOL_OPT.CPP` | — |
| Not started | `REDALERT/WSPIPX.CPP` | — |
| Not started | `REDALERT/WSPROTO.CPP` | — |
| Not started | `REDALERT/WSPUDP.CPP` | — |
| Complete | `REDALERT/XPIPE.CPP` | rts-formats/src/io_streams.rs — fixed-buffer and file-backed push endpoints |
| Complete | `REDALERT/XSTRAW.CPP` | rts-formats/src/io_streams.rs — fixed-buffer and file-backed pull endpoints |
| Not started | `REDALERT/_WSPROTO.CPP` | — |

### `REDALERT/WIN32LIB/`

| Status | Upstream C++ file | Rust port / notes |
|---|---|---|
| Not started | `REDALERT/WIN32LIB/ALLOC.CPP` | — |
| Not started | `REDALERT/WIN32LIB/BUFFER.CPP` | — |
| Not started | `REDALERT/WIN32LIB/BUFFGLBL.CPP` | — |
| Not started | `REDALERT/WIN32LIB/DDRAW.CPP` | — |
| Complete | `REDALERT/WIN32LIB/DELAY.CPP` | rts-core/src/delay.rs — timer threshold and duration loop. |
| Not started | `REDALERT/WIN32LIB/DIPTHONG.CPP` | — |
| Not started | `REDALERT/WIN32LIB/DRAWRECT.CPP` | — |
| Not started | `REDALERT/WIN32LIB/DrawMisc.cpp` | — |
| Not started | `REDALERT/WIN32LIB/FONT.CPP` | — |
| Not started | `REDALERT/WIN32LIB/GBUFFER.CPP` | — |
| Not started | `REDALERT/WIN32LIB/GETSHAPE.CPP` | — |
| Not started | `REDALERT/WIN32LIB/ICONSET.CPP` | — |
| Not started | `REDALERT/WIN32LIB/IFF.CPP` | — |
| Not started | `REDALERT/WIN32LIB/IRANDOM.CPP` | — |
| Not started | `REDALERT/WIN32LIB/LOAD.CPP` | — |
| Not started | `REDALERT/WIN32LIB/LOADFONT.CPP` | — |
| Not started | `REDALERT/WIN32LIB/MOUSEWW.CPP` | — |
| Not started | `REDALERT/WIN32LIB/PALETTE.CPP` | — |
| Not started | `REDALERT/WIN32LIB/SET_FONT.CPP` | — |
| Not started | `REDALERT/WIN32LIB/TIMER.CPP` | — |
| Not started | `REDALERT/WIN32LIB/TIMERDWN.CPP` | — |
| Not started | `REDALERT/WIN32LIB/TIMERINI.CPP` | — |
| Not started | `REDALERT/WIN32LIB/WINDOWS.CPP` | — |
| Complete | `REDALERT/WIN32LIB/WINHIDE.CPP` | rts-core/src/window_mouse.rs — mouse hide/show calls are commented out upstream; observable no-op. |
| Not started | `REDALERT/WIN32LIB/WRITEPCX.CPP` | — |
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
| Not started | `TIBERIANDAWN/CHECKBOX.CPP` | — |
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
| Not started | `TIBERIANDAWN/CREW.CPP` | — |
| Not started | `TIBERIANDAWN/DDE.CPP` | — |
| Not started | `TIBERIANDAWN/DEBUG.CPP` | — |
| Not started | `TIBERIANDAWN/DESCDLG.CPP` | — |
| Not started | `TIBERIANDAWN/DIAL8.CPP` | — |
| Not started | `TIBERIANDAWN/DIALOG.CPP` | — |
| Not started | `TIBERIANDAWN/DISPLAY.CPP` | — |
| Not started | `TIBERIANDAWN/DLLInterface.cpp` | — |
| Not started | `TIBERIANDAWN/DLLInterfaceEditor.cpp` | — |
| Not started | `TIBERIANDAWN/DOOR.CPP` | — |
| Not started | `TIBERIANDAWN/DPMI.CPP` | — |
| Not started | `TIBERIANDAWN/DRIVE.CPP` | — |
| Not started | `TIBERIANDAWN/EDIT.CPP` | — |
| Not started | `TIBERIANDAWN/ENDING.CPP` | — |
| Not started | `TIBERIANDAWN/EVENT.CPP` | — |
| Not started | `TIBERIANDAWN/EXPAND.CPP` | — |
| Complete | `TIBERIANDAWN/FACING.CPP` | rts-core/src/facing.rs — DirType and FacingClass current/desired headings, signed shortest-path rotation, capped rotation rate, sprite rotation zones. |
| Not started | `TIBERIANDAWN/FACTORY.CPP` | — |
| Not started | `TIBERIANDAWN/FIELD.CPP` | — |
| Not started | `TIBERIANDAWN/FINDPATH.CPP` | — |
| Not started | `TIBERIANDAWN/FLASHER.CPP` | — |
| Not started | `TIBERIANDAWN/FLY.CPP` | — |
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
| Not started | `TIBERIANDAWN/IPX95.CPP` | — |
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
| Not started | `TIBERIANDAWN/RULES.CPP` | — |
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
| Not started | `TIBERIANDAWN/TXTLABEL.CPP` | — |
| Not started | `TIBERIANDAWN/UDATA.CPP` | — |
| Not started | `TIBERIANDAWN/UNIT.CPP` | — |
| Not started | `TIBERIANDAWN/UTRACKER.CPP` | — |
| Not started | `TIBERIANDAWN/VECTOR.CPP` | — |
| Not started | `TIBERIANDAWN/VISUDLG.CPP` | — |
| Not started | `TIBERIANDAWN/WINSTUB.CPP` | — |

### `TIBERIANDAWN/WIN32LIB/`

| Status | Upstream C++ file | Rust port / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/WIN32LIB/ALLOC.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/BUFFER.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/BUFFGLBL.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/DDRAW.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/DIPTHONG.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/DRAWRECT.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/DrawMisc.cpp` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/FONT.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/GBUFFER.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/ICONSET.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/IFF.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/IRANDOM.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/LOAD.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/LOADFONT.CPP` | — |
| Complete | `TIBERIANDAWN/WIN32LIB/LOADPAL.CPP` | rts-formats/src/palette.rs — 768-byte IBM palette loading contract. |
| Not started | `TIBERIANDAWN/WIN32LIB/MEM.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/MORPHPAL.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/NEWDEL.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/PALETTE.CPP` | — |
| Complete | `TIBERIANDAWN/WIN32LIB/REGIONSZ.CPP` | rts-core/src/region_size.rs — signed 32-bit width × height helper with explicit wrapping semantics. |
| Not started | `TIBERIANDAWN/WIN32LIB/SET_FONT.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/TIMER.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/TIMERDWN.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/TIMERINI.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/WINDOWS.CPP` | — |
| Complete | `TIBERIANDAWN/WIN32LIB/WINHIDE.CPP` | rts-core/src/window_mouse.rs — shared upstream no-op implementation. |
| Not started | `TIBERIANDAWN/WIN32LIB/WRITEPCX.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/WSA.CPP` | — |
| Complete | `TIBERIANDAWN/WIN32LIB/_DIPTABL.CPP` | rts-formats/src/dipthong_tables.rs — identical shared lookup tables. |

## Completion requirements

Port each translation unit with its headers, callers, assembly dependencies, build settings, and relevant data structures. Preserve integer widths, signedness, overflow behavior, byte order, call order, state mutation order, and variant-specific behavior. Platform-specific functionality must be replaced with portable implementations, not placeholders. All file parsing must validate untrusted offsets and lengths.
