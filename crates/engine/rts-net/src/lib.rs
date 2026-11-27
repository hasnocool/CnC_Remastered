//! Asynchronous multiplayer transport primitives.
//!
//! This crate provides bounded message framing, UDP server discovery, protocol
//! compatibility checks, and stable lockstep command ordering. It does not
//! claim to implement the original Westwood wire protocol yet.

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{TcpStream, UdpSocket},
    time::{timeout, Instant},
};

pub const PROTOCOL_VERSION: u16 = 1;
pub const DISCOVERY_PORT: u16 = 47_831;
pub const DISCOVERY_REQUEST: &[u8] = b"CNC_RTS_DISCOVER_V1";
pub const MAX_FRAME_BYTES: usize = 1_048_576;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum GameId {
    TiberianDawn,
    RedAlert,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveryBeacon {
    pub protocol_version: u16,
    pub server_id: String,
    pub name: String,
    pub game: GameId,
    pub players: u16,
    pub max_players: u16,
    pub map_name: String,
    pub build_id: String,
}

impl DiscoveryBeacon {
    pub fn compatible_with(&self, expected_build: &str, game: GameId) -> bool {
        self.protocol_version == PROTOCOL_VERSION
            && self.game == game
            && self.build_id == expected_build
            && self.players <= self.max_players
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityFingerprint {
    pub protocol_version: u16,
    pub game: GameId,
    pub build_id: String,
    pub simulation_hash: String,
    pub mod_hash: String,
}

impl CompatibilityFingerprint {
    pub fn compatible_with(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockstepCommand {
    pub tick: u64,
    pub peer_id: u32,
    pub sequence: u64,
    pub payload: Vec<u8>,
}

impl Ord for LockstepCommand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.tick, self.peer_id, self.sequence, &self.payload)
            .cmp(&(other.tick, other.peer_id, other.sequence, &other.payload))
    }
}

impl PartialOrd for LockstepCommand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum LobbyMessage {
    Hello { fingerprint: CompatibilityFingerprint, player_name: String },
    PlayerJoined { peer_id: u32, player_name: String },
    PlayerLeft { peer_id: u32 },
    Chat { peer_id: u32, message: String },
    StartGame { seed: u32, map_name: String },
    Command(LockstepCommand),
    Ping { nonce: u64 },
    Pong { nonce: u64 },
}

#[derive(Debug)]
pub enum NetError {
    Io(std::io::Error),
    Codec(serde_json::Error),
    FrameTooLarge(usize),
    InvalidFrameLength,
}

impl std::fmt::Display for NetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "network I/O: {error}"),
            Self::Codec(error) => write!(f, "message codec: {error}"),
            Self::FrameTooLarge(size) => write!(f, "network frame too large: {size} bytes"),
            Self::InvalidFrameLength => write!(f, "invalid zero-length network frame"),
        }
    }
}

impl std::error::Error for NetError {}
impl From<std::io::Error> for NetError {
    fn from(value: std::io::Error) -> Self { Self::Io(value) }
}
impl From<serde_json::Error> for NetError {
    fn from(value: serde_json::Error) -> Self { Self::Codec(value) }
}

/// Write one length-prefixed JSON message. The length is a big-endian u32.
pub async fn write_message<W, T>(writer: &mut W, message: &T) -> Result<(), NetError>
where
    W: AsyncWrite + Unpin,
    T: Serialize,
{
    let bytes = serde_json::to_vec(message)?;
    if bytes.is_empty() { return Err(NetError::InvalidFrameLength); }
    if bytes.len() > MAX_FRAME_BYTES { return Err(NetError::FrameTooLarge(bytes.len())); }
    writer.write_u32(bytes.len() as u32).await?;
    writer.write_all(&bytes).await?;
    writer.flush().await?;
    Ok(())
}

