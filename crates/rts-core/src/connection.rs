//! Tiberian Dawn connection base class compatibility layer.
//!
//! Port of `TIBERIANDAWN/CONNECT.CPP` at upstream commit
//! `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.
//!
//! The original class is deliberately abstract: derived transports own the
//! send/receive queues and implement their service routines. This module
//! preserves that split instead of providing a transport implementation.

use std::mem::size_of;

/// Number of command names exposed by the legacy connection base class.
pub const PACKET_COUNT: usize = 3;

/// The legacy packet command values, in their original wire-header order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketCommand {
    DataAck = 0,
    DataNoAck = 1,
    Ack = 2,
}

/// Names returned by the original `ConnectionClass::Command_Name`.
pub const PACKET_COMMAND_NAMES: [&str; PACKET_COUNT] = ["ADATA", "NDATA", "ACK"];

/// The Win32 layout of the upstream `CommHeaderType`.
///
/// The original source uses `unsigned long`, which is 32-bit in its Win32
/// game target. Using `u32` preserves the target layout on all Rust hosts.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct CommHeaderType {
    pub magic_number: u16,
    pub code: u8,
    pub packet_id: u32,
}

/// Size of the Win32 connection header (including its alignment byte).
pub const COMM_HEADER_SIZE: usize = size_of::<CommHeaderType>();

const _: () = assert!(COMM_HEADER_SIZE == 8);

/// Invalid or unallocatable packet-buffer configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionConfigError {
    NegativeMaximumPayload(i32),
    PacketLengthOverflow,
    AllocationFailed,
}

/// State owned by the abstract Tiberian Dawn `ConnectionClass`.
///
/// This stores the maximum packet length, staging buffer, magic number and
/// timeout/retry settings. It does not own a network socket or queue.
#[derive(Debug)]
pub struct ConnectionBase {
    max_packet_len: usize,
    packet_buffer: Vec<u8>,
    magic_number: u16,
    retry_delta: u32,
    max_retries: u32,
    timeout: u32,
}

impl ConnectionBase {
    /// Constructs the base state using the source constructor's argument order.
    ///
    /// The source allocates `maxlen + sizeof(CommHeaderType)` bytes.
    pub fn new(
        max_payload_len: i32,
        magic_number: u16,
        retry_delta: u32,
        max_retries: u32,
        timeout: u32,
    ) -> Result<Self, ConnectionConfigError> {
        let payload_len = usize::try_from(max_payload_len)
            .map_err(|_| ConnectionConfigError::NegativeMaximumPayload(max_payload_len))?;
        let max_packet_len = payload_len
            .checked_add(COMM_HEADER_SIZE)
            .ok_or(ConnectionConfigError::PacketLengthOverflow)?;

        let mut packet_buffer = Vec::new();
        packet_buffer
            .try_reserve_exact(max_packet_len)
            .map_err(|_| ConnectionConfigError::AllocationFailed)?;
        packet_buffer.resize(max_packet_len, 0);

        Ok(Self {
            max_packet_len,
            packet_buffer,
            magic_number,
            retry_delta,
            max_retries,
            timeout,
        })
    }

    /// The original base `Init` method is an empty virtual hook.
    pub fn init(&mut self) {}

    pub const fn max_packet_len(&self) -> usize {
        self.max_packet_len
    }

    pub const fn magic_number(&self) -> u16 {
        self.magic_number
    }

    pub const fn retry_delta(&self) -> u32 {
        self.retry_delta
    }

    pub fn set_retry_delta(&mut self, delta: u32) {
        self.retry_delta = delta;
    }

    pub const fn max_retries(&self) -> u32 {
        self.max_retries
    }

    pub fn set_max_retries(&mut self, retries: u32) {
        self.max_retries = retries;
    }

    pub const fn timeout(&self) -> u32 {
        self.timeout
    }

    pub fn set_timeout(&mut self, timeout: u32) {
        self.timeout = timeout;
    }

    pub fn packet_buffer(&self) -> &[u8] {
        &self.packet_buffer
    }

    pub fn packet_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.packet_buffer
    }

    /// Returns the legacy command name, or `None` for an out-of-range code.
    pub fn command_name(command: i32) -> Option<&'static str> {
        usize::try_from(command)
            .ok()
            .and_then(|index| PACKET_COMMAND_NAMES.get(index).copied())
    }
}

