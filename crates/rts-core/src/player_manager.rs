//! Red Alert DOS-side multiplayer queue manager from MPMGRD.CPP.
//!
//! Queue IDs and packet layout follow the source. MGen queue movement and
//! shared-memory node access are injected through MPlayerTransport.

pub const STATUS_BAD: i32 = 0;
pub const STATUS_OK: i32 = 1;
pub const CONNECTION_NONE: i32 = -1;
pub const BROADCAST_ADDRESS: u32 = 0;
pub const CONNECT_MAX: usize = 7;
pub const MAX_NAME_LEN: usize = 128;
pub const PACKET_ADDRESS_SIZE: usize = 4;

pub const FREE_QUEUE: u16 = 0;
pub const DOS_WORK_QUEUE: u16 = 1;
pub const WIN_WORK_QUEUE: u16 = 2;
pub const WIN_SEND_QUEUE: u16 = 3;
pub const DOS_PENDING_QUEUE: u16 = 4;
pub const WIN_WORK_QUEUE_2: u16 = 5;
pub const GLOBAL_DOS_WORK_QUEUE: u16 = 14;
pub const GLOBAL_WIN_WORK_QUEUE: u16 = 15;
pub const GLOBAL_WIN_SEND_QUEUE: u16 = 16;
pub const GLOBAL_DOS_PENDING_QUEUE: u16 = 17;
pub const GLOBAL_WIN_WORK_QUEUE_2: u16 = 18;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PacketNode {
    pub address: u32,
    /// Legacy RTQ_NODE::rtqUpCtr includes the four-byte address prefix.
    pub up_count: u16,
    pub data: Vec<u8>,
}

