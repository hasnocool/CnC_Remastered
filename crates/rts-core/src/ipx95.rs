//! Windows 95 IPX compatibility stubs from both variants' IPX95.CPP.
//!
//! The pinned upstream sources deliberately disable the THIPX32 loader and
//! return fixed failure/success values for every legacy IPX entry point.

pub static WINDOWS_NT: bool = false;

pub fn load_ipx_dll() -> bool { false }
pub fn unload_ipx_dll() {}

pub fn ipx_open_socket(_socket: u16) -> i32 { -1 }
pub fn ipx_close_socket(_socket: u16) -> i32 { 0 }
pub fn ipx_get_connection_number() -> i32 { -1 }
pub fn ipx_broadcast_packet(_buffer: &[u8]) -> i32 { 0 }
pub fn ipx_get_local_target(
    _destination_network: &mut [u8],
    _destination_node: &mut [u8],
    _destination_socket: u16,
    _bridge_address: &mut [u8],
) -> i32 { 0 }

/// C-linkage stub equivalents defined in each upstream IPX95 translation unit.
pub fn ipx_initialise() -> bool { false }
pub fn ipx_get_outstanding_buffer(_buffer: &mut [u8]) -> bool { false }
pub fn ipx_shutdown() {}
pub fn ipx_send_packet(
    _destination_network: &[u8],
    _destination_node: &[u8],
    _length: i32,
    _buffer: &[u8],
    _source_address: &[u8],
) -> i32 { 0 }
pub fn ipx_broadcast_packet95(_buffer: &[u8], _length: i32) -> i32 { 0 }
pub fn ipx_start_listening() -> bool { false }
pub fn ipx_open_socket95(_socket: i32) -> i32 { 0 }
pub fn ipx_close_socket95(_socket: i32) {}
pub fn ipx_get_connection_number95() -> i32 { 0 }
pub fn ipx_get_local_target95(
    _destination_network: &mut [u8],
    _destination_node: &mut [u8],
    _destination_socket: u16,
    _bridge_address: &mut [u8],
) -> i32 { 0 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_ipx_api_matches_disabled_windows95_provider() {
        assert!(!load_ipx_dll());
        assert_eq!(ipx_open_socket(0x452), -1);
        assert_eq!(ipx_close_socket(0x452), 0);
        assert_eq!(ipx_get_connection_number(), -1);
        assert_eq!(ipx_broadcast_packet(b"packet"), 0);
        assert_eq!(ipx_get_local_target(&mut [0; 4], &mut [0; 6], 0x452, &mut [0; 6]), 0);
        unload_ipx_dll();
    }

    #[test]
    fn exported_ipx95_entry_points_are_explicit_stubs() {
        assert!(!ipx_initialise());
        assert!(!ipx_get_outstanding_buffer(&mut [0; 64]));
        assert_eq!(ipx_send_packet(&[], &[], 0, &[], &[]), 0);
        assert_eq!(ipx_broadcast_packet95(&[], 0), 0);
        assert!(!ipx_start_listening());
        assert_eq!(ipx_open_socket95(0x452), 0);
        ipx_close_socket95(0x452);
        assert_eq!(ipx_get_connection_number95(), 0);
        assert_eq!(ipx_get_local_target95(&mut [0; 4], &mut [0; 6], 0x452, &mut [0; 6]), 0);
    }
}
