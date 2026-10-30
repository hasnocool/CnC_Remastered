//! Red Alert Winsock/IPX adapter ported from REDALERT/WSPIPX.CPP.
//!
//! Upstream baseline: electronicarts/CnC_Remastered_Collection at
//! f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae.
//!
//! Implements the three active IPXInterfaceClass methods: adapter/network-card
//! discovery, IPX socket creation/bind/options, and asynchronous read/write
//! message handling. Winsock calls are injected through IpxSocketApi so this
//! layer remains portable and does not block a game thread.

use std::collections::VecDeque;
use crate::ipx_address::IpxAddress;

pub const IPX_MESSAGE_ID: u32 = 0x0400 + 115;
pub const IPX_RECEIVE_BUFFER_SIZE: usize = 1024;
pub const IPX_SOCKET_ADDRESS_LENGTH: usize = 16;
pub const IPX_PACKET_TYPE: u32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpxSocketFamily { AfIpx, AfNs }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpxSocketKind { Datagram }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpxSocketProtocol { NspIpx }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpxSocketOption {
    Broadcast,
    PacketType,
    FilterPacketType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpxSocketError {
    WouldBlock,
    Failure(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpxSocketEvent { Read, Write, Other(u16) }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpxSocketAddress {
    pub family: IpxSocketFamily,
    pub network: [u8; 4],
    pub node: [u8; 6],
    /// Already in network byte order, as htons(socketnum) in the C++ source.
    pub socket: u16,
}

impl IpxSocketAddress {
    pub const fn wildcard() -> Self {
        Self {
            family: IpxSocketFamily::AfIpx,
            network: [0; 4],
            node: [0; 6],
            socket: 0,
        }
    }

    pub fn new(
        family: IpxSocketFamily,
        network: [u8; 4],
        node: [u8; 6],
        host_socket: u16,
    ) -> Self {
        Self { family, network, node, socket: host_socket.to_be() }
    }

    pub const fn ipx_address(self) -> IpxAddress {
        IpxAddress::new(self.network, self.node)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdapterAddress {
    pub network: [u8; 4],
    pub node: [u8; 6],
}

/// Platform boundary for the Winsock functions used by WSPIPX.CPP.
pub trait IpxSocketApi {
    type Socket: Copy + Eq;

    fn initialize_winsock(&mut self) -> bool;
    fn open_socket(
        &mut self,
        family: IpxSocketFamily,
        kind: IpxSocketKind,
        protocol: IpxSocketProtocol,
    ) -> Result<Self::Socket, IpxSocketError>;
    fn bind(&mut self, socket: Self::Socket, address: &IpxSocketAddress) -> Result<(), IpxSocketError>;
    fn get_adapter_address(
        &mut self,
        socket: Self::Socket,
        card_number: i32,
    ) -> Result<AdapterAddress, IpxSocketError>;
    fn set_option(
        &mut self,
        socket: Self::Socket,
        option: IpxSocketOption,
        value: u32,
    ) -> Result<(), IpxSocketError>;
    fn set_base_socket_options(&mut self, socket: Self::Socket) -> bool;
    fn close_socket(&mut self, socket: Self::Socket);
    fn receive_from(
        &mut self,
        socket: Self::Socket,
        destination: &mut [u8],
    ) -> Result<Option<(usize, IpxSocketAddress)>, IpxSocketError>;
    fn send_to(
        &mut self,
        socket: Self::Socket,
        packet: &[u8],
        destination: &IpxSocketAddress,
    ) -> Result<usize, IpxSocketError>;
    fn clear_socket_error(&mut self, socket: Self::Socket);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpxPortError {
    WinsockInitializationFailed,
    SocketCreationFailed,
    SocketBindFailed,
    AdapterAddressUnavailable,
    PacketTooLarge,
    SocketNotOpen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpxReceivedPacket {
    pub source: IpxAddress,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpxOutgoingPacket {
    pub address: IpxAddress,
    pub payload: Vec<u8>,
    pub is_broadcast: bool,
}

/// IPXInterfaceClass fields and queues from WSPIPX.H/WSPROTO.H.
#[derive(Debug)]
pub struct RedAlertIpxInterface<S> {
    winsock_initialized: bool,
    socket: Option<S>,
    ipx_socket_number: u16,
    broadcast_network: [u8; 4],
    broadcast_node: [u8; 6],
    my_node: [u8; 6],
    incoming: VecDeque<IpxReceivedPacket>,
    outgoing: VecDeque<IpxOutgoingPacket>,
}

impl<S: Copy + Eq> Default for RedAlertIpxInterface<S> {
    fn default() -> Self { Self::new() }
}

impl<S: Copy + Eq> RedAlertIpxInterface<S> {
    /// Port of IPXInterfaceClass constructor. All three addresses start as FF.
    pub fn new() -> Self {
        Self {
            winsock_initialized: false,
            socket: None,
            ipx_socket_number: 0,
            broadcast_network: [0xFF; 4],
            broadcast_node: [0xFF; 6],
            my_node: [0xFF; 6],
            incoming: VecDeque::new(),
            outgoing: VecDeque::new(),
        }
    }

    pub fn winsock_initialized(&self) -> bool { self.winsock_initialized }
    pub fn socket(&self) -> Option<S> { self.socket }
    pub fn socket_number(&self) -> u16 { self.ipx_socket_number }
    pub fn broadcast_network(&self) -> [u8; 4] { self.broadcast_network }
    pub fn broadcast_node(&self) -> [u8; 6] { self.broadcast_node }
    pub fn my_node(&self) -> [u8; 6] { self.my_node }
    pub fn incoming_count(&self) -> usize { self.incoming.len() }
    pub fn outgoing_count(&self) -> usize { self.outgoing.len() }
    pub fn pop_incoming(&mut self) -> Option<IpxReceivedPacket> { self.incoming.pop_front() }

    /// The original source initializes MyNode to FF and other networking code
    /// may replace it with the local node identifier.
    pub fn set_my_node(&mut self, node: [u8; 6]) { self.my_node = node; }

    pub fn queue_send_to(
        &mut self,
        address: IpxAddress,
        payload: &[u8],
    ) -> Result<(), IpxPortError> {
        if payload.len() > IPX_RECEIVE_BUFFER_SIZE { return Err(IpxPortError::PacketTooLarge); }
        self.outgoing.push_back(IpxOutgoingPacket {
            address,
            payload: payload.to_vec(),
            is_broadcast: false,
        });
        Ok(())
    }

    pub fn queue_broadcast(&mut self, payload: &[u8]) -> Result<(), IpxPortError> {
        if payload.len() > IPX_RECEIVE_BUFFER_SIZE { return Err(IpxPortError::PacketTooLarge); }
        self.outgoing.push_back(IpxOutgoingPacket {
            address: IpxAddress::broadcast(),
            payload: payload.to_vec(),
            is_broadcast: true,
        });
        Ok(())
    }

    /// Port of IPXInterfaceClass::Get_Network_Card_Address. A temporary AF_IPX
    /// datagram socket is opened, bound, queried, and always closed on success
    /// or on bind/query failure.
    pub fn get_network_card_address<A: IpxSocketApi>(
        &mut self,
        card_number: i32,
        address: &mut IpxSocketAddress,
        api: &mut A,
    ) -> bool {
        let socket = match api.open_socket(
            IpxSocketFamily::AfIpx,
            IpxSocketKind::Datagram,
            IpxSocketProtocol::NspIpx,
        ) {
            Ok(socket) => socket,
            Err(_) => return false,
        };

        if api.bind(socket, &IpxSocketAddress::wildcard()).is_err() {
            api.close_socket(socket);
            return false;
        }

        let adapter = match api.get_adapter_address(socket, card_number) {
            Ok(adapter) => adapter,
            Err(_) => {
                api.close_socket(socket);
                return false;
            }
        };

        address.network = adapter.network;
        address.node = adapter.node;
        self.broadcast_network = adapter.network;
        api.close_socket(socket);
        true
    }

    /// Port of IPXInterfaceClass::Open_Socket. Socket options whose failures
    /// were assert-only in the legacy debug build remain best-effort.
    pub fn open_socket<A: IpxSocketApi<Socket = S>>(
        &mut self,
        socket_number: u16,
        api: &mut A,
    ) -> Result<(), IpxPortError> {
        if !self.winsock_initialized {
            if !api.initialize_winsock() {
                return Err(IpxPortError::WinsockInitializationFailed);
            }
            self.winsock_initialized = true;
        }

        self.ipx_socket_number = socket_number;
        let mut address = IpxSocketAddress::new(
            IpxSocketFamily::AfIpx,
            [0; 4],
            [0xFF; 6],
            socket_number,
        );

        let socket = api.open_socket(
            IpxSocketFamily::AfNs,
            IpxSocketKind::Datagram,
            IpxSocketProtocol::NspIpx,
        ).map_err(|_| IpxPortError::SocketCreationFailed)?;
        self.socket = Some(socket);

        if !self.get_network_card_address(0, &mut address, api) {
            api.close_socket(socket);
            self.socket = None;
            return Err(IpxPortError::AdapterAddressUnavailable);
        }

        if api.bind(socket, &address).is_err() {
            api.close_socket(socket);
            self.socket = None;
            return Err(IpxPortError::SocketBindFailed);
        }

        // SO_BROADCAST is needed for Win95 broadcast compatibility.
        let _ = api.set_option(socket, IpxSocketOption::Broadcast, 1);
        let _ = api.set_option(socket, IpxSocketOption::PacketType, IPX_PACKET_TYPE);
        let _ = api.set_option(socket, IpxSocketOption::FilterPacketType, IPX_PACKET_TYPE);
        let _ = api.set_base_socket_options(socket);
        Ok(())
    }

    /// Port of IPXInterfaceClass::Message_Handler.
    /// Returns 1 for messages owned by another handler, otherwise 0.
    pub fn message_handler<A: IpxSocketApi<Socket = S>>(
        &mut self,
        message: u32,
        event: IpxSocketEvent,
        event_error: i32,
        api: &mut A,
    ) -> i32 {
        if message != IPX_MESSAGE_ID { return 1; }
        let Some(socket) = self.socket else { return 0; };

        if event_error != 0 {
            api.clear_socket_error(socket);
            return 0;
        }

        match event {
            IpxSocketEvent::Read => {
                let mut receive_buffer = [0_u8; IPX_RECEIVE_BUFFER_SIZE];
                match api.receive_from(socket, &mut receive_buffer) {
                    Ok(Some((received, sender))) if received > 0 => {
                        if received > receive_buffer.len() { return 0; }
                        let sender_ipx = sender.ipx_address();
                        if *sender_ipx.network_number() == self.broadcast_network
                            && *sender_ipx.node_address() == self.my_node
                        {
                            return 0;
                        }
                        self.incoming.push_back(IpxReceivedPacket {
                            source: sender_ipx,
                            payload: receive_buffer[..received].to_vec(),
                        });
                    }
                    Ok(_) | Err(IpxSocketError::WouldBlock) => {}
                    Err(IpxSocketError::Failure(_)) => api.clear_socket_error(socket),
                }
                0
            }
            IpxSocketEvent::Write => {
                while let Some(packet) = self.outgoing.front() {
                    let destination = if packet.is_broadcast {
                        IpxSocketAddress::new(
                            IpxSocketFamily::AfIpx,
                            self.broadcast_network,
                            self.broadcast_node,
                            self.ipx_socket_number,
                        )
                    } else {
                        IpxSocketAddress::new(
                            IpxSocketFamily::AfIpx,
                            *packet.address.network_number(),
                            *packet.address.node_address(),
                            self.ipx_socket_number,
                        )
                    };

                    match api.send_to(socket, &packet.payload, &destination) {
                        Ok(_) => {
                            self.outgoing.pop_front();
                        }
                        Err(IpxSocketError::WouldBlock) => {
                            // WSPIPX.CPP falls through to Delete(packet) even
                            // after WSAEWOULDBLOCK; preserve that legacy behavior.
                            self.outgoing.pop_front();
                        }
                        Err(IpxSocketError::Failure(_)) => {
                            api.clear_socket_error(socket);
                            break;
                        }
                    }
                }
                0
            }
            IpxSocketEvent::Other(_) => 0,
        }
    }
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
        fail_adapter: bool,
        adapter: AdapterAddress,
        closed: Vec<u64>,
        bound: Vec<(u64, IpxSocketAddress)>,
        opened: Vec<(u64, IpxSocketFamily, IpxSocketKind, IpxSocketProtocol)>,
        options: Vec<(u64, IpxSocketOption, u32)>,
        reads: VecDeque<Result<Option<(usize, IpxSocketAddress)>, IpxSocketError>>,
        writes: VecDeque<Result<usize, IpxSocketError>>,
        sent: Vec<(Vec<u8>, IpxSocketAddress)>,
        clear_count: usize,
        base_options_count: usize,
    }

    impl FakeApi {
        fn new() -> Self {
            Self {
                adapter: AdapterAddress { network: [1, 2, 3, 4], node: [5, 6, 7, 8, 9, 10] },
                ..Self::default()
            }
        }
    }

    impl IpxSocketApi for FakeApi {
        type Socket = u64;

        fn initialize_winsock(&mut self) -> bool {
            if self.fail_initialize { return false; }
            self.initialized = true;
            true
        }

        fn open_socket(
            &mut self,
            family: IpxSocketFamily,
            kind: IpxSocketKind,
            protocol: IpxSocketProtocol,
        ) -> Result<Self::Socket, IpxSocketError> {
            if self.fail_open { return Err(IpxSocketError::Failure(100)); }
            self.next_socket += 1;
            let socket = self.next_socket;
            self.opened.push((socket, family, kind, protocol));
            Ok(socket)
        }

        fn bind(&mut self, socket: Self::Socket, address: &IpxSocketAddress) -> Result<(), IpxSocketError> {
            self.bound.push((socket, *address));
            if self.fail_bind { Err(IpxSocketError::Failure(101)) } else { Ok(()) }
        }

        fn get_adapter_address(
            &mut self,
            _socket: Self::Socket,
            _card_number: i32,
        ) -> Result<AdapterAddress, IpxSocketError> {
            if self.fail_adapter { Err(IpxSocketError::Failure(102)) } else { Ok(self.adapter) }
        }

        fn set_option(
            &mut self,
            socket: Self::Socket,
            option: IpxSocketOption,
            value: u32,
        ) -> Result<(), IpxSocketError> {
            self.options.push((socket, option, value));
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
        ) -> Result<Option<(usize, IpxSocketAddress)>, IpxSocketError> {
            match self.reads.pop_front().unwrap_or(Ok(None)) {
                Ok(Some((length, address))) => {
                    let bytes = b"test-packet";
                    let count = length.min(bytes.len()).min(destination.len());
                    destination[..count].copy_from_slice(&bytes[..count]);
                    Ok(Some((count, address)))
                }
                other => other,
            }
        }

        fn send_to(
            &mut self,
            _socket: Self::Socket,
            packet: &[u8],
            destination: &IpxSocketAddress,
        ) -> Result<usize, IpxSocketError> {
            self.sent.push((packet.to_vec(), *destination));
            self.writes.pop_front().unwrap_or(Ok(packet.len()))
        }

        fn clear_socket_error(&mut self, _socket: Self::Socket) { self.clear_count += 1; }
    }

    fn interface() -> (RedAlertIpxInterface<u64>, FakeApi) {
        let mut interface = RedAlertIpxInterface::new();
        let mut api = FakeApi::new();
        interface.open_socket(0x8813, &mut api).unwrap();
        (interface, api)
    }

    #[test]
    fn constructor_defaults_all_ipx_addresses_to_broadcast_values() {
        let interface: RedAlertIpxInterface<u64> = RedAlertIpxInterface::new();
        assert_eq!(interface.broadcast_network(), [0xFF; 4]);
        assert_eq!(interface.broadcast_node(), [0xFF; 6]);
        assert_eq!(interface.my_node(), [0xFF; 6]);
        assert!(!interface.winsock_initialized());
        assert_eq!(interface.socket(), None);
    }

    #[test]
    fn get_network_card_address_binds_temporary_socket_and_updates_broadcast_net() {
        let mut interface: RedAlertIpxInterface<u64> = RedAlertIpxInterface::new();
        let mut api = FakeApi::new();
        let mut address = IpxSocketAddress::wildcard();
        assert!(interface.get_network_card_address(0, &mut address, &mut api));
        assert_eq!(address.network, [1, 2, 3, 4]);
        assert_eq!(address.node, [5, 6, 7, 8, 9, 10]);
        assert_eq!(interface.broadcast_network(), [1, 2, 3, 4]);
        assert_eq!(api.closed, vec![1]);
        assert_eq!(api.bound[0].1, IpxSocketAddress::wildcard());
    }

    #[test]
    fn adapter_discovery_closes_temporary_socket_on_bind_or_query_failure() {
        let mut interface: RedAlertIpxInterface<u64> = RedAlertIpxInterface::new();
        let mut api = FakeApi::new();
        api.fail_bind = true;
        assert!(!interface.get_network_card_address(0, &mut IpxSocketAddress::wildcard(), &mut api));
        assert_eq!(api.closed, vec![1]);

        let mut api = FakeApi::new();
        api.fail_adapter = true;
        assert!(!interface.get_network_card_address(0, &mut IpxSocketAddress::wildcard(), &mut api));
        assert_eq!(api.closed, vec![1]);
    }

    #[test]
    fn open_socket_initializes_winsock_binds_adapter_and_sets_ipx_options() {
        let mut interface: RedAlertIpxInterface<u64> = RedAlertIpxInterface::new();
        let mut api = FakeApi::new();
        assert!(interface.open_socket(0x8813, &mut api).is_ok());
        assert!(interface.winsock_initialized());
        assert_eq!(interface.socket(), Some(1));
        assert_eq!(interface.socket_number(), 0x8813);
        assert_eq!(api.opened.len(), 2);
        assert_eq!(api.opened[0].1, IpxSocketFamily::AfNs);
        assert_eq!(api.opened[1].1, IpxSocketFamily::AfIpx);
        let bound = api.bound.iter().find(|(socket, _)| *socket == 1).unwrap().1;
        assert_eq!(bound.family, IpxSocketFamily::AfIpx);
        assert_eq!(bound.network, [1, 2, 3, 4]);
        assert_eq!(bound.node, [5, 6, 7, 8, 9, 10]);
        assert_eq!(bound.socket, 0x8813_u16.to_be());
        assert!(api.options.contains(&(1, IpxSocketOption::Broadcast, 1)));
        assert!(api.options.contains(&(1, IpxSocketOption::PacketType, IPX_PACKET_TYPE)));
        assert!(api.options.contains(&(1, IpxSocketOption::FilterPacketType, IPX_PACKET_TYPE)));
        assert_eq!(api.base_options_count, 1);
    }

    #[test]
    fn open_socket_rolls_back_socket_on_adapter_or_bind_failure() {
        let mut interface: RedAlertIpxInterface<u64> = RedAlertIpxInterface::new();
        let mut api = FakeApi::new();
        api.fail_adapter = true;
        assert_eq!(interface.open_socket(0x8813, &mut api), Err(IpxPortError::AdapterAddressUnavailable));
        assert_eq!(api.closed, vec![1]);
        assert_eq!(interface.socket(), None);

        let mut interface: RedAlertIpxInterface<u64> = RedAlertIpxInterface::new();
        let mut api = FakeApi::new();
        api.fail_bind = true;
        assert_eq!(interface.open_socket(0x8813, &mut api), Err(IpxPortError::SocketBindFailed));
        assert!(api.closed.contains(&1));
        assert_eq!(interface.socket(), None);
    }

    #[test]
    fn handler_ignores_other_messages_and_clears_reported_socket_errors() {
        let (mut interface, mut api) = interface();
        assert_eq!(interface.message_handler(1, IpxSocketEvent::Read, 0, &mut api), 1);
        assert_eq!(interface.message_handler(IPX_MESSAGE_ID, IpxSocketEvent::Read, 5, &mut api), 0);
        assert_eq!(api.clear_count, 1);
    }

    #[test]
    fn read_event_drops_self_packets_and_queues_remote_packets() {
        let (mut interface, mut api) = interface();
        interface.set_my_node([5, 6, 7, 8, 9, 10]);
        api.reads.push_back(Ok(Some((11, IpxSocketAddress::new(
            IpxSocketFamily::AfIpx, [1, 2, 3, 4], [5, 6, 7, 8, 9, 10], 0,
        )))));
        interface.message_handler(IPX_MESSAGE_ID, IpxSocketEvent::Read, 0, &mut api);
        assert_eq!(interface.incoming_count(), 0);

        api.reads.push_back(Ok(Some((11, IpxSocketAddress::new(
            IpxSocketFamily::AfIpx, [1, 2, 3, 4], [9, 8, 7, 6, 5, 4], 0,
        )))));
        interface.message_handler(IPX_MESSAGE_ID, IpxSocketEvent::Read, 0, &mut api);
        let packet = interface.pop_incoming().unwrap();
        assert_eq!(packet.payload, b"test-packet");
        assert_eq!(*packet.source.node_address(), [9, 8, 7, 6, 5, 4]);
    }

    #[test]
    fn read_would_block_is_not_cleared_but_other_recv_error_is() {
        let (mut interface, mut api) = interface();
        api.reads.push_back(Err(IpxSocketError::WouldBlock));
        interface.message_handler(IPX_MESSAGE_ID, IpxSocketEvent::Read, 0, &mut api);
        assert_eq!(api.clear_count, 0);
        api.reads.push_back(Err(IpxSocketError::Failure(88)));
        interface.message_handler(IPX_MESSAGE_ID, IpxSocketEvent::Read, 0, &mut api);
        assert_eq!(api.clear_count, 1);
    }

    #[test]
    fn write_event_sends_broadcast_and_unicast_to_correct_destinations() {
        let (mut interface, mut api) = interface();
        interface.queue_broadcast(b"all").unwrap();
        let target = IpxAddress::new([10, 11, 12, 13], [14, 15, 16, 17, 18, 19]);
        interface.queue_send_to(target, b"one").unwrap();
        interface.message_handler(IPX_MESSAGE_ID, IpxSocketEvent::Write, 0, &mut api);
        assert_eq!(interface.outgoing_count(), 0);
        assert_eq!(api.sent.len(), 2);
        assert_eq!(api.sent[0].0, b"all");
        assert_eq!(api.sent[0].1.network, [1, 2, 3, 4]);
        assert_eq!(api.sent[0].1.node, [0xFF; 6]);
        assert_eq!(api.sent[1].0, b"one");
        assert_eq!(api.sent[1].1.network, [10, 11, 12, 13]);
        assert_eq!(api.sent[1].1.node, [14, 15, 16, 17, 18, 19]);
        assert_eq!(api.sent[1].1.socket, 0x8813_u16.to_be());
    }

    #[test]
    fn write_event_preserves_legacy_would_block_dequeue_and_fatal_error_retention() {
        let (mut interface, mut api) = interface();
        interface.queue_broadcast(b"first").unwrap();
        interface.queue_broadcast(b"second").unwrap();
        api.writes.push_back(Err(IpxSocketError::WouldBlock));
        interface.message_handler(IPX_MESSAGE_ID, IpxSocketEvent::Write, 0, &mut api);
        assert_eq!(interface.outgoing_count(), 0);
        assert_eq!(api.sent.len(), 2);

        interface.queue_broadcast(b"retry").unwrap();
        api.writes.push_back(Err(IpxSocketError::Failure(90)));
        interface.message_handler(IPX_MESSAGE_ID, IpxSocketEvent::Write, 0, &mut api);
        assert_eq!(interface.outgoing_count(), 1);
        assert_eq!(api.clear_count, 1);
    }

    #[test]
    fn packets_larger_than_fixed_winsock_buffer_are_rejected() {
        let (mut interface, _) = interface();
        let oversized = vec![0; IPX_RECEIVE_BUFFER_SIZE + 1];
        assert_eq!(
            interface.queue_broadcast(&oversized),
            Err(IpxPortError::PacketTooLarge),
        );
    }
}
