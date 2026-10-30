//! DDE bridge from REDALERT/CCDDE.CPP and TIBERIANDAWN/CCDDE.CPP,
//! with protocol constants from DDE.H and server state from CCDDE.H.
//!
//! Both pinned Win32 variants actively process client connect/disconnect
//! advisories, multiplayer-start payloads, tickles and heartbeats. The
//! DDEML/Instance_Class operations are supplied by a host adapter.

pub const DDE_ADVISE_CONNECT: i32 = -1;
pub const DDE_ADVISE_DISCONNECT: i32 = -2;

pub const DDE_PACKET_START_MPLAYER_GAME: u32 = 0;
pub const DDE_PACKET_GAME_RESULTS: u32 = 1;
pub const DDE_PACKET_HEART_BEAT: u32 = 2;
pub const DDE_TICKLE: u32 = 3;
pub const DDE_CONNECTION_FAILED: u32 = 4;

pub const DDE_PACKET_HEADER_SIZE: usize = 2 * core::mem::size_of::<u32>();

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DdeVariant {
    RedAlert,
    TiberianDawn,
}

impl DdeVariant {
    pub const fn local_service_name(self) -> &'static str {
        match self {
            Self::RedAlert => "REDALERT",
            Self::TiberianDawn => "CONQUER",
        }
    }
    pub const fn remote_service_name(self) -> &'static str { "WCHAT" }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DdeAdvisory {
    ClientConnected,
    ClientDisconnected,
    Unknown(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DdePacketType {
    StartMultiplayerGame,
    GameResults,
    Heartbeat,
    Tickle,
    ConnectionFailed,
    Unknown(u32),
}

impl From<u32> for DdePacketType {
    fn from(value: u32) -> Self {
        match value {
            DDE_PACKET_START_MPLAYER_GAME => Self::StartMultiplayerGame,
            DDE_PACKET_GAME_RESULTS => Self::GameResults,
            DDE_PACKET_HEART_BEAT => Self::Heartbeat,
            DDE_TICKLE => Self::Tickle,
            DDE_CONNECTION_FAILED => Self::ConnectionFailed,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DdeCallbackResult {
    Advisory(DdeAdvisory),
    StartGameReceived { payload_length: usize },
    TickleReceived,
    HeartbeatReceived { tick: u32 },
    OtherPacket(DdePacketType),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DdeError {
    InvalidPositivePacketLength { supplied: usize },
    TruncatedPacketHeader { supplied: usize },
    InvalidDeclaredLength { declared: usize, available: usize },
    NegativePayloadLength,
    PayloadTooLarge { length: usize },
    InvalidSendLength { supplied: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DdeBridge {
    pub variant: DdeVariant,
    pub already_running: bool,
    is_enabled: bool,
    last_heartbeat: u32,
    multiplayer_game_info: Option<Vec<u8>>,
    multiplayer_game_info_length: usize,
}

pub trait DdeTransport {
    fn enable_callback(&mut self, enabled: bool);
    fn test_server_running(&mut self, local_service: &str) -> bool;
    fn open_poke_connection(&mut self, remote_service: &str) -> bool;
    fn poke_server(&mut self, packet: &[u8]) -> bool;
    fn close_poke_connection(&mut self);
}

pub trait DdeClock {
    fn game_timer_in_use(&self) -> bool;
    fn game_timer_time(&self) -> u32;
}

impl DdeBridge {
    /// DDEServerClass construction. The source initializes the game-info
    /// pointer, creates Instance_Class(local, "WCHAT"), enables callbacks,
    /// and tests whether an instance of its own local service is running.
    pub fn new(variant: DdeVariant, transport: &mut impl DdeTransport) -> Self {
        let already_running = transport.test_server_running(variant.local_service_name());
        transport.enable_callback(true);
        Self {
            variant,
            already_running,
            is_enabled: true,
            last_heartbeat: 0,
            multiplayer_game_info: None,
            multiplayer_game_info_length: 0,
        }
    }

    pub const fn is_enabled(&self) -> bool { self.is_enabled }
    pub const fn last_heartbeat(&self) -> u32 { self.last_heartbeat }
    pub const fn multiplayer_game_info_length(&self) -> usize {
        self.multiplayer_game_info_length
    }
    pub fn multiplayer_game_info(&self) -> Option<&[u8]> {
        self.multiplayer_game_info.as_deref()
    }

    pub fn enable(&mut self, transport: &mut impl DdeTransport) {
        if !self.is_enabled {
            transport.enable_callback(true);
            self.is_enabled = true;
        }
    }

    pub fn disable(&mut self, transport: &mut impl DdeTransport) {
        if self.is_enabled {
            transport.enable_callback(false);
            self.is_enabled = false;
        }
    }

    /// The C++ destructor releases the DDE game-info allocation and deletes
    /// the Instance_Class object. The owner invokes this before dropping its
    /// transport object; here the payload is cleared explicitly.
    pub fn shutdown(&mut self) {
        self.delete_multiplayer_game_info();
    }

    pub fn delete_multiplayer_game_info(&mut self) {
        self.multiplayer_game_info = None;
        // Source only nulls the pointer; retain its recorded length for 1:1
        // observable compatibility until another start-game packet arrives.
    }

    pub fn time_since_heartbeat(&self, clock: &impl DdeClock) -> i32 {
        clock.game_timer_time().wrapping_sub(self.last_heartbeat) as i32
    }

    /// DDEServerClass::Callback. For positive DDE payloads the first two
    /// network-order 32-bit integers are total packet size and packet type.
    /// The reported DDE length is used to bound input; the declared packet
    /// size is checked before copying to avoid legacy raw-pointer overreads.
    pub fn callback(
        &mut self,
        data: &[u8],
        length: i32,
        clock: &impl DdeClock,
    ) -> Result<DdeCallbackResult, DdeError> {
        if length < 0 {
            return Ok(DdeCallbackResult::Advisory(match length {
                DDE_ADVISE_CONNECT => DdeAdvisory::ClientConnected,
                DDE_ADVISE_DISCONNECT => DdeAdvisory::ClientDisconnected,
                other => DdeAdvisory::Unknown(other),
            }));
        }

        let supplied = length as usize;
        if supplied < DDE_PACKET_HEADER_SIZE {
            return Err(DdeError::InvalidPositivePacketLength { supplied });
        }
        if data.len() < supplied {
            return Err(DdeError::TruncatedPacketHeader { supplied: data.len() });
        }

        let declared_total = u32::from_be_bytes(data[0..4].try_into().unwrap()) as usize;
        let packet_type = u32::from_be_bytes(data[4..8].try_into().unwrap());
        let payload_length = declared_total.checked_sub(DDE_PACKET_HEADER_SIZE)
            .ok_or(DdeError::NegativePayloadLength)?;
        let available = supplied - DDE_PACKET_HEADER_SIZE;
        if payload_length > available {
            return Err(DdeError::InvalidDeclaredLength {
                declared: payload_length,
                available,
            });
        }

        match DdePacketType::from(packet_type) {
            DdePacketType::StartMultiplayerGame => {
                let mut payload = Vec::new();
                payload.try_reserve_exact(payload_length)
                    .map_err(|_| DdeError::PayloadTooLarge { length: payload_length })?;
                payload.extend_from_slice(&data[DDE_PACKET_HEADER_SIZE..DDE_PACKET_HEADER_SIZE + payload_length]);
                // The C++ allocation reserves one additional byte and null terminates
                // the payload for INI/string consumers.
                payload.push(0);
                self.multiplayer_game_info = Some(payload);
                self.multiplayer_game_info_length = payload_length;
                self.last_heartbeat = 0;
                Ok(DdeCallbackResult::StartGameReceived { payload_length })
            }
            DdePacketType::Tickle => Ok(DdeCallbackResult::TickleReceived),
            DdePacketType::Heartbeat => {
                self.last_heartbeat = if clock.game_timer_in_use() {
                    clock.game_timer_time()
                } else {
                    0
                };
                Ok(DdeCallbackResult::HeartbeatReceived { tick: self.last_heartbeat })
            }
            other => Ok(DdeCallbackResult::OtherPacket(other)),
        }
    }

    /// Send_Data_To_DDE_Server builds the 8-byte network-order WChat frame,
    /// then opens, pokes and closes the DDE connection. A successful open is
    /// always followed by Close_Poke_Connection, including failed pokes.
    pub fn send_data_to_server(
        &self,
        data: &[u8],
        packet_type: i32,
        transport: &mut impl DdeTransport,
    ) -> bool {
        let Some(total_length) = data.len().checked_add(DDE_PACKET_HEADER_SIZE) else {
            return false;
        };
        if total_length > u32::MAX as usize { return false; }
        if !transport.open_poke_connection(DdeVariant::remote_service_name()) {
            return false;
        }

        let mut packet = Vec::with_capacity(total_length);
        packet.extend_from_slice(&(total_length as u32).to_be_bytes());
        packet.extend_from_slice(&(packet_type as u32).to_be_bytes());
        packet.extend_from_slice(data);
        let success = transport.poke_server(&packet);
        transport.close_poke_connection();
        success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeTransport {
        callback_enabled: Vec<bool>,
        running: bool,
        open_ok: bool,
        poke_ok: bool,
        pokes: Vec<Vec<u8>>,
        opened_names: Vec<String>,
        closes: usize,
    }

    impl DdeTransport for FakeTransport {
        fn enable_callback(&mut self, enabled: bool) { self.callback_enabled.push(enabled); }
        fn test_server_running(&mut self, _local_service: &str) -> bool { self.running }
        fn open_poke_connection(&mut self, remote: &str) -> bool {
            self.opened_names.push(remote.to_string());
            self.open_ok
        }
        fn poke_server(&mut self, packet: &[u8]) -> bool {
            self.pokes.push(packet.to_vec());
            self.poke_ok
        }
        fn close_poke_connection(&mut self) { self.closes += 1; }
    }

    #[derive(Default)]
    struct FakeClock { tick: u32, in_use: bool }
    impl DdeClock for FakeClock {
        fn game_timer_in_use(&self) -> bool { self.in_use }
        fn game_timer_time(&self) -> u32 { self.tick }
    }

    fn packet(kind: u32, payload: &[u8]) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&((payload.len() + 8) as u32).to_be_bytes());
        data.extend_from_slice(&kind.to_be_bytes());
        data.extend_from_slice(payload);
        data
    }

    #[test]
    fn constructor_uses_variant_service_names_and_enables_callbacks() {
        let mut transport = FakeTransport { running: true, ..FakeTransport::default() };
        let ra = DdeBridge::new(DdeVariant::RedAlert, &mut transport);
        assert_eq!(DdeVariant::RedAlert.local_service_name(), "REDALERT");
        assert_eq!(DdeVariant::TiberianDawn.local_service_name(), "CONQUER");
        assert_eq!(DdeVariant::RedAlert.remote_service_name(), "WCHAT");
        assert!(ra.already_running);
        assert!(ra.is_enabled());
        assert_eq!(transport.callback_enabled, vec![true]);
    }

    #[test]
    fn callback_handles_advisories_and_rejects_unknown_negative_codes() {
        let mut transport = FakeTransport::default();
        let mut dde = DdeBridge::new(DdeVariant::RedAlert, &mut transport);
        let clock = FakeClock::default();
        assert_eq!(dde.callback(&[], DDE_ADVISE_CONNECT, &clock).unwrap(),
            DdeCallbackResult::Advisory(DdeAdvisory::ClientConnected));
        assert_eq!(dde.callback(&[], DDE_ADVISE_DISCONNECT, &clock).unwrap(),
            DdeCallbackResult::Advisory(DdeAdvisory::ClientDisconnected));
        assert_eq!(dde.callback(&[], -99, &clock).unwrap(),
            DdeCallbackResult::Advisory(DdeAdvisory::Unknown(-99)));
    }

    #[test]
    fn start_game_payload_is_copied_null_terminated_and_resets_heartbeat() {
        let mut transport = FakeTransport::default();
        let mut dde = DdeBridge::new(DdeVariant::TiberianDawn, &mut transport);
        let clock = FakeClock { tick: 500, in_use: true };
        assert_eq!(dde.callback(&packet(DDE_PACKET_HEART_BEAT, &[]), 8, &clock).unwrap(),
            DdeCallbackResult::HeartbeatReceived { tick: 500 });
        assert_eq!(dde.callback(&packet(DDE_PACKET_START_MPLAYER_GAME, b"[Basic]\nName=Test"), 25, &clock).unwrap(),
            DdeCallbackResult::StartGameReceived { payload_length: 17 });
        assert_eq!(dde.multiplayer_game_info().unwrap().last(), Some(&0));
        assert_eq!(dde.multiplayer_game_info_length(), 17);
        assert_eq!(dde.last_heartbeat(), 0);
    }

    #[test]
    fn heartbeat_is_zero_when_game_timer_is_not_in_use() {
        let mut transport = FakeTransport::default();
        let mut dde = DdeBridge::new(DdeVariant::RedAlert, &mut transport);
        let clock = FakeClock { tick: 999, in_use: false };
        dde.callback(&packet(DDE_PACKET_HEART_BEAT, &[]), 8, &clock).unwrap();
        assert_eq!(dde.last_heartbeat(), 0);
        assert_eq!(dde.time_since_heartbeat(&clock), 999);
    }

    #[test]
    fn enable_disable_are_idempotent() {
        let mut transport = FakeTransport::default();
        let mut dde = DdeBridge::new(DdeVariant::RedAlert, &mut transport);
        dde.enable(&mut transport);
        dde.disable(&mut transport);
        dde.disable(&mut transport);
        dde.enable(&mut transport);
        assert_eq!(transport.callback_enabled, vec![true, false, true]);
    }

    #[test]
    fn send_poke_builds_network_order_length_and_type() {
        let mut transport = FakeTransport { open_ok: true, poke_ok: true, ..FakeTransport::default() };
        let dde = DdeBridge::new(DdeVariant::RedAlert, &mut transport);
        assert!(dde.send_data_to_server(b"abc", 7, &mut transport));
        assert_eq!(transport.opened_names, vec!["WCHAT"]);
        assert_eq!(transport.closes, 1);
        assert_eq!(transport.pokes[0], vec![0,0,0,11, 0,0,0,7, b'a',b'b',b'c']);
    }

    #[test]
    fn failed_open_does_not_close_but_failed_poke_does_close() {
        let mut transport = FakeTransport { open_ok: false, poke_ok: true, ..FakeTransport::default() };
        let dde = DdeBridge::new(DdeVariant::TiberianDawn, &mut transport);
        assert!(!dde.send_data_to_server(b"x", 1, &mut transport));
        assert_eq!(transport.closes, 0);
        transport.open_ok = true;
        transport.poke_ok = false;
        assert!(!dde.send_data_to_server(b"x", 1, &mut transport));
        assert_eq!(transport.closes, 1);
    }

    #[test]
    fn malformed_declared_lengths_are_rejected() {
        let mut transport = FakeTransport::default();
        let mut dde = DdeBridge::new(DdeVariant::RedAlert, &mut transport);
        let clock = FakeClock::default();
        let mut bad = vec![0,0,0,1, 0,0,0,DDE_PACKET_START_MPLAYER_GAME as u8];
        assert!(dde.callback(&bad, 8, &clock).is_err());
        bad[3] = 32;
        assert!(dde.callback(&bad, 8, &clock).is_err());
        assert!(dde.callback(&[0; 4], 4, &clock).is_err());
    }
}
