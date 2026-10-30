//! Checked readers and compatibility codecs for legacy game data.

pub mod binary;
pub mod crc32;
pub mod crc_streams;
pub mod legacy_crc;
pub mod mix;

pub use binary::{BinaryReadError, BinaryReader};
pub use mix::{MixArchive, MixEntry, MixParseError};
