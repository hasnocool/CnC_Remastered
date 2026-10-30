//! Tiberian Dawn WWKeyboardClass port from WIN32LIB/KEYBOARD.CPP.
//!
//! This variant builds VK/ASCII/toggle remap tables at construction and
//! intentionally preserves its source quirks: Message_Handler returns
//! immediately, and To_ASCII's logical-AND release test rejects every
//! nonzero key.

use crate::keyboard::{
    WWKEY_SHIFT_BIT, WWKEY_CTRL_BIT, WWKEY_ALT_BIT, WWKEY_RLS_BIT,
    WWKEY_VK_BIT, WWKEY_DBL_BIT, WWKEY_BTN_BIT, VK_LBUTTON, VK_MBUTTON,
    VK_RBUTTON, VK_SHIFT, VK_CONTROL, VK_MENU, VK_CAPITAL, VK_NUMLOCK,
};

pub const KEY_BUFFER_SIZE: usize = 256;
pub const KEY_BUFFER_MASK: usize = 255;
pub const VK_NUMPAD0: u16 = 0x60;
pub const VK_DIVIDE: u16 = 0x6f;
pub const VK_NONE: u16 = 0;
pub const KA_NONE: u16 = 0;
pub const KN_NONE: u16 = 0;

pub trait KeyboardLayout {
    fn is_printable(&self, value: u8) -> bool;
    fn vk_key_scan(&self, value: u8) -> i16;
    fn is_alpha(&self, value: u8) -> bool;
    fn is_upper(&self, value: u8) -> bool;
}

pub trait TiberianDawnKeyboardPlatform {
    fn key_state(&self, virtual_key: u16) -> u16;
    fn async_key_state(&self, virtual_key: u16) -> i32;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TiberianDawnKeyboard {
    pub mouse_qx: i32,
    pub mouse_qy: i32,
    pub vk_remap: [u8; 256],
    ascii_remap: [u8; 2048],
    toggle_keys: [u8; 256],
    buffer: [u16; KEY_BUFFER_SIZE],
    head: usize,
    tail: usize,
    pub mouse_state: i32,
    pub conditional: i32,
    pub current_cursor: Option<usize>,
}

impl TiberianDawnKeyboard {
    pub fn new(layout: &impl KeyboardLayout) -> Self {
        let mut keyboard = Self {
            mouse_qx: 0, mouse_qy: 0, vk_remap: [0; 256],
            ascii_remap: [0; 2048], toggle_keys: [0; 256],
            buffer: [0; KEY_BUFFER_SIZE], head: 0, tail: 0,
            mouse_state: 0, conditional: 0, current_cursor: None,
        };
        for value in 31u16..255 {
            let byte = value as u8;
            if layout.is_printable(byte) {
                let vk = layout.vk_key_scan(byte);
                if vk > 0 && vk < 2048 {
                    keyboard.ascii_remap[vk as usize] = byte;
                    keyboard.vk_remap[byte as usize] = (vk & 0xff) as u8;
                }
            }
        }
        for value in 0u16..255 {
            let byte = value as u8;
            if layout.is_alpha(byte) && layout.is_upper(byte) {
                keyboard.toggle_keys[byte as usize] = 1;
            }
            if value >= VK_NUMPAD0 && value <= VK_DIVIDE {
                keyboard.toggle_keys[byte as usize] = 2;
            }
        }
        keyboard
    }

    pub fn is_mouse_key(key: u16) -> bool {
        matches!(key & 0xff, VK_LBUTTON | VK_MBUTTON | VK_RBUTTON)
    }

    pub fn len(&self) -> usize {
        self.tail.wrapping_sub(self.head) & KEY_BUFFER_MASK
    }
    pub fn is_empty(&self) -> bool { self.head == self.tail }
    pub fn is_full(&self) -> bool { ((self.tail + 1) & KEY_BUFFER_MASK) == self.head }

    pub fn put(&mut self, key: u16) -> bool {
        let next = (self.tail + 1) & KEY_BUFFER_MASK;
        if next == self.head { return false; }
        self.buffer[self.tail] = key;
        self.tail = next;
        true
    }

    pub fn clear(&mut self) { self.head = self.tail; }

    pub fn check(&self) -> u16 {
        if self.head == self.tail { 0 } else { self.buffer[self.head] }
    }

    pub fn get(&mut self) -> u16 {
        let temp = self.buff_get();
        let bits = temp & 0xff00;
        if bits & WWKEY_VK_BIT == 0 {
            self.ascii_remap[(temp & 0x01ff) as usize] as u16 | bits
        } else {
            temp
        }
    }

    pub fn buff_get(&mut self) -> u16 {
        if self.head == self.tail { return 0; }
        let temp = self.buffer[self.head];
        let new_head = if Self::is_mouse_key(temp) {
            self.mouse_qx = self.buffer[(self.head + 1) & KEY_BUFFER_MASK] as i32;
            self.mouse_qy = self.buffer[(self.head + 2) & KEY_BUFFER_MASK] as i32;
            self.head.wrapping_add(3)
        } else {
            self.head.wrapping_add(1)
        };
        self.head = new_head & KEY_BUFFER_MASK;
        temp
    }

