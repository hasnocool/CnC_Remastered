//! IPXAddressClass from both IPXADDR.CPP variants and IPXADDR.H.
//! Equality ignores the network portion if either address has an all-zero
//! network number; relational operators use raw lexicographic 10-byte order.

use std::cmp::Ordering;

pub const IPX_NETWORK_BYTES: usize = 4;
pub const IPX_NODE_BYTES: usize = 6;
pub const IPX_ADDRESS_BYTES: usize = 10;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IpxVariant { RedAlert, TiberianDawn }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PacketTransport { Ipx, Udp }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IpxHeaderAddress {
    pub source_network: [u8; 4],
    pub source_node: [u8; 6],
    pub destination_network: [u8; 4],
    pub destination_node: [u8; 6],
}
impl Default for IpxHeaderAddress {
    fn default() -> Self {
        Self { source_network: [0; 4], source_node: [0; 6], destination_network: [0; 4], destination_node: [0; 6] }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IpxAddressContext {
    pub winsock_ipx_build: bool,
    pub packet_transport: PacketTransport,
    /// Compile-time VIRTUAL_SUBNET_SERVER branch was included.
    pub virtual_subnet_server_build: bool,
    /// Runtime UseVirtualSubnetServer toggle.
    pub use_virtual_subnet_server: bool,
    pub winsock_connected: bool,
    pub planet_westwood_is_host: u8,
}
impl Default for IpxAddressContext {
    fn default() -> Self {
        Self {
            winsock_ipx_build: false, packet_transport: PacketTransport::Ipx,
            virtual_subnet_server_build: false, use_virtual_subnet_server: false,
            winsock_connected: false, planet_westwood_is_host: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct IpxAddress {
    network_number: [u8; 4],
    node_address: [u8; 6],
}
impl Default for IpxAddress {
    fn default() -> Self { Self::broadcast() }
}
impl IpxAddress {
    pub const fn new(network_number: [u8; 4], node_address: [u8; 6]) -> Self {
        Self { network_number, node_address }
    }
    pub const fn broadcast() -> Self { Self { network_number: [0xff; 4], node_address: [0xff; 6] } }
    pub const fn network_number(&self) -> &[u8; 4] { &self.network_number }
    pub const fn node_address(&self) -> &[u8; 6] { &self.node_address }

    pub fn set_address(&mut self, network: [u8; 4], node: [u8; 6]) {
        self.network_number = network; self.node_address = node;
    }
    pub fn from_source_header(header: &IpxHeaderAddress) -> Self {
        Self::new(header.source_network, header.source_node)
    }

    /// Variant-dependent Set_Address(IPXHeaderType*) path. raw_packet_prefix
    /// contains the first four bytes at the source header pointer for legacy
    /// Winsock UDP and virtual-subnet target-mask processing.
    pub fn set_from_header(
        &mut self, variant: IpxVariant, header: Option<&IpxHeaderAddress>,
        context: IpxAddressContext, raw_packet_prefix: Option<[u8; 4]>,
    ) {
        match variant {
            IpxVariant::RedAlert if context.winsock_ipx_build => match context.packet_transport {
                PacketTransport::Ipx => {
                    if let Some(header) = header {
                        self.network_number = header.source_network;
                        self.node_address = header.source_node;
                    }
                }
                PacketTransport::Udp => {
                    self.network_number = [0; 4];
                    self.node_address = [0; 6];
                    if let Some(raw) = raw_packet_prefix {
                        self.node_address[..4].copy_from_slice(&raw);
                    }
                }
            },
            IpxVariant::RedAlert => {
                if let Some(header) = header {
                    self.network_number = header.source_network;
                    self.node_address = header.source_node;
                } else {
                    self.network_number = [1; 4];
                    self.node_address = [1; 6];
                }
            }
            IpxVariant::TiberianDawn if context.virtual_subnet_server_build && context.winsock_connected => {
                self.network_number = [1; 4];
                self.node_address = [0; 6];
                let raw = raw_packet_prefix.unwrap_or([0; 4]);
                let mut target_mask = u16::from_le_bytes([raw[0], raw[1]]);
                // In head-to-head mode without a VSS server, map a zero
                // broadcast mask to the host bit, then XOR with 3.
                if !context.use_virtual_subnet_server {
                    if target_mask == 0 {
                        target_mask = 1u16.wrapping_shl(context.planet_westwood_is_host as u32);
                    }
                    target_mask ^= 3;
                }
                self.node_address[..2].copy_from_slice(&target_mask.to_le_bytes());
            }
            IpxVariant::TiberianDawn => {
                if let Some(header) = header {
                    self.network_number = header.source_network;
                    self.node_address = header.source_node;
                } else {
                    self.network_number = [1; 4];
                    self.node_address = [1; 6];
                }
            }
        }
    }

    pub fn set_red_alert_udp_raw(&mut self, raw_address: [u8; 4]) {
        self.network_number = [0; 4];
        self.node_address = [0; 6];
        self.node_address[..4].copy_from_slice(&raw_address);
    }

    pub fn write_destination_header(&self, header: &mut IpxHeaderAddress) {
        header.destination_network = self.network_number;
        header.destination_node = self.node_address;
    }
    pub const fn is_broadcast(&self) -> bool {
        self.network_number == [0xff; 4] && self.node_address == [0xff; 6]
    }
    fn network_is_zero(&self) -> bool { self.network_number == [0; 4] }

    pub fn legacy_equals(&self, other: &Self) -> bool {
        if self.network_is_zero() || other.network_is_zero() {
            self.node_address == other.node_address
        } else {
            self.node_address == other.node_address && self.network_number == other.network_number
        }
    }
    pub fn legacy_not_equals(&self, other: &Self) -> bool { !self.legacy_equals(other) }

    /// memcmp(this, &addr, 10) from the original relational operators.
    pub fn raw_ordering(&self, other: &Self) -> Ordering {
        self.network_number.iter().chain(self.node_address.iter())
            .cmp(other.network_number.iter().chain(other.node_address.iter()))
    }
    pub fn raw_greater_than(&self, other: &Self) -> bool { self.raw_ordering(other) == Ordering::Greater }
    pub fn raw_less_than(&self, other: &Self) -> bool { self.raw_ordering(other) == Ordering::Less }
    pub fn raw_greater_or_equal(&self, other: &Self) -> bool { self.raw_ordering(other) != Ordering::Less }
    pub fn raw_less_or_equal(&self, other: &Self) -> bool { self.raw_ordering(other) != Ordering::Greater }

    pub fn to_bytes(self) -> [u8; IPX_ADDRESS_BYTES] {
        let mut bytes = [0; IPX_ADDRESS_BYTES];
        bytes[..4].copy_from_slice(&self.network_number);
        bytes[4..].copy_from_slice(&self.node_address);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_and_broadcast_are_all_ones() {
        assert_eq!(IpxAddress::default(), IpxAddress::broadcast());
        assert!(IpxAddress::default().is_broadcast());
        assert_eq!(IpxAddress::new([1,2,3,4],[5,6,7,8,9,10]).to_bytes(), [1,2,3,4,5,6,7,8,9,10]);
    }
    #[test]
    fn source_fields_are_read_and_destination_fields_written() {
        let header = IpxHeaderAddress {
            source_network:[1,2,3,4],source_node:[5,6,7,8,9,10],
            destination_network:[11,12,13,14],destination_node:[15,16,17,18,19,20],
        };
        let address=IpxAddress::from_source_header(&header);
        let mut target=IpxHeaderAddress::default();
        address.write_destination_header(&mut target);
        assert_eq!(target.destination_network,[1,2,3,4]);
        assert_eq!(target.destination_node,[5,6,7,8,9,10]);
    }
    #[test]
    fn netx_zero_network_equality_compares_only_node_address() {
        let a=IpxAddress::new([0;4],[1,2,3,4,5,6]);
        let b=IpxAddress::new([8,9,10,11],[1,2,3,4,5,6]);
        assert!(a.legacy_equals(&b));
        assert!(!a.legacy_not_equals(&b));
        assert!(!a.legacy_equals(&IpxAddress::new([8;4],[1,2,3,4,5,7])));
    }
    #[test]
    fn ordering_uses_network_then_node_raw_bytes() {
        let a=IpxAddress::new([1,0,0,0],[0;6]);
        let b=IpxAddress::new([1,0,0,1],[0;6]);
        assert!(a.raw_less_than(&b));
        assert!(b.raw_greater_than(&a));
        assert!(a.raw_less_or_equal(&a));
        assert!(b.raw_greater_or_equal(&b));
    }
    #[test]
    fn red_alert_udp_copies_four_bytes_and_null_header_fills_ones() {
        let mut a=IpxAddress::broadcast();
        a.set_red_alert_udp_raw([1,2,3,4]);
        assert_eq!(a,IpxAddress::new([0;4],[1,2,3,4,0,0]));
        a.set_from_header(IpxVariant::RedAlert,None,IpxAddressContext::default(),None);
        assert_eq!(a,IpxAddress::new([1;4],[1;6]));
    }
    #[test]
    fn tiberian_dawn_vss_head_to_head_mask_is_little_endian_and_xored() {
        let mut a=IpxAddress::default();
        let context=IpxAddressContext{virtual_subnet_server_build:true,use_virtual_subnet_server:false,winsock_connected:true,planet_westwood_is_host:0,..IpxAddressContext::default()};
        a.set_from_header(IpxVariant::TiberianDawn,None,context,Some([0x34,0x12,0,0]));
        assert_eq!(a.network_number(),&[1;4]);
        assert_eq!(a.node_address(),&[0x37,0x12,0,0,0,0]);
    }
}
