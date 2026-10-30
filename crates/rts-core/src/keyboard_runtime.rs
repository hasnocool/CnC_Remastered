//! Win32-era keyboard ring-buffer behavior from REDALERT/KEY.CPP and KEY.H.
//!
//! The original Message_Handler body is enclosed by #if (0), so the active
//! translation unit returns false for window messages. Queue operations,
//! modifier snapshots, mouse-coordinate triplets, ASCII translation and
//! system-message pumping remain represented through KeyboardHost.

pub const KEY_BUFFER_SIZE: usize = 256;
pub const VK_LBUTTON: u8 = 0x01;
pub const VK_RBUTTON: u8 = 0x02;
pub const VK_MBUTTON: u8 = 0x04;
pub const VK_SHIFT: u8 = 0x10;
pub const VK_CONTROL: u8 = 0x11;
pub const VK_MENU: u8 = 0x12;
pub const VK_CAPITAL: u8 = 0x14;
pub const VK_NUMLOCK: u8 = 0x90;

pub const WWKEY_SHIFT_BIT: u16 = 0x0100;
pub const WWKEY_CTRL_BIT: u16 = 0x0200;
pub const WWKEY_ALT_BIT: u16 = 0x0400;
pub const WWKEY_RLS_BIT: u16 = 0x0800;
pub const WWKEY_VK_BIT: u16 = 0x1000;
pub const WWKEY_DBL_BIT: u16 = 0x2000;
pub const WWKEY_BTN_BIT: u16 = 0x8000;

pub trait KeyboardHost {
    fn key_state(&mut self, virtual_key: u8) -> i16;
    fn async_key_state(&mut self, virtual_key: u8) -> i16;
    fn map_virtual_key(&mut self, virtual_key: u8) -> u32;
    fn to_ascii(&mut self, virtual_key: u8, scan_code: u32, state: &[u8; 256]) -> Option<u8>;

    /// Drain pending host messages. The adapter may route events through
    /// push_key/push_mouse; the source's own Message_Handler is a no-op.
    fn dispatch_pending_messages(&mut self, keyboard: &mut WWKeyboard);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WWKeyboard {
    buffer: [u16; KEY_BUFFER_SIZE],
    head: usize,
    tail: usize,
    key_state: [u8; 256],
    pub mouse_qx: i32,
    pub mouse_qy: i32,
}

impl Default for WWKeyboard {
    fn default() -> Self { Self::new() }
}

impl WWKeyboard {
    pub fn new() -> Self {
        Self {
            buffer: [0; KEY_BUFFER_SIZE],
            head: 0,
            tail: 0,
            key_state: [0; 256],
            mouse_qx: 0,
            mouse_qy: 0,
        }
    }

    pub fn is_mouse_key(key: u16) -> bool {
        matches!(key as u8, VK_LBUTTON | VK_MBUTTON | VK_RBUTTON)
    }

    pub fn is_buffer_empty(&self) -> bool { self.head == self.tail }
    pub fn is_buffer_full(&self) -> bool { (self.tail + 1) % KEY_BUFFER_SIZE == self.head }
    pub fn head_index(&self) -> usize { self.head }
    pub fn tail_index(&self) -> usize { self.tail }
    pub fn queued_element_count(&self) -> usize {
        (self.tail + KEY_BUFFER_SIZE - self.head) % KEY_BUFFER_SIZE
    }

    /// Preserve the source Available_Buffer_Room implementation exactly.
    /// Its return value is not the conventional free-slot count.
    pub fn available_buffer_room(&self) -> i32 {
        if self.head == self.tail {
            KEY_BUFFER_SIZE as i32
        } else if self.head < self.tail {
            (self.tail - self.head) as i32
        } else {
            (self.tail + KEY_BUFFER_SIZE - self.head) as i32
        }
    }

    pub fn put(&mut self, key: u16) -> bool {
        if self.is_buffer_full() { return false; }
        self.put_element(key)
    }

    pub fn put_element(&mut self, value: u16) -> bool {
        if self.is_buffer_full() { return false; }
        self.buffer[self.tail] = value;
        self.tail = (self.tail + 1) % KEY_BUFFER_SIZE;
        true
    }

