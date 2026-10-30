//! Network FieldClass constructors and endian conversion from both FIELD.CPPs.

pub const FIELD_HEADER_SIZE: usize = 8;
pub const TYPE_CHAR: u16 = 1;
pub const TYPE_UNSIGNED_CHAR: u16 = 2;
pub const TYPE_SHORT: u16 = 3;
pub const TYPE_UNSIGNED_SHORT: u16 = 4;
pub const TYPE_LONG: u16 = 5;
pub const TYPE_UNSIGNED_LONG: u16 = 6;
pub const TYPE_STRING: u16 = 7;
pub const TYPE_CHUNK: u16 = 20;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldByteOrder { Host, Network }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    pub id: [u8; 4],
    pub data_type: u16,
    pub size: u16,
    pub data: Vec<u8>,
    pub byte_order: FieldByteOrder,
}

impl Field {
    fn new(id: &[u8], data_type: u16, data: Vec<u8>) -> Self {
        let size = data.len() as u16;
        Self { id: field_id(id), data_type, size, data, byte_order: FieldByteOrder::Host }
    }

    pub fn from_char(id: &[u8], value: i8) -> Self { Self::new(id, TYPE_CHAR, value.to_ne_bytes().to_vec()) }
    pub fn from_unsigned_char(id: &[u8], value: u8) -> Self { Self::new(id, TYPE_UNSIGNED_CHAR, vec![value]) }
    pub fn from_short(id: &[u8], value: i16) -> Self { Self::new(id, TYPE_SHORT, value.to_ne_bytes().to_vec()) }
    pub fn from_unsigned_short(id: &[u8], value: u16) -> Self { Self::new(id, TYPE_UNSIGNED_SHORT, value.to_ne_bytes().to_vec()) }
    /// Windows long is 32 bits; use i32 even on LP64 hosts.
    pub fn from_long(id: &[u8], value: i32) -> Self { Self::new(id, TYPE_LONG, value.to_ne_bytes().to_vec()) }
    pub fn from_unsigned_long(id: &[u8], value: u32) -> Self { Self::new(id, TYPE_UNSIGNED_LONG, value.to_ne_bytes().to_vec()) }

    pub fn from_string(id: &[u8], value: &[u8]) -> Self {
        let value = nul_terminated(value);
        let size = value.len().saturating_add(1).min(u16::MAX as usize);
        let mut data = value[..size.saturating_sub(1).min(value.len())].to_vec();
        data.push(0);
        Self::new(id, TYPE_STRING, data)
    }

    pub fn from_chunk(id: &[u8], value: &[u8]) -> Self {
        let size = value.len().min(u16::MAX as usize);
        Self::new(id, TYPE_CHUNK, value[..size].to_vec())
    }

    /// Host_To_Net mutates numeric payloads first, then DataType and Size.
    pub fn host_to_net(&mut self) {
        if self.byte_order == FieldByteOrder::Network { return; }
        match self.data_type {
            TYPE_SHORT | TYPE_UNSIGNED_SHORT if self.data.len() >= 2 => {
                let value = u16::from_ne_bytes([self.data[0], self.data[1]]);
                self.data[..2].copy_from_slice(&value.to_be_bytes());
            }
            TYPE_LONG | TYPE_UNSIGNED_LONG if self.data.len() >= 4 => {
                let value = u32::from_ne_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]);
                self.data[..4].copy_from_slice(&value.to_be_bytes());
            }
            _ => {}
        }
        self.data_type = self.data_type.to_be();
        self.size = self.size.to_be();
        self.byte_order = FieldByteOrder::Network;
    }

    /// Net_To_Host restores header fields before selecting the payload type.
    pub fn net_to_host(&mut self) {
        if self.byte_order == FieldByteOrder::Host { return; }
        self.size = u16::from_be(self.size);
        self.data_type = u16::from_be(self.data_type);
        match self.data_type {
            TYPE_SHORT | TYPE_UNSIGNED_SHORT if self.data.len() >= 2 => {
                let value = u16::from_be_bytes([self.data[0], self.data[1]]);
                self.data[..2].copy_from_slice(&value.to_ne_bytes());
            }
            TYPE_LONG | TYPE_UNSIGNED_LONG if self.data.len() >= 4 => {
                let value = u32::from_be_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]);
                self.data[..4].copy_from_slice(&value.to_ne_bytes());
            }
            _ => {}
        }
        self.byte_order = FieldByteOrder::Host;
    }

    /// Serialize the 8-byte FIELD_HEADER_SIZE prefix followed by payload.
    /// Call Host_To_Net first to match the legacy packet wire representation.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(FIELD_HEADER_SIZE + self.data.len());
        bytes.extend_from_slice(&self.id);
        bytes.extend_from_slice(&self.data_type.to_ne_bytes());
        bytes.extend_from_slice(&self.size.to_ne_bytes());
        bytes.extend_from_slice(&self.data);
        bytes
    }

    pub fn host_u16(&self) -> Option<u16> {
        if self.byte_order != FieldByteOrder::Host || self.data.len() < 2 { return None; }
        Some(u16::from_ne_bytes([self.data[0], self.data[1]]))
    }
    pub fn host_u32(&self) -> Option<u32> {
        if self.byte_order != FieldByteOrder::Host || self.data.len() < 4 { return None; }
        Some(u32::from_ne_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]))
    }
}

