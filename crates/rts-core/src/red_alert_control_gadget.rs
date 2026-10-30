//! Red Alert ControlClass from REDALERT/CONTROL.CPP.
//!
//! This variant is intentionally separate from Tiberian Dawn's ControlClass:
//! Red Alert provides a copy constructor and updates the pseudo-key before
//! notifying its peer. The GadgetClass base is represented by a copied value.

use crate::control_gadget::{
    ControlPeerContext, ControlRect, GadgetId, KEY_BUTTON_FLAG, KEY_NONE,
    CONTROL_DEFAULT_FLAGS,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertGadgetBase {
    pub rect: ControlRect,
    pub flags: u32,
    pub sticky: bool,
    pub redraw_pending: bool,
    pub enabled: bool,
    pub focus: bool,
}

impl RedAlertGadgetBase {
    pub const fn new(rect: ControlRect, flags: u32, sticky: bool) -> Self {
        Self { rect, flags, sticky, redraw_pending: false, enabled: true, focus: false }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertControlClass {
    pub base: RedAlertGadgetBase,
    pub id: u32,
    pub peer: Option<GadgetId>,
}

pub trait RedAlertControlHost {
    fn peer_to_peer(
        &mut self,
        peer: GadgetId,
        flags: u32,
        key: &mut u16,
        source: ControlPeerContext,
    );
    fn gadget_action(&mut self, base: &mut RedAlertGadgetBase, flags: u32, key: &mut u16) -> i32;
    fn draw_peer(&mut self, peer: GadgetId);
    fn gadget_draw_me(&mut self, base: &mut RedAlertGadgetBase, forced: bool) -> i32;
}

impl RedAlertControlClass {
    pub const fn new(id: u32, x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::with_options(id, x, y, width, height, CONTROL_DEFAULT_FLAGS, false)
    }

    pub const fn with_options(
        id: u32, x: i32, y: i32, width: i32, height: i32, flags: u32, sticky: bool,
    ) -> Self {
        Self {
            base: RedAlertGadgetBase::new(ControlRect { x, y, width, height }, flags, sticky),
            id,
            peer: None,
        }
    }

    /// Red Alert NoInit constructor: copy the base gadget state without
    /// initializing the subclass ID/peer. The caller must set those fields
    /// before using the returned control.
    pub fn from_base_uninitialized(base: RedAlertGadgetBase) -> Self {
        Self { base, id: 0, peer: None }
    }

    /// Red Alert ControlClass copy constructor duplicates the GadgetClass
    /// subobject, ID and peer reference.
    pub fn copy_construct(source: &Self) -> Self {
        Self { base: source.base.clone(), id: source.id, peer: source.peer }
    }

    pub fn make_peer(&mut self, peer: GadgetId) {
        self.peer = Some(peer);
    }

    pub const fn get_id(&self) -> u32 { self.id }

    pub const fn peer_context(&self) -> ControlPeerContext {
        ControlPeerContext {
            id: self.id,
            rect: self.base.rect,
            flags: self.base.flags,
            sticky: self.base.sticky,
        }
    }

    /// Red Alert Action: rewrite the button key before the peer callback.
    pub fn action(
        &mut self,
        flags: u32,
        key: &mut u16,
        host: &mut impl RedAlertControlHost,
    ) -> i32 {
        if flags != 0 {
            *key = if self.id != 0 {
                self.id as u16 | KEY_BUTTON_FLAG
            } else {
                KEY_NONE
            };
        }

        if let Some(peer) = self.peer {
            host.peer_to_peer(peer, flags, key, self.peer_context());
        }

        host.gadget_action(&mut self.base, flags, key)
    }

    /// Peer draw precedes the GadgetClass::Draw_Me base implementation.
    pub fn draw_me(
        &mut self,
        forced: bool,
        host: &mut impl RedAlertControlHost,
    ) -> i32 {
        if let Some(peer) = self.peer {
            host.draw_peer(peer);
        }
        host.gadget_draw_me(&mut self.base, forced)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockHost {
        calls: Vec<String>,
        keys_seen_by_peer: Vec<u16>,
        contexts: Vec<ControlPeerContext>,
        base_action_result: i32,
        base_draw_result: i32,
    }

    impl RedAlertControlHost for MockHost {
        fn peer_to_peer(&mut self, peer: GadgetId, flags: u32, key: &mut u16, source: ControlPeerContext) {
            self.calls.push(format!("peer:{}:{flags}", peer.0));
            self.keys_seen_by_peer.push(*key);
            self.contexts.push(source);
        }
        fn gadget_action(&mut self, base: &mut RedAlertGadgetBase, flags: u32, key: &mut u16) -> i32 {
            self.calls.push(format!("action:{flags}:{key}"));
            base.redraw_pending = true;
            self.base_action_result
        }
        fn draw_peer(&mut self, peer: GadgetId) {
            self.calls.push(format!("peer-draw:{}", peer.0));
        }
        fn gadget_draw_me(&mut self, base: &mut RedAlertGadgetBase, forced: bool) -> i32 {
            self.calls.push(format!("base-draw:{forced}"));
            base.redraw_pending = false;
            self.base_draw_result
        }
    }

    #[test]
    fn constructor_defaults_match_red_alert_control_gadget() {
        let control = RedAlertControlClass::new(7, 1, 2, 3, 4);
        assert_eq!(control.id, 7);
        assert_eq!(control.base.rect, ControlRect { x: 1, y: 2, width: 3, height: 4 });
        assert_eq!(control.base.flags, CONTROL_DEFAULT_FLAGS);
        assert!(!control.base.sticky);
        assert_eq!(control.peer, None);
    }

    #[test]
    fn copy_constructor_copies_base_id_and_peer() {
        let mut source = RedAlertControlClass::with_options(9, 10, 20, 30, 40, 0x31, true);
        source.make_peer(GadgetId(55));
        source.base.focus = true;
        source.base.redraw_pending = true;
        let copy = RedAlertControlClass::copy_construct(&source);
        assert_eq!(copy, source);
    }

    #[test]
    fn no_init_constructor_copies_only_base_state() {
        let base = RedAlertGadgetBase {
            rect: ControlRect { x: 4, y: 5, width: 6, height: 7 },
            flags: 0x20, sticky: true, redraw_pending: true, enabled: false, focus: true,
        };
        let control = RedAlertControlClass::from_base_uninitialized(base.clone());
        assert_eq!(control.base, base);
        assert_eq!(control.id, 0);
        assert_eq!(control.peer, None);
    }

    #[test]
    fn action_changes_key_before_notifying_peer() {
        let mut control = RedAlertControlClass::new(0x123, 1, 2, 3, 4);
        control.make_peer(GadgetId(8));
        let mut host = MockHost { base_action_result: 4, ..MockHost::default() };
        let mut key = 0x44;
        assert_eq!(control.action(1, &mut key, &mut host), 4);
        assert_eq!(key, 0x8123);
        assert_eq!(host.keys_seen_by_peer, vec![0x8123]);
        assert_eq!(host.calls, vec!["peer:8:1", "action:1:33059"]);
    }

    #[test]
    fn zero_id_maps_recognized_events_to_no_key_before_peer_callback() {
        let mut control = RedAlertControlClass::new(0, 0, 0, 1, 1);
        control.make_peer(GadgetId(2));
        let mut host = MockHost::default();
        let mut key = 0x99;
        control.action(1, &mut key, &mut host);
        assert_eq!(key, KEY_NONE);
        assert_eq!(host.keys_seen_by_peer, vec![KEY_NONE]);
    }

    #[test]
    fn zero_flags_leave_key_unchanged_and_still_notify_peer() {
        let mut control = RedAlertControlClass::new(22, 0, 0, 1, 1);
        control.make_peer(GadgetId(3));
        let mut host = MockHost::default();
        let mut key = 0x42;
        control.action(0, &mut key, &mut host);
        assert_eq!(key, 0x42);
        assert_eq!(host.keys_seen_by_peer, vec![0x42]);
    }

    #[test]
    fn peer_callback_receives_the_post_conversion_key_and_control_context() {
        let mut control = RedAlertControlClass::with_options(5, 6, 7, 8, 9, 0x33, true);
        control.make_peer(GadgetId(7));
        let mut host = MockHost::default();
        let mut key = 1;
        control.action(2, &mut key, &mut host);
        assert_eq!(host.keys_seen_by_peer, vec![0x8005]);
        assert_eq!(host.contexts, vec![ControlPeerContext {
            id: 5,
            rect: ControlRect { x: 6, y: 7, width: 8, height: 9 },
            flags: 0x33,
            sticky: true,
        }]);
    }

    #[test]
    fn draw_calls_peer_before_base_and_returns_base_result() {
        let mut control = RedAlertControlClass::new(1, 0, 0, 1, 1);
        control.make_peer(GadgetId(90));
        let mut host = MockHost { base_draw_result: 3, ..MockHost::default() };
        assert_eq!(control.draw_me(true, &mut host), 3);
        assert_eq!(host.calls, vec!["peer-draw:90", "base-draw:true"]);
    }
}
