//! Set_Font state update from the shared WIN32LIB/SET_FONT.CPP implementation.

pub const FONT_INFO_BLOCK_OFFSET: usize = 4;
pub const FONT_WIDTH_BLOCK_OFFSET: usize = 8;
pub const FONT_INFO_MAX_HEIGHT_OFFSET: usize = 4;
pub const FONT_INFO_MAX_WIDTH_OFFSET: usize = 5;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontState {
    pub font_address: Option<usize>,
    pub width_block_address: Option<usize>,
    pub height: i8,
    pub width: i8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontError { HeaderOutOfBounds, InfoBlockOutOfBounds, AddressOverflow }

impl FontState {
    /// Set the active font from its in-memory bytes. None mirrors a null pointer.
    pub fn set_font(&mut self, font_address: Option<usize>, bytes: &[u8]) -> Result<Option<usize>, FontError> {
        let old = self.font_address;
        let Some(base) = font_address else { return Ok(old) };
        let read_offset = |at: usize| -> Result<usize, FontError> {
            let end = at.checked_add(2).ok_or(FontError::AddressOverflow)?;
            let pair = bytes.get(at..end).ok_or(FontError::HeaderOutOfBounds)?;
            Ok(u16::from_le_bytes([pair[0], pair[1]]) as usize)
        };
        let width_offset = read_offset(FONT_WIDTH_BLOCK_OFFSET)?;
        let info_offset = read_offset(FONT_INFO_BLOCK_OFFSET)?;
        let height_at = info_offset.checked_add(FONT_INFO_MAX_HEIGHT_OFFSET).ok_or(FontError::AddressOverflow)?;
        let width_at = info_offset.checked_add(FONT_INFO_MAX_WIDTH_OFFSET).ok_or(FontError::AddressOverflow)?;
        let height = *bytes.get(height_at).ok_or(FontError::InfoBlockOutOfBounds)? as i8;
        let width = *bytes.get(width_at).ok_or(FontError::InfoBlockOutOfBounds)? as i8;
        let width_block_address = base.checked_add(width_offset).ok_or(FontError::AddressOverflow)?;
        self.font_address = Some(base);
        self.width_block_address = Some(width_block_address);
        self.height = height;
        self.width = width;
        Ok(old)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reads_little_endian_block_offsets_and_metrics() {
        let mut bytes=vec![0u8;32];
        bytes[4..6].copy_from_slice(&12u16.to_le_bytes());
        bytes[8..10].copy_from_slice(&20u16.to_le_bytes());
        bytes[16]=18; bytes[17]=9;
        let mut state=FontState::default();
        assert_eq!(state.set_font(Some(0x1000),&bytes),Ok(None));
        assert_eq!(state.font_address,Some(0x1000));
        assert_eq!(state.width_block_address,Some(0x1014));
        assert_eq!((state.height,state.width),(18,9));
    }
    #[test]
    fn null_font_preserves_state() {
        let mut state=FontState{font_address:Some(4),..FontState::default()};
        assert_eq!(state.set_font(None,&[]),Ok(Some(4)));
        assert_eq!(state.font_address,Some(4));
    }
}
