# Upstream C++ translation-unit port inventory

Pinned upstream: [electronicarts/CnC_Remastered_Collection](https://github.com/electronicarts/CnC_Remastered_Collection)
Commit: `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`

- Total C++ translation units: **494**
- Fully ported in Rust: **4**
- Not yet fully ported: **490**
- A file is marked ported only when its behavior is implemented, not when a placeholder is created.
- Header-inline and assembly dependencies are tracked separately in the architecture and TODO documents.

## Coverage by source directory

| Source directory | C++ files | Fully ported | Remaining |
|---|---:|---:|---:|
| `REDALERT/_WSPROTO.CPP/` | 1 | 0 | 1 |
| `REDALERT/2KEYFRAM.CPP/` | 1 | 0 | 1 |
| `REDALERT/AADATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/ABSTRACT.CPP/` | 1 | 0 | 1 |
| `REDALERT/ADATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/ADPCM.CPP/` | 1 | 0 | 1 |
| `REDALERT/AIRCRAFT.CPP/` | 1 | 0 | 1 |
| `REDALERT/ANIM.CPP/` | 1 | 0 | 1 |
| `REDALERT/AUDIO.CPP/` | 1 | 0 | 1 |
| `REDALERT/B64PIPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/B64STRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/BAR.CPP/` | 1 | 0 | 1 |
| `REDALERT/BASE.CPP/` | 1 | 0 | 1 |
| `REDALERT/BASE64.CPP/` | 1 | 0 | 1 |
| `REDALERT/BBDATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/BDATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/BENCH.CPP/` | 1 | 0 | 1 |
| `REDALERT/BFIOFILE.CPP/` | 1 | 0 | 1 |
| `REDALERT/BIGCHECK.CPP/` | 1 | 0 | 1 |
| `REDALERT/BLOWFISH.CPP/` | 1 | 0 | 1 |
| `REDALERT/BLOWPIPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/BLWSTRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/BMP8.CPP/` | 1 | 0 | 1 |
| `REDALERT/BUFF.CPP/` | 1 | 0 | 1 |
| `REDALERT/BUILDING.CPP/` | 1 | 0 | 1 |
| `REDALERT/BULLET.CPP/` | 1 | 0 | 1 |
| `REDALERT/CARGO.CPP/` | 1 | 0 | 1 |
| `REDALERT/CARRY.CPP/` | 1 | 0 | 1 |
| `REDALERT/CCDDE.CPP/` | 1 | 0 | 1 |
| `REDALERT/CCFILE.CPP/` | 1 | 0 | 1 |
| `REDALERT/CCINI.CPP/` | 1 | 0 | 1 |
| `REDALERT/CCMPATH.CPP/` | 1 | 0 | 1 |
| `REDALERT/CCPTR.CPP/` | 1 | 0 | 1 |
| `REDALERT/CCTEN.CPP/` | 1 | 0 | 1 |
| `REDALERT/CDATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/CDFILE.CPP/` | 1 | 0 | 1 |
| `REDALERT/CELL.CPP/` | 1 | 0 | 1 |
| `REDALERT/CHECKBOX.CPP/` | 1 | 0 | 1 |
| `REDALERT/CHEKLIST.CPP/` | 1 | 0 | 1 |
| `REDALERT/CLASS.CPP/` | 1 | 0 | 1 |
| `REDALERT/COLRLIST.CPP/` | 1 | 0 | 1 |
| `REDALERT/COMBAT.CPP/` | 1 | 0 | 1 |
| `REDALERT/COMBUF.CPP/` | 1 | 0 | 1 |
| `REDALERT/COMINIT.CPP/` | 1 | 0 | 1 |
| `REDALERT/COMQUEUE.CPP/` | 1 | 0 | 1 |
| `REDALERT/CONFDLG.CPP/` | 1 | 0 | 1 |
| `REDALERT/CONNECT.CPP/` | 1 | 0 | 1 |
| `REDALERT/CONQUER.CPP/` | 1 | 0 | 1 |
| `REDALERT/CONST.CPP/` | 1 | 0 | 1 |
| `REDALERT/CONTROL.CPP/` | 1 | 0 | 1 |
| `REDALERT/COORD.CPP/` | 1 | 0 | 1 |
| `REDALERT/CRATE.CPP/` | 1 | 0 | 1 |
| `REDALERT/CRC.CPP/` | 1 | 1 | 0 |
| `REDALERT/CRCPIPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/CRCSTRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/CREDITS.CPP/` | 1 | 0 | 1 |
| `REDALERT/CREW.CPP/` | 1 | 0 | 1 |
| `REDALERT/CSTRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/DDE.CPP/` | 1 | 0 | 1 |
| `REDALERT/DEBUG.CPP/` | 1 | 0 | 1 |
| `REDALERT/DESCDLG.CPP/` | 1 | 0 | 1 |
| `REDALERT/DIAL8.CPP/` | 1 | 0 | 1 |
| `REDALERT/DIALOG.CPP/` | 1 | 0 | 1 |
| `REDALERT/DIBFILE.CPP/` | 1 | 0 | 1 |
| `REDALERT/DIBUTIL.CPP/` | 1 | 0 | 1 |
| `REDALERT/DISPLAY.CPP/` | 1 | 0 | 1 |
| `REDALERT/DLLInterface.cpp/` | 1 | 0 | 1 |
| `REDALERT/DLLInterfaceEditor.cpp/` | 1 | 0 | 1 |
| `REDALERT/DOOR.CPP/` | 1 | 0 | 1 |
| `REDALERT/DPMI.CPP/` | 1 | 0 | 1 |
| `REDALERT/DRIVE.CPP/` | 1 | 0 | 1 |
| `REDALERT/DROP.CPP/` | 1 | 0 | 1 |
| `REDALERT/DTABLE.CPP/` | 1 | 0 | 1 |
| `REDALERT/DYNAVEC.CPP/` | 1 | 0 | 1 |
| `REDALERT/EDIT.CPP/` | 1 | 0 | 1 |
| `REDALERT/EGOS.CPP/` | 1 | 0 | 1 |
| `REDALERT/ENDING.CPP/` | 1 | 0 | 1 |
| `REDALERT/EVENT.CPP/` | 1 | 0 | 1 |
| `REDALERT/EXPAND.CPP/` | 1 | 0 | 1 |
| `REDALERT/FACE.CPP/` | 1 | 0 | 1 |
| `REDALERT/FACING.CPP/` | 1 | 0 | 1 |
| `REDALERT/FACTORY.CPP/` | 1 | 0 | 1 |
| `REDALERT/FIELD.CPP/` | 1 | 0 | 1 |
| `REDALERT/FINDPATH.CPP/` | 1 | 0 | 1 |
| `REDALERT/FIXED.CPP/` | 1 | 1 | 0 |
| `REDALERT/FLASHER.CPP/` | 1 | 0 | 1 |
| `REDALERT/FLY.CPP/` | 1 | 0 | 1 |
| `REDALERT/FOOT.CPP/` | 1 | 0 | 1 |
| `REDALERT/FUSE.CPP/` | 1 | 0 | 1 |
| `REDALERT/GADGET.CPP/` | 1 | 0 | 1 |
| `REDALERT/GAMEDLG.CPP/` | 1 | 0 | 1 |
| `REDALERT/GAUGE.CPP/` | 1 | 0 | 1 |
| `REDALERT/GETCPU.CPP/` | 1 | 0 | 1 |
| `REDALERT/GLOBALS.CPP/` | 1 | 0 | 1 |
| `REDALERT/GOPTIONS.CPP/` | 1 | 0 | 1 |
| `REDALERT/GSCREEN.CPP/` | 1 | 0 | 1 |
| `REDALERT/HDATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/HEAP.CPP/` | 1 | 0 | 1 |
| `REDALERT/HELP.CPP/` | 1 | 0 | 1 |
| `REDALERT/HOUSE.CPP/` | 1 | 0 | 1 |
| `REDALERT/HSV.CPP/` | 1 | 0 | 1 |
| `REDALERT/ICONLIST.CPP/` | 1 | 0 | 1 |
| `REDALERT/IDATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/INFANTRY.CPP/` | 1 | 0 | 1 |
| `REDALERT/INI.CPP/` | 1 | 0 | 1 |
| `REDALERT/INIBIN.CPP/` | 1 | 0 | 1 |
| `REDALERT/INICODE.CPP/` | 1 | 0 | 1 |
| `REDALERT/INIT.CPP/` | 1 | 0 | 1 |
| `REDALERT/INT.CPP/` | 1 | 0 | 1 |
| `REDALERT/INTERNET.CPP/` | 1 | 0 | 1 |
| `REDALERT/INTERPAL.CPP/` | 1 | 0 | 1 |
| `REDALERT/INTRO.CPP/` | 1 | 0 | 1 |
| `REDALERT/IOMAP.CPP/` | 1 | 0 | 1 |
| `REDALERT/IOOBJ.CPP/` | 1 | 0 | 1 |
| `REDALERT/IPX.CPP/` | 1 | 0 | 1 |
| `REDALERT/IPX95.CPP/` | 1 | 0 | 1 |
| `REDALERT/IPXADDR.CPP/` | 1 | 0 | 1 |
| `REDALERT/IPXCONN.CPP/` | 1 | 0 | 1 |
| `REDALERT/IPXGCONN.CPP/` | 1 | 0 | 1 |
| `REDALERT/IPXMGR.CPP/` | 1 | 0 | 1 |
| `REDALERT/ITABLE.CPP/` | 1 | 0 | 1 |
| `REDALERT/JSHELL.CPP/` | 1 | 0 | 1 |
| `REDALERT/KEY.CPP/` | 1 | 0 | 1 |
| `REDALERT/KEYBOARD.CPP/` | 1 | 0 | 1 |
| `REDALERT/LAYER.CPP/` | 1 | 0 | 1 |
| `REDALERT/LCW.CPP/` | 1 | 0 | 1 |
| `REDALERT/LCWPIPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/LCWSTRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/LCWUNCMP.CPP/` | 1 | 0 | 1 |
| `REDALERT/LINK.CPP/` | 1 | 0 | 1 |
| `REDALERT/LIST.CPP/` | 1 | 0 | 1 |
| `REDALERT/LOADDLG.CPP/` | 1 | 0 | 1 |
| `REDALERT/LOGIC.CPP/` | 1 | 0 | 1 |
| `REDALERT/LZO1X_C.CPP/` | 1 | 0 | 1 |
| `REDALERT/LZO1X_D.CPP/` | 1 | 0 | 1 |
| `REDALERT/LZOPIPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/LZOSTRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/LZW.CPP/` | 1 | 0 | 1 |
| `REDALERT/LZWOTRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/LZWPIPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/LZWSTRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/MAP.CPP/` | 1 | 0 | 1 |
| `REDALERT/MAPEDDLG.CPP/` | 1 | 0 | 1 |
| `REDALERT/MAPEDIT.CPP/` | 1 | 0 | 1 |
| `REDALERT/MAPEDPLC.CPP/` | 1 | 0 | 1 |
| `REDALERT/MAPEDSEL.CPP/` | 1 | 0 | 1 |
| `REDALERT/MAPEDTM.CPP/` | 1 | 0 | 1 |
| `REDALERT/MAPSEL.CPP/` | 1 | 0 | 1 |
| `REDALERT/MCI.CPP/` | 1 | 0 | 1 |
| `REDALERT/MCIMOVIE.CPP/` | 1 | 0 | 1 |
| `REDALERT/MENUS.CPP/` | 1 | 0 | 1 |
| `REDALERT/MiscAsm.cpp/` | 1 | 0 | 1 |
| `REDALERT/MISSION.CPP/` | 1 | 0 | 1 |
| `REDALERT/MIXFILE.CPP/` | 1 | 0 | 1 |
| `REDALERT/MONOC.CPP/` | 1 | 0 | 1 |
| `REDALERT/MOUSE.CPP/` | 1 | 0 | 1 |
| `REDALERT/MP.CPP/` | 1 | 0 | 1 |
| `REDALERT/MPGSET.CPP/` | 1 | 0 | 1 |
| `REDALERT/MPLAYER.CPP/` | 1 | 0 | 1 |
| `REDALERT/MPLIB.CPP/` | 1 | 0 | 1 |
| `REDALERT/MPMGRD.CPP/` | 1 | 0 | 1 |
| `REDALERT/MPMGRW.CPP/` | 1 | 0 | 1 |
| `REDALERT/MPU.CPP/` | 1 | 0 | 1 |
| `REDALERT/MSGBOX.CPP/` | 1 | 0 | 1 |
| `REDALERT/MSGLIST.CPP/` | 1 | 0 | 1 |
| `REDALERT/NETDLG.CPP/` | 1 | 0 | 1 |
| `REDALERT/NOSEQCON.CPP/` | 1 | 0 | 1 |
| `REDALERT/NULLCONN.CPP/` | 1 | 0 | 1 |
| `REDALERT/NULLDLG.CPP/` | 1 | 0 | 1 |
| `REDALERT/NULLMGR.CPP/` | 1 | 0 | 1 |
| `REDALERT/OBJECT.CPP/` | 1 | 0 | 1 |
| `REDALERT/ODATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/OPTIONS.CPP/` | 1 | 0 | 1 |
| `REDALERT/OVERLAY.CPP/` | 1 | 0 | 1 |
| `REDALERT/PACKET.CPP/` | 1 | 0 | 1 |
| `REDALERT/PALETTEC.CPP/` | 1 | 0 | 1 |
| `REDALERT/PIPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/PK.CPP/` | 1 | 0 | 1 |
| `REDALERT/PKPIPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/PKSTRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/POWER.CPP/` | 1 | 0 | 1 |
| `REDALERT/PROFILE.CPP/` | 1 | 0 | 1 |
| `REDALERT/QUEUE.CPP/` | 1 | 0 | 1 |
| `REDALERT/RADAR.CPP/` | 1 | 0 | 1 |
| `REDALERT/RADIO.CPP/` | 1 | 0 | 1 |
| `REDALERT/RAMFILE.CPP/` | 1 | 0 | 1 |
| `REDALERT/RAND.CPP/` | 1 | 1 | 0 |
| `REDALERT/RANDOM.CPP/` | 1 | 0 | 1 |
| `REDALERT/RAWFILE.CPP/` | 1 | 0 | 1 |
| `REDALERT/RAWOLAPI.CPP/` | 1 | 0 | 1 |
| `REDALERT/READLINE.CPP/` | 1 | 0 | 1 |
| `REDALERT/RECT.CPP/` | 1 | 0 | 1 |
| `REDALERT/REINF.CPP/` | 1 | 0 | 1 |
| `REDALERT/RGB.CPP/` | 1 | 0 | 1 |
| `REDALERT/RNDSTRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/ROTBMP.CPP/` | 1 | 0 | 1 |
| `REDALERT/RULES.CPP/` | 1 | 0 | 1 |
| `REDALERT/SAVELOAD.CPP/` | 1 | 0 | 1 |
| `REDALERT/SCENARIO.CPP/` | 1 | 0 | 1 |
| `REDALERT/SCORE.CPP/` | 1 | 0 | 1 |
| `REDALERT/SCROLL.CPP/` | 1 | 0 | 1 |
| `REDALERT/SDATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/SEDITDLG.CPP/` | 1 | 0 | 1 |
| `REDALERT/SENDFILE.CPP/` | 1 | 0 | 1 |
| `REDALERT/SEQCONN.CPP/` | 1 | 0 | 1 |
| `REDALERT/SESSION.CPP/` | 1 | 0 | 1 |
| `REDALERT/SHA.CPP/` | 1 | 0 | 1 |
| `REDALERT/Shape.cpp/` | 1 | 0 | 1 |
| `REDALERT/SHAPEBTN.CPP/` | 1 | 0 | 1 |
| `REDALERT/SHAPIPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/SHASTRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/SIDEBAR.CPP/` | 1 | 0 | 1 |
| `REDALERT/SIDEBARGlyphx.CPP/` | 1 | 0 | 1 |
| `REDALERT/SLIDER.CPP/` | 1 | 0 | 1 |
| `REDALERT/SMUDGE.CPP/` | 1 | 0 | 1 |
| `REDALERT/SOUNDDLG.CPP/` | 1 | 0 | 1 |
| `REDALERT/SPECIAL.CPP/` | 1 | 0 | 1 |
| `REDALERT/SPRITE.CPP/` | 1 | 0 | 1 |
| `REDALERT/STARTUP.CPP/` | 1 | 0 | 1 |
| `REDALERT/STATBTN.CPP/` | 1 | 0 | 1 |
| `REDALERT/STATS.CPP/` | 1 | 0 | 1 |
| `REDALERT/STRAW.CPP/` | 1 | 0 | 1 |
| `REDALERT/STUB.CPP/` | 1 | 0 | 1 |
| `REDALERT/SUPER.CPP/` | 1 | 0 | 1 |
| `REDALERT/SURFACE.CPP/` | 1 | 0 | 1 |
| `REDALERT/TAB.CPP/` | 1 | 0 | 1 |
| `REDALERT/TACTION.CPP/` | 1 | 0 | 1 |
| `REDALERT/TARGET.CPP/` | 1 | 0 | 1 |
| `REDALERT/TCPIP.CPP/` | 1 | 0 | 1 |
| `REDALERT/TDATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/TEAM.CPP/` | 1 | 0 | 1 |
| `REDALERT/TEAMTYPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/TECHNO.CPP/` | 1 | 0 | 1 |
| `REDALERT/TEMPLATE.CPP/` | 1 | 0 | 1 |
| `REDALERT/TENMGR.CPP/` | 1 | 0 | 1 |
| `REDALERT/TERRAIN.CPP/` | 1 | 0 | 1 |
| `REDALERT/TEVENT.CPP/` | 1 | 0 | 1 |
| `REDALERT/TEXTBTN.CPP/` | 1 | 0 | 1 |
| `REDALERT/THEME.CPP/` | 1 | 0 | 1 |
| `REDALERT/TOGGLE.CPP/` | 1 | 0 | 1 |
| `REDALERT/TOOLTIP.CPP/` | 1 | 0 | 1 |
| `REDALERT/TRACKER.CPP/` | 1 | 0 | 1 |
| `REDALERT/TRIGGER.CPP/` | 1 | 0 | 1 |
| `REDALERT/TRIGTYPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/TURRET.CPP/` | 1 | 0 | 1 |
| `REDALERT/TXTLABEL.CPP/` | 1 | 0 | 1 |
| `REDALERT/UDATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/UDPADDR.CPP/` | 1 | 0 | 1 |
| `REDALERT/UNIT.CPP/` | 1 | 0 | 1 |
| `REDALERT/UTRACKER.CPP/` | 1 | 0 | 1 |
| `REDALERT/VDATA.CPP/` | 1 | 0 | 1 |
| `REDALERT/VECTOR.CPP/` | 1 | 0 | 1 |
| `REDALERT/VERSION.CPP/` | 1 | 0 | 1 |
| `REDALERT/VESSEL.CPP/` | 1 | 0 | 1 |
| `REDALERT/VISUDLG.CPP/` | 1 | 0 | 1 |
| `REDALERT/VORTEX.CPP/` | 1 | 0 | 1 |
| `REDALERT/W95TRACE.CPP/` | 1 | 0 | 1 |
| `REDALERT/WARHEAD.CPP/` | 1 | 0 | 1 |
| `REDALERT/WEAPON.CPP/` | 1 | 0 | 1 |
| `REDALERT/WIN32LIB/` | 26 | 0 | 26 |
| `REDALERT/WINSTUB.CPP/` | 1 | 0 | 1 |
| `REDALERT/WOL_CGAM.CPP/` | 1 | 0 | 1 |
| `REDALERT/WOL_CHAT.CPP/` | 1 | 0 | 1 |
| `REDALERT/WOL_DNLD.CPP/` | 1 | 0 | 1 |
| `REDALERT/WOL_GSUP.CPP/` | 1 | 0 | 1 |
| `REDALERT/WOL_LOGN.CPP/` | 1 | 0 | 1 |
| `REDALERT/WOL_MAIN.CPP/` | 1 | 0 | 1 |
| `REDALERT/WOL_OPT.CPP/` | 1 | 0 | 1 |
| `REDALERT/WOLAPIOB.CPP/` | 1 | 0 | 1 |
| `REDALERT/WOLEDIT.CPP/` | 1 | 0 | 1 |
| `REDALERT/WOLSTRNG.CPP/` | 1 | 0 | 1 |
| `REDALERT/WSPIPX.CPP/` | 1 | 0 | 1 |
| `REDALERT/WSPROTO.CPP/` | 1 | 0 | 1 |
| `REDALERT/WSPUDP.CPP/` | 1 | 0 | 1 |
| `REDALERT/XPIPE.CPP/` | 1 | 0 | 1 |
| `REDALERT/XSTRAW.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/AADATA.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/ABSTRACT.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/ADATA.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/AIRCRAFT.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/ALLOC.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/ANIM.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/AUDIO.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/BASE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/BBDATA.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/BDATA.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/BUILDING.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/BULLET.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CARGO.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CCDDE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CCFILE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CDATA.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CDFILE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CELL.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CHECKBOX.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CHEKLIST.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/COLRLIST.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/COMBAT.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/COMBUF.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/COMQUEUE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CONFDLG.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CONNECT.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CONQUER.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CONST.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CONTROL.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/COORD.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CREDITS.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/CREW.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/DDE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/DEBUG.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/DESCDLG.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/DIAL8.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/DIALOG.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/DISPLAY.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/DLLInterface.cpp/` | 1 | 0 | 1 |
| `TIBERIANDAWN/DLLInterfaceEditor.cpp/` | 1 | 0 | 1 |
| `TIBERIANDAWN/DOOR.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/DPMI.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/DRIVE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/EDIT.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/ENDING.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/EVENT.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/EXPAND.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/FACING.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/FACTORY.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/FIELD.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/FINDPATH.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/FLASHER.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/FLY.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/FOOT.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/FUSE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/GADGET.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/GAMEDLG.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/GAUGE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/GLOBALS.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/GOPTIONS.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/GSCREEN.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/HDATA.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/HEAP.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/HELP.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/HOUSE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/IDATA.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/INFANTRY.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/INI.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/INIT.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/INTERNET.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/INTERPAL.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/INTRO.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/IOMAP.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/IOOBJ.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/IPX.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/IPX95.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/IPXADDR.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/IPXCONN.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/IPXGCONN.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/IPXMGR.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/JSHELL.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/KEYFRAME.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/LAYER.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/LINK.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/LIST.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/LOADDLG.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/LOGIC.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MAP.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MAPEDDLG.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MAPEDIT.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MAPEDPLC.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MAPEDSEL.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MAPEDTM.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MAPSEL.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MENUS.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MiscAsm.cpp/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MISSION.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MIXFILE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MONOC.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MOUSE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MPLAYER.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MSGBOX.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/MSGLIST.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/NETDLG.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/NOSEQCON.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/NULLCONN.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/NULLDLG.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/NULLMGR.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/OBJECT.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/ODATA.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/OPTIONS.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/OVERLAY.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/PACKET.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/POWER.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/PROFILE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/QUEUE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/RADAR.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/RADIO.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/RAND.CPP/` | 1 | 1 | 0 |
| `TIBERIANDAWN/RAWFILE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/REINF.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/RULES.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SAVELOAD.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SCENARIO.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SCORE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SCROLL.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SDATA.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SEQCONN.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/Shape.cpp/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SHAPEBTN.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SIDEBAR.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SIDEBARGlyphx.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SLIDER.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SMUDGE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SOUNDDLG.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SPECIAL.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/STARTUP.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/STATS.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/SUPER.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TAB.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TARCOM.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TARGET.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TCPIP.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TDATA.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TEAM.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TEAMTYPE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TECHNO.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TEMPLATE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TERRAIN.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TEXTBTN.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/THEME.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TOGGLE.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TRIGGER.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TURRET.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/TXTLABEL.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/UDATA.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/UNIT.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/UTRACKER.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/VECTOR.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/VISUDLG.CPP/` | 1 | 0 | 1 |
| `TIBERIANDAWN/WIN32LIB/` | 32 | 0 | 32 |
| `TIBERIANDAWN/WINSTUB.CPP/` | 1 | 0 | 1 |

## File-by-file status

### `REDALERT/_WSPROTO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/_WSPROTO.CPP` | — |

### `REDALERT/2KEYFRAM.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/2KEYFRAM.CPP` | — |

### `REDALERT/AADATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/AADATA.CPP` | — |

### `REDALERT/ABSTRACT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/ABSTRACT.CPP` | — |

### `REDALERT/ADATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/ADATA.CPP` | — |

### `REDALERT/ADPCM.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/ADPCM.CPP` | — |

### `REDALERT/AIRCRAFT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/AIRCRAFT.CPP` | — |

### `REDALERT/ANIM.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/ANIM.CPP` | — |

### `REDALERT/AUDIO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/AUDIO.CPP` | — |

### `REDALERT/B64PIPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/B64PIPE.CPP` | — |

### `REDALERT/B64STRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/B64STRAW.CPP` | — |

### `REDALERT/BAR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BAR.CPP` | — |

### `REDALERT/BASE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BASE.CPP` | — |

### `REDALERT/BASE64.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BASE64.CPP` | — |

### `REDALERT/BBDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BBDATA.CPP` | — |

### `REDALERT/BDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BDATA.CPP` | — |

### `REDALERT/BENCH.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BENCH.CPP` | — |

### `REDALERT/BFIOFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BFIOFILE.CPP` | — |

### `REDALERT/BIGCHECK.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BIGCHECK.CPP` | — |

### `REDALERT/BLOWFISH.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BLOWFISH.CPP` | — |

### `REDALERT/BLOWPIPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BLOWPIPE.CPP` | — |

### `REDALERT/BLWSTRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BLWSTRAW.CPP` | — |

### `REDALERT/BMP8.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BMP8.CPP` | — |

### `REDALERT/BUFF.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BUFF.CPP` | — |

### `REDALERT/BUILDING.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BUILDING.CPP` | — |

### `REDALERT/BULLET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/BULLET.CPP` | — |

### `REDALERT/CARGO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CARGO.CPP` | — |

### `REDALERT/CARRY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CARRY.CPP` | — |

### `REDALERT/CCDDE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CCDDE.CPP` | — |

### `REDALERT/CCFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CCFILE.CPP` | — |

### `REDALERT/CCINI.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CCINI.CPP` | — |

### `REDALERT/CCMPATH.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CCMPATH.CPP` | — |

### `REDALERT/CCPTR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CCPTR.CPP` | — |

### `REDALERT/CCTEN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CCTEN.CPP` | — |

### `REDALERT/CDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CDATA.CPP` | — |

### `REDALERT/CDFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CDFILE.CPP` | — |

### `REDALERT/CELL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CELL.CPP` | — |

### `REDALERT/CHECKBOX.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CHECKBOX.CPP` | — |

### `REDALERT/CHEKLIST.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CHEKLIST.CPP` | — |

### `REDALERT/CLASS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CLASS.CPP` | — |

### `REDALERT/COLRLIST.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/COLRLIST.CPP` | — |

### `REDALERT/COMBAT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/COMBAT.CPP` | — |

### `REDALERT/COMBUF.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/COMBUF.CPP` | — |

### `REDALERT/COMINIT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/COMINIT.CPP` | — |

### `REDALERT/COMQUEUE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/COMQUEUE.CPP` | — |

### `REDALERT/CONFDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CONFDLG.CPP` | — |

### `REDALERT/CONNECT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CONNECT.CPP` | — |

### `REDALERT/CONQUER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CONQUER.CPP` | — |

### `REDALERT/CONST.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CONST.CPP` | — |

### `REDALERT/CONTROL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CONTROL.CPP` | — |

### `REDALERT/COORD.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/COORD.CPP` | — |

### `REDALERT/CRATE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CRATE.CPP` | — |

### `REDALERT/CRC.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| <badge>Ported</badge> | `REDALERT/CRC.CPP` | rts-formats::legacy_crc — full CRCEngine implementation |

### `REDALERT/CRCPIPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CRCPIPE.CPP` | — |

### `REDALERT/CRCSTRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CRCSTRAW.CPP` | — |

### `REDALERT/CREDITS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CREDITS.CPP` | — |

### `REDALERT/CREW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CREW.CPP` | — |

### `REDALERT/CSTRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/CSTRAW.CPP` | — |

### `REDALERT/DDE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DDE.CPP` | — |

### `REDALERT/DEBUG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DEBUG.CPP` | — |

### `REDALERT/DESCDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DESCDLG.CPP` | — |

### `REDALERT/DIAL8.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DIAL8.CPP` | — |

### `REDALERT/DIALOG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DIALOG.CPP` | — |

### `REDALERT/DIBFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DIBFILE.CPP` | — |

### `REDALERT/DIBUTIL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DIBUTIL.CPP` | — |

### `REDALERT/DISPLAY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DISPLAY.CPP` | — |

### `REDALERT/DLLInterface.cpp/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DLLInterface.cpp` | — |

### `REDALERT/DLLInterfaceEditor.cpp/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DLLInterfaceEditor.cpp` | — |

### `REDALERT/DOOR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DOOR.CPP` | — |

### `REDALERT/DPMI.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DPMI.CPP` | — |

### `REDALERT/DRIVE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DRIVE.CPP` | — |

### `REDALERT/DROP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DROP.CPP` | — |

### `REDALERT/DTABLE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DTABLE.CPP` | — |

### `REDALERT/DYNAVEC.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/DYNAVEC.CPP` | — |

### `REDALERT/EDIT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/EDIT.CPP` | — |

### `REDALERT/EGOS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/EGOS.CPP` | — |

### `REDALERT/ENDING.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/ENDING.CPP` | — |

### `REDALERT/EVENT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/EVENT.CPP` | — |

### `REDALERT/EXPAND.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/EXPAND.CPP` | — |

### `REDALERT/FACE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/FACE.CPP` | — |

### `REDALERT/FACING.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/FACING.CPP` | — |

### `REDALERT/FACTORY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/FACTORY.CPP` | — |

### `REDALERT/FIELD.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/FIELD.CPP` | — |

### `REDALERT/FINDPATH.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/FINDPATH.CPP` | — |

### `REDALERT/FIXED.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| <badge>Ported</badge> | `REDALERT/FIXED.CPP` | rts-core::fixed16 — constructors and ASCII conversion |

### `REDALERT/FLASHER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/FLASHER.CPP` | — |

### `REDALERT/FLY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/FLY.CPP` | — |

### `REDALERT/FOOT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/FOOT.CPP` | — |

### `REDALERT/FUSE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/FUSE.CPP` | — |

### `REDALERT/GADGET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/GADGET.CPP` | — |

### `REDALERT/GAMEDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/GAMEDLG.CPP` | — |

### `REDALERT/GAUGE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/GAUGE.CPP` | — |

### `REDALERT/GETCPU.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/GETCPU.CPP` | — |

### `REDALERT/GLOBALS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/GLOBALS.CPP` | — |

### `REDALERT/GOPTIONS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/GOPTIONS.CPP` | — |

### `REDALERT/GSCREEN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/GSCREEN.CPP` | — |

### `REDALERT/HDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/HDATA.CPP` | — |

### `REDALERT/HEAP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/HEAP.CPP` | — |

### `REDALERT/HELP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/HELP.CPP` | — |

### `REDALERT/HOUSE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/HOUSE.CPP` | — |

### `REDALERT/HSV.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/HSV.CPP` | — |

### `REDALERT/ICONLIST.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/ICONLIST.CPP` | — |

### `REDALERT/IDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/IDATA.CPP` | — |

### `REDALERT/INFANTRY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/INFANTRY.CPP` | — |

### `REDALERT/INI.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/INI.CPP` | — |

### `REDALERT/INIBIN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/INIBIN.CPP` | — |

### `REDALERT/INICODE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/INICODE.CPP` | — |

### `REDALERT/INIT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/INIT.CPP` | — |

### `REDALERT/INT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/INT.CPP` | — |

### `REDALERT/INTERNET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/INTERNET.CPP` | — |

### `REDALERT/INTERPAL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/INTERPAL.CPP` | — |

### `REDALERT/INTRO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/INTRO.CPP` | — |

### `REDALERT/IOMAP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/IOMAP.CPP` | — |

### `REDALERT/IOOBJ.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/IOOBJ.CPP` | — |

### `REDALERT/IPX.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/IPX.CPP` | — |

### `REDALERT/IPX95.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/IPX95.CPP` | — |

### `REDALERT/IPXADDR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/IPXADDR.CPP` | — |

### `REDALERT/IPXCONN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/IPXCONN.CPP` | — |

### `REDALERT/IPXGCONN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/IPXGCONN.CPP` | — |

### `REDALERT/IPXMGR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/IPXMGR.CPP` | — |

### `REDALERT/ITABLE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/ITABLE.CPP` | — |

### `REDALERT/JSHELL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/JSHELL.CPP` | — |

### `REDALERT/KEY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/KEY.CPP` | — |

### `REDALERT/KEYBOARD.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/KEYBOARD.CPP` | — |

### `REDALERT/LAYER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LAYER.CPP` | — |

### `REDALERT/LCW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LCW.CPP` | — |

### `REDALERT/LCWPIPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LCWPIPE.CPP` | — |

### `REDALERT/LCWSTRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LCWSTRAW.CPP` | — |

### `REDALERT/LCWUNCMP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LCWUNCMP.CPP` | — |

### `REDALERT/LINK.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LINK.CPP` | — |

### `REDALERT/LIST.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LIST.CPP` | — |

### `REDALERT/LOADDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LOADDLG.CPP` | — |

### `REDALERT/LOGIC.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LOGIC.CPP` | — |

### `REDALERT/LZO1X_C.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LZO1X_C.CPP` | — |

### `REDALERT/LZO1X_D.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LZO1X_D.CPP` | — |

### `REDALERT/LZOPIPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LZOPIPE.CPP` | — |

### `REDALERT/LZOSTRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LZOSTRAW.CPP` | — |

### `REDALERT/LZW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LZW.CPP` | — |

### `REDALERT/LZWOTRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LZWOTRAW.CPP` | — |

### `REDALERT/LZWPIPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LZWPIPE.CPP` | — |

### `REDALERT/LZWSTRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/LZWSTRAW.CPP` | — |

### `REDALERT/MAP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MAP.CPP` | — |

### `REDALERT/MAPEDDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MAPEDDLG.CPP` | — |

### `REDALERT/MAPEDIT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MAPEDIT.CPP` | — |

### `REDALERT/MAPEDPLC.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MAPEDPLC.CPP` | — |

### `REDALERT/MAPEDSEL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MAPEDSEL.CPP` | — |

### `REDALERT/MAPEDTM.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MAPEDTM.CPP` | — |

### `REDALERT/MAPSEL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MAPSEL.CPP` | — |

### `REDALERT/MCI.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MCI.CPP` | — |

### `REDALERT/MCIMOVIE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MCIMOVIE.CPP` | — |

### `REDALERT/MENUS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MENUS.CPP` | — |

### `REDALERT/MiscAsm.cpp/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MiscAsm.cpp` | — |

### `REDALERT/MISSION.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MISSION.CPP` | — |

### `REDALERT/MIXFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MIXFILE.CPP` | — |

### `REDALERT/MONOC.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MONOC.CPP` | — |

### `REDALERT/MOUSE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MOUSE.CPP` | — |

### `REDALERT/MP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MP.CPP` | — |

### `REDALERT/MPGSET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MPGSET.CPP` | — |

### `REDALERT/MPLAYER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MPLAYER.CPP` | — |

### `REDALERT/MPLIB.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MPLIB.CPP` | — |

### `REDALERT/MPMGRD.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MPMGRD.CPP` | — |

### `REDALERT/MPMGRW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MPMGRW.CPP` | — |

### `REDALERT/MPU.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MPU.CPP` | — |

### `REDALERT/MSGBOX.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MSGBOX.CPP` | — |

### `REDALERT/MSGLIST.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/MSGLIST.CPP` | — |

### `REDALERT/NETDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/NETDLG.CPP` | — |

### `REDALERT/NOSEQCON.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/NOSEQCON.CPP` | — |

### `REDALERT/NULLCONN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/NULLCONN.CPP` | — |

### `REDALERT/NULLDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/NULLDLG.CPP` | — |

### `REDALERT/NULLMGR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/NULLMGR.CPP` | — |

### `REDALERT/OBJECT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/OBJECT.CPP` | — |

### `REDALERT/ODATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/ODATA.CPP` | — |

### `REDALERT/OPTIONS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/OPTIONS.CPP` | — |

### `REDALERT/OVERLAY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/OVERLAY.CPP` | — |

### `REDALERT/PACKET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/PACKET.CPP` | — |

### `REDALERT/PALETTEC.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/PALETTEC.CPP` | — |

### `REDALERT/PIPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/PIPE.CPP` | — |

### `REDALERT/PK.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/PK.CPP` | — |

### `REDALERT/PKPIPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/PKPIPE.CPP` | — |

### `REDALERT/PKSTRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/PKSTRAW.CPP` | — |

### `REDALERT/POWER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/POWER.CPP` | — |

### `REDALERT/PROFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/PROFILE.CPP` | — |

### `REDALERT/QUEUE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/QUEUE.CPP` | — |

### `REDALERT/RADAR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/RADAR.CPP` | — |

### `REDALERT/RADIO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/RADIO.CPP` | — |

### `REDALERT/RAMFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/RAMFILE.CPP` | — |

### `REDALERT/RAND.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| <badge>Ported</badge> | `REDALERT/RAND.CPP` | rts-core::random — Sim_Random and Sim_IRandom |

### `REDALERT/RANDOM.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/RANDOM.CPP` | — |

### `REDALERT/RAWFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/RAWFILE.CPP` | — |

### `REDALERT/RAWOLAPI.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/RAWOLAPI.CPP` | — |

### `REDALERT/READLINE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/READLINE.CPP` | — |

### `REDALERT/RECT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/RECT.CPP` | — |

### `REDALERT/REINF.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/REINF.CPP` | — |

### `REDALERT/RGB.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/RGB.CPP` | — |

### `REDALERT/RNDSTRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/RNDSTRAW.CPP` | — |

### `REDALERT/ROTBMP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/ROTBMP.CPP` | — |

### `REDALERT/RULES.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/RULES.CPP` | — |

### `REDALERT/SAVELOAD.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SAVELOAD.CPP` | — |

### `REDALERT/SCENARIO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SCENARIO.CPP` | — |

### `REDALERT/SCORE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SCORE.CPP` | — |

### `REDALERT/SCROLL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SCROLL.CPP` | — |

### `REDALERT/SDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SDATA.CPP` | — |

### `REDALERT/SEDITDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SEDITDLG.CPP` | — |

### `REDALERT/SENDFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SENDFILE.CPP` | — |

### `REDALERT/SEQCONN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SEQCONN.CPP` | — |

### `REDALERT/SESSION.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SESSION.CPP` | — |

### `REDALERT/SHA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SHA.CPP` | — |

### `REDALERT/Shape.cpp/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/Shape.cpp` | — |

### `REDALERT/SHAPEBTN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SHAPEBTN.CPP` | — |

### `REDALERT/SHAPIPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SHAPIPE.CPP` | — |

### `REDALERT/SHASTRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SHASTRAW.CPP` | — |

### `REDALERT/SIDEBAR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SIDEBAR.CPP` | — |

### `REDALERT/SIDEBARGlyphx.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SIDEBARGlyphx.CPP` | — |

### `REDALERT/SLIDER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SLIDER.CPP` | — |

### `REDALERT/SMUDGE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SMUDGE.CPP` | — |

### `REDALERT/SOUNDDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SOUNDDLG.CPP` | — |

### `REDALERT/SPECIAL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SPECIAL.CPP` | — |

### `REDALERT/SPRITE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SPRITE.CPP` | — |

### `REDALERT/STARTUP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/STARTUP.CPP` | — |

### `REDALERT/STATBTN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/STATBTN.CPP` | — |

### `REDALERT/STATS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/STATS.CPP` | — |

### `REDALERT/STRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/STRAW.CPP` | — |

### `REDALERT/STUB.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/STUB.CPP` | — |

### `REDALERT/SUPER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SUPER.CPP` | — |

### `REDALERT/SURFACE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/SURFACE.CPP` | — |

### `REDALERT/TAB.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TAB.CPP` | — |

### `REDALERT/TACTION.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TACTION.CPP` | — |

### `REDALERT/TARGET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TARGET.CPP` | — |

### `REDALERT/TCPIP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TCPIP.CPP` | — |

### `REDALERT/TDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TDATA.CPP` | — |

### `REDALERT/TEAM.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TEAM.CPP` | — |

### `REDALERT/TEAMTYPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TEAMTYPE.CPP` | — |

### `REDALERT/TECHNO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TECHNO.CPP` | — |

### `REDALERT/TEMPLATE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TEMPLATE.CPP` | — |

### `REDALERT/TENMGR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TENMGR.CPP` | — |

### `REDALERT/TERRAIN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TERRAIN.CPP` | — |

### `REDALERT/TEVENT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TEVENT.CPP` | — |

### `REDALERT/TEXTBTN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TEXTBTN.CPP` | — |

### `REDALERT/THEME.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/THEME.CPP` | — |

### `REDALERT/TOGGLE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TOGGLE.CPP` | — |

### `REDALERT/TOOLTIP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TOOLTIP.CPP` | — |

### `REDALERT/TRACKER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TRACKER.CPP` | — |

### `REDALERT/TRIGGER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TRIGGER.CPP` | — |

### `REDALERT/TRIGTYPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TRIGTYPE.CPP` | — |

### `REDALERT/TURRET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TURRET.CPP` | — |

### `REDALERT/TXTLABEL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/TXTLABEL.CPP` | — |

### `REDALERT/UDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/UDATA.CPP` | — |

### `REDALERT/UDPADDR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/UDPADDR.CPP` | — |

### `REDALERT/UNIT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/UNIT.CPP` | — |

### `REDALERT/UTRACKER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/UTRACKER.CPP` | — |

### `REDALERT/VDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/VDATA.CPP` | — |

### `REDALERT/VECTOR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/VECTOR.CPP` | — |

### `REDALERT/VERSION.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/VERSION.CPP` | — |

### `REDALERT/VESSEL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/VESSEL.CPP` | — |

### `REDALERT/VISUDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/VISUDLG.CPP` | — |

### `REDALERT/VORTEX.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/VORTEX.CPP` | — |

### `REDALERT/W95TRACE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/W95TRACE.CPP` | — |

### `REDALERT/WARHEAD.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WARHEAD.CPP` | — |

### `REDALERT/WEAPON.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WEAPON.CPP` | — |

### `REDALERT/WIN32LIB/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WIN32LIB/_DIPTABL.CPP` | — |
| Not started | `REDALERT/WIN32LIB/ALLOC.CPP` | — |
| Not started | `REDALERT/WIN32LIB/BUFFER.CPP` | — |
| Not started | `REDALERT/WIN32LIB/BUFFGLBL.CPP` | — |
| Not started | `REDALERT/WIN32LIB/DDRAW.CPP` | — |
| Not started | `REDALERT/WIN32LIB/DELAY.CPP` | — |
| Not started | `REDALERT/WIN32LIB/DIPTHONG.CPP` | — |
| Not started | `REDALERT/WIN32LIB/DrawMisc.cpp` | — |
| Not started | `REDALERT/WIN32LIB/DRAWRECT.CPP` | — |
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
| Not started | `REDALERT/WIN32LIB/WINHIDE.CPP` | — |
| Not started | `REDALERT/WIN32LIB/WRITEPCX.CPP` | — |

### `REDALERT/WINSTUB.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WINSTUB.CPP` | — |

### `REDALERT/WOL_CGAM.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WOL_CGAM.CPP` | — |

### `REDALERT/WOL_CHAT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WOL_CHAT.CPP` | — |

### `REDALERT/WOL_DNLD.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WOL_DNLD.CPP` | — |

### `REDALERT/WOL_GSUP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WOL_GSUP.CPP` | — |

### `REDALERT/WOL_LOGN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WOL_LOGN.CPP` | — |

### `REDALERT/WOL_MAIN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WOL_MAIN.CPP` | — |

### `REDALERT/WOL_OPT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WOL_OPT.CPP` | — |

### `REDALERT/WOLAPIOB.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WOLAPIOB.CPP` | — |

### `REDALERT/WOLEDIT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WOLEDIT.CPP` | — |

### `REDALERT/WOLSTRNG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WOLSTRNG.CPP` | — |

### `REDALERT/WSPIPX.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WSPIPX.CPP` | — |

### `REDALERT/WSPROTO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WSPROTO.CPP` | — |

### `REDALERT/WSPUDP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/WSPUDP.CPP` | — |

### `REDALERT/XPIPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/XPIPE.CPP` | — |

### `REDALERT/XSTRAW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `REDALERT/XSTRAW.CPP` | — |

### `TIBERIANDAWN/AADATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/AADATA.CPP` | — |

### `TIBERIANDAWN/ABSTRACT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/ABSTRACT.CPP` | — |

### `TIBERIANDAWN/ADATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/ADATA.CPP` | — |

### `TIBERIANDAWN/AIRCRAFT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/AIRCRAFT.CPP` | — |

### `TIBERIANDAWN/ALLOC.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/ALLOC.CPP` | — |

### `TIBERIANDAWN/ANIM.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/ANIM.CPP` | — |

### `TIBERIANDAWN/AUDIO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/AUDIO.CPP` | — |

### `TIBERIANDAWN/BASE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/BASE.CPP` | — |

### `TIBERIANDAWN/BBDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/BBDATA.CPP` | — |

### `TIBERIANDAWN/BDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/BDATA.CPP` | — |

### `TIBERIANDAWN/BUILDING.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/BUILDING.CPP` | — |

### `TIBERIANDAWN/BULLET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/BULLET.CPP` | — |

### `TIBERIANDAWN/CARGO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CARGO.CPP` | — |

### `TIBERIANDAWN/CCDDE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CCDDE.CPP` | — |

### `TIBERIANDAWN/CCFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CCFILE.CPP` | — |

### `TIBERIANDAWN/CDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CDATA.CPP` | — |

### `TIBERIANDAWN/CDFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CDFILE.CPP` | — |

### `TIBERIANDAWN/CELL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CELL.CPP` | — |

### `TIBERIANDAWN/CHECKBOX.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CHECKBOX.CPP` | — |

### `TIBERIANDAWN/CHEKLIST.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CHEKLIST.CPP` | — |

### `TIBERIANDAWN/COLRLIST.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/COLRLIST.CPP` | — |

### `TIBERIANDAWN/COMBAT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/COMBAT.CPP` | — |

### `TIBERIANDAWN/COMBUF.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/COMBUF.CPP` | — |

### `TIBERIANDAWN/COMQUEUE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/COMQUEUE.CPP` | — |

### `TIBERIANDAWN/CONFDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CONFDLG.CPP` | — |

### `TIBERIANDAWN/CONNECT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CONNECT.CPP` | — |

### `TIBERIANDAWN/CONQUER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CONQUER.CPP` | — |

### `TIBERIANDAWN/CONST.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CONST.CPP` | — |

### `TIBERIANDAWN/CONTROL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CONTROL.CPP` | — |

### `TIBERIANDAWN/COORD.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/COORD.CPP` | — |

### `TIBERIANDAWN/CREDITS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CREDITS.CPP` | — |

### `TIBERIANDAWN/CREW.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/CREW.CPP` | — |

### `TIBERIANDAWN/DDE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/DDE.CPP` | — |

### `TIBERIANDAWN/DEBUG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/DEBUG.CPP` | — |

### `TIBERIANDAWN/DESCDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/DESCDLG.CPP` | — |

### `TIBERIANDAWN/DIAL8.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/DIAL8.CPP` | — |

### `TIBERIANDAWN/DIALOG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/DIALOG.CPP` | — |

### `TIBERIANDAWN/DISPLAY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/DISPLAY.CPP` | — |

### `TIBERIANDAWN/DLLInterface.cpp/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/DLLInterface.cpp` | — |

### `TIBERIANDAWN/DLLInterfaceEditor.cpp/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/DLLInterfaceEditor.cpp` | — |

### `TIBERIANDAWN/DOOR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/DOOR.CPP` | — |

### `TIBERIANDAWN/DPMI.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/DPMI.CPP` | — |

### `TIBERIANDAWN/DRIVE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/DRIVE.CPP` | — |

### `TIBERIANDAWN/EDIT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/EDIT.CPP` | — |

### `TIBERIANDAWN/ENDING.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/ENDING.CPP` | — |

### `TIBERIANDAWN/EVENT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/EVENT.CPP` | — |

### `TIBERIANDAWN/EXPAND.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/EXPAND.CPP` | — |

### `TIBERIANDAWN/FACING.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/FACING.CPP` | — |

### `TIBERIANDAWN/FACTORY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/FACTORY.CPP` | — |

### `TIBERIANDAWN/FIELD.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/FIELD.CPP` | — |

### `TIBERIANDAWN/FINDPATH.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/FINDPATH.CPP` | — |

### `TIBERIANDAWN/FLASHER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/FLASHER.CPP` | — |

### `TIBERIANDAWN/FLY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/FLY.CPP` | — |

### `TIBERIANDAWN/FOOT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/FOOT.CPP` | — |

### `TIBERIANDAWN/FUSE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/FUSE.CPP` | — |

### `TIBERIANDAWN/GADGET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/GADGET.CPP` | — |

### `TIBERIANDAWN/GAMEDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/GAMEDLG.CPP` | — |

### `TIBERIANDAWN/GAUGE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/GAUGE.CPP` | — |

### `TIBERIANDAWN/GLOBALS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/GLOBALS.CPP` | — |

### `TIBERIANDAWN/GOPTIONS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/GOPTIONS.CPP` | — |

### `TIBERIANDAWN/GSCREEN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/GSCREEN.CPP` | — |

### `TIBERIANDAWN/HDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/HDATA.CPP` | — |

### `TIBERIANDAWN/HEAP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/HEAP.CPP` | — |

### `TIBERIANDAWN/HELP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/HELP.CPP` | — |

### `TIBERIANDAWN/HOUSE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/HOUSE.CPP` | — |

### `TIBERIANDAWN/IDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/IDATA.CPP` | — |

### `TIBERIANDAWN/INFANTRY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/INFANTRY.CPP` | — |

### `TIBERIANDAWN/INI.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/INI.CPP` | — |

### `TIBERIANDAWN/INIT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/INIT.CPP` | — |

### `TIBERIANDAWN/INTERNET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/INTERNET.CPP` | — |

### `TIBERIANDAWN/INTERPAL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/INTERPAL.CPP` | — |

### `TIBERIANDAWN/INTRO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/INTRO.CPP` | — |

### `TIBERIANDAWN/IOMAP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/IOMAP.CPP` | — |

### `TIBERIANDAWN/IOOBJ.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/IOOBJ.CPP` | — |

### `TIBERIANDAWN/IPX.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/IPX.CPP` | — |

### `TIBERIANDAWN/IPX95.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/IPX95.CPP` | — |

### `TIBERIANDAWN/IPXADDR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/IPXADDR.CPP` | — |

### `TIBERIANDAWN/IPXCONN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/IPXCONN.CPP` | — |

### `TIBERIANDAWN/IPXGCONN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/IPXGCONN.CPP` | — |

### `TIBERIANDAWN/IPXMGR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/IPXMGR.CPP` | — |

### `TIBERIANDAWN/JSHELL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/JSHELL.CPP` | — |

### `TIBERIANDAWN/KEYFRAME.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/KEYFRAME.CPP` | — |

### `TIBERIANDAWN/LAYER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/LAYER.CPP` | — |

### `TIBERIANDAWN/LINK.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/LINK.CPP` | — |

### `TIBERIANDAWN/LIST.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/LIST.CPP` | — |

### `TIBERIANDAWN/LOADDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/LOADDLG.CPP` | — |

### `TIBERIANDAWN/LOGIC.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/LOGIC.CPP` | — |

### `TIBERIANDAWN/MAP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MAP.CPP` | — |

### `TIBERIANDAWN/MAPEDDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MAPEDDLG.CPP` | — |

### `TIBERIANDAWN/MAPEDIT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MAPEDIT.CPP` | — |

### `TIBERIANDAWN/MAPEDPLC.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MAPEDPLC.CPP` | — |

### `TIBERIANDAWN/MAPEDSEL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MAPEDSEL.CPP` | — |

### `TIBERIANDAWN/MAPEDTM.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MAPEDTM.CPP` | — |

### `TIBERIANDAWN/MAPSEL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MAPSEL.CPP` | — |

### `TIBERIANDAWN/MENUS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MENUS.CPP` | — |

### `TIBERIANDAWN/MiscAsm.cpp/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MiscAsm.cpp` | — |

### `TIBERIANDAWN/MISSION.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MISSION.CPP` | — |

### `TIBERIANDAWN/MIXFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MIXFILE.CPP` | — |

### `TIBERIANDAWN/MONOC.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MONOC.CPP` | — |

### `TIBERIANDAWN/MOUSE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MOUSE.CPP` | — |

### `TIBERIANDAWN/MPLAYER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MPLAYER.CPP` | — |

### `TIBERIANDAWN/MSGBOX.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MSGBOX.CPP` | — |

### `TIBERIANDAWN/MSGLIST.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/MSGLIST.CPP` | — |

### `TIBERIANDAWN/NETDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/NETDLG.CPP` | — |

### `TIBERIANDAWN/NOSEQCON.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/NOSEQCON.CPP` | — |

### `TIBERIANDAWN/NULLCONN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/NULLCONN.CPP` | — |

### `TIBERIANDAWN/NULLDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/NULLDLG.CPP` | — |

### `TIBERIANDAWN/NULLMGR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/NULLMGR.CPP` | — |

### `TIBERIANDAWN/OBJECT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/OBJECT.CPP` | — |

### `TIBERIANDAWN/ODATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/ODATA.CPP` | — |

### `TIBERIANDAWN/OPTIONS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/OPTIONS.CPP` | — |

### `TIBERIANDAWN/OVERLAY.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/OVERLAY.CPP` | — |

### `TIBERIANDAWN/PACKET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/PACKET.CPP` | — |

### `TIBERIANDAWN/POWER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/POWER.CPP` | — |

### `TIBERIANDAWN/PROFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/PROFILE.CPP` | — |

### `TIBERIANDAWN/QUEUE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/QUEUE.CPP` | — |

### `TIBERIANDAWN/RADAR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/RADAR.CPP` | — |

### `TIBERIANDAWN/RADIO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/RADIO.CPP` | — |

### `TIBERIANDAWN/RAND.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| <badge>Ported</badge> | `TIBERIANDAWN/RAND.CPP` | rts-core::random — Sim_Random and Sim_IRandom |

### `TIBERIANDAWN/RAWFILE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/RAWFILE.CPP` | — |

### `TIBERIANDAWN/REINF.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/REINF.CPP` | — |

### `TIBERIANDAWN/RULES.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/RULES.CPP` | — |

### `TIBERIANDAWN/SAVELOAD.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SAVELOAD.CPP` | — |

### `TIBERIANDAWN/SCENARIO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SCENARIO.CPP` | — |

### `TIBERIANDAWN/SCORE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SCORE.CPP` | — |

### `TIBERIANDAWN/SCROLL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SCROLL.CPP` | — |

### `TIBERIANDAWN/SDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SDATA.CPP` | — |

### `TIBERIANDAWN/SEQCONN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SEQCONN.CPP` | — |

### `TIBERIANDAWN/Shape.cpp/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/Shape.cpp` | — |

### `TIBERIANDAWN/SHAPEBTN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SHAPEBTN.CPP` | — |

### `TIBERIANDAWN/SIDEBAR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SIDEBAR.CPP` | — |

### `TIBERIANDAWN/SIDEBARGlyphx.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SIDEBARGlyphx.CPP` | — |

### `TIBERIANDAWN/SLIDER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SLIDER.CPP` | — |

### `TIBERIANDAWN/SMUDGE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SMUDGE.CPP` | — |

### `TIBERIANDAWN/SOUNDDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SOUNDDLG.CPP` | — |

### `TIBERIANDAWN/SPECIAL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SPECIAL.CPP` | — |

### `TIBERIANDAWN/STARTUP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/STARTUP.CPP` | — |

### `TIBERIANDAWN/STATS.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/STATS.CPP` | — |

### `TIBERIANDAWN/SUPER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/SUPER.CPP` | — |

### `TIBERIANDAWN/TAB.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TAB.CPP` | — |

### `TIBERIANDAWN/TARCOM.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TARCOM.CPP` | — |

### `TIBERIANDAWN/TARGET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TARGET.CPP` | — |

### `TIBERIANDAWN/TCPIP.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TCPIP.CPP` | — |

### `TIBERIANDAWN/TDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TDATA.CPP` | — |

### `TIBERIANDAWN/TEAM.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TEAM.CPP` | — |

### `TIBERIANDAWN/TEAMTYPE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TEAMTYPE.CPP` | — |

### `TIBERIANDAWN/TECHNO.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TECHNO.CPP` | — |

### `TIBERIANDAWN/TEMPLATE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TEMPLATE.CPP` | — |

### `TIBERIANDAWN/TERRAIN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TERRAIN.CPP` | — |

### `TIBERIANDAWN/TEXTBTN.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TEXTBTN.CPP` | — |

### `TIBERIANDAWN/THEME.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/THEME.CPP` | — |

### `TIBERIANDAWN/TOGGLE.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TOGGLE.CPP` | — |

### `TIBERIANDAWN/TRIGGER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TRIGGER.CPP` | — |

### `TIBERIANDAWN/TURRET.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TURRET.CPP` | — |

### `TIBERIANDAWN/TXTLABEL.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/TXTLABEL.CPP` | — |

### `TIBERIANDAWN/UDATA.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/UDATA.CPP` | — |

### `TIBERIANDAWN/UNIT.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/UNIT.CPP` | — |

### `TIBERIANDAWN/UTRACKER.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/UTRACKER.CPP` | — |

### `TIBERIANDAWN/VECTOR.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/VECTOR.CPP` | — |

### `TIBERIANDAWN/VISUDLG.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/VISUDLG.CPP` | — |

### `TIBERIANDAWN/WIN32LIB/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/WIN32LIB/_DIPTABL.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/ALLOC.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/BUFFER.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/BUFFGLBL.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/DDRAW.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/DIPTHONG.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/DrawMisc.cpp` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/DRAWRECT.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/FONT.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/GBUFFER.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/ICONSET.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/IFF.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/IRANDOM.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/LOAD.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/LOADFONT.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/LOADPAL.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/MEM.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/MORPHPAL.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/NEWDEL.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/PALETTE.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/REGIONSZ.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/SET_FONT.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/TIMER.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/TIMERDWN.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/TIMERINI.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/WINDOWS.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/WINHIDE.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/WRITEPCX.CPP` | — |
| Not started | `TIBERIANDAWN/WIN32LIB/WSA.CPP` | — |

### `TIBERIANDAWN/WINSTUB.CPP/`

| Status | Upstream C++ file | Rust implementation / notes |
|---|---|---|
| Not started | `TIBERIANDAWN/WINSTUB.CPP` | — |

## Porting rule

Each translation unit must be inspected with its headers and callers. Preserve source-defined integer widths, signedness, overflow, byte order, state mutation order, and variant differences. Platform-specific code is reimplemented behind portable interfaces; it is not replaced by an empty module. All format parsers must reject malformed offsets and lengths safely.
