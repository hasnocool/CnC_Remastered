//! Global initializer ported from REDALERT/_WSPROTO.CPP.
//!
//! The legacy translation unit defines PacketTransport as a null pointer.
//! The Winsock interface itself is owned by the networking implementation.

/// Opaque handle for the optional legacy Winsock packet transport.
pub static PACKET_TRANSPORT: Option<usize> = None;
