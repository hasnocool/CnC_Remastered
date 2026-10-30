//! Red Alert MPATH integration from REDALERT/CCMPATH.CPP.
//!
//! In the checked-in RedAlert.vcxproj WIN32 is defined but MPATH is not, so
//! the compiled default is Init_MPATH() => success and the other functions
//! are no-ops. The optional MPATH code is also represented here, but its
//! blocking Connect_MPATH loop is exposed one frame at a time.

pub const MPATH_RESEND_TICKS: u32 = 240;
pub const MPATH_WAIT_TEXT_HEIGHT: i32 = 7;
pub const MPATH_DIALOG_MARGIN: i32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MPathBuildFeatures {
    pub mpath: bool,
    pub win32: bool,
}

impl MPathBuildFeatures {
    pub const fn checked_in_win32_project() -> Self {
        Self { mpath: false, win32: true }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MPathInitOutcome {
    DisabledSuccess,
    Connecting,
    EmergencyExit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MPathCommand {
    Ping,
    Other(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MPathPingPacket {
    pub command: MPathCommand,
    pub player_name: String,
    pub house_id: u8,
    pub color_id: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MPathDestination {
    pub unicast: bool,
    pub address: u32,
}

impl MPathDestination {
    pub const fn broadcast() -> Self { Self { unicast: false, address: 0 } }
    pub const fn unicast(address: u32) -> Self { Self { unicast: true, address } }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MPathReceivedPacket {
    pub packet: MPathPingPacket,
    pub address: u32,
    pub packet_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MPathPlayer {
    pub name: String,
    pub address: Option<u32>,
    pub house_id: u8,
    pub color_id: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MPathDialogRect {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
}

pub trait MPathInitHost {
    fn allocate_packet_buffer(&mut self, bytes: usize) -> bool;
    fn read_multiplayer_settings(&mut self);
    fn read_game_options(&mut self) -> bool;
    fn show_settings_load_error(&mut self);
    fn emergency_exit(&mut self, exit_code: i32);
    fn flush_all_packets(&mut self);
    fn configured_remote_count(&mut self) -> usize;
    fn local_player(&self) -> MPathPlayer;
    fn set_session_num_players(&mut self, count: usize);
    fn set_multiplayer_communications_protocol(&mut self);
}

pub trait MPathConnectHost: MPathInitHost {
    fn send_global_message(&mut self, packet: &MPathPingPacket, destination: MPathDestination);
    fn service_mpath(&mut self);
    fn receive_global_message(&mut self) -> Option<MPathReceivedPacket>;
    fn surfaces_restored(&mut self) -> bool;
    fn clear_surfaces_restored(&mut self);
    fn seen_buffer_width(&self) -> i32;
    fn resolution_factor(&self) -> i32;
    fn string_pixel_width(&self, text: &str) -> i32;
    fn hide_mouse(&mut self);
    fn draw_wait_dialog(&mut self, rect: MPathDialogRect, waiting_text: &str, escape_text: &str);
    fn show_mouse(&mut self);
    fn keyboard_escape_pressed(&mut self);
    fn timer_expired(&mut self);
    fn set_timer(&mut self, ticks: u32);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MPathConnectState {
    NotStarted,
    Waiting,
    Complete,
    EmergencyExit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MPathConnectStep {
    pub state: MPathConnectState,
    pub expected_players: usize,
    pub players_found: usize,
    pub players: Vec<MPathPlayer>,
    pub displayed_dialog: Option<MPathDialogRect>,
    pub packet_received: bool,
    pub duplicate_ignored: bool,
    pub ping_rebroadcast: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MPathConnect {
    pub state: MPathConnectState,
    pub expected_remote_players: usize,
    pub found_remote_players: usize,
    pub players: Vec<MPathPlayer>,
    pub local_ping: MPathPingPacket,
    pub display: bool,
}

impl MPathConnect {
    pub fn new(remote_player_count: usize, local: MPathPlayer) -> Self {
        let local_ping = MPathPingPacket {
            command: MPathCommand::Ping,
            player_name: local.name.clone(),
            house_id: local.house_id,
            color_id: local.color_id,
        };
        Self {
            state: if remote_player_count == 0 { MPathConnectState::Complete } else { MPathConnectState::NotStarted },
            expected_remote_players: remote_player_count,
            found_remote_players: 0,
            players: vec![local],
            local_ping,
            display: true,
        }
    }

    pub fn start(&mut self, host: &mut impl MPathConnectHost) {
        if self.state != MPathConnectState::NotStarted { return; }
        host.set_session_num_players(self.expected_remote_players + 1);
        host.send_global_message(&self.local_ping, MPathDestination::broadcast());
        host.set_timer(MPATH_RESEND_TICKS);
        self.state = MPathConnectState::Waiting;
    }

    fn waiting_dialog_rect(host: &impl MPathConnectHost) -> MPathDialogRect {
        let waiting = "Waiting for connections";
        let escape = "Press ESC to exit";
        let factor = host.resolution_factor();
        let width = host.string_pixel_width(waiting)
            .max(host.string_pixel_width(escape))
            + MPATH_DIALOG_MARGIN * 2;
        let height = MPATH_WAIT_TEXT_HEIGHT * 2 + MPATH_DIALOG_MARGIN * 7;
        MPathDialogRect {
            x: (160 - width / 2) * factor,
            y: (100 - height / 2) * factor,
            width: width * factor,
            height: height * factor,
        }
    }

    /// One nonblocking iteration of Connect_MPATH. It services the transport
    /// once, processes at most one packet, handles ESC and resends the ping
    /// when the 240-tick timer has expired.
    pub fn step(&mut self, host: &mut impl MPathConnectHost) -> MPathConnectStep {
        if self.state == MPathConnectState::NotStarted {
            self.start(host);
        }
        if self.state != MPathConnectState::Waiting {
            return self.snapshot(None, false, false, false);
        }

        if host.surfaces_restored() {
            host.clear_surfaces_restored();
            self.display = true;
        }

        let mut displayed_dialog = None;
        if self.display {
            let rect = Self::waiting_dialog_rect(host);
            host.hide_mouse();
            host.draw_wait_dialog(rect, "Waiting for connections", "Press ESC to exit");
            host.show_mouse();
            displayed_dialog = Some(rect);
            self.display = false;
        }

        host.service_mpath();

        let mut packet_received = false;
        let mut duplicate_ignored = false;
        if let Some(received) = host.receive_global_message() {
            if received.packet.command == MPathCommand::Ping {
                packet_received = true;
                let found = self.players.iter().skip(1).any(|player| player.address == Some(received.address));
                if found {
                    duplicate_ignored = true;
                } else {
                    self.players.push(MPathPlayer {
                        name: received.packet.player_name.clone(),
                        address: Some(received.address),
                        house_id: received.packet.house_id,
                        color_id: received.packet.color_id,
                    });
                    self.found_remote_players += 1;
                    host.send_global_message(&self.local_ping, MPathDestination::unicast(received.address));
                    if self.found_remote_players >= self.expected_remote_players {
                        self.state = MPathConnectState::Complete;
                        host.set_session_num_players(self.players.len());
                        host.set_multiplayer_communications_protocol();
                    }
                }
            }
        }

        if self.state == MPathConnectState::Waiting && host.keyboard_escape_pressed() {
            host.emergency_exit(0);
            self.state = MPathConnectState::EmergencyExit;
        }

        let mut ping_rebroadcast = false;
        if self.state == MPathConnectState::Waiting && host.timer_expired() {
            host.send_global_message(&self.local_ping, MPathDestination::broadcast());
            host.set_timer(MPATH_RESEND_TICKS);
            ping_rebroadcast = true;
        }

        self.snapshot(displayed_dialog, packet_received, duplicate_ignored, ping_rebroadcast)
    }

    fn snapshot(
        &self,
        displayed_dialog: Option<MPathDialogRect>,
        packet_received: bool,
        duplicate_ignored: bool,
        ping_rebroadcast: bool,
    ) -> MPathConnectStep {
        MPathConnectStep {
            state: self.state,
            expected_players: self.expected_remote_players + 1,
            players_found: self.found_remote_players + 1,
            players: self.players.clone(),
            displayed_dialog,
            packet_received,
            duplicate_ignored,
            ping_rebroadcast,
        }
    }
}

pub fn init_mpath(
    features: MPathBuildFeatures,
    packet_buffer_size: usize,
    host: &mut impl MPathInitHost,
) -> Result<Option<MPathConnect>, MPathInitError> {
    if !features.mpath {
        return Ok(None);
    }
    if !host.allocate_packet_buffer(packet_buffer_size) {
        return Err(MPathInitError::PacketBufferAllocationFailed);
    }

    host.read_multiplayer_settings();
    if !host.read_game_options() {
        host.show_settings_load_error();
        host.emergency_exit(0);
        return Err(MPathInitError::EmergencyExit);
    }
    host.flush_all_packets();

    let remote_count = host.configured_remote_count();
    let local = host.local_player();
    Ok(Some(MPathConnect::new(remote_count, local)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MPathInitError {
    PacketBufferAllocationFailed,
    EmergencyExit,
}

pub fn shutdown_mpath(
    features: MPathBuildFeatures,
    wait_ticks: &mut u32,
    packet_buffer_allocated: &mut bool,
    mpath_allocated: &mut bool,
) -> bool {
    if !features.mpath {
        return true;
    }
    if *wait_ticks < 60 {
        *wait_ticks += 1;
        return false;
    }
    *packet_buffer_allocated = false;
    *mpath_allocated = false;
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MPathDisconnectMessage {
    ConnectionLost,
    LeftGame,
    JustYouAndMe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MPathHouse {
    pub house_id: u8,
    pub is_human: bool,
    pub ini_name: String,
    pub remap_color: u8,
    pub iq: u8,
}

pub trait MPathDisconnectHost {
    fn house(&self, house_id: u8) -> Option<MPathHouse>;
    fn connection_name(&self, connection_id: i32) -> String;
    fn max_iq(&self) -> u8;
    fn computer_name(&self) -> String;
    fn add_message(
        &mut self,
        message: MPathDisconnectMessage,
        player_name: Option<&str>,
        remap_color: u8,
        duration_ticks: u32,
    );
    fn flag_map_to_redraw(&mut self, full_redraw: bool);
    fn player_count(&self) -> usize;
    fn player_name(&self, index: usize) -> Option<String>;
    fn remove_player(&mut self, index: usize);
    fn delete_connection(&mut self, connection_id: i32);
    fn update_house(&mut self, house: MPathHouse);
    fn set_session_player_count(&mut self, count: usize);
}

/// Full Destroy_MPATH_Connection state transfer. MPATH-disabled builds
/// compile this body out; invalid/non-human houses are ignored.
pub fn destroy_mpath_connection(
    features: MPathBuildFeatures,
    connection_id: i32,
    error: i32,
    message_delay_ticks: u32,
    session_player_count: usize,
    host: &mut impl MPathDisconnectHost,
) -> bool {
    if !features.mpath { return false; }

    let Ok(house_id) = u8::try_from(connection_id) else { return false; };
    let Some(mut house) = host.house(house_id) else { return false; };
    if !house.is_human { return false; }

    let message = match error {
        1 => Some(MPathDisconnectMessage::ConnectionLost),
        0 => Some(MPathDisconnectMessage::LeftGame),
        _ => None,
    };
    if let Some(message) = message {
        let connection_name = host.connection_name(connection_id);
        host.add_message(message, Some(&connection_name), house.remap_color, message_delay_ticks);
        host.flag_map_to_redraw(false);
    }

    for index in 0..host.player_count() {
        if host.player_name(index).is_some_and(|name| name.eq_ignore_ascii_case(&house.ini_name)) {
            host.remove_player(index);
            break;
        }
    }

    host.delete_connection(connection_id);
    house.is_human = false;
    house.iq = host.max_iq();
    house.ini_name = host.computer_name();
    host.update_house(house);

    let remaining_players = session_player_count.saturating_sub(1);
    host.set_session_player_count(remaining_players);
    if remaining_players == 1 {
        host.add_message(MPathDisconnectMessage::JustYouAndMe, None, 0, message_delay_ticks);
        host.flag_map_to_redraw(false);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct MockHost {
        packet_allocated: bool,
        settings_read: usize,
        options_ok: bool,
        error_shown: usize,
        emergency_exit_codes: Vec<i32>,
        flush_calls: usize,
        remote_count: usize,
        local: Option<MPathPlayer>,
        session_player_count: usize,
        protocol_set: usize,
        sends: Vec<(MPathPingPacket, MPathDestination)>,
        service_calls: usize,
        incoming: Option<MPathReceivedPacket>,
        restored: bool,
        clears: usize,
        width: i32,
        factor: i32,
        width_per_char: i32,
        draw_calls: Vec<MPathDialogRect>,
        mouse_calls: Vec<&'static str>,
        escape: bool,
        timer: bool,
        timers: Vec<u32>,
        houses: BTreeMap<u8, MPathHouse>,
        connection_names: BTreeMap<i32, String>,
        removed_players: Vec<usize>,
        player_names: Vec<String>,
        deleted_connections: Vec<i32>,
        updated_houses: Vec<MPathHouse>,
        messages: Vec<(MPathDisconnectMessage, Option<String>, u8, u32)>,
        redraws: Vec<bool>,
        max_iq_value: u8,
        computer_name_value: String,
    }

    impl MPathInitHost for MockHost {
        fn allocate_packet_buffer(&mut self, _bytes: usize) -> bool {
            self.packet_allocated = true;
            true
        }
        fn read_multiplayer_settings(&mut self) { self.settings_read += 1; }
        fn read_game_options(&mut self) -> bool { self.options_ok }
        fn show_settings_load_error(&mut self) { self.error_shown += 1; }
        fn emergency_exit(&mut self, code: i32) { self.emergency_exit_codes.push(code); }
        fn flush_all_packets(&mut self) { self.flush_calls += 1; }
        fn configured_remote_count(&mut self) -> usize { self.remote_count }
        fn local_player(&self) -> MPathPlayer { self.local.clone().unwrap() }
        fn set_session_num_players(&mut self, count: usize) { self.session_player_count = count; }
        fn set_multiplayer_communications_protocol(&mut self) { self.protocol_set += 1; }
    }

    impl MPathConnectHost for MockHost {
        fn send_global_message(&mut self, packet: &MPathPingPacket, dest: MPathDestination) {
            self.sends.push((packet.clone(), dest));
        }
        fn service_mpath(&mut self) { self.service_calls += 1; }
        fn receive_global_message(&mut self) -> Option<MPathReceivedPacket> { self.incoming.take() }
        fn surfaces_restored(&mut self) -> bool { self.restored }
        fn clear_surfaces_restored(&mut self) { self.clears += 1; self.restored = false; }
        fn seen_buffer_width(&self) -> i32 { self.width }
        fn resolution_factor(&self) -> i32 { self.factor }
        fn string_pixel_width(&self, text: &str) -> i32 { text.len() as i32 * self.width_per_char }
        fn hide_mouse(&mut self) { self.mouse_calls.push("hide"); }
        fn draw_wait_dialog(&mut self, rect: MPathDialogRect, _waiting: &str, _escape: &str) { self.draw_calls.push(rect); }
        fn show_mouse(&mut self) { self.mouse_calls.push("show"); }
        fn keyboard_escape_pressed(&mut self) { self.escape }
        fn timer_expired(&mut self) -> bool { let v = self.timer; self.timer = false; v }
        fn set_timer(&mut self, ticks: u32) { self.timers.push(ticks); }
    }

    impl MPathDisconnectHost for MockHost {
        fn house(&self, id: u8) -> Option<MPathHouse> { self.houses.get(&id).cloned() }
        fn connection_name(&self, id: i32) -> String { self.connection_names.get(&id).cloned().unwrap_or_default() }
        fn max_iq(&self) -> u8 { self.max_iq_value }
        fn computer_name(&self) -> String { self.computer_name_value.clone() }
        fn add_message(&mut self, kind: MPathDisconnectMessage, player: Option<&str>, color: u8, duration: u32) {
            self.messages.push((kind, player.map(str::to_owned), color, duration));
        }
        fn flag_map_to_redraw(&mut self, full: bool) { self.redraws.push(full); }
        fn player_count(&self) -> usize { self.player_names.len() }
        fn player_name(&self, index: usize) -> Option<String> { self.player_names.get(index).cloned() }
        fn remove_player(&mut self, index: usize) { self.removed_players.push(index); self.player_names.remove(index); }
        fn delete_connection(&mut self, id: i32) { self.deleted_connections.push(id); }
        fn update_house(&mut self, house: MPathHouse) { self.updated_houses.push(house.clone()); self.houses.insert(house.house_id, house); }
        fn set_session_player_count(&mut self, count: usize) { self.session_player_count = count; }
    }

    fn local_player() -> MPathPlayer {
        MPathPlayer { name: "Local".into(), address: None, house_id: 1, color_id: 2 }
    }

    #[test]
    fn disabled_mpath_build_matches_compiled_success_and_noop() {
        let mut host = MockHost::default();
        assert!(init_mpath(MPathBuildFeatures::checked_in_win32_project(), 1024, &mut host).unwrap().is_none());
        assert!(host.sends.is_empty());
        assert_eq!(host.settings_read, 0);
        let mut wait = 0;
        let mut packet = true;
        let mut mpath = true;
        assert!(shutdown_mpath(MPathBuildFeatures::checked_in_win32_project(), &mut wait, &mut packet, &mut mpath));
        assert!(packet && mpath);
        assert!(!destroy_mpath_connection(MPathBuildFeatures::checked_in_win32_project(), 1, 0, 60, 2, &mut host));
    }

    #[test]
    fn mpath_init_allocates_reads_flushes_and_returns_connect_state() {
        let mut host = MockHost {
            options_ok: true,
            remote_count: 2,
            local: Some(local_player()),
            ..MockHost::default()
        };
        let connect = init_mpath(MPathBuildFeatures { mpath: true, win32: true }, 4096, &mut host).unwrap().unwrap();
        assert!(host.packet_allocated);
        assert_eq!(host.settings_read, 1);
        assert_eq!(host.flush_calls, 1);
        assert_eq!(connect.expected_remote_players, 2);
        assert_eq!(connect.players, vec![local_player()]);
    }

    #[test]
    fn options_load_failure_shows_message_and_emergency_exits() {
        let mut host = MockHost { options_ok: false, local: Some(local_player()), ..MockHost::default() };
        assert_eq!(
            init_mpath(MPathBuildFeatures { mpath: true, win32: true }, 100, &mut host),
            Err(MPathInitError::EmergencyExit)
        );
        assert_eq!(host.error_shown, 1);
        assert_eq!(host.emergency_exit_codes, vec![0]);
        assert_eq!(host.flush_calls, 0);
    }

    #[test]
    fn start_broadcasts_ping_sets_timer_and_waits_for_remote_players() {
        let mut host = MockHost { local: Some(local_player()), remote_count: 1, ..MockHost::default() };
        let mut connect = MPathConnect::new(1, local_player());
        connect.start(&mut host);
        assert_eq!(connect.state, MPathConnectState::Waiting);
        assert_eq!(host.session_player_count, 2);
        assert_eq!(host.sends[0].1, MPathDestination::broadcast());
        assert_eq!(host.timers, vec![240]);
    }

    #[test]
    fn waiting_loop_redraws_after_surface_restore_and_rebroadcasts_ping_on_timer() {
        let mut host = MockHost {
            local: Some(local_player()),
            width: 640,
            factor: 2,
            width_per_char: 8,
            restored: true,
            timer: true,
            ..MockHost::default()
        };
        let mut connect = MPathConnect::new(1, local_player());
        connect.start(&mut host);
        let result = connect.step(&mut host);
        assert_eq!(result.state, MPathConnectState::Waiting);
        assert!(result.displayed_dialog.is_some());
        assert_eq!(host.clears, 1);
        assert_eq!(host.mouse_calls, vec!["hide", "show"]);
        assert_eq!(host.sends.len(), 2);
        assert!(result.ping_rebroadcast);
    }

    #[test]
    fn new_ping_adds_remote_player_and_replies_by_unicast_address() {
        let mut host = MockHost {
            local: Some(local_player()),
            remote_count: 1,
            incoming: Some(MPathReceivedPacket {
                packet: MPathPingPacket {
                    command: MPathCommand::Ping,
                    player_name: "Remote".into(),
                    house_id: 3,
                    color_id: 4,
                },
                address: 0x1234,
                packet_size: 64,
            }),
            ..MockHost::default()
        };
        let mut connect = MPathConnect::new(1, local_player());
        connect.start(&mut host);
        let result = connect.step(&mut host);
        assert_eq!(result.state, MPathConnectState::Complete);
        assert_eq!(result.players.len(), 2);
        assert_eq!(result.players[1].address, Some(0x1234));
        assert_eq!(host.sends[1].1, MPathDestination::unicast(0x1234));
        assert_eq!(host.protocol_set, 1);
    }

    #[test]
    fn duplicate_ping_address_is_ignored_without_incrementing_found_count() {
        let mut host = MockHost {
            local: Some(local_player()),
            remote_count: 2,
            incoming: Some(MPathReceivedPacket {
                packet: MPathPingPacket { command: MPathCommand::Ping, player_name: "Remote".into(), house_id: 3, color_id: 4 },
                address: 9,
                packet_size: 64,
            }),
            ..MockHost::default()
        };
        let mut connect = MPathConnect::new(2, local_player());
        connect.players.push(MPathPlayer { name: "Remote".into(), address: Some(9), house_id: 3, color_id: 4 });
        connect.found_remote_players = 1;
        connect.state = MPathConnectState::Waiting;
        let result = connect.step(&mut host);
        assert!(result.duplicate_ignored);
        assert_eq!(connect.found_remote_players, 1);
        assert_eq!(host.sends.len(), 0);
    }

    #[test]
    fn escape_during_wait_requests_emergency_exit() {
        let mut host = MockHost { escape: true, ..MockHost::default() };
        let mut connect = MPathConnect::new(1, local_player());
        connect.start(&mut host);
        let result = connect.step(&mut host);
        assert_eq!(result.state, MPathConnectState::EmergencyExit);
        assert_eq!(host.emergency_exit_codes, vec![0]);
    }

    #[test]
    fn no_remote_players_skips_the_wait_loop() {
        let mut host = MockHost { local: Some(local_player()), remote_count: 0, ..MockHost::default() };
        let connect = MPathConnect::new(0, local_player());
        assert_eq!(connect.state, MPathConnectState::Complete);
        assert!(host.sends.is_empty());
    }

    #[test]
    fn shutdown_waits_sixty_steps_then_frees_mpath_buffers() {
        let features = MPathBuildFeatures { mpath: true, win32: true };
        let mut wait = 0;
        let mut packet = true;
        let mut mpath = true;
        for _ in 0..60 {
            assert!(!shutdown_mpath(features, &mut wait, &mut packet, &mut mpath));
        }
        assert!(shutdown_mpath(features, &mut wait, &mut packet, &mut mpath));
        assert!(!packet && !mpath);
    }

    #[test]
    fn destroy_connection_shows_error_and_changes_human_house_to_ai() {
        let mut host = MockHost {
            houses: BTreeMap::from([(2, MPathHouse { house_id: 2, is_human: true, ini_name: "Remote".into(), remap_color: 5, iq: 2 })]),
            connection_names: BTreeMap::from([(2, "Remote Player".into())]),
            player_names: vec!["Local".into(), "Remote".into()],
            max_iq_value: 7,
            computer_name_value: "Computer".into(),
            ..MockHost::default()
        };
        assert!(destroy_mpath_connection(
            MPathBuildFeatures { mpath: true, win32: true }, 2, 1, 600, 2, &mut host,
        ));
        assert_eq!(host.messages[0], (MPathDisconnectMessage::ConnectionLost, Some("Remote Player".into()), 5, 600));
        assert_eq!(host.removed_players, vec![1]);
        assert_eq!(host.deleted_connections, vec![2]);
        assert!(!host.updated_houses[0].is_human);
        assert_eq!(host.updated_houses[0].iq, 7);
        assert_eq!(host.updated_houses[0].ini_name, "Computer");
        assert_eq!(host.session_player_count, 1);
        assert_eq!(host.messages[1].0, MPathDisconnectMessage::JustYouAndMe);
    }

    #[test]
    fn unknown_disconnect_error_suppresses_first_message_but_removes_player() {
        let mut host = MockHost {
            houses: BTreeMap::from([(1, MPathHouse { house_id: 1, is_human: true, ini_name: "Peer".into(), remap_color: 3, iq: 1 })]),
            player_names: vec!["Peer".into(), "Other".into()],
            max_iq_value: 5,
            computer_name_value: "Computer".into(),
            ..MockHost::default()
        };
        assert!(destroy_mpath_connection(
            MPathBuildFeatures { mpath: true, win32: true }, 1, 8, 60, 3, &mut host,
        ));
        assert!(host.messages.is_empty());
        assert_eq!(host.removed_players, vec![0]);
        assert_eq!(host.deleted_connections, vec![1]);
    }

    #[test]
    fn nonhuman_or_missing_house_is_ignored() {
        let mut host = MockHost::default();
        assert!(!destroy_mpath_connection(
            MPathBuildFeatures { mpath: true, win32: true }, 1, 0, 60, 2, &mut host,
        ));
    }
}
