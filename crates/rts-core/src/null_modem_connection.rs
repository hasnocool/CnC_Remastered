//! Red Alert null-modem connection wrapper from REDALERT/NULLCONN.CPP/H.
//!
//! The active implementation allocates a serial staging buffer and initializes
//! the inherited ConnectionClass state, but its Send() routine immediately
//! returns false/zero. The packet serialization code is under #if (0).
//! Tiberian Dawn's entire NULLCONN.CPP implementation is also under #if (0).

use std::fmt;

pub const PACKET_SERIAL_START: u16 = 0xdabd;
pub const PACKET_SERIAL_VERIFY: u16 = 0xdeaf;
pub const SERIAL_HEADER_SIZE: usize = 6;
pub const SERIAL_CRC_SIZE: usize = 4;
pub const SERIAL_TRAILER_SIZE: usize = 1;
pub const PACKET_SERIAL_OVERHEAD_SIZE: usize =
    SERIAL_HEADER_SIZE + SERIAL_CRC_SIZE;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NullModemError {
    CapacityOverflow,
    OutOfMemory,
}

impl fmt::Display for NullModemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for NullModemError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NullModemConnection {
    send_queue_entries: usize,
    receive_queue_entries: usize,
    max_packet_len: usize,
    magic_number: u16,
    retry_delta: u32,
    max_retries: i32,
    timeout_ticks: u32,
    port_handle: Option<usize>,
    base_initialized: bool,
    send_buffer: Vec<u8>,
}

impl NullModemConnection {
    /// Constructor delegates to ConnectionClass with retry delta 60,
    /// unlimited retries (-1), timeout 1200 ticks, then allocates
    /// Actual_Max_Packet() = MaxPacketLen + 6-byte header + 4-byte CRC + CR.
    pub fn new(
        send_queue_entries: usize,
        receive_queue_entries: usize,
        max_packet_len: usize,
        magic_number: u16,
    ) -> Result<Self, NullModemError> {
        let actual_size = max_packet_len
            .checked_add(SERIAL_HEADER_SIZE)
            .and_then(|n| n.checked_add(SERIAL_CRC_SIZE))
            .and_then(|n| n.checked_add(SERIAL_TRAILER_SIZE))
            .ok_or(NullModemError::CapacityOverflow)?;
        if actual_size > isize::MAX as usize {
            return Err(NullModemError::CapacityOverflow);
        }
        let mut send_buffer = Vec::new();
        send_buffer.try_reserve_exact(actual_size).map_err(|_| NullModemError::OutOfMemory)?;
        send_buffer.resize(actual_size, 0);
        Ok(Self {
            send_queue_entries,
            receive_queue_entries,
            max_packet_len,
            magic_number,
            retry_delta: 60,
            max_retries: -1,
            timeout_ticks: 1200,
            port_handle: None,
            base_initialized: false,
            send_buffer,
        })
    }

    /// Init calls ConnectionClass::Init before recording the opaque OS/serial
    /// port handle. None is the legacy NULL handle.
    pub fn init(&mut self, port_handle: Option<usize>) {
        self.base_initialized = true;
        self.port_handle = port_handle;
    }

    /// The pinned Red Alert Send() begins with an unconditional return 0.
    /// The serial header, payload copy, CRC and CR terminator are dead code.
    pub const fn send(&self, _buffer: &[u8], _extra_buffer: Option<&[u8]>) -> i32 { 0 }

    /// Compute_CRC uses a 32-bit shift, adds the old high bit as carry, then
    /// adds each source byte as unsigned. This is not a standard CRC polynomial.
    pub fn compute_crc(buffer: &[u8]) -> i32 {
        let mut sum = 0u32;
        for byte in buffer {
            let high_bit = sum >> 31;
            sum = sum.rotate_left(1)
                .wrapping_add(high_bit)
                .wrapping_add(*byte as u32);
        }
        sum as i32
    }

