//! Checked readers and compatibility codecs for legacy game data.

pub mod adpcm;
pub mod binary;
pub mod crc32;
pub mod crc_streams;
pub mod lcw;
pub mod lcw_blocks;
pub mod lzw;
pub mod legacy_crc;
pub mod pipe;
pub mod readline;
pub mod straw;
pub mod mix;

pub use binary::{BinaryReadError, BinaryReader};
pub use mix::{MixArchive, MixEntry, MixParseError};
