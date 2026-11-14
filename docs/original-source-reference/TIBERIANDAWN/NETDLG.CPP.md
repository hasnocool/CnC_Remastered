# TIBERIANDAWN NETDLG.CPP — source guide

> **Audit status: partially reviewed.** Based on the pinned source translation unit, direct includes, and original function index. Exact header/member inventory, complete function-body explanations, call graphs, and Rust parity remain pending.

- Source: [TIBERIANDAWN/NETDLG.CPP](https://github.com/electronicarts/CnC_Remastered_Collection/blob/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae/TIBERIANDAWN/NETDLG.CPP)
- Pinned upstream commit: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Source size: 211680 characters.

## Plain-English purpose

Multiplayer networking and lobby orchestration: initialize/shutdown, connection lifecycle, discovery, join/create flows, packet processing, join requests, reconnect. Red Alert adds WOL/TCP-IP protocol dependencies and additional unjoin flows.

## Direct includes

- function.h
- time.h
- tcpip.h
- ccdde.h

## Original function index

- Clear_Game_List — Clears the game-name listbox & 'Games' Vector
- Clear_Player_List — Clears the player-name listbox & Vector
- Destroy_Connection — destroys the given connection
- Get_Join_Responses — sends queries for the Join Dialog
- Get_NewGame_Responses — processes packets for New Game dialog
- Init_Network — initializes network stuff
- Net_Join_Dialog — lets user join an existing game, or start a new one
- Net_New_Dialog — lets user start a new game
- Process_Global_Packet — responds to remote queries
- Remote_Connect — handles connecting this user to others
- Request_To_Join — Sends a JOIN request packet to game owner
- Send_Join_Queries — sends queries for the Join Dialog
- Shutdown_Network — shuts down network stuff
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
