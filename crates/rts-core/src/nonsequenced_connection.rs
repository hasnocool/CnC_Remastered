//! Non-sequenced connection reliability layer from REDALERT/NOSEQCON.CPP and
//! TIBERIANDAWN/NOSEQCON.CPP, together with the relevant CONNECT.H/COMBUF.H
//! contracts. The method bodies are identical across both pinned variants.
//!
//! The CommHeaderType is represented as its Win32 wire layout: USHORT magic,
//! UCHAR code, one alignment byte, and a 32-bit packet ID (8 bytes total).
//! Hardware sends and the ConnectionClass tick source are injected by caller.

use std::collections::VecDeque;

pub const COMM_HEADER_SIZE: usize = 8;
pub const PACKET_DATA_ACK: u8 = 0;
pub const PACKET_DATA_NOACK: u8 = 1;
pub const PACKET_ACK: u8 = 2;
pub const NO_PACKET_ID: u32 = u32::MAX;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum PacketCode {
    DataAck = PACKET_DATA_ACK,
    DataNoAck = PACKET_DATA_NOACK,
    Ack = PACKET_ACK,
}

impl TryFrom<u8> for PacketCode {
    type Error = PacketError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            PACKET_DATA_ACK => Ok(Self::DataAck),
            PACKET_DATA_NOACK => Ok(Self::DataNoAck),
            PACKET_ACK => Ok(Self::Ack),
            other => Err(PacketError::InvalidCode(other)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommPacket {
    pub magic_number: u16,
    pub code: PacketCode,
    pub packet_id: u32,
    pub payload: Vec<u8>,
}

impl CommPacket {
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; COMM_HEADER_SIZE + self.payload.len()];
        bytes[0..2].copy_from_slice(&self.magic_number.to_le_bytes());
        bytes[2] = self.code as u8;
        // bytes[3] is the original C structure's alignment byte.
        bytes[4..8].copy_from_slice(&self.packet_id.to_le_bytes());
        bytes[COMM_HEADER_SIZE..].copy_from_slice(&self.payload);
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, PacketError> {
        if bytes.len() < COMM_HEADER_SIZE {
            return Err(PacketError::TruncatedHeader { actual: bytes.len() });
        }
        let magic_number = u16::from_le_bytes([bytes[0], bytes[1]]);
        let code = PacketCode::try_from(bytes[2])?;
        let packet_id = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        Ok(Self {
            magic_number,
            code,
            packet_id,
            payload: bytes[COMM_HEADER_SIZE..].to_vec(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PacketError {
    TruncatedHeader { actual: usize },
    InvalidCode(u8),
    PacketTooLarge { actual: usize, maximum: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SendEntry {
    packet: Vec<u8>,
    is_ack: bool,
    first_time: u32,
    last_time: u32,
    send_count: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReceiveEntry {
    packet: Vec<u8>,
    is_read: bool,
    ack_sent: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ConnectionCounters {
    pub received_no_ack: u32,
    pub received_ack: u32,
    pub sent_no_ack: u32,
    pub sent_ack: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResponseDelayStats {
    pub sample_count: u32,
    pub average_ticks: u32,
    pub maximum_ticks: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonSequencedConnection {
    magic_number: u16,
    max_packet_len: usize,
    max_send: usize,
    max_receive: usize,
    retry_delta: u32,
    max_retries: Option<u32>,
    timeout: Option<u32>,
    send_queue: VecDeque<SendEntry>,
    receive_queue: VecDeque<ReceiveEntry>,
    counters: ConnectionCounters,
    last_seq_id: u32,
    last_read_id: u32,
    delay_sum: u32,
    num_delay: u32,
    mean_delay: u32,
    max_delay: u32,
    send_total: u32,
    receive_total: u32,
}

impl NonSequencedConnection {
    /// Red Alert constructor's inherited ConnectionClass configuration plus
    /// CommBufferClass(nsend, nreceive, MaxPacketLen). Tiberian Dawn uses the
    /// same queue/protocol methods and supplies the same parameters through
    /// its adjusted ConnectionClass constructor.
    pub fn new(
        send_slots: usize,
        receive_slots: usize,
        max_payload_len: usize,
        magic_number: u16,
        retry_delta: u32,
        max_retries: Option<u32>,
        timeout: Option<u32>,
    ) -> Self {
        Self {
            magic_number,
            max_packet_len: max_payload_len.saturating_add(COMM_HEADER_SIZE),
            max_send: send_slots,
            max_receive: receive_slots,
            retry_delta,
            max_retries,
            timeout,
            send_queue: VecDeque::with_capacity(send_slots),
            receive_queue: VecDeque::with_capacity(receive_slots),
            counters: ConnectionCounters::default(),
            last_seq_id: NO_PACKET_ID,
            last_read_id: NO_PACKET_ID,
            delay_sum: 0,
            num_delay: 0,
            mean_delay: 0,
            max_delay: 0,
            send_total: 0,
            receive_total: 0,
        }
    }

    /// NonSequencedConnClass::Init resets all packet counters, sequencing
    /// sentinels and both CommBuffer queues.
    pub fn init(&mut self) {
        self.counters = ConnectionCounters::default();
        self.last_seq_id = NO_PACKET_ID;
        self.last_read_id = NO_PACKET_ID;
        self.send_queue.clear();
        self.receive_queue.clear();
        self.delay_sum = 0;
        self.num_delay = 0;
        self.mean_delay = 0;
        self.max_delay = 0;
        self.send_total = 0;
        self.receive_total = 0;
    }

    pub fn magic_number(&self) -> u16 { self.magic_number }
    pub fn max_packet_len(&self) -> usize { self.max_packet_len }
    pub fn max_send(&self) -> usize { self.max_send }
    pub fn max_receive(&self) -> usize { self.max_receive }
    pub fn retry_delta(&self) -> u32 { self.retry_delta }
    pub fn max_retries(&self) -> Option<u32> { self.max_retries }
    pub fn timeout(&self) -> Option<u32> { self.timeout }
    pub fn counters(&self) -> ConnectionCounters { self.counters }
    pub fn last_sequential_received_id(&self) -> u32 { self.last_seq_id }
    pub fn last_sequential_read_id(&self) -> u32 { self.last_read_id }
    pub fn queued_send_packets(&self) -> usize { self.send_queue.len() }
    pub fn queued_receive_packets(&self) -> usize { self.receive_queue.len() }
    pub fn send_total(&self) -> u32 { self.send_total }
    pub fn receive_total(&self) -> u32 { self.receive_total }
    pub fn response_delay_stats(&self) -> ResponseDelayStats {
        ResponseDelayStats {
            sample_count: self.num_delay,
            average_ticks: self.mean_delay,
            maximum_ticks: self.max_delay,
        }
    }

    /// Send_Packet creates the CommHeaderType prefix and queues it. Packet IDs
    /// are independent counters for DATA_ACK and DATA_NOACK traffic.
    pub fn send_packet(
        &mut self,
        payload: &[u8],
        ack_required: bool,
    ) -> bool {
        let total_len = match COMM_HEADER_SIZE.checked_add(payload.len()) {
            Some(value) => value,
            None => return false,
        };
        if total_len > self.max_packet_len || self.send_queue.len() >= self.max_send {
            return false;
        }

        let (code, packet_id) = if ack_required {
            (PacketCode::DataAck, self.counters.sent_ack)
        } else {
            (PacketCode::DataNoAck, self.counters.sent_no_ack)
        };
        let packet = CommPacket {
            magic_number: self.magic_number,
            code,
            packet_id,
            payload: payload.to_vec(),
        }.encode();

        self.send_queue.push_back(SendEntry {
            packet,
            is_ack: false,
            first_time: 0,
            last_time: 0,
            send_count: 0,
        });
        if ack_required {
            self.counters.sent_ack = self.counters.sent_ack.wrapping_add(1);
        } else {
            self.counters.sent_no_ack = self.counters.sent_no_ack.wrapping_add(1);
        }
        self.send_total = self.send_total.wrapping_add(1);
        true
    }

    /// Receive_Packet validates the application magic value, processes ACKs,
    /// filters duplicate DATA_ACK packets, applies the one-free-slot rule,
    /// and emits an ACK for each accepted DATA_ACK packet (including resends).
    pub fn receive_packet(
        &mut self,
        bytes: &[u8],
        mut send_ack: impl FnMut(&[u8]),
    ) -> bool {
        let packet = match CommPacket::decode(bytes) {
            Ok(packet) => packet,
            Err(_) => return false,
        };
        if packet.magic_number != self.magic_number {
            return false;
        }

        match packet.code {
            PacketCode::Ack => {
                if let Some(entry) = self.send_queue.iter_mut().find(|entry| {
                    CommPacket::decode(&entry.packet).is_ok_and(|sent| {
                        sent.packet_id == packet.packet_id && sent.code == PacketCode::DataAck
                    })
                }) {
                    entry.is_ack = true;
                }
                true
            }
            PacketCode::DataNoAck => {
                if self.max_receive.saturating_sub(self.receive_queue.len()) <= 1 {
                    return false;
                }
                if self.receive_queue.len() >= self.max_receive {
                    return false;
                }
                self.receive_queue.push_back(ReceiveEntry {
                    packet: packet.encode(),
                    is_read: false,
                    ack_sent: false,
                });
                self.counters.received_no_ack = self.counters.received_no_ack.wrapping_add(1);
                self.receive_total = self.receive_total.wrapping_add(1);
                true
            }
            PacketCode::DataAck => {
                let mut save_packet = true;

                // A packet ID at or behind the last contiguous ID is a resend.
                if self.last_seq_id != NO_PACKET_ID && packet.packet_id <= self.last_seq_id {
                    save_packet = false;
                } else if self.receive_queue.iter().any(|entry| {
                    CommPacket::decode(&entry.packet).is_ok_and(|queued| {
                        queued.code == PacketCode::DataAck && queued.packet_id == packet.packet_id
                    })
                }) {
                    save_packet = false;
                }

                if save_packet {
                    if self.max_receive.saturating_sub(self.receive_queue.len()) <= 1
                        && packet.packet_id != self.last_seq_id.wrapping_add(1)
                    {
                        return false;
                    }
                    if self.receive_queue.len() >= self.max_receive {
                        return false;
                    }

                    self.receive_queue.push_back(ReceiveEntry {
                        packet: packet.encode(),
                        is_read: false,
                        ack_sent: false,
                    });
                    self.counters.received_ack = self.counters.received_ack.wrapping_add(1);
                    self.receive_total = self.receive_total.wrapping_add(1);

                    // Advance LastSeqID over the full contiguous range already
                    // resident in the receive queue, not merely this packet.
                    if packet.packet_id == self.last_seq_id.wrapping_add(1) {
                        self.last_seq_id = packet.packet_id;
                        loop {
                            let next = self.last_seq_id.wrapping_add(1);
                            let found = self.receive_queue.iter().any(|entry| {
                                CommPacket::decode(&entry.packet).is_ok_and(|queued| {
                                    queued.code == PacketCode::DataAck && queued.packet_id == next
                                })
                            });
                            if !found { break; }
                            self.last_seq_id = next;
                        }
                    }
                }

                // The source sends an ACK even when the incoming packet was a
                // duplicate/resend (provided it passed the one-slot admission check).
                let ack = CommPacket {
                    magic_number: self.magic_number,
                    code: PacketCode::Ack,
                    packet_id: packet.packet_id,
                    payload: Vec::new(),
                }.encode();
                send_ack(&ack);
                true
            }
        }
    }

    /// Get_Packet scans in queue order. DATA_ACK packets are delivered only
    /// when exactly next in sequence; DATA_NOACK packets ignore their IDs.
    /// Successfully delivered entries are marked read and removed by the
    /// later Service_Receive_Queue call.
    pub fn get_packet(&mut self) -> Option<Vec<u8>> {
        for entry in &mut self.receive_queue {
            if entry.is_read { continue; }
            let packet = CommPacket::decode(&entry.packet).ok()?;
            match packet.code {
                PacketCode::DataAck if packet.packet_id == self.last_read_id.wrapping_add(1) => {
                    self.last_read_id = packet.packet_id;
                    entry.is_read = true;
                    return Some(packet.payload);
                }
                PacketCode::DataNoAck => {
                    entry.is_read = true;
                    return Some(packet.payload);
                }
                _ => {}
            }
        }
        None
    }

    /// Service_Send_Queue first removes ACKed packets, adding DATA_ACK
    /// round-trip time to the CommBuffer rolling delay statistic. It then
    /// resends entries whose retry interval has elapsed.
    pub fn service_send_queue(
        &mut self,
        current_time: u32,
        mut send: impl FnMut(&[u8]),
    ) -> bool {
        let mut index = 0usize;
        while index < self.send_queue.len() {
            if self.send_queue[index].is_ack {
                let entry = self.send_queue.remove(index).unwrap();
                if CommPacket::decode(&entry.packet)
                    .is_ok_and(|packet| packet.code == PacketCode::DataAck)
                {
                    self.add_response_delay(current_time.wrapping_sub(entry.first_time));
                }
            } else {
                index += 1;
            }
        }

        let num_entries = self.send_queue.len();
        let mut bad_connection = false;
        for entry in self.send_queue.iter_mut().take(num_entries) {
            if entry.is_ack { continue; }
            let elapsed_since_last = current_time.wrapping_sub(entry.last_time);
            if elapsed_since_last > self.retry_delta {
                send(&entry.packet);
                entry.last_time = current_time;
                if entry.send_count == 0 {
                    entry.first_time = current_time;
                    if CommPacket::decode(&entry.packet)
                        .is_ok_and(|packet| packet.code == PacketCode::DataNoAck)
                    {
                        entry.is_ack = true;
                    }
                }
                entry.send_count = entry.send_count.wrapping_add(1);

                if let Some(max_retries) = self.max_retries {
                    if entry.send_count > max_retries { bad_connection = true; }
                }
                if let Some(timeout) = self.timeout {
                    if entry.last_time.wrapping_sub(entry.first_time) > timeout {
                        bad_connection = true;
                    }
                }
            }
        }
        !bad_connection
    }

    /// Service_Receive_Queue removes read DATA_NOACK records immediately,
    /// and read DATA_ACK records only after their ID is older than LastSeqID.
    pub fn service_receive_queue(&mut self) -> bool {
        self.receive_queue.retain(|entry| {
            if !entry.is_read { return true; }
            let Ok(packet) = CommPacket::decode(&entry.packet) else { return false };
            match packet.code {
                PacketCode::DataNoAck => false,
                PacketCode::DataAck => packet.packet_id >= self.last_seq_id,
                PacketCode::Ack => false,
            }
        });
        true
    }

    /// Base ConnectionClass::Service calls the two queue services in order.
    pub fn service(
        &mut self,
        current_time: u32,
        mut send: impl FnMut(&[u8]),
    ) -> bool {
        let sent_ok = self.service_send_queue(current_time, |packet| send(packet));
        let received_ok = self.service_receive_queue();
        sent_ok && received_ok
    }

    fn add_response_delay(&mut self, delay: u32) {
        // CommBufferClass::Add_Delay uses a 256-sample fixed-point moving average.
        // After the first 256 samples it subtracts the current mean (not the
        // oldest sample), adds the new delay, then rounds the high-byte result.
        if self.num_delay == 256 {
            self.delay_sum = self.delay_sum.wrapping_sub(self.mean_delay).wrapping_add(delay);
            let roundoff = u32::from((self.delay_sum & 0xff) > 127);
            self.mean_delay = (self.delay_sum >> 8).wrapping_add(roundoff);
        } else {
            self.num_delay += 1;
            self.delay_sum = self.delay_sum.wrapping_add(delay);
            self.mean_delay = self.delay_sum / self.num_delay;
        }
        if delay > self.max_delay { self.max_delay = delay; }
    }

    pub fn queued_send_wire_packets(&self) -> impl Iterator<Item = &[u8]> {
        self.send_queue.iter().map(|entry| entry.packet.as_slice())
    }

    pub fn queued_receive_wire_packets(&self) -> impl Iterator<Item = &[u8]> {
        self.receive_queue.iter().map(|entry| entry.packet.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn remote_packet(magic: u16, code: PacketCode, id: u32, payload: &[u8]) -> Vec<u8> {
        CommPacket {
            magic_number: magic,
            code,
            packet_id: id,
            payload: payload.to_vec(),
        }.encode()
    }

    #[test]
    fn comm_header_uses_win32_alignment_and_little_endian_fields() {
        let packet = CommPacket {
            magic_number: 0x1234,
            code: PacketCode::DataAck,
            packet_id: 0x7856_3412,
            payload: b"abc".to_vec(),
        };
        let bytes = packet.encode();
        assert_eq!(bytes.len(), COMM_HEADER_SIZE + 3);
        assert_eq!(&bytes[..8], &[0x34, 0x12, 0, 0, 0x12, 0x34, 0x56, 0x78]);
        assert_eq!(CommPacket::decode(&bytes).unwrap(), packet);
    }

    #[test]
    fn send_packet_assigns_independent_ack_and_noack_ids() {
        let mut connection = NonSequencedConnection::new(4, 4, 128, 0x55aa, 60, Some(5), Some(1200));
        assert!(connection.send_packet(b"one", true));
        assert!(connection.send_packet(b"two", false));
        assert!(connection.send_packet(b"three", true));
        assert_eq!(connection.counters(), ConnectionCounters {
            received_no_ack: 0,
            received_ack: 0,
            sent_no_ack: 1,
            sent_ack: 2,
        });
        let packets: Vec<_> = connection.queued_send_wire_packets()
            .map(|bytes| CommPacket::decode(bytes).unwrap())
            .collect();
        assert_eq!((packets[0].code, packets[0].packet_id), (PacketCode::DataAck, 0));
        assert_eq!((packets[1].code, packets[1].packet_id), (PacketCode::DataNoAck, 0));
        assert_eq!((packets[2].code, packets[2].packet_id), (PacketCode::DataAck, 1));
    }

    #[test]
    fn received_ack_marks_outgoing_packet_then_service_retires_it() {
        let mut connection = NonSequencedConnection::new(4, 4, 128, 0xaaaa, 1, None, None);
        assert!(connection.send_packet(b"payload", true));
        let ack = remote_packet(0xaaaa, PacketCode::Ack, 0, &[]);
        assert!(connection.receive_packet(&ack, |_| panic!("ACK packet must not emit another ACK")));
        assert!(connection.service_send_queue(10, |_| panic!("ACKed packet is not resent")));
        assert_eq!(connection.queued_send_packets(), 0);
        assert_eq!(connection.response_delay_stats().sample_count, 1);
        assert_eq!(connection.response_delay_stats().average_ticks, 10);
    }

    #[test]
    fn data_ack_is_buffered_in_order_and_duplicate_gets_an_ack() {
        let mut connection = NonSequencedConnection::new(4, 4, 128, 0x1111, 60, None, None);
        let mut acknowledgements = Vec::new();
        let packet_1 = remote_packet(0x1111, PacketCode::DataAck, 1, b"second");
        assert!(connection.receive_packet(&packet_1, |ack| acknowledgements.push(ack.to_vec())));
        assert_eq!(connection.last_sequential_received_id(), NO_PACKET_ID);
        assert_eq!(connection.get_packet(), None);

        let packet_0 = remote_packet(0x1111, PacketCode::DataAck, 0, b"first");
        assert!(connection.receive_packet(&packet_0, |ack| acknowledgements.push(ack.to_vec())));
        assert_eq!(connection.last_sequential_received_id(), 1);
        assert_eq!(connection.get_packet(), Some(b"first".to_vec()));
        assert_eq!(connection.get_packet(), Some(b"second".to_vec()));
        assert_eq!(connection.get_packet(), None);

        assert!(connection.receive_packet(&packet_0, |ack| acknowledgements.push(ack.to_vec())));
        assert_eq!(acknowledgements.len(), 3);
        assert!(acknowledgements.iter().all(|bytes| {
            let packet = CommPacket::decode(bytes).unwrap();
            packet.code == PacketCode::Ack
        }));
        connection.service_receive_queue();
        assert_eq!(connection.queued_receive_packets(), 1); // LastSeqID equality is retained.
    }

    #[test]
    fn data_noack_can_be_delivered_without_sequencing() {
        let mut connection = NonSequencedConnection::new(4, 4, 128, 0x2222, 60, None, None);
        let noack = remote_packet(0x2222, PacketCode::DataNoAck, 99, b"independent");
        assert!(connection.receive_packet(&noack, |_| panic!("NOACK packet must not emit ACK")));
        assert_eq!(connection.get_packet(), Some(b"independent".to_vec()));
        connection.service_receive_queue();
        assert_eq!(connection.queued_receive_packets(), 0);
    }

    #[test]
    fn receive_admission_reserves_one_slot_for_next_contiguous_ack_packet() {
        let mut connection = NonSequencedConnection::new(4, 2, 128, 0x3333, 60, None, None);
        let later = remote_packet(0x3333, PacketCode::DataAck, 1, b"later");
        let next = remote_packet(0x3333, PacketCode::DataAck, 0, b"next");
        let mut ack_count = 0;
        assert!(connection.receive_packet(&later, |_| ack_count += 1));
        assert!(connection.receive_packet(&later, |_| ack_count += 1)); // duplicate is ACKed but not queued.
        assert!(connection.receive_packet(&next, |_| ack_count += 1));
        assert_eq!(ack_count, 3);
    }

    #[test]
    fn noack_send_is_retired_on_the_next_service_pass() {
        let mut connection = NonSequencedConnection::new(4, 4, 128, 0x4444, 0, None, None);
        assert!(connection.send_packet(b"best effort", false));
        let mut sent = Vec::new();
        assert!(connection.service_send_queue(1, |packet| sent.push(packet.to_vec())));
        assert_eq!(sent.len(), 1);
        assert_eq!(connection.queued_send_packets(), 1);
        assert!(connection.service_send_queue(2, |_| panic!("first send already marks NOACK complete")));
        assert_eq!(connection.queued_send_packets(), 0);
    }

    #[test]
    fn rolling_response_delay_uses_the_legacy_256_sample_filter() {
        let mut connection = NonSequencedConnection::new(300, 4, 64, 0x5151, 0, None, None);
        for tick in 1..=256u32 {
            assert!(connection.send_packet(&[tick as u8], true));
            assert!(connection.service_send_queue(tick, |_| {}));
            assert!(connection.receive_packet(&remote_packet(0x5151, PacketCode::Ack, tick - 1, &[]), |_| {}));
            assert!(connection.service_send_queue(tick, |_| {}));
        }
        assert_eq!(connection.response_delay_stats().sample_count, 256);
        assert_eq!(connection.response_delay_stats().average_ticks, 0);
        assert_eq!(connection.response_delay_stats().maximum_ticks, 0);
    }

    #[test]
    fn retry_and_timeout_limits_report_failed_connection() {
        let mut connection = NonSequencedConnection::new(4, 4, 128, 0x5555, 0, Some(1), Some(2));
        assert!(connection.send_packet(b"retry", true));
        assert!(connection.service_send_queue(1, |_| {}));
        assert!(!connection.service_send_queue(2, |_| {}));
    }

    #[test]
    fn init_resets_protocol_state_and_queues() {
        let mut connection = NonSequencedConnection::new(4, 4, 128, 0x6666, 60, None, None);
        connection.send_packet(b"queued", true);
        connection.receive_packet(
            &remote_packet(0x6666, PacketCode::DataAck, 0, b"incoming"),
            |_| {},
        );
        connection.get_packet();
        connection.init();
        assert_eq!(connection.queued_send_packets(), 0);
        assert_eq!(connection.queued_receive_packets(), 0);
        assert_eq!(connection.last_sequential_received_id(), NO_PACKET_ID);
        assert_eq!(connection.last_sequential_read_id(), NO_PACKET_ID);
        assert_eq!(connection.counters(), ConnectionCounters::default());
    }
}