    /// Packet_Overhead_Size is the 10-byte serial wrapper plus the inherited
    /// CommHeaderType size, whose exact size is supplied by the parent protocol.
    pub const fn packet_overhead_size(comm_header_size: usize) -> usize {
        PACKET_SERIAL_OVERHEAD_SIZE + comm_header_size
    }

    pub const fn actual_max_packet(&self) -> usize { self.send_buffer.len() }
    pub const fn send_buffer_capacity(&self) -> usize { self.send_buffer.len() }
    pub const fn send_queue_entries(&self) -> usize { self.send_queue_entries }
    pub const fn receive_queue_entries(&self) -> usize { self.receive_queue_entries }
    pub const fn max_packet_len(&self) -> usize { self.max_packet_len }
    pub const fn magic_number(&self) -> u16 { self.magic_number }
    pub const fn retry_delta(&self) -> u32 { self.retry_delta }
    pub const fn max_retries(&self) -> i32 { self.max_retries }
    pub const fn timeout_ticks(&self) -> u32 { self.timeout_ticks }
    pub const fn port_handle(&self) -> Option<usize> { self.port_handle }
    pub const fn is_initialized(&self) -> bool { self.base_initialized }
    pub fn staging_buffer(&self) -> &[u8] { &self.send_buffer }
}

/// The Tiberian Dawn NULLCONN.CPP has no active definitions: every constructor,
/// destructor, Init, Send, CRC, and overhead method is inside #if (0).
pub const TIBERIAN_DAWN_NULLCONNECTION_IMPLEMENTATION_ACTIVE: bool = false;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_preserves_connection_class_timing_and_buffer_size() {
        let connection = NullModemConnection::new(16, 8, 1024, 0x1234).unwrap();
        assert_eq!(connection.send_queue_entries(), 16);
        assert_eq!(connection.receive_queue_entries(), 8);
        assert_eq!(connection.max_packet_len(), 1024);
        assert_eq!(connection.magic_number(), 0x1234);
        assert_eq!(connection.retry_delta(), 60);
        assert_eq!(connection.max_retries(), -1);
        assert_eq!(connection.timeout_ticks(), 1200);
        assert_eq!(connection.actual_max_packet(), 1024 + 11);
        assert!(!connection.is_initialized());
        assert_eq!(connection.port_handle(), None);
    }

    #[test]
    fn initialization_records_port_and_send_remains_disabled() {
        let mut connection = NullModemConnection::new(4, 4, 256, 0xbeef).unwrap();
        assert_eq!(connection.send(b"payload", None), 0);
        connection.init(Some(0x1234));
        assert!(connection.is_initialized());
        assert_eq!(connection.port_handle(), Some(0x1234));
        assert_eq!(connection.send(b"payload", Some(b"ignored")), 0);
        assert!(connection.staging_buffer().iter().all(|value| *value == 0));
    }

    #[test]
    fn serial_crc_is_wrapping_rotate_add_not_standard_crc32() {
        assert_eq!(NullModemConnection::compute_crc(&[]), 0);
        let mut expected = 0u32;
        for byte in [1u8, 2, 3] {
            let carry = expected >> 31;
            expected = expected.rotate_left(1).wrapping_add(carry).wrapping_add(byte as u32);
        }
        assert_eq!(NullModemConnection::compute_crc(&[1, 2, 3]), expected as i32);
    }

    #[test]
    fn packet_overhead_adds_serial_and_parent_queue_headers() {
        assert_eq!(NullModemConnection::packet_overhead_size(8), 18);
        assert_eq!(PACKET_SERIAL_START, 0xdabd);
        assert_eq!(PACKET_SERIAL_VERIFY, 0xdeaf);
        assert!(!TIBERIAN_DAWN_NULLCONNECTION_IMPLEMENTATION_ACTIVE);
    }

    #[test]
    fn size_overflow_is_reported_before_allocation() {
        assert_eq!(
            NullModemConnection::new(1, 1, usize::MAX, 0),
            Err(NullModemError::CapacityOverflow)
        );
    }
}
