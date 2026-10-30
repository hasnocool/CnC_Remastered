//! Active null-modem fallback code from REDALERT/NULLDLG.CPP.
//!
//! The remainder of NULLDLG.CPP is enclosed by an unconditional #if (0).
//! The only active function returns GAME_NORMAL from Select_Serial_Dialog.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SerialDialogGameType {
    Normal,
}

pub const fn select_serial_dialog() -> SerialDialogGameType {
    SerialDialogGameType::Normal
}
