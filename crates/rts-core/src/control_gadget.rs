//! Tiberian Dawn ControlClass from TIBERIANDAWN/CONTROL.CPP.
//!
//! This class adapts GadgetClass input and drawing to an ID-bearing control.
//! The base GadgetClass and peer gadget are invoked through ControlGadgetHost,
//! preserving source call order without raw cross-object pointers.

pub const GADGET_LEFT_PRESS: u32 = 0x0001;
pub const GADGET_LEFT_HELD: u32 = 0x0002;
pub const GADGET_LEFT_RELEASE: u32 = 0x0004;
pub const GADGET_LEFT_UP: u32 = 0x0008;
pub const GADGET_RIGHT_PRESS: u32 = 0x0010;
pub const GADGET_RIGHT_HELD: u32 = 0x0020;
pub const GADGET_RIGHT_RELEASE: u32 = 0x0040;
pub const GADGET_RIGHT_UP: u32 = 0x0080;
pub const GADGET_KEYBOARD: u32 = 0x0100;
pub const CONTROL_DEFAULT_FLAGS: u32 = GADGET_LEFT_PRESS | GADGET_RIGHT_PRESS;

/// The source uses KN_BUTTON to distinguish a gadget ID from an ordinary key.
pub const KEY_BUTTON_FLAG: u16 = 0x8000;
pub const KEY_NONE: u16 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GadgetId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlPeerContext {
    pub id: u32,
    pub rect: ControlRect,
    pub flags: u32,
    pub sticky: bool,
}

pub trait ControlGadgetHost {
    /// Equivalent to Peer->Peer_To_Peer(flags, key, *this).
    fn peer_to_peer(
        &mut self,
        peer: GadgetId,
        flags: u32,
        key: &mut u16,
        source: ControlPeerContext,
    );

    /// Delegates to GadgetClass::Action after ControlClass adjusts the key.
    fn gadget_action(&mut self, flags: u32, key: &mut u16) -> i32;

    /// Equivalent to Peer->Draw_Me() with its default forced=false argument.
    fn draw_peer(&mut self, peer: GadgetId);

    /// Delegates to GadgetClass::Draw_Me(forced).
    fn gadget_draw_me(&mut self, forced: bool) -> i32;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlClass {
    pub id: u32,
    pub rect: ControlRect,
    pub flags: u32,
    pub sticky: bool,
    pub peer: Option<GadgetId>,
}

impl ControlClass {
    /// The C++ constructor defaults to LEFTPRESS|RIGHTPRESS and sticky=false.
    pub const fn new(id: u32, x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::with_options(
            id, x, y, width, height, CONTROL_DEFAULT_FLAGS, false,
        )
    }

    pub const fn with_options(
        id: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        flags: u32,
        sticky: bool,
    ) -> Self {
        Self {
            id,
            rect: ControlRect { x, y, width, height },
            flags,
            sticky,
            peer: None,
        }
    }

    /// Port of ControlClass::Make_Peer.
    pub fn make_peer(&mut self, peer: GadgetId) {
        self.peer = Some(peer);
    }

    pub const fn get_id(&self) -> u32 { self.id }

    pub const fn peer_context(&self) -> ControlPeerContext {
        ControlPeerContext { id: self.id, rect: self.rect, flags: self.flags, sticky: self.sticky }
    }

    /// Port of ControlClass::Action. Peer notification occurs before the
    /// control rewrites the key, then GadgetClass::Action receives the result.
    pub fn action(
        &self,
        flags: u32,
        key: &mut u16,
        host: &mut impl ControlGadgetHost,
    ) -> i32 {
        if let Some(peer) = self.peer {
            host.peer_to_peer(peer, flags, key, self.peer_context());
        }

        if flags != 0 {
            if self.id != 0 {
                *key = (self.id as u16) | KEY_BUTTON_FLAG;
            } else {
                *key = KEY_NONE;
            }
        }

        host.gadget_action(flags, key)
    }