    pub fn put_key_message(
        &mut self, virtual_key: u16, release: bool, double_click: bool,
        platform: &impl TiberianDawnKeyboardPlatform,
    ) -> bool {
        let mut bits = 0u16;
        if !matches!(virtual_key, VK_LBUTTON | VK_MBUTTON | VK_RBUTTON) {
            let shift = platform.key_state(VK_SHIFT) & 0xff00 != 0;
            let ctrl = platform.key_state(VK_CONTROL) & 0xff00 != 0;
            let alt = platform.key_state(VK_MENU) & 0xff00 != 0;
            let caps = platform.key_state(VK_CAPITAL) & 0x00ff != 0
                && self.toggle_keys[virtual_key as usize] == 1;
            let nums = platform.key_state(VK_NUMLOCK) & 0x00ff != 0
                && self.toggle_keys[virtual_key as usize] == 2;
            if shift || caps || nums { bits |= WWKEY_SHIFT_BIT; }
            if ctrl { bits |= WWKEY_CTRL_BIT; }
            if alt { bits |= WWKEY_ALT_BIT; }
        }
        let index = (virtual_key | bits) as usize;
        if self.ascii_remap[index] == 0 { bits |= WWKEY_VK_BIT; }
        if release { bits |= WWKEY_RLS_BIT; }
        if double_click { bits |= WWKEY_DBL_BIT; }
        self.put(virtual_key | bits)
    }

    /// The active upstream Message_Handler is a no-op; its entire switch is
    /// under #if(0), and the function returns before reaching it.
    pub fn message_handler_noop(&mut self) {}

    pub fn down(&self, key: u16, platform: &impl TiberianDawnKeyboardPlatform) -> i32 {
        platform.async_key_state(key & 0xff)
    }

    /// Source To_ASCII uses (key && WWKEY_RLS_BIT), not a bitwise test.
    pub fn to_ascii(&self, key: i32) -> u16 {
        if key != 0 && WWKEY_RLS_BIT != 0 { KN_NONE } else { key as u16 }
    }

    pub fn split(key: i32) -> (u8, bool, bool, bool, bool, bool) {
        (
            (key & 0xff) as u8,
            key & WWKEY_SHIFT_BIT as i32 != 0,
            key & WWKEY_CTRL_BIT as i32 != 0,
            key & WWKEY_ALT_BIT as i32 != 0,
            key & WWKEY_RLS_BIT as i32 != 0,
            key & WWKEY_DBL_BIT as i32 != 0,
        )
    }

    pub fn check_key(&self) -> u16 { self.check() & !WWKEY_SHIFT_BIT }
    pub fn check_key_num(&self) -> u16 { key_to_num(self.check()) }
    pub fn get_key_num(&mut self) -> u16 { key_to_num(self.get()) }

    pub fn get_key(&mut self) -> u16 {
        let value = self.get() & !WWKEY_SHIFT_BIT;
        if value & WWKEY_RLS_BIT != 0 { KN_NONE } else { value }
    }

    pub fn kn_to_ka(key: u16) -> u16 {
        if key & WWKEY_RLS_BIT != 0 { return KA_NONE; }
        if key & WWKEY_VK_BIT == 0 {
            let mut value = key & 0x00ff;
            let mut flags = key & 0xff00;
            if flags & WWKEY_SHIFT_BIT != 0 {
                value = (value as u8).to_ascii_uppercase() as u16;
                flags &= !WWKEY_SHIFT_BIT;
            }
            value | flags
        } else {
            let value = key & 0xff;
            if (VK_NUMPAD0..=VK_NUMPAD0 + 9).contains(&value) {
                value - VK_NUMPAD0 + b'0' as u16
            } else { key }
        }
    }

    pub fn kn_to_vk(&self, key: u16) -> u16 {
        if key & WWKEY_RLS_BIT != 0 { return VK_NONE; }
        let flags = key & 0xff00;
        let mut value = key;
        if flags & WWKEY_VK_BIT == 0 {
            value = self.vk_remap[(key & 0xff) as usize] as u16 | flags;
        }
        value & !WWKEY_VK_BIT
    }

