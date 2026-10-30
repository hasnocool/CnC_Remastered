//! Red Alert Winsock/UDP adapter ported from REDALERT/WSPUDP.CPP.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection at
//! f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! Covers dotted-decimal broadcast address setup, UDP socket initialization,
//! local-address discovery, socket options, broadcast fan-out, and asynchronous
//! FD_READ/FD_WRITE event handling. Hostname resolution and socket operations
//! are injected through UdpSocketApi; no blocking networking is performed here.

use std::collections::VecDeque;

pub const UDP_MESSAGE_ID: u32 = 0x0400 + 116;
pub const UDP_RECEIVE_BUFFER_SIZE: usize = 1024;
pub const UDP_PACKET_BUFFER_SIZE: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdpSocketError {
    WouldBlock,
    Failure(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdpSocketEvent {
    Read,
    Write,
    Other(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UdpSocketAddress {
    pub ipv4: [u8; 4],
    /// Socket port in network byte order, equivalent to htons(port).
    pub port: u16,
}

impl UdpSocketAddress {
    pub fn new(ipv4: [u8; 4], host_port: u16) -> Self {
        Self { ipv4, port: host_port.to_be() }
    }

    pub fn any(host_port: u16) -> Self {
        Self::new([0; 4], host_port)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdpSocketOption {
    LingerDisabled,
}

pub trait UdpSocketApi {
    type Socket: Copy + Eq;

    fn initialize_winsock(&mut self) -> bool;
    fn open_udp_socket(&mut self) -> Result<Self::Socket, UdpSocketError>;
    fn bind(&mut self, socket: Self::Socket, address: UdpSocketAddress) -> Result<(), UdpSocketError>;
    /// Corresponds to gethostname/gethostbyname. Addresses are returned as
    /// their four in-memory IPv4 octets, in the same order as h_addr_list.
    fn discover_local_ipv4_addresses(&mut self) -> Result<Vec<[u8; 4]>, UdpSocketError>;
    fn set_option(&mut self, socket: Self::Socket, option: UdpSocketOption) -> Result<(), UdpSocketError>;
    fn set_base_socket_options(&mut self, socket: Self::Socket) -> bool;
    fn close_socket(&mut self, socket: Self::Socket);
    fn receive_from(
        &mut self,
        socket: Self::Socket,
        destination: &mut [u8],
    ) -> Result<Option<(usize, [u8; 4])>, UdpSocketError>;
    fn send_to(
        &mut self,
        socket: Self::Socket,
        payload: &[u8],
        destination: UdpSocketAddress,
    ) -> Result<usize, UdpSocketError>;
    fn clear_socket_error(&mut self, socket: Self::Socket);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdpPortError {
    WinsockInitializationFailed,
    SocketCreationFailed,
    SocketBindFailed,
    LocalAddressDiscoveryFailed,
    InvalidBroadcastAddress,
    BroadcastAddressTooLong,
    PacketTooLarge,
    SocketNotOpen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdpReceivedPacket {
    pub source_ipv4: [u8; 4],
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdpOutgoingPacket {
    pub destination_ipv4: [u8; 4],
    pub payload: Vec<u8>,
    /// The legacy base packet struct carries this flag, although UDP
    /// Message_Handler routes by Address+4 for both broadcast and unicast.
    pub is_broadcast: bool,
}

#[derive(Debug)]
pub struct RedAlertUdpInterface<S> {
    winsock_initialized: bool,
    socket: Option<S>,
    port: u16,
    broadcast_addresses: Vec<[u8; 4]>,
    local_addresses: Vec<[u8; 4]>,
    incoming: VecDeque<UdpReceivedPacket>,
    outgoing: VecDeque<UdpOutgoingPacket>,
}

impl<S: Copy + Eq> Default for RedAlertUdpInterface<S> {
    fn default() -> Self { Self::new() }
}

impl<S: Copy + Eq> RedAlertUdpInterface<S> {
    pub fn new() -> Self {
        Self {
            winsock_initialized: false,
            socket: None,
            port: 0,
            broadcast_addresses: Vec::new(),
            local_addresses: Vec::new(),
            incoming: VecDeque::new(),
            outgoing: VecDeque::new(),
        }
    }

    pub fn winsock_initialized(&self) -> bool { self.winsock_initialized }
    pub fn socket(&self) -> Option<S> { self.socket }
    pub fn port(&self) -> u16 { self.port }
    pub fn broadcast_addresses(&self) -> &[[u8; 4]] { &self.broadcast_addresses }
    pub fn local_addresses(&self) -> &[[u8; 4]] { &self.local_addresses }
    pub fn incoming_count(&self) -> usize { self.incoming.len() }
    pub fn outgoing_count(&self) -> usize { self.outgoing.len() }
    pub fn pop_incoming(&mut self) -> Option<UdpReceivedPacket> { self.incoming.pop_front() }

    /// Port of UDPInterfaceClass::Set_Broadcast_Address. The source accepts
    /// dotted decimal text up to 15 characters and stores the four octets.
    pub fn set_broadcast_address(&mut self, address: &str) -> Result<[u8; 4], UdpPortError> {
        if address.len() > 15 { return Err(UdpPortError::BroadcastAddressTooLong); }
        let parts: Vec<&str> = address.split('.').collect();
        if parts.len() != 4 { return Err(UdpPortError::InvalidBroadcastAddress); }
        let mut bytes = [0_u8; 4];
        for (index, part) in parts.into_iter().enumerate() {
            if part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(UdpPortError::InvalidBroadcastAddress);
            }
            let value = part.parse::<u16>().map_err(|_| UdpPortError::InvalidBroadcastAddress)?;
            if value > u8::MAX as u16 { return Err(UdpPortError::InvalidBroadcastAddress); }
            bytes[index] = value as u8;
        }
        self.broadcast_addresses.push(bytes);
        Ok(bytes)
    }

    /// Port of UDPInterfaceClass::Open_Socket. The caller supplies the
    /// configured PlanetWestwoodPortNumber. Local addresses are refreshed
    /// from the platform's host address discovery adapter.
    pub fn open_socket<A: UdpSocketApi<Socket = S>>(
        &mut self,
        port: u16,
        api: &mut A,
    ) -> Result<(), UdpPortError> {
        if !self.winsock_initialized {
            if !api.initialize_winsock() {
                return Err(UdpPortError::WinsockInitializationFailed);
            }
            self.winsock_initialized = true;
        }

        let socket = api.open_udp_socket().map_err(|_| UdpPortError::SocketCreationFailed)?;
        self.socket = Some(socket);
        self.port = port;

        if api.bind(socket, UdpSocketAddress::any(port)).is_err() {
            api.close_socket(socket);
            self.socket = None;
            return Err(UdpPortError::SocketBindFailed);
        }

        self.local_addresses.clear();
        match api.discover_local_ipv4_addresses() {
            Ok(addresses) => self.local_addresses = addresses,
            Err(_) => {
                api.close_socket(socket);
                self.socket = None;
                return Err(UdpPortError::LocalAddressDiscoveryFailed);
            }
        }

        // The source sets SO_LINGER {on=0, linger=0}; linger is disabled.
        let _ = api.set_option(socket, UdpSocketOption::LingerDisabled);
        let _ = api.set_base_socket_options(socket);
        Ok(())
    }

    pub fn close<A: UdpSocketApi<Socket = S>>(&mut self, api: &mut A) {
        if let Some(socket) = self.socket.take() {
            api.close_socket(socket);
        }
        self.incoming.clear();
        self.outgoing.clear();
        self.broadcast_addresses.clear();
        self.local_addresses.clear();
        self.winsock_initialized = false;
    }

    pub fn queue_send_to(
        &mut self,
        destination_ipv4: [u8; 4],
        payload: &[u8],
    ) -> Result<(), UdpPortError> {
        if payload.len() > UDP_PACKET_BUFFER_SIZE { return Err(UdpPortError::PacketTooLarge); }
        self.outgoing.push_back(UdpOutgoingPacket {
            destination_ipv4,
            payload: payload.to_vec(),
            is_broadcast: false,
        });
        Ok(())
    }

    /// Port of UDPInterfaceClass::Broadcast: fan out one queued datagram per
    /// configured broadcast address, preserving the address insertion order.
    pub fn broadcast(&mut self, payload: &[u8]) -> Result<usize, UdpPortError> {
        if payload.len() > UDP_PACKET_BUFFER_SIZE { return Err(UdpPortError::PacketTooLarge); }
        for destination_ipv4 in &self.broadcast_addresses {
            self.outgoing.push_back(UdpOutgoingPacket {
                destination_ipv4: *destination_ipv4,
                payload: payload.to_vec(),
                is_broadcast: true,
            });
        }
        Ok(self.broadcast_addresses.len())
    }

    /// Port of UDPInterfaceClass::Message_Handler. Returns 1 for messages
    /// owned by another protocol; recognized UDP messages return 0.
    pub fn message_handler<A: UdpSocketApi<Socket = S>>(
        &mut self,
        message: u32,
        event: UdpSocketEvent,
        event_error: i32,
        api: &mut A,
    ) -> i32 {
        if message != UDP_MESSAGE_ID { return 1; }
        let Some(socket) = self.socket else { return 0; };

        if event_error != 0 {
            api.clear_socket_error(socket);
            return 0;
        }

        match event {
            UdpSocketEvent::Read => {
                let mut receive_buffer = [0_u8; UDP_RECEIVE_BUFFER_SIZE];
                match api.receive_from(socket, &mut receive_buffer) {
                    Ok(Some((received, source_ipv4))) if received > 0 => {
                        if received > receive_buffer.len() { return 0; }
                        if self.local_addresses.iter().any(|local| *local == source_ipv4) {
                            return 0;
                        }
                        self.incoming.push_back(UdpReceivedPacket {
                            source_ipv4,
                            payload: receive_buffer[..received].to_vec(),
                        });
                    }
                    Ok(_) => {}
                    // Unlike WSPIPX.CPP, WSPUDP.CPP clears socket errors on
                    // every recvfrom failure, including WSAEWOULDBLOCK.
                    Err(_) => api.clear_socket_error(socket),
                }
                0
            }
            UdpSocketEvent::Write => {
                let Some(packet) = self.outgoing.front() else { return 0; };
                let destination = UdpSocketAddress::new(packet.destination_ipv4, self.port);
                match api.send_to(socket, &packet.payload, destination) {
                    Ok(_) => {
                        self.outgoing.pop_front();
                    }
                    Err(UdpSocketError::WouldBlock) => {
                        // The source dequeues WSAEWOULDBLOCK due to fallthrough.
                        self.outgoing.pop_front();
                    }
                    Err(UdpSocketError::Failure(_)) => {
                        api.clear_socket_error(socket);
                        // Fatal send errors retain the head packet for later.
                    }
                }
                0
            }
            UdpSocketEvent::Other(_) => 0,
        }
    }
}

/// Parse a dotted-decimal IPv4 literal. Unlike permissive libc parsing this
/// adapter rejects malformed fields instead of silently leaving bytes unset.
pub fn parse_ipv4_dotted_decimal(address: &str) -> Result<[u8; 4], UdpPortError> {
    let mut parser = RedAlertUdpInterface::<()>::new();
    parser.set_broadcast_address(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeApi {
        next_socket: u64,
        initialized: bool,
        fail_initialize: bool,
        fail_open: bool,
        fail_bind: bool,
        fail_discovery: bool,
        local_addresses: Vec<[u8; 4]>,
        closed: Vec<u64>,
        binds: Vec<(u64, UdpSocketAddress)>,
        sent: Vec<(Vec<u8>, UdpSocketAddress)>,
        reads: VecDeque<Result<Option<(usize, [u8; 4])>, UdpSocketError>>,
        writes: VecDeque<Result<usize, UdpSocketError>>,
        options: Vec<(u64, UdpSocketOption)>,
        base_options_count: usize,
        clear_count: usize,
    }

    impl FakeApi {
        fn new() -> Self {
            Self { local_addresses: vec![[10, 0, 0, 2], [192, 168, 1, 5]], ..Self::default() }
        }
    }

    impl UdpSocketApi for FakeApi {
        type Socket = u64;

        fn initialize_winsock(&mut self) -> bool {
            if self.fail_initialize { return false; }
            self.initialized = true;
            true
        }
        fn open_udp_socket(&mut self) -> Result<Self::Socket, UdpSocketError> {
            if self.fail_open { return Err(UdpSocketError::Failure(1)); }
            self.next_socket += 1;
            Ok(self.next_socket)
        }
        fn bind(&mut self, socket: Self::Socket, address: UdpSocketAddress) -> Result<(), UdpSocketError> {
            self.binds.push((socket, address));
            if self.fail_bind { Err(UdpSocketError::Failure(2)) } else { Ok(()) }
        }
        fn discover_local_ipv4_addresses(&mut self) -> Result<Vec<[u8; 4]>, UdpSocketError> {
            if self.fail_discovery { Err(UdpSocketError::Failure(3)) }
            else { Ok(self.local_addresses.clone()) }
        }
        fn set_option(&mut self, socket: Self::Socket, option: UdpSocketOption) -> Result<(), UdpSocketError> {
            self.options.push((socket, option));
            Ok(())
        }
        fn set_base_socket_options(&mut self, _socket: Self::Socket) -> bool {
            self.base_options_count += 1;
            true
        }
        fn close_socket(&mut self, socket: Self::Socket) { self.closed.push(socket); }
        fn receive_from(
            &mut self,
            _socket: Self::Socket,
            destination: &mut [u8],
        ) -> Result<Option<(usize, [u8; 4])>, UdpSocketError> {
            match self.reads.pop_front().unwrap_or(Ok(None)) {
                Ok(Some((length, address))) => {
                    let payload = b"udp-packet";
                    let count = length.min(payload.len()).min(destination.len());
                    destination[..count].copy_from_slice(&payload[..count]);
                    Ok(Some((count, address)))
                }
                other => other,
            }
        }
        fn send_to(
            &mut self,
            _socket: Self::Socket,
            payload: &[u8],
            destination: UdpSocketAddress,
        ) -> Result<usize, UdpSocketError> {
            self.sent.push((payload.to_vec(), destination));
            self.writes.pop_front().unwrap_or(Ok(payload.len()))
        }
        fn clear_socket_error(&mut self, _socket: Self::Socket) { self.clear_count += 1; }
    }

    fn opened() -> (RedAlertUdpInterface<u64>, FakeApi) {
        let mut interface = RedAlertUdpInterface::new();
        let mut api = FakeApi::new();
        interface.open_socket(8054, &mut api).unwrap();
        (interface, api)
    }

    #[test]
    fn dotted_ipv4_parser_and_broadcast_address_list_validate_source_input() {
        assert_eq!(parse_ipv4_dotted_decimal("255.255.255.255"), Ok([255; 4]));
        assert_eq!(parse_ipv4_dotted_decimal("192.168.1.10"), Ok([192, 168, 1, 10]));
        assert_eq!(parse_ipv4_dotted_decimal("192.168.1"), Err(UdpPortError::InvalidBroadcastAddress));
        assert_eq!(parse_ipv4_dotted_decimal("256.1.1.1"), Err(UdpPortError::InvalidBroadcastAddress));
        assert_eq!(parse_ipv4_dotted_decimal("1234567890123456"), Err(UdpPortError::BroadcastAddressTooLong));
    }

    #[test]
    fn open_socket_binds_udp_port_discovers_local_ips_and_sets_socket_options() {
        let (interface, api) = opened();
        assert_eq!(interface.port(), 8054);
        assert_eq!(interface.socket(), Some(1));
        assert!(interface.winsock_initialized());
        assert_eq!(api.binds[0], (1, UdpSocketAddress::any(8054)));
        assert_eq!(interface.local_addresses(), &[[10, 0, 0, 2], [192, 168, 1, 5]]);
        assert!(api.options.contains(&(1, UdpSocketOption::LingerDisabled)));
        assert_eq!(api.base_options_count, 1);
    }

    #[test]
    fn open_socket_rolls_back_socket_when_bind_or_local_discovery_fails() {
        let mut interface = RedAlertUdpInterface::new();
        let mut api = FakeApi::new();
        api.fail_bind = true;
        assert_eq!(interface.open_socket(8054, &mut api), Err(UdpPortError::SocketBindFailed));
        assert_eq!(api.closed, vec![1]);
        assert_eq!(interface.socket(), None);

        let mut interface = RedAlertUdpInterface::new();
        let mut api = FakeApi::new();
        api.fail_discovery = true;
        assert_eq!(interface.open_socket(8054, &mut api), Err(UdpPortError::LocalAddressDiscoveryFailed));
        assert_eq!(api.closed, vec![1]);
    }

    #[test]
    fn configured_broadcasts_fan_out_in_insertion_order() {
        let mut interface = RedAlertUdpInterface::<u64>::new();
        interface.set_broadcast_address("10.0.0.255").unwrap();
        interface.set_broadcast_address("192.168.1.255").unwrap();
        assert_eq!(interface.broadcast(b"hello"), Ok(2));
        assert_eq!(interface.outgoing_count(), 2);
    }

    #[test]
    fn read_event_discards_local_addresses_and_queues_remote_sources() {
        let (mut interface, mut api) = opened();
        api.reads.push_back(Ok(Some((10, [10, 0, 0, 2]))));
        interface.message_handler(UDP_MESSAGE_ID, UdpSocketEvent::Read, 0, &mut api);
        assert_eq!(interface.incoming_count(), 0);

        api.reads.push_back(Ok(Some((10, [203, 0, 113, 20]))));
        interface.message_handler(UDP_MESSAGE_ID, UdpSocketEvent::Read, 0, &mut api);
        let packet = interface.pop_incoming().unwrap();
        assert_eq!(packet.payload, b"udp-packet");
        assert_eq!(packet.source_ipv4, [203, 0, 113, 20]);
    }

    #[test]
    fn udp_read_clears_errors_even_when_recv_would_block() {
        let (mut interface, mut api) = opened();
        api.reads.push_back(Err(UdpSocketError::WouldBlock));
        interface.message_handler(UDP_MESSAGE_ID, UdpSocketEvent::Read, 0, &mut api);
        assert_eq!(api.clear_count, 1);
    }

    #[test]
    fn write_sends_the_first_packet_to_configured_port_and_destination() {
        let (mut interface, mut api) = opened();
        interface.set_broadcast_address("192.0.2.255").unwrap();
        interface.broadcast(b"discovery").unwrap();
        interface.message_handler(UDP_MESSAGE_ID, UdpSocketEvent::Write, 0, &mut api);
        assert_eq!(interface.outgoing_count(), 0);
        assert_eq!(api.sent.len(), 1);
        assert_eq!(api.sent[0].0, b"discovery");
        assert_eq!(api.sent[0].1, UdpSocketAddress::new([192, 0, 2, 255], 8054));
    }

    #[test]
    fn would_block_dequeues_packet_but_fatal_send_error_keeps_it() {
        let (mut interface, mut api) = opened();
        interface.queue_send_to([198, 51, 100, 5], b"first").unwrap();
        api.writes.push_back(Err(UdpSocketError::WouldBlock));
        interface.message_handler(UDP_MESSAGE_ID, UdpSocketEvent::Write, 0, &mut api);
        assert_eq!(interface.outgoing_count(), 0);

        interface.queue_send_to([198, 51, 100, 6], b"retry").unwrap();
        api.writes.push_back(Err(UdpSocketError::Failure(9)));
        interface.message_handler(UDP_MESSAGE_ID, UdpSocketEvent::Write, 0, &mut api);
        assert_eq!(interface.outgoing_count(), 1);
        assert_eq!(api.clear_count, 1);
    }

    #[test]
    fn event_error_and_wrong_message_follow_winsock_contract() {
        let (mut interface, mut api) = opened();
        assert_eq!(interface.message_handler(1, UdpSocketEvent::Read, 0, &mut api), 1);
        assert_eq!(interface.message_handler(UDP_MESSAGE_ID, UdpSocketEvent::Read, 5, &mut api), 0);
        assert_eq!(api.clear_count, 1);
    }

    #[test]
    fn outgoing_and_incoming_packets_respect_fixed_buffer_bounds() {
        let (mut interface, _) = opened();
        let too_large = vec![0; UDP_PACKET_BUFFER_SIZE + 1];
        assert_eq!(interface.queue_send_to([127, 0, 0, 1], &too_large), Err(UdpPortError::PacketTooLarge));
        assert_eq!(interface.broadcast(&too_large), Err(UdpPortError::PacketTooLarge));
    }

    #[test]
    fn close_releases_socket_and_clears_address_and_packet_state() {
        let (mut interface, mut api) = opened();
        interface.set_broadcast_address("239.255.0.1").unwrap();
        interface.queue_send_to([1, 2, 3, 4], b"queued").unwrap();
        interface.close(&mut api);
        assert_eq!(api.closed, vec![1]);
        assert_eq!(interface.socket(), None);
        assert!(interface.local_addresses().is_empty());
        assert!(interface.broadcast_addresses().is_empty());
        assert_eq!(interface.outgoing_count(), 0);
        assert!(!interface.winsock_initialized());
    }
}