fn field_id(id: &[u8]) -> [u8; 4] {
    let mut out = [0; 4];
    let count = id.len().min(4);
    out[..count].copy_from_slice(&id[..count]);
    out
}
fn nul_terminated(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_match_field_type_and_size_values() {
        let c = Field::from_char(b"CHAR", -2);
        assert_eq!((c.data_type, c.size), (TYPE_CHAR, 1));
        assert_eq!(Field::from_unsigned_char(b"BYTE", 255).data, [255]);
        assert_eq!(Field::from_short(b"SHRT", -2).size, 2);
        assert_eq!(Field::from_unsigned_short(b"USH", 0x1234).data.len(), 2);
        assert_eq!(Field::from_long(b"LONG", -2).size, 4);
        assert_eq!(Field::from_unsigned_long(b"ULONG", 0x12345678).data.len(), 4);
        assert_eq!(Field::from_string(b"TEXT", b"abc").data, b"abc\0");
        assert_eq!(Field::from_chunk(b"DATA", &[1, 2, 3]).data, [1, 2, 3]);
        assert_eq!(field_id(b"TOOLONG"), *b"TOOL");
    }

    #[test]
    fn short_and_long_fields_round_trip_network_order() {
        let mut short = Field::from_unsigned_short(b"SIZE", 0x1234);
        short.host_to_net();
        assert_eq!(short.data, [0x12, 0x34]);
        assert_eq!(short.wire_bytes()[4..8], [0, TYPE_UNSIGNED_SHORT as u8, 0, 2]);
        short.net_to_host();
        assert_eq!(short.host_u16(), Some(0x1234));

        let mut long = Field::from_unsigned_long(b"DATA", 0x12345678);
        long.host_to_net();
        assert_eq!(long.data, [0x12, 0x34, 0x56, 0x78]);
        long.net_to_host();
        assert_eq!(long.host_u32(), Some(0x12345678));
    }

    #[test]
    fn character_string_and_chunk_payloads_are_not_byte_swapped() {
        for mut field in [
            Field::from_char(b"CHAR", -1),
            Field::from_string(b"TEXT", b"abc"),
            Field::from_chunk(b"DATA", &[1, 2, 3]),
        ] {
            let original = field.data.clone();
            field.host_to_net();
            assert_eq!(field.data, original);
            field.net_to_host();
            assert_eq!(field.data, original);
        }
    }

    #[test]
    fn conversion_is_idempotent_for_current_byte_order() {
        let mut field = Field::from_unsigned_long(b"SIZE", 0x12345678);
        field.host_to_net();
        let network = field.wire_bytes();
        field.host_to_net();
        assert_eq!(field.wire_bytes(), network);
        field.net_to_host();
        assert_eq!(field.host_u32(), Some(0x12345678));
    }
}
