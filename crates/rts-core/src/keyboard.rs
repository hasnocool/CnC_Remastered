//! Westwood WWKeyboardClass port from REDALERT/KEYBOARD.CPP.

pub const WWKEY_SHIFT_BIT: u16 = 0x0100;
pub const WWKEY_CTRL_BIT: u16 = 0x0200;
pub const WWKEY_ALT_BIT: u16 = 0x0400;
pub const WWKEY_RLS_BIT: u16 = 0x0800;
pub const WWKEY_VK_BIT: u16 = 0x1000;
pub const WWKEY_DBL_BIT: u16 = 0x2000;
pub const WWKEY_BTN_BIT: u16 = 0x8000;

pub const VK_LBUTTON: u16 = 0x01;
pub const VK_RBUTTON: u16 = 0x02;
pub const VK_MBUTTON: u16 = 0x04;
pub const VK_SHIFT: u16 = 0x10;
pub const VK_CONTROL: u16 = 0x11;
pub const VK_MENU: u16 = 0x12;
pub const VK_CAPITAL: u16 = 0x14;
pub const VK_NUMLOCK: u16 = 0x90;
pub const VK_SCROLL: u16 = 0x91;

pub const KEY_BUFFER_SIZE: usize = 256;
pub const KEY_BUFFER_CAPACITY: usize = KEY_BUFFER_SIZE - 1;