    /// Port of ControlClass::Draw_Me. The peer is drawn first; its return value
    /// is ignored. The result from GadgetClass::Draw_Me is returned unchanged.
    pub fn draw_me(
        &self,
        forced: bool,
        host: &mut impl ControlGadgetHost,
    ) -> i32 {
        if let Some(peer) = self.peer {
            host.draw_peer(peer);
        }
        host.gadget_draw_me(forced)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockHost {
        calls: Vec<String>,
        peer_key_before_action: Vec<u16>,
        peer_contexts: Vec<ControlPeerContext>,
        base_action_result: i32,
        base_draw_result: i32,
    }

    impl ControlGadgetHost for MockHost {
        fn peer_to_peer(
            &mut self,
            peer: GadgetId,
            flags: u32,
            key: &mut u16,
            source: ControlPeerContext,
        ) {
            self.calls.push(format!("peer:{}:{flags}", peer.0));
            self.peer_key_before_action.push(*key);
            self.peer_contexts.push(source);
        }

        fn gadget_action(&mut self, flags: u32, key: &mut u16) -> i32 {
            self.calls.push(format!("action:{flags}:{key}"));
            self.base_action_result
        }

        fn draw_peer(&mut self, peer: GadgetId) {
            self.calls.push(format!("draw-peer:{}", peer.0));
        }

        fn gadget_draw_me(&mut self, forced: bool) -> i32 {
            self.calls.push(format!("draw-base:{forced}"));
            self.base_draw_result
        }
    }

    #[test]
    fn constructor_stores_id_geometry_and_default_input_flags() {
        let control = ControlClass::new(12, 10, 20, 30, 40);
        assert_eq!(control.id, 12);
        assert_eq!(control.rect, ControlRect { x: 10, y: 20, width: 30, height: 40 });
        assert_eq!(control.flags, GADGET_LEFT_PRESS | GADGET_RIGHT_PRESS);
        assert!(!control.sticky);
        assert_eq!(control.peer, None);
    }

    #[test]
    fn explicit_constructor_preserves_flags_and_sticky_setting() {
        let control = ControlClass::with_options(4, 1, 2, 3, 4, GADGET_KEYBOARD, true);
        assert_eq!(control.flags, GADGET_KEYBOARD);
        assert!(control.sticky);
    }

    #[test]
    fn make_peer_and_get_id_match_public_control_api() {
        let mut control = ControlClass::new(17, 0, 0, 1, 1);
        control.make_peer(GadgetId(99));
        assert_eq!(control.get_id(), 17);
        assert_eq!(control.peer, Some(GadgetId(99)));
    }

    #[test]
    fn action_notifies_peer_before_replacing_key_with_button_code() {
        let mut control = ControlClass::new(0x123, 1, 2, 3, 4);
        control.make_peer(GadgetId(5));
        let mut host = MockHost { base_action_result: 7, ..MockHost::default() };
        let mut key = 0x0041;
        let result = control.action(GADGET_LEFT_PRESS, &mut key, &mut host);
        assert_eq!(host.peer_key_before_action, vec![0x0041]);
        assert_eq!(key, 0x8123);
        assert_eq!(result, 7);
        assert_eq!(host.calls, vec!["peer:5:1", "action:1:33059"]);
    }

    #[test]
    fn zero_id_converts_recognized_action_to_no_key() {
        let control = ControlClass::new(0, 0, 0, 10, 10);
        let mut host = MockHost::default();
        let mut key = 0x1234;
        control.action(GADGET_RIGHT_PRESS, &mut key, &mut host);
        assert_eq!(key, KEY_NONE);
    }

    #[test]
    fn zero_flags_leave_key_untouched_but_still_delegate_to_base() {
        let control = ControlClass::new(50, 0, 0, 10, 10);
        let mut host = MockHost::default();
        let mut key = 0x1234;
        control.action(0, &mut key, &mut host);
        assert_eq!(key, 0x1234);
        assert_eq!(host.calls, vec!["action:0:4660"]);
    }

    #[test]
    fn peer_notification_receives_the_source_control_context() {
        let mut control = ControlClass::with_options(7, 8, 9, 10, 11, 0x31, true);
        control.make_peer(GadgetId(22));
        let mut host = MockHost::default();
        let mut key = 3;
        control.action(GADGET_LEFT_RELEASE, &mut key, &mut host);
        assert_eq!(host.peer_contexts, vec![ControlPeerContext {
            id: 7,
            rect: ControlRect { x: 8, y: 9, width: 10, height: 11 },
            flags: 0x31,
            sticky: true,
        }]);
    }

    #[test]
    fn draw_peer_precedes_base_draw_and_base_result_is_returned() {
        let mut control = ControlClass::new(1, 0, 0, 10, 10);
        control.make_peer(GadgetId(8));
        let mut host = MockHost { base_draw_result: 42, ..MockHost::default() };
        assert_eq!(control.draw_me(true, &mut host), 42);
        assert_eq!(host.calls, vec!["draw-peer:8", "draw-base:true"]);
    }

    #[test]
    fn draw_without_peer_delegates_directly_to_base() {
        let control = ControlClass::new(1, 0, 0, 10, 10);
        let mut host = MockHost { base_draw_result: 9, ..MockHost::default() };
        assert_eq!(control.draw_me(false, &mut host), 9);
        assert_eq!(host.calls, vec!["draw-base:false"]);
    }
}