    pub fn fetch_element(&mut self) -> u16 {
        if self.is_buffer_empty() { return 0; }
        let value = self.buffer[self.head];
        self.head = (self.head + 1) % KEY_BUFFER_SIZE;
        value
    }

    pub fn peek_element(&self) -> u16 {
        if self.is_buffer_empty() { 0 } else { self.buffer[self.head] }
    }

    pub fn check(&mut self, host: &mut impl KeyboardHost) -> u16 {
        self.fill_buffer_from_system(host);
        if self.is_buffer_empty() { 0 } else { self.peek_element() }
    }

    pub fn fill_buffer_from_system(&mut self, host: &mut impl KeyboardHost) {
        if !self.is_buffer_full() {
            host.dispatch_pending_messages(self);
        }
    }

    pub fn buff_get(&mut self, host: &mut impl KeyboardHost) -> u16 {
        while self.check(host) == 0 {}
        let key = self.fetch_element();
        if Self::is_mouse_key(key) {
            self.mouse_qx = self.fetch_element() as i16 as i32;
            self.mouse_qy = self.fetch_element() as i16 as i32;
        }
        key
    }

    pub fn get(&mut self, host: &mut impl KeyboardHost) -> u16 {
        while self.check(host) == 0 {}
        self.buff_get(host)
    }

    /// Snapshot Win32 modifier/toggle state and pack it into the key word.
    pub fn put_key_message(
        &mut self,
        host: &mut impl KeyboardHost,
        mut virtual_key: u16,
        release: bool,
    ) -> bool {
        if !Self::is_mouse_key(virtual_key) {
            let shift = host.key_state(VK_SHIFT) & 0x8000 != 0;
            let caps = host.key_state(VK_CAPITAL) & 0x0008 != 0;
            let num = host.key_state(VK_NUMLOCK) & 0x0008 != 0;
            if shift || caps || num { virtual_key |= WWKEY_SHIFT_BIT; }
            if host.key_state(VK_CONTROL) & 0x8000 != 0 { virtual_key |= WWKEY_CTRL_BIT; }
            if host.key_state(VK_MENU) & 0x8000 != 0 { virtual_key |= WWKEY_ALT_BIT; }
        }
        if release { virtual_key |= WWKEY_RLS_BIT; }
        self.put(virtual_key)
    }

    /// Mouse events are inserted as three consecutive 16-bit elements. As in
    /// the source, the event is accepted only when Available_Buffer_Room >= 3.
    pub fn put_mouse_message(
        &mut self,
        host: &mut impl KeyboardHost,
        virtual_key: u16,
        x: i32,
        y: i32,
        release: bool,
    ) -> bool {
        if self.available_buffer_room() >= 3 && Self::is_mouse_key(virtual_key) {
            let _ = self.put_key_message(host, virtual_key, release);
            let _ = self.put(x as u16);
            let _ = self.put(y as u16);
            true
        } else {
            false
        }
    }

    pub fn to_ascii(&mut self, host: &mut impl KeyboardHost, key: u16) -> u8 {
        if key & WWKEY_RLS_BIT != 0 { return 0; }
        if key & WWKEY_SHIFT_BIT != 0 { self.key_state[VK_SHIFT as usize] = 0x80; }
        if key & WWKEY_CTRL_BIT != 0 { self.key_state[VK_CONTROL as usize] = 0x80; }
        if key & WWKEY_ALT_BIT != 0 { self.key_state[VK_MENU as usize] = 0x80; }

        let virtual_key = key as u8;
        let scan = host.map_virtual_key(virtual_key);
        let result = host.to_ascii(virtual_key, scan, &self.key_state).unwrap_or(0);

        if key & WWKEY_SHIFT_BIT != 0 { self.key_state[VK_SHIFT as usize] = 0; }
        if key & WWKEY_CTRL_BIT != 0 { self.key_state[VK_CONTROL as usize] = 0; }
        if key & WWKEY_ALT_BIT != 0 { self.key_state[VK_MENU as usize] = 0; }
        result
    }

    pub fn down(&mut self, host: &mut impl KeyboardHost, key: u16) -> bool {
        host.async_key_state(key as u8) != 0
    }

    pub fn clear(&mut self, host: &mut impl KeyboardHost) {
        self.fill_buffer_from_system(host);
        self.head = self.tail;
        self.fill_buffer_from_system(host);
        self.head = self.tail;
    }