/// Queue operations supplied by a concrete connection transport.
///
/// This trait models the two pure-virtual service hooks in Tiberian Dawn's
/// `CONNECT.H`. The default service method intentionally short-circuits:
/// receive servicing is skipped when send servicing reports a broken link.
pub trait ConnectionQueueService {
    fn service_send_queue(&mut self) -> bool;
    fn service_receive_queue(&mut self) -> bool;

    fn service(&mut self) -> bool {
        self.service_send_queue() && self.service_receive_queue()
    }
}

/// Reproduces the non-Westwood timer fallback in `ConnectionClass::Time`.
///
/// The legacy Win32 build normally uses `TickCount.Time()` directly. Its
/// fallback combines 32-bit seconds and milliseconds, then converts each
/// 100 ms interval to six 60 Hz ticks. Wrapping arithmetic matches Win32
/// `unsigned long` behavior.
pub const fn legacy_time_60hz(seconds: u32, milliseconds: u16) -> u32 {
    let elapsed_millis = seconds
        .wrapping_mul(1_000)
        .wrapping_add(milliseconds as u32);
    (elapsed_millis / 100).wrapping_mul(6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockQueues {
        send_result: bool,
        receive_result: bool,
        send_calls: usize,
        receive_calls: usize,
    }

    impl ConnectionQueueService for MockQueues {
        fn service_send_queue(&mut self) -> bool {
            self.send_calls += 1;
            self.send_result
        }

        fn service_receive_queue(&mut self) -> bool {
            self.receive_calls += 1;
            self.receive_result
        }
    }

    #[test]
    fn win32_header_layout_is_eight_bytes() {
        assert_eq!(size_of::<CommHeaderType>(), 8);
        assert_eq!(COMM_HEADER_SIZE, 8);
    }

    #[test]
    fn constructor_reserves_payload_plus_header() {
        let connection = ConnectionBase::new(512, 0xCAFE, 6, 5, 120)
            .expect("valid connection configuration");
        assert_eq!(connection.max_packet_len(), 520);
        assert_eq!(connection.packet_buffer().len(), 520);
        assert_eq!(connection.magic_number(), 0xCAFE);
        assert_eq!(connection.retry_delta(), 6);
        assert_eq!(connection.max_retries(), 5);
        assert_eq!(connection.timeout(), 120);
    }

    #[test]
    fn constructor_rejects_negative_payload_length() {
        assert_eq!(
            ConnectionBase::new(-1, 1, 2, 3, 4).err(),
            Some(ConnectionConfigError::NegativeMaximumPayload(-1))
        );
    }

    #[test]
    fn retry_and_timeout_settings_are_mutable() {
        let mut connection =
            ConnectionBase::new(1, 2, 3, 4, 5).expect("valid connection configuration");
        connection.set_retry_delta(30);
        connection.set_max_retries(9);
        connection.set_timeout(600);
        assert_eq!(connection.retry_delta(), 30);
        assert_eq!(connection.max_retries(), 9);
        assert_eq!(connection.timeout(), 600);
    }

    #[test]
    fn command_names_preserve_legacy_order_and_bounds() {
        assert_eq!(ConnectionBase::command_name(0), Some("ADATA"));
        assert_eq!(ConnectionBase::command_name(1), Some("NDATA"));
        assert_eq!(ConnectionBase::command_name(2), Some("ACK"));
        assert_eq!(ConnectionBase::command_name(-1), None);
        assert_eq!(ConnectionBase::command_name(3), None);
    }

    #[test]
    fn queue_service_short_circuits_after_send_failure() {
        let mut queues = MockQueues {
            send_result: false,
            receive_result: true,
            ..MockQueues::default()
        };
        assert!(!queues.service());
        assert_eq!(queues.send_calls, 1);
        assert_eq!(queues.receive_calls, 0);
    }

    #[test]
    fn queue_service_runs_receive_after_successful_send() {
        let mut queues = MockQueues {
            send_result: true,
            receive_result: false,
            ..MockQueues::default()
        };
        assert!(!queues.service());
        assert_eq!(queues.send_calls, 1);
        assert_eq!(queues.receive_calls, 1);
    }

    #[test]
    fn legacy_time_uses_sixty_ticks_per_second() {
        assert_eq!(legacy_time_60hz(1, 0), 60);
        assert_eq!(legacy_time_60hz(1, 100), 66);
        assert_eq!(legacy_time_60hz(0, 99), 0);
    }
}