pub trait MPlayerTransport {
    type NodeHandle: Copy;
    fn master_node_size(&mut self) -> u32;
    fn expected_node_size(&self) -> u32;
    fn local_address(&mut self) -> i32;
    fn move_node(&mut self, from_queue: u16, to_queue: u16) -> Option<Self::NodeHandle>;
    fn node(&self, handle: Self::NodeHandle) -> Option<&PacketNode>;
    fn node_mut(&mut self, handle: Self::NodeHandle) -> Option<&mut PacketNode>;
    fn queue_count(&mut self, queue: u16) -> i32;
    fn post_windows_message(&mut self);
    fn yield_to_windows(&mut self);
    fn flush_nodes(&mut self, from_queue: u16, to_queue: u16);
    fn game_num_players(&mut self) -> i32;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlayerManagerError {
    NodeSizeMismatch { actual: u32, expected: u32 },
    NoFreeNode,
    InvalidConnection(i32),
    PayloadTooLarge { length: usize, capacity: usize },
    MissingNode,
    MalformedPacket,
    ConnectionLimit,
    InvalidConnectionIndex(usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerConnection {
    pub id: i32,
    pub address: i32,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MPlayerManager {
    pub my_address: i32,
    connections: Vec<PlayerConnection>,
}

impl MPlayerManager {
    pub fn new(transport: &mut impl MPlayerTransport) -> Result<Self, PlayerManagerError> {
        let actual = transport.master_node_size();
        let expected = transport.expected_node_size();
        if actual != expected {
            return Err(PlayerManagerError::NodeSizeMismatch { actual, expected });
        }
        Ok(Self { my_address: transport.local_address(), connections: Vec::new() })
    }

    pub fn num_connections(&self) -> usize { self.connections.len() }
    pub fn connection_index(&self, id: i32) -> Option<usize> {
        self.connections.iter().position(|connection| connection.id == id)
    }
    pub fn connection_id(&self, index: usize) -> Result<i32, PlayerManagerError> {
        self.connections.get(index).map(|c| c.id).ok_or(PlayerManagerError::InvalidConnectionIndex(index))
    }
    pub fn connection_name(&self, id: i32) -> Option<&str> {
        self.connection_index(id).map(|i| self.connections[i].name.as_str())
    }
    pub fn connection_address(&self, id: i32) -> i32 {
        self.connection_index(id).map_or(0, |i| self.connections[i].address)
    }

    pub fn create_connection(&mut self, id: i32, name: &str, address: i32) -> Result<i32, PlayerManagerError> {
        if self.connections.len() >= CONNECT_MAX { return Err(PlayerManagerError::ConnectionLimit); }
        let mut bounded_name = name.to_owned();
        bounded_name.truncate(MAX_NAME_LEN - 1);
        self.connections.push(PlayerConnection { id, address, name: bounded_name });
        Ok(STATUS_OK)
    }

    pub fn delete_connection(&mut self, id: i32) -> i32 {
        let Some(index) = self.connection_index(id) else { return STATUS_BAD };
        self.connections.remove(index);
        STATUS_OK
    }

    pub fn send_private_message(
        &self, transport: &mut impl MPlayerTransport, data: &[u8], conn_id: i32,
    ) -> Result<i32, PlayerManagerError> {
        if self.connections.is_empty() { return Ok(STATUS_OK); }
        let address = if conn_id == CONNECTION_NONE {
            BROADCAST_ADDRESS
        } else {
            let index = self.connection_index(conn_id).ok_or(PlayerManagerError::InvalidConnection(conn_id))?;
            self.connections[index].address as u32
        };
        self.send_packet(transport, data, address, WIN_SEND_QUEUE)
    }

    pub fn get_private_message(
        &self, transport: &mut impl MPlayerTransport, capacity: usize,
    ) -> Result<Option<(Vec<u8>, i32)>, PlayerManagerError> {
        let Some(handle) = transport.move_node(DOS_PENDING_QUEUE, DOS_WORK_QUEUE) else { return Ok(None) };
        let node = transport.node(handle).cloned().ok_or(PlayerManagerError::MissingNode)?;
        let payload = packet_payload(&node, capacity)?;
        let connection = self.connections.iter()
            .find(|c| c.address as u32 == node.address)
            .map_or(CONNECTION_NONE, |c| c.id);
        transport.move_node(DOS_WORK_QUEUE, FREE_QUEUE);
        Ok(Some((payload, connection)))
    }

    pub fn send_global_message(
        &self, transport: &mut impl MPlayerTransport, data: &[u8], address: i32,
    ) -> Result<i32, PlayerManagerError> {
        let address = if address == 0 { BROADCAST_ADDRESS } else { address as u32 };
        self.send_packet(transport, data, address, GLOBAL_WIN_SEND_QUEUE)
    }

    pub fn get_global_message(
        &self, transport: &mut impl MPlayerTransport, capacity: usize,
    ) -> Result<Option<(Vec<u8>, i32)>, PlayerManagerError> {
        let Some(handle) = transport.move_node(GLOBAL_DOS_PENDING_QUEUE, DOS_WORK_QUEUE) else { return Ok(None) };
        let node = transport.node(handle).cloned().ok_or(PlayerManagerError::MissingNode)?;
        let payload = packet_payload(&node, capacity)?;
        let address = if node.address == BROADCAST_ADDRESS { 0 } else { node.address as i32 };
        transport.move_node(DOS_WORK_QUEUE, FREE_QUEUE);
        Ok(Some((payload, address)))
    }

    fn send_packet(
        &self, transport: &mut impl MPlayerTransport, data: &[u8], address: u32, destination_queue: u16,
    ) -> Result<i32, PlayerManagerError> {
        let handle = transport.move_node(FREE_QUEUE, DOS_WORK_QUEUE).ok_or(PlayerManagerError::NoFreeNode)?;
        let node = transport.node_mut(handle).ok_or(PlayerManagerError::MissingNode)?;
        if data.len() > node.data.len() {
            return Err(PlayerManagerError::PayloadTooLarge { length: data.len(), capacity: node.data.len() });
        }
        node.address = address;
        node.data[..data.len()].copy_from_slice(data);
        node.up_count = (data.len() as u16).wrapping_add(PACKET_ADDRESS_SIZE as u16);
        transport.move_node(DOS_WORK_QUEUE, destination_queue);
        transport.post_windows_message();
        transport.yield_to_windows();
        Ok(STATUS_OK)
    }

    pub fn service(&self) -> i32 { STATUS_OK }
    pub fn init(&self) -> i32 { STATUS_OK }
    pub fn find_num_connections(&self, transport: &mut impl MPlayerTransport) -> i32 {
        transport.game_num_players() - 1
    }
    pub fn global_num_send(&self) -> i32 { 0 }
    pub fn private_num_send(&self, _id: i32) -> i32 { 0 }
    pub fn global_num_receive(&self, transport: &mut impl MPlayerTransport) -> i32 {
        transport.queue_count(GLOBAL_DOS_PENDING_QUEUE)
    }
    pub fn private_num_receive(&self, transport: &mut impl MPlayerTransport) -> i32 {
        transport.queue_count(DOS_PENDING_QUEUE)
    }
    pub fn response_time(&self) -> u32 { (160 * 60) / 1000 }
    pub fn reset_response_time(&self) {}
    pub fn set_timing(&self, _retry_delta: u32, _max_retries: u32, _timeout: u32) {}
    pub fn configure_debug(&self, _index: i32, _type_offset: i32, _type_size: i32, _name_start: i32, _name_count: i32) {}
    pub fn mono_debug_print(&self, _index: i32, _refresh: i32) {}

    pub fn flush_all(&self, transport: &mut impl MPlayerTransport) {
        transport.flush_nodes(DOS_PENDING_QUEUE, FREE_QUEUE);
        transport.flush_nodes(GLOBAL_DOS_PENDING_QUEUE, FREE_QUEUE);
    }
}

fn packet_payload(node: &PacketNode, capacity: usize) -> Result<Vec<u8>, PlayerManagerError> {
    let length = (node.up_count as usize).checked_sub(PACKET_ADDRESS_SIZE)
        .ok_or(PlayerManagerError::MalformedPacket)?;
    if length > capacity {
        return Err(PlayerManagerError::PayloadTooLarge { length, capacity });
    }
    let bytes = node.data.get(..length).ok_or(PlayerManagerError::MalformedPacket)?;
    Ok(bytes.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeTransport {
        queues: std::collections::HashMap<u16, std::collections::VecDeque<u32>>,
        nodes: Vec<PacketNode>,
        posts: usize,
        yields: usize,
        flushes: Vec<(u16, u16)>,
        node_size: u32,
        expected_size: u32,
        players: i32,
    }
    impl FakeTransport {
        fn add_free_node(&mut self) {
            let id = self.nodes.len() as u32;
            self.nodes.push(PacketNode { address: 0, up_count: 0, data: vec![0; 128] });
            self.queues.entry(FREE_QUEUE).or_default().push_back(id);
        }
        fn push_pending(&mut self, queue: u16, address: u32, data: &[u8]) {
            let id = self.nodes.len() as u32;
            self.nodes.push(PacketNode { address, up_count: (data.len() as u16) + 4, data: data.to_vec() });
            self.queues.entry(queue).or_default().push_back(id);
        }
    }
    impl MPlayerTransport for FakeTransport {
        type NodeHandle = u32;
        fn master_node_size(&mut self) -> u32 { self.node_size }
        fn expected_node_size(&self) -> u32 { if self.expected_size == 0 { self.node_size } else { self.expected_size } }
        fn local_address(&mut self) -> i32 { 77 }
        fn move_node(&mut self, from: u16, to: u16) -> Option<u32> {
            let handle = self.queues.get_mut(&from)?.pop_front()?;
            self.queues.entry(to).or_default().push_back(handle);
            Some(handle)
        }
        fn node(&self, handle: u32) -> Option<&PacketNode> { self.nodes.get(handle as usize) }
        fn node_mut(&mut self, handle: u32) -> Option<&mut PacketNode> { self.nodes.get_mut(handle as usize) }
        fn queue_count(&mut self, queue: u16) -> i32 { self.queues.get(&queue).map_or(0, |q| q.len() as i32) }
        fn post_windows_message(&mut self) { self.posts += 1; }
        fn yield_to_windows(&mut self) { self.yields += 1; }
        fn flush_nodes(&mut self, from: u16, to: u16) {
            let moved = self.queues.entry(from).or_default().drain(..).collect::<Vec<_>>();
            self.queues.entry(to).or_default().extend(moved);
            self.flushes.push((from, to));
        }
        fn game_num_players(&mut self) -> i32 { self.players }
    }

    #[test]
    fn constructor_checks_rtq_node_size_and_reads_local_address() {
        let mut transport = FakeTransport { node_size: 64, ..Default::default() };
        let manager = MPlayerManager::new(&mut transport).unwrap();
        assert_eq!(manager.my_address, 77);
        transport.node_size = 32;
        transport.expected_size = 64;
        assert_eq!(MPlayerManager::new(&mut transport), Err(PlayerManagerError::NodeSizeMismatch { actual: 32, expected: 64 }));
    }

    #[test]
    fn connection_table_adds_finds_deletes_and_bounds_names() {
        let mut transport = FakeTransport::default();
        let mut manager = MPlayerManager::new(&mut transport).unwrap();
        manager.create_connection(9, "Player", 123).unwrap();
        manager.create_connection(10, &"x".repeat(140), 456).unwrap();
        assert_eq!(manager.num_connections(), 2);
        assert_eq!(manager.connection_index(10), Some(1));
        assert_eq!(manager.connection_name(9), Some("Player"));
        assert_eq!(manager.connection_name(10).unwrap().len(), 127);
        assert_eq!(manager.delete_connection(9), STATUS_OK);
        assert_eq!(manager.connection_id(0), Ok(10));
        assert_eq!(manager.delete_connection(99), STATUS_BAD);
    }

    #[test]
    fn private_send_prepends_address_and_posts_then_yields() {
        let mut transport = FakeTransport { node_size: 64, ..Default::default() };
        let mut manager = MPlayerManager::new(&mut transport).unwrap();
        manager.create_connection(5, "peer", 0x1234).unwrap();
        transport.add_free_node();
        assert_eq!(manager.send_private_message(&mut transport, b"hello", 5), Ok(STATUS_OK));
        let node = &transport.nodes[0];
        assert_eq!(node.address, 0x1234);
        assert_eq!(&node.data[..5], b"hello");
        assert_eq!(node.up_count, 9);
        assert_eq!(transport.posts, 1);
        assert_eq!(transport.yields, 1);
        assert_eq!(transport.queues.get(&WIN_SEND_QUEUE).unwrap().len(), 1);
    }

    #[test]
    fn private_broadcast_and_receive_restore_connection_id() {
        let mut transport = FakeTransport { node_size: 64, ..Default::default() };
        let mut manager = MPlayerManager::new(&mut transport).unwrap();
        manager.create_connection(42, "peer", 77).unwrap();
        transport.push_pending(DOS_PENDING_QUEUE, 77, b"payload");
        let received = manager.get_private_message(&mut transport, 32).unwrap().unwrap();
        assert_eq!(received, (b"payload".to_vec(), 42));
        assert_eq!(transport.queues.get(&FREE_QUEUE).unwrap().len(), 1);
        transport.add_free_node();
        manager.send_private_message(&mut transport, b"all", CONNECTION_NONE).unwrap();
        assert_eq!(transport.nodes[1].address, BROADCAST_ADDRESS);
    }

    #[test]
    fn global_messages_map_broadcast_address_to_zero() {
        let mut transport = FakeTransport { node_size: 64, ..Default::default() };
        let manager = MPlayerManager::new(&mut transport).unwrap();
        transport.add_free_node();
        manager.send_global_message(&mut transport, b"global", 0).unwrap();
        assert_eq!(transport.nodes[0].address, 0);
        transport.push_pending(GLOBAL_DOS_PENDING_QUEUE, 0, b"broadcast");
        assert_eq!(manager.get_global_message(&mut transport, 32).unwrap(), Some((b"broadcast".to_vec(), 0)));
    }

    #[test]
    fn empty_private_send_is_success_and_queue_stats_match_source() {
        let mut transport = FakeTransport { node_size: 64, players: 4, ..Default::default() };
        let manager = MPlayerManager::new(&mut transport).unwrap();
        assert_eq!(manager.send_private_message(&mut transport, b"x", 999), Ok(STATUS_OK));
        transport.push_pending(DOS_PENDING_QUEUE, 1, b"a");
        transport.push_pending(GLOBAL_DOS_PENDING_QUEUE, 2, b"b");
        assert_eq!(manager.private_num_receive(&mut transport), 1);
        assert_eq!(manager.global_num_receive(&mut transport), 1);
        assert_eq!(manager.find_num_connections(&mut transport), 3);
        assert_eq!(manager.response_time(), 9);
        manager.flush_all(&mut transport);
        assert_eq!(transport.flushes, vec![(DOS_PENDING_QUEUE, FREE_QUEUE), (GLOBAL_DOS_PENDING_QUEUE, FREE_QUEUE)]);
    }
}