    /// The active KEY.CPP implementation has its entire switch body under
    /// #if (0), and therefore always returns false.
    pub fn message_handler(&mut self, _window: usize, _message: u32, _wparam: u32, _lparam: i32) -> bool {
        false
    }

    pub fn key_state_snapshot(&self) -> &[u8; 256] { &self.key_state }

    /// Useful for deterministic tests and hosts that already normalized
    /// modifiers into the legacy key bit-field.
    pub fn push_key(&mut self, key: u16) -> bool { self.put(key) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeHost {
        keys: [i16; 256],
        async_keys: [i16; 256],
        pending: Vec<u16>,
    }

    impl KeyboardHost for FakeHost {
        fn key_state(&mut self, key: u8) -> i16 { self.keys[key as usize] }
        fn async_key_state(&mut self, key: u8) -> i16 { self.async_keys[key as usize] }
        fn map_virtual_key(&mut self, key: u8) -> u32 { key as u32 }
        fn to_ascii(&mut self, key: u8, _scan: u32, state: &[u8; 256]) -> Option<u8> {
            if key == b'A' && state[VK_SHIFT as usize] != 0 { Some(b'A') }
            else if key == b'A' { Some(b'a') }
            else { Some(key) }
        }
        fn dispatch_pending_messages(&mut self, keyboard: &mut WWKeyboard) {
            for key in self.pending.drain(..) { let _ = keyboard.put(key); }
        }
    }

    #[test]
    fn ring_buffer_reserves_one_slot_and_wraps() {
        let mut keyboard = WWKeyboard::new();
        for i in 0..255 { assert!(keyboard.put(i + 1)); }
        assert!(keyboard.is_buffer_full());
        assert!(!keyboard.put(999));
        assert_eq!(keyboard.fetch_element(), 1);
        assert!(keyboard.put(999));
        assert_eq!(keyboard.fetch_element(), 2);
    }

    #[test]
    fn mouse_events_pack_key_and_signed_16_bit_coordinates() {
        let mut keyboard = WWKeyboard::new();
        let mut host = FakeHost::default();
        assert!(keyboard.put_mouse_message(&mut host, VK_LBUTTON as u16, 320, -20, true));
        assert_eq!(keyboard.get(&mut host), VK_LBUTTON as u16 | WWKEY_RLS_BIT);
        assert_eq!((keyboard.mouse_qx, keyboard.mouse_qy), (320, -20));
    }

    #[test]
    fn modifiers_are_captured_at_enqueue_time_and_ascii_state_is_restored() {
        let mut keyboard = WWKeyboard::new();
        let mut host = FakeHost::default();
        host.keys[VK_SHIFT as usize] = 0x8000;
        assert!(keyboard.put_key_message(&mut host, b'A' as u16, false));
        let key = keyboard.fetch_element();
        assert_eq!(key & WWKEY_SHIFT_BIT, WWKEY_SHIFT_BIT);
        assert_eq!(keyboard.to_ascii(&mut host, key), b'A');
        assert_eq!(keyboard.key_state_snapshot()[VK_SHIFT as usize], 0);
        assert_eq!(keyboard.to_ascii(&mut host, key | WWKEY_RLS_BIT), 0);
    }

    #[test]
    fn system_pump_is_injected_and_message_handler_is_disabled() {
        let mut keyboard = WWKeyboard::new();
        let mut host = FakeHost::default();
        host.pending.push(0x41);
        assert_eq!(keyboard.check(&mut host), 0x41);
        assert!(!keyboard.message_handler(0, 0x0100, 0x41, 0));
        keyboard.clear(&mut host);
        assert!(keyboard.is_buffer_empty());
    }

    #[test]
    fn async_key_state_and_available_room_follow_legacy_calls() {
        let mut keyboard = WWKeyboard::new();
        let mut host = FakeHost::default();
        host.async_keys[b'X' as usize] = 1;
        assert!(keyboard.down(&mut host, b'X' as u16));
        assert_eq!(keyboard.available_buffer_room(), 256);
        assert!(WWKeyboard::is_mouse_key(0x100 | VK_RBUTTON as u16));
    }
}