    pub fn mouse_x(&self) -> i32 { self.mouse_qx }
    pub fn mouse_y(&self) -> i32 { self.mouse_qy }
    pub fn ascii_mapping(&self, vk_with_modifiers: usize) -> u8 {
        self.ascii_remap.get(vk_with_modifiers).copied().unwrap_or(0)
    }
    pub fn toggle_kind(&self, virtual_key: u8) -> u8 { self.toggle_keys[virtual_key as usize] }
}

fn key_to_num(value: u16) -> u16 {
    let mut flags = value & 0xff00;
    let mut key = value & 0x00ff;
    if (key as u8).is_ascii_uppercase() {
        key = (key as u8).to_ascii_lowercase() as u16;
        if flags & WWKEY_VK_BIT == 0 { flags |= WWKEY_SHIFT_BIT; }
    }
    key | flags
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AsciiLayout;
    impl KeyboardLayout for AsciiLayout {
        fn is_printable(&self, value: u8) -> bool { (32..=126).contains(&value) }
        fn vk_key_scan(&self, value: u8) -> i16 {
            match value {
                b'a'..=b'z' => (value - 32) as i16,
                b'A'..=b'Z' => (((value - 32) as i16) | 0x0100),
                b'0'..=b'9' => value as i16,
                b'!' => 0x0131,
                _ => -1,
            }
        }
        fn is_alpha(&self, value: u8) -> bool { value.is_ascii_alphabetic() }
        fn is_upper(&self, value: u8) -> bool { value.is_ascii_uppercase() }
    }

    #[derive(Default)]
    struct FakePlatform { shift: bool, ctrl: bool, alt: bool, caps: bool, num: bool, down: bool }
    impl TiberianDawnKeyboardPlatform for FakePlatform {
        fn key_state(&self, key: u16) -> u16 {
            match key {
                VK_SHIFT if self.shift => 0xff00,
                VK_CONTROL if self.ctrl => 0xff00,
                VK_MENU if self.alt => 0xff00,
                VK_CAPITAL if self.caps => 1,
                VK_NUMLOCK if self.num => 1,
                _ => 0,
            }
        }
        fn async_key_state(&self, _key: u16) -> i32 { if self.down { -1 } else { 0 } }
    }

    #[test]
    fn constructor_builds_ascii_vk_and_toggle_tables() {
        let keyboard = TiberianDawnKeyboard::new(&AsciiLayout);
        assert_eq!(keyboard.ascii_mapping(0x41), b'a');
        assert_eq!(keyboard.ascii_mapping(0x141), b'A');
        assert_eq!(keyboard.vk_remap[b'a' as usize], 0x41);
        assert_eq!(keyboard.toggle_kind(b'A'), 1);
        assert_eq!(keyboard.toggle_kind(0x60), 2);
    }

    #[test]
    fn put_and_get_translate_ascii_and_virtual_keys() {
        let mut keyboard = TiberianDawnKeyboard::new(&AsciiLayout);
        let platform = FakePlatform::default();
        assert!(keyboard.put_key_message(0x41, false, false, &platform));
        assert_eq!(keyboard.get(), b'a' as u16);
        assert!(keyboard.put_key_message(0x70, false, false, &platform));
        assert_eq!(keyboard.get(), 0x70 | WWKEY_VK_BIT);
    }

    #[test]
    fn caps_and_shift_only_affect_alpha_keys() {
        let mut keyboard = TiberianDawnKeyboard::new(&AsciiLayout);
        let caps = FakePlatform { caps: true, ..Default::default() };
        assert!(keyboard.put_key_message(0x41, false, false, &caps));
        assert_eq!(keyboard.get(), b'A' as u16 | WWKEY_SHIFT_BIT);
        let caps_digit = FakePlatform { caps: true, ..Default::default() };
        assert!(keyboard.put_key_message(b'1' as u16, false, false, &caps_digit));
        assert_eq!(keyboard.get(), b'1' as u16);
    }

    #[test]
    fn buffer_mouse_triples_and_double_click_bits_match_source() {
        let mut keyboard = TiberianDawnKeyboard::new(&AsciiLayout);
        let platform = FakePlatform::default();
        assert!(keyboard.put_key_message(VK_LBUTTON, true, true, &platform));
        keyboard.put(123);
        keyboard.put(456);
        assert_eq!(keyboard.get(), VK_LBUTTON | WWKEY_VK_BIT | WWKEY_RLS_BIT | WWKEY_DBL_BIT);
        assert_eq!((keyboard.mouse_x(), keyboard.mouse_y()), (123, 456));
    }

    #[test]
    fn compatibility_key_conversions_preserve_shift_and_numpad_rules() {
        let keyboard = TiberianDawnKeyboard::new(&AsciiLayout);
        assert_eq!(TiberianDawnKeyboard::kn_to_ka(b'a' as u16 | WWKEY_SHIFT_BIT), b'A' as u16);
        assert_eq!(TiberianDawnKeyboard::kn_to_ka(0x60 | WWKEY_VK_BIT), b'0' as u16);
        assert_eq!(keyboard.kn_to_vk(b'a' as u16), 0x41);
        assert_eq!(TiberianDawnKeyboard::kn_to_ka(0x41 | WWKEY_RLS_BIT), KA_NONE);
    }

    #[test]
    fn source_to_ascii_logical_and_bug_returns_none_for_nonzero() {
        let keyboard = TiberianDawnKeyboard::new(&AsciiLayout);
        assert_eq!(keyboard.to_ascii(0), 0);
        assert_eq!(keyboard.to_ascii(b'A' as i32), KN_NONE);
    }

    #[test]
    fn ring_buffer_and_down_semantics() {
        let mut keyboard = TiberianDawnKeyboard::new(&AsciiLayout);
        for i in 0..255 { assert!(keyboard.put(i)); }
        assert!(keyboard.is_full());
        assert!(!keyboard.put(256));
        keyboard.clear();
        let platform = FakePlatform { down: true, ..Default::default() };
        assert_eq!(keyboard.down(0x41, &platform), -1);
    }
}
