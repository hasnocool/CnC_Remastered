//! Checked readers and compatibility codecs for legacy game data.

pub mod adpcm;
pub mod base64;
pub mod base64_streams;
pub mod binary;
pub mod crc32;
pub mod crc_streams;
pub mod lcw;
pub mod lcw_blocks;
pub mod lzw;
pub mod lzw_blocks;
pub mod legacy_crc;
pub mod lzo1x;
pub mod lzo_blocks;
pub mod pipe;
pub mod random_straw;
pub mod readline;
pub mod sha1;
pub mod straw;
pub mod mix;

pub use binary::{BinaryReadError, BinaryReader};
pub use mix::{MixArchive, MixEntry, MixParseError};
