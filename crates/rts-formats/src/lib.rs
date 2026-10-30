//! Checked readers and compatibility codecs for legacy game data.

pub mod adpcm;
pub mod base64;
pub mod base64_streams;
pub mod cache_straw;
pub mod binary;
pub mod blowfish;
pub mod blowfish_streams;
pub mod crc32;
pub mod crc_streams;
pub mod lcw;
pub mod lcw_blocks;
pub mod lzw;
pub mod lzw_blocks;
pub mod legacy_crc;
pub mod lzo1x;
pub mod lzo_blocks;
pub mod io_streams;
pub mod pipe;
pub mod random_straw;
pub mod readline;
pub mod sha1;
pub mod sha_streams;
pub mod straw;
pub mod mix;
pub mod palette;
pub mod dipthong_tables;
pub mod dipthong;

pub use binary::{BinaryReadError, BinaryReader};
pub use mix::{MixArchive, MixEntry, MixParseError};
