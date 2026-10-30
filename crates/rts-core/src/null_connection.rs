//! Red Alert null-modem serial connection from REDALERT/NULLCONN.CPP/H.

use std::fmt;

pub const PACKET_SERIAL_START: u16 = 0xDABD;
pub const PACKET_SERIAL_VERIFY: u16 = 0xDEAF;
pub const COMM_HEADER_SIZE: usize = 8;
pub const SERIAL_HEADER_SIZE: usize = 6;
pub const SERIAL_CRC_SIZE: usize = 4;
pub const SERIAL_TERMINATOR_SIZE: usize = 1;
pub const PACKET_SERIAL_OVERHEAD_SIZE: usize = SERIAL_HEADER_SIZE + SERIAL_CRC_SIZE;
pub const RETRY_DELTA_TICKS: u32 = 60;
pub const MAX_RETRIES: i32 = -1;
pub const CONNECTION_TIMEOUT_TICKS: u32 = 1200;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NullModemConfig { pub num_send: usize, pub num_receive: usize, pub application_max_packet_len: usize, pub magic_number: u16 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SerialPort { Handle(usize), GreenleafPort(usize) }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConnectionBaseConfig { pub num_send: usize, pub num_receive: usize, pub max_packet_len: usize, pub magic_number: u16, pub retry_delta_ticks: u32, pub max_retries: i32, pub timeout_ticks: u32 }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NullModemError { SizeOverflow, BufferAllocationFailed }
impl fmt::Display for NullModemError { fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result { write!(f,"{self:?}") } }
impl std::error::Error for NullModemError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NullModemConnection { config: NullModemConfig, base: ConnectionBaseConfig, port: Option<SerialPort>, send_buffer: Vec<u8>, initialized: bool }
impl NullModemConnection {
    pub fn new(config: NullModemConfig) -> Result<Self, NullModemError> {
        let max_packet_len=config.application_max_packet_len.checked_add(COMM_HEADER_SIZE).ok_or(NullModemError::SizeOverflow)?;
        let size=max_packet_len.checked_add(SERIAL_HEADER_SIZE).and_then(|n|n.checked_add(SERIAL_CRC_SIZE)).and_then(|n|n.checked_add(SERIAL_TERMINATOR_SIZE)).ok_or(NullModemError::SizeOverflow)?;
        let mut send_buffer=Vec::new();send_buffer.try_reserve_exact(size).map_err(|_|NullModemError::BufferAllocationFailed)?;send_buffer.resize(size,0);
        Ok(Self{config,base:ConnectionBaseConfig{num_send:config.num_send,num_receive:config.num_receive,max_packet_len,magic_number:config.magic_number,retry_delta_ticks:RETRY_DELTA_TICKS,max_retries:MAX_RETRIES,timeout_ticks:CONNECTION_TIMEOUT_TICKS},port:None,send_buffer,initialized:false})
    }
    pub const fn config(&self)->NullModemConfig{self.config}
    pub const fn base_config(&self)->ConnectionBaseConfig{self.base}
    pub const fn port(&self)->Option<SerialPort>{self.port}
    pub const fn is_initialized(&self)->bool{self.initialized}
    pub fn send_buffer(&self)->&[u8]{&self.send_buffer}
    pub fn send_buffer_mut(&mut self)->&mut [u8]{&mut self.send_buffer}
    pub fn init(&mut self,port:SerialPort){self.initialized=true;self.port=Some(port);}
    /// The active Send body immediately returns zero; serial framing and writes are #if (0).
    pub const fn send(&self,_packet:&[u8],_extra:Option<&[u8]>)->i32{0}
    /// MSB-first u32 rolling checksum, preserving C++ unsigned wrap semantics.
    pub fn compute_crc(buffer:&[u8])->u32{let mut sum=0u32;for &byte in buffer{let hibit=sum>>31;sum=sum.wrapping_shl(1).wrapping_add(hibit).wrapping_add(byte as u32);}sum}
    /// SerialHeaderType + SerialCRCType + parent CommHeaderType.
    pub const fn packet_overhead_size()->usize{PACKET_SERIAL_OVERHEAD_SIZE+COMM_HEADER_SIZE}
    /// Parent MaxPacketLen plus serial header, CRC and the trailing carriage return.
    pub const fn actual_max_packet_size(&self)->usize{self.base.max_packet_len+SERIAL_HEADER_SIZE+SERIAL_CRC_SIZE+SERIAL_TERMINATOR_SIZE}
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]fn constructor_uses_parent_defaults_and_staging_size(){let c=NullModemConnection::new(NullModemConfig{num_send:8,num_receive:12,application_max_packet_len:512,magic_number:0x1234}).unwrap();assert_eq!(c.base_config(),ConnectionBaseConfig{num_send:8,num_receive:12,max_packet_len:520,magic_number:0x1234,retry_delta_ticks:60,max_retries:-1,timeout_ticks:1200});assert_eq!(c.actual_max_packet_size(),531);assert_eq!(c.send_buffer().len(),531);}
 #[test]fn init_sets_port_and_send_remains_disabled(){let mut c=NullModemConnection::new(NullModemConfig{num_send:1,num_receive:1,application_max_packet_len:4,magic_number:1}).unwrap();c.init(SerialPort::Handle(1));assert!(c.is_initialized());assert_eq!(c.send(b"data",None),0);assert_eq!(c.send_buffer().len(),23);}
 #[test]fn crc_matches_legacy_rolling_checksum(){assert_eq!(NullModemConnection::compute_crc(b""),0);assert_eq!(NullModemConnection::compute_crc(b"123456789"),0x0000_63C5);assert_eq!(NullModemConnection::compute_crc(&[0xFF,0x80,0x01]),0x0000_04FD);}
 #[test]fn packet_sizes_and_magic_values_match_header(){assert_eq!(PACKET_SERIAL_START,0xDABD);assert_eq!(PACKET_SERIAL_VERIFY,0xDEAF);assert_eq!(PACKET_SERIAL_OVERHEAD_SIZE,10);assert_eq!(NullModemConnection::packet_overhead_size(),18);}
 #[test]fn rejects_overflowing_packet_size(){let result=NullModemConnection::new(NullModemConfig{num_send:1,num_receive:1,application_max_packet_len:usize::MAX,magic_number:0});assert_eq!(result,Err(NullModemError::SizeOverflow));}
}
