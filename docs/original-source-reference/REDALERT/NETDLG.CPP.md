# REDALERT NETDLG.CPP — source guide

> **Audit status: partially reviewed.** Based on the pinned source translation unit, direct includes, and original function index. Exact header/member inventory, complete function-body explanations, call graphs, and Rust parity remain pending.

- Source: [REDALERT/NETDLG.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/REDALERT/NETDLG.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 337753 characters.

## Plain-English purpose

Multiplayer networking and lobby orchestration: initialize/shutdown, connection lifecycle, discovery, join/create flows, packet processing, join requests, reconnect. Red Alert adds WOL/TCP-IP protocol dependencies and additional unjoin flows.

## Direct includes

- function.h
- WSProto.h
- tcpip.h
- ccdde.h
- fakesock.h
- time.h
- WolStrng.h
- WolapiOb.h
- COORDA.h

## Original function index

- Init_Network — initializes network stuff
- Shutdown_Network — shuts down network stuff
- Process_Global_Packet — responds to remote queries
- Destroy_Connection — destroys the given connection
- Remote_Connect — handles connecting this user to others
- Net_Join_Dialog — lets user join an existing game, or start a new one
- Request_To_Join — Sends a JOIN request packet to game owner
- Unjoin_Game — Cancels joining a game
- Send_Join_Queries — sends queries for the Join Dialog
- Get_Join_Responses — sends queries for the Join Dialog
- Net_New_Dialog — lets user start a new game
- Get_NewGame_Responses — processes packets for New Game dialog
- Compute_Name_CRC — computes CRC from char string
- Net_Reconnect_Dialog — Draws/updates the network reconnect dialog

## Integration and porting concerns

Preserve packet framing/order, timeouts, reconnect behavior, and clean shutdown. Red Alert has WOL-specific protocol dependencies and is not a drop-in match for TD. A modern Rust port should avoid blocking UI/network operations and use bounded async I/O with deterministic message dispatch.

## Outstanding audit
- [ ] Paired header, exact declarations, members, globals, constants, and signatures.
- [ ] Every function body, branch, return contract, side effect, and failure path.
- [ ] Cross-file callers and native/protocol dependencies.
- [ ] TD/RA behavior comparison.
- [ ] Rust paths and tests with explicit parity status.