pub trait KeyboardPlatform {
    fn key_state(&self, virtual_key: u16) -> u16;
    fn map_virtual_key(&self, virtual_key: u16) -> u32;
    fn to_ascii(&self, virtual_key: u16, scan_code: u32, key_state: &[u8; 256]) -> Option<u8>;
    fn async_key_state(&self, virtual_key: u16) -> i16;
    fn stop_execution(&mut self) {}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MouseButton { Left, Middle, Right }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyboardMessage {
    KeyDown { virtual_key: u16 },
    KeyUp { virtual_key: u16 },
    MouseDown { button: MouseButton, x: u16, y: u16 },
    MouseUp { button: MouseButton, x: u16, y: u16 },
    MouseDoubleClick { button: MouseButton, x: u16, y: u16 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WWKeyboard {
    pub mouse_qx: i32,
    pub mouse_qy: i32,
    buffer: [u16; KEY_BUFFER_SIZE],
    key_state: [u8; 256],
    head: usize,
    tail: usize,
}

impl Default for WWKeyboard {
    fn default() -> Self { Self::new() }
}

impl WWKeyboard {
    pub fn new() -> Self {
        Self {
            mouse_qx: 0, mouse_qy: 0, buffer: [0; KEY_BUFFER_SIZE],
            key_state: [0; 256], head: 0, tail: 0,
        }
    }

    pub fn is_mouse_key(key: u16) -> bool {
        matches!(key & 0xff, VK_LBUTTON | VK_MBUTTON | VK_RBUTTON)
    }

    pub fn is_buffer_empty(&self) -> bool { self.head == self.tail }
    pub fn is_buffer_full(&self) -> bool { (self.tail + 1) % KEY_BUFFER_SIZE == self.head }
    pub fn len(&self) -> usize {
        if self.tail >= self.head { self.tail - self.head }
        else { KEY_BUFFER_SIZE - self.head + self.tail }
    }
    pub fn is_empty(&self) -> bool { self.is_buffer_empty() }

    pub fn put(&mut self, key: u16) -> bool {
        if self.is_buffer_full() { return false; }
        self.put_element(key)
    }

    fn put_element(&mut self, value: u16) -> bool {
        if self.is_buffer_full() { return false; }
        let next = (self.tail + 1) % KEY_BUFFER_SIZE;
        self.buffer[self.tail] = value;
        self.tail = next;
        true
    }

    fn fetch_element(&mut self) -> u16 {
        if self.head == self.tail { return 0; }
        let value = self.buffer[self.head];
        self.head = (self.head + 1) % KEY_BUFFER_SIZE;
        value
    }

    fn peek_element(&self) -> Option<u16> {
        if self.is_buffer_empty() { None } else { Some(self.buffer[self.head]) }
    }

    /// Check is nonblocking; call fill_buffer_from_system first to pump host
    /// messages, matching the legacy Check -> Fill_Buffer_From_System flow.
    pub fn check(&self) -> Option<u16> { self.peek_element() }

    pub fn get(&mut self) -> Option<u16> { self.buff_get() }

    /// Nonblocking form of Buff_Get. Mouse events consume the following two
    /// ring entries as X/Y and update the public mouse coordinate fields.
    pub fn buff_get(&mut self) -> Option<u16> {
        let key = self.fetch_element_option()?;
        if Self::is_mouse_key(key) {
            self.mouse_qx = self.fetch_element() as i32;
            self.mouse_qy = self.fetch_element() as i32;
        }
        Some(key)
    }

    fn fetch_element_option(&mut self) -> Option<u16> {
        if self.is_buffer_empty() { None } else { Some(self.fetch_element()) }
    }

    pub fn clear(&mut self) { self.head = self.tail; }

    pub fn put_key_message(&mut self, mut virtual_key: u16, release: bool, platform: &impl KeyboardPlatform) -> bool {
        if !Self::is_mouse_key(virtual_key) {
            if (platform.key_state(VK_SHIFT) & 0x8000) != 0
                || (platform.key_state(VK_CAPITAL) & 0x0008) != 0
                || (platform.key_state(VK_NUMLOCK) & 0x0008) != 0
            {
                virtual_key |= WWKEY_SHIFT_BIT;
            }
            if (platform.key_state(VK_CONTROL) & 0x8000) != 0 {
                virtual_key |= WWKEY_CTRL_BIT;
            }
            if (platform.key_state(VK_MENU) & 0x8000) != 0 {
                virtual_key |= WWKEY_ALT_BIT;
            }
        }
        if release { virtual_key |= WWKEY_RLS_BIT; }
        self.put(virtual_key)
    }

    pub fn to_ascii(&mut self, key: u16, platform: &impl KeyboardPlatform) -> u8 {
        if key & WWKEY_RLS_BIT != 0 { return 0; }
        if key & WWKEY_SHIFT_BIT != 0 { self.key_state[VK_SHIFT as usize] = 0x80; }
        if key & WWKEY_CTRL_BIT != 0 { self.key_state[VK_CONTROL as usize] = 0x80; }
        if key & WWKEY_ALT_BIT != 0 { self.key_state[VK_MENU as usize] = 0x80; }

        let virtual_key = key & 0xff;
        let scan_code = platform.map_virtual_key(virtual_key);
        let translated = platform.to_ascii(virtual_key, scan_code, &self.key_state);

        if key & WWKEY_SHIFT_BIT != 0 { self.key_state[VK_SHIFT as usize] = 0; }
        if key & WWKEY_CTRL_BIT != 0 { self.key_state[VK_CONTROL as usize] = 0; }
        if key & WWKEY_ALT_BIT != 0 { self.key_state[VK_MENU as usize] = 0; }
        translated.unwrap_or(0)
    }

    pub fn down(&self, key: u16, platform: &impl KeyboardPlatform) -> bool {
        platform.async_key_state(key & 0xff) != 0
    }

    pub fn fill_buffer_from_system<I>(&mut self, messages: I, platform: &mut impl KeyboardPlatform)
    where I: IntoIterator<Item = KeyboardMessage> {
        if self.is_buffer_full() { return; }
        for message in messages {
            if self.is_buffer_full() { break; }
            self.message_handler(message, platform);
        }
    }

    pub fn message_handler(&mut self, message: KeyboardMessage, platform: &mut impl KeyboardPlatform) {
        match message {
            KeyboardMessage::KeyDown { virtual_key } => {
                if virtual_key == VK_SCROLL { platform.stop_execution(); }
                else { self.put_key_message(virtual_key, false, platform); }
            }
            KeyboardMessage::KeyUp { virtual_key } => {
                self.put_key_message(virtual_key, true, platform);
            }
            KeyboardMessage::MouseDown { button, x, y } => {
                self.put_key_message(mouse_vk(button), false, platform);
                self.put(x);
                self.put(y);
            }
            KeyboardMessage::MouseUp { button, x, y } => {
                self.put_key_message(mouse_vk(button), true, platform);
                self.put(x);
                self.put(y);
            }
            KeyboardMessage::MouseDoubleClick { button, x, y } => {
                self.put_key_message(mouse_vk(button), true, platform);
                self.put(x);
                self.put(y);
                self.put_key_message(mouse_vk(button), true, platform);
                self.put(x);
                self.put(y);
            }
        }
    }

    pub fn mouse_x(&self) -> i32 { self.mouse_qx }
    pub fn mouse_y(&self) -> i32 { self.mouse_qy }
}

fn mouse_vk(button: MouseButton) -> u16 {
    match button { MouseButton::Left => VK_LBUTTON, MouseButton::Middle => VK_MBUTTON, MouseButton::Right => VK_RBUTTON }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakePlatform {
        shift: bool, ctrl: bool, alt: bool, caps: bool, num: bool,
        async_down: bool, stopped: bool,
    }
    impl KeyboardPlatform for FakePlatform {
        fn key_state(&self, key: u16) -> u16 {
            match key {
                VK_SHIFT if self.shift => 0x8000,
                VK_CONTROL if self.ctrl => 0x8000,
                VK_MENU if self.alt => 0x8000,
                VK_CAPITAL if self.caps => 0x0008,
                VK_NUMLOCK if self.num => 0x0008,
                _ => 0,
            }
        }
        fn map_virtual_key(&self, key: u16) -> u32 { key as u32 + 1 }
        fn to_ascii(&self, key: u16, scan: u32, state: &[u8; 256]) -> Option<u8> {
            assert_eq!(scan, key as u32 + 1);
            if state[VK_SHIFT as usize] != 0 { Some((key as u8).to_ascii_uppercase()) }
            else { Some(key as u8) }
        }
        fn async_key_state(&self, _key: u16) -> i16 { if self.async_down { -1 } else { 0 } }
        fn stop_execution(&mut self) { self.stopped = true; }
    }

    #[test]
    fn ring_buffer_has_255_usable_slots_and_clear_discards_entries() {
        let mut keyboard = WWKeyboard::new();
        for i in 0..KEY_BUFFER_CAPACITY { assert!(keyboard.put(i as u16)); }
        assert!(keyboard.is_buffer_full());
        assert!(!keyboard.put(999));
        assert_eq!(keyboard.len(), KEY_BUFFER_CAPACITY);
        keyboard.clear();
        assert!(keyboard.is_empty());
        assert_eq!(keyboard.check(), None);
    }

    #[test]
    fn mouse_events_consume_coordinates_after_the_virtual_key() {
        let mut keyboard = WWKeyboard::new();
        let mut platform = FakePlatform::default();
        keyboard.message_handler(KeyboardMessage::MouseDown { button: MouseButton::Left, x: 123, y: 45 }, &mut platform);
        assert_eq!(keyboard.check(), Some(VK_LBUTTON));
        assert_eq!(keyboard.get(), Some(VK_LBUTTON));
        assert_eq!((keyboard.mouse_x(), keyboard.mouse_y()), (123, 45));
        assert!(keyboard.is_empty());
    }

    #[test]
    fn double_click_enqueues_two_release_events_and_coordinate_pairs() {
        let mut keyboard = WWKeyboard::new();
        let mut platform = FakePlatform::default();
        keyboard.message_handler(KeyboardMessage::MouseDoubleClick { button: MouseButton::Right, x: 7, y: 8 }, &mut platform);
        assert_eq!(keyboard.len(), 6);
        assert_eq!(keyboard.get(), Some(VK_RBUTTON | WWKEY_RLS_BIT));
        assert_eq!((keyboard.mouse_x(), keyboard.mouse_y()), (7, 8));
        assert_eq!(keyboard.get(), Some(VK_RBUTTON | WWKEY_RLS_BIT));
        assert_eq!(keyboard.len(), 0);
    }

    #[test]
    fn key_messages_capture_modifiers_and_release_bit() {
        let mut keyboard = WWKeyboard::new();
        let mut platform = FakePlatform { shift: true, ctrl: true, alt: true, ..Default::default() };
        assert!(keyboard.put_key_message(0x41, false, &platform));
        assert_eq!(keyboard.get(), Some(0x41 | WWKEY_SHIFT_BIT | WWKEY_CTRL_BIT | WWKEY_ALT_BIT));
        assert!(keyboard.put_key_message(0x41, true, &platform));
        assert_eq!(keyboard.get(), Some(0x41 | WWKEY_SHIFT_BIT | WWKEY_CTRL_BIT | WWKEY_ALT_BIT | WWKEY_RLS_BIT));
    }

    #[test]
    fn ascii_translation_restores_temporary_key_state() {
        let mut keyboard = WWKeyboard::new();
        let platform = FakePlatform::default();
        assert_eq!(keyboard.to_ascii(0x41 | WWKEY_SHIFT_BIT, &platform), b'A');
        assert_eq!(keyboard.key_state[VK_SHIFT as usize], 0);
        assert_eq!(keyboard.to_ascii(0x41 | WWKEY_RLS_BIT, &platform), 0);
    }

    #[test]
    fn scroll_lock_stops_execution_and_down_checks_async_state() {
        let mut keyboard = WWKeyboard::new();
        let mut platform = FakePlatform::default();
        keyboard.message_handler(KeyboardMessage::KeyDown { virtual_key: VK_SCROLL }, &mut platform);
        assert!(platform.stopped);
        platform.async_down = true;
        assert!(keyboard.down(0x41, &platform));
    }

    #[test]
    fn fill_buffer_stops_pumping_when_full() {
        let mut keyboard = WWKeyboard::new();
        let mut platform = FakePlatform::default();
        keyboard.fill_buffer_from_system(
            (0..300).map(|i| KeyboardMessage::KeyDown { virtual_key: i as u16 }),
            &mut platform,
        );
        assert_eq!(keyboard.len(), KEY_BUFFER_CAPACITY);
    }
}
