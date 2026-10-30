//! Palette file loading contract from TIBERIANDAWN/WIN32LIB/LOADPAL.CPP.

use std::io;

/// Number of bytes in the legacy IBM 256-colour RGB palette.
pub const IBM_PALETTE_BYTES: usize = 256 * 3;

/// Load a palette through the same fixed-size data-loading abstraction used
/// by the original Load_Data call.
pub fn load_palette_with<F>(file_name: &str, palette: &mut [u8], load_data: F) -> io::Result<()>
where
    F: FnOnce(&str, &mut [u8], usize) -> io::Result<()>,
{
    if palette.len() < IBM_PALETTE_BYTES {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "palette buffer must hold 768 bytes"));
    }
    load_data(file_name, palette, IBM_PALETTE_BYTES)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_exact_legacy_palette_length() {
        let mut bytes = [0u8; IBM_PALETTE_BYTES];
        let mut requested = 0;
        load_palette_with("temperat.pal", &mut bytes, |name, dst, len| {
            assert_eq!(name, "temperat.pal");
            requested = len;
            dst[0] = 7;
            Ok(())
        }).unwrap();
        assert_eq!(requested, 768);
        assert_eq!(bytes[0], 7);
    }
}