/// Read and validate one length-prefixed JSON message before allocating its payload.
pub async fn read_message<R, T>(reader: &mut R) -> Result<T, NetError>
where
    R: AsyncRead + Unpin,
    T: DeserializeOwned,
{
    let length = reader.read_u32().await? as usize;
    if length == 0 { return Err(NetError::InvalidFrameLength); }
    if length > MAX_FRAME_BYTES { return Err(NetError::FrameTooLarge(length)); }
    let mut bytes = vec![0_u8; length];
    reader.read_exact(&mut bytes).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub struct LobbyConnection {
    stream: TcpStream,
}

impl LobbyConnection {
    pub async fn connect(address: SocketAddr) -> Result<Self, NetError> {
        Ok(Self { stream: TcpStream::connect(address).await? })
    }

    pub async fn send(&mut self, message: &LobbyMessage) -> Result<(), NetError> {
        write_message(&mut self.stream, message).await
    }

    pub async fn receive(&mut self) -> Result<LobbyMessage, NetError> {
        read_message(&mut self.stream).await
    }
}

/// Send a discovery query and collect unique compatible beacons until timeout.
pub async fn discover_servers(
    port: u16,
    wait: Duration,
) -> Result<Vec<(SocketAddr, DiscoveryBeacon)>, NetError> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).await?;
    socket.set_broadcast(true)?;
    socket.send_to(DISCOVERY_REQUEST, SocketAddrV4::new(Ipv4Addr::BROADCAST, port)).await?;
    let deadline = Instant::now() + wait;
    let mut found = BTreeMap::new();
    let mut buffer = vec![0_u8; 4096];

    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() { break; }
        match timeout(remaining, socket.recv_from(&mut buffer)).await {
            Ok(Ok((length, address))) => {
                if let Ok(beacon) = serde_json::from_slice::<DiscoveryBeacon>(&buffer[..length]) {
                    found.insert((address, beacon.server_id.clone()), beacon);
                }
            }
            Ok(Err(error)) => return Err(NetError::Io(error)),
            Err(_) => break,
        }
    }
    Ok(found.into_iter().map(|((address, _), beacon)| (address, beacon)).collect())
}

/// Run on a server's bound discovery socket until the task is cancelled.
pub async fn serve_discovery(socket: UdpSocket, beacon: DiscoveryBeacon) -> Result<(), NetError> {
    let payload = serde_json::to_vec(&beacon)?;
    let mut buffer = [0_u8; 256];
    loop {
        let (length, peer) = socket.recv_from(&mut buffer).await?;
        if &buffer[..length] == DISCOVERY_REQUEST {
            socket.send_to(&payload, peer).await?;
        }
    }
}

pub fn sort_lockstep_commands(commands: &mut [LockstepCommand]) {
    commands.sort_unstable();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fingerprint() -> CompatibilityFingerprint {
        CompatibilityFingerprint {
            protocol_version: PROTOCOL_VERSION,
            game: GameId::RedAlert,
            build_id: "dev".into(),
            simulation_hash: "sim-a".into(),
            mod_hash: "mods-a".into(),
        }
    }

    #[test]
    fn compatibility_requires_exact_simulation_and_mod_fingerprint() {
        let expected = fingerprint();
        assert!(expected.compatible_with(&fingerprint()));
        let mut changed = fingerprint();
        changed.mod_hash.push('b');
        assert!(!expected.compatible_with(&changed));
    }

    #[test]
    fn lockstep_order_is_tick_then_peer_then_sequence() {
        let mut commands = vec![
            LockstepCommand { tick: 8, peer_id: 2, sequence: 0, payload: vec![] },
            LockstepCommand { tick: 7, peer_id: 9, sequence: 0, payload: vec![] },
            LockstepCommand { tick: 8, peer_id: 1, sequence: 3, payload: vec![] },
        ];
        sort_lockstep_commands(&mut commands);
        assert_eq!((commands[0].tick, commands[1].peer_id, commands[2].peer_id), (7, 1, 2));
    }

    #[test]
    fn beacon_rejects_wrong_game_or_build() {
        let beacon = DiscoveryBeacon {
            protocol_version: PROTOCOL_VERSION,
            server_id: "server-1".into(),
            name: "Test".into(),
            game: GameId::RedAlert,
            players: 1,
            max_players: 4,
            map_name: "test".into(),
            build_id: "dev".into(),
        };
        assert!(beacon.compatible_with("dev", GameId::RedAlert));
        assert!(!beacon.compatible_with("other", GameId::RedAlert));
        assert!(!beacon.compatible_with("dev", GameId::TiberianDawn));
    }
}
