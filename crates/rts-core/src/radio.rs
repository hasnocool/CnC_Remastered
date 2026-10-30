//! Tiberian Dawn RadioClass from TIBERIANDAWN/RADIO.CPP.
//!
//! Peer-to-peer delivery is delegated through RadioNetwork so the game
//! runtime can route the remote RadioClass instance. Base ObjectClass
//! receive/limbo behavior is explicit in RadioMissionHost.

pub const RADIO_COUNT: usize = 35;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RadioMessageType {
    Static = 0,
    Roger = 1,
    Hello = 2,
    OverOut = 3,
    PickUp = 4,
    Attach = 5,
    Delivery = 6,
    HoldStill = 7,
    Unloaded = 8,
    Unload = 9,
    Negative = 10,
    Building = 11,
    Complete = 12,
    Redraw = 13,
    Docking = 14,
    CanLoad = 15,
    AreRefinery = 16,
    TryingToLoad = 17,
    MoveHere = 18,
    NeedToMove = 19,
    YeaNowWhat = 20,
    ImIn = 21,
    BackupNow = 22,
    RunAway = 23,
    Tether = 24,
    Untether = 25,
    Repair = 26,
    Prepared = 27,
    AttackThis = 28,
    Reload = 29,
    Kick = 30,
    Punch = 31,
    PrepareToBox = 32,
    NeedRepair = 33,
    OnDepot = 34,
}

impl RadioMessageType {
    pub const fn from_id(id: u8) -> Option<Self> {
        Some(match id {
            0 => Self::Static, 1 => Self::Roger, 2 => Self::Hello, 3 => Self::OverOut,
            4 => Self::PickUp, 5 => Self::Attach, 6 => Self::Delivery, 7 => Self::HoldStill,
            8 => Self::Unloaded, 9 => Self::Unload, 10 => Self::Negative, 11 => Self::Building,
            12 => Self::Complete, 13 => Self::Redraw, 14 => Self::Docking, 15 => Self::CanLoad,
            16 => Self::AreRefinery, 17 => Self::TryingToLoad, 18 => Self::MoveHere,
            19 => Self::NeedToMove, 20 => Self::YeaNowWhat, 21 => Self::ImIn,
            22 => Self::BackupNow, 23 => Self::RunAway, 24 => Self::Tether,
            25 => Self::Untether, 26 => Self::Repair, 27 => Self::Prepared,
            28 => Self::AttackThis, 29 => Self::Reload, 30 => Self::Kick,
            31 => Self::Punch, 32 => Self::PrepareToBox, 33 => Self::NeedRepair,
            34 => Self::OnDepot, _ => return None,
        })
    }

    pub const fn id(self) -> u8 { self as u8 }
}

pub const RADIO_MESSAGES: [Option<&str>; RADIO_COUNT] = [
    Some("hisssss"),
    Some("Roger."),
    Some("Come in."),
    Some("Over and out."),
    Some("Requesting transport."),
    Some("Attach to transport."),
    Some("I've got a delivery for you."),
    Some("I'm performing load/unload maneuver. Be careful."),
    Some("I'm clear."),
    Some("You are clear to unload. Driving away now."),
    Some("Am unable to comply."),
    Some("I'm starting construction now... act busy."),
    Some("I've finished construction. You are free."),
    Some("We bumped, redraw yourself please."),
    Some("I'm trying to load up now."),
    Some("May I become a passenger?"),
    Some("Are you ready to receive shipment?"),
    Some("Are you trying to become a passenger?"),
    Some("Move to location X."),
    Some("Do you need to move?"),
    Some("All right already. Now what?"),
    Some("I'm a passenger now."),
    Some("Backup into refinery now."),
    Some("Run away!"),
    Some("Tether established."),
    Some("Tether broken."),
    Some("Repair one step."),
    Some("Are you prepared to fight?"),
    Some("Attack this target please."),
    Some("Reload one step."),
    Some("Take this kick! You... You..."),
    Some("Take this punch! You... You..."),
    Some("Fancy a little fisticuffs, eh?"),
    None,
    None,
];

pub trait RadioMissionHost {
    fn receive_base_message(
        &mut self,
        receiver: RadioObjectId,
        from: RadioObjectId,
        message: RadioMessageType,
        parameter: &mut i32,
    ) -> RadioMessageType;
    fn is_in_limbo(&self, object: RadioObjectId) -> bool;
    fn limbo_base(&mut self, object: RadioObjectId) -> bool;
}

pub trait RadioNetwork {
    fn receive_message(
        &mut self,
        receiver: RadioObjectId,
        from: RadioObjectId,
        message: RadioMessageType,
        parameter: &mut i32,
    ) -> RadioMessageType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RadioObjectId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioClass {
    pub object_id: RadioObjectId,
    pub last_message: RadioMessageType,
    pub radio: Option<RadioObjectId>,
    pub strength: u32,
    pub last_parameter: i32,
}

impl RadioClass {
    pub const fn new(object_id: RadioObjectId, strength: u32) -> Self {
        Self {
            object_id,
            last_message: RadioMessageType::Static,
            radio: None,
            strength,
            last_parameter: 0,
        }
    }

    pub const fn in_radio_contact(&self) -> bool { self.radio.is_some() }
    pub const fn contact_with_whom(&self) -> Option<RadioObjectId> { self.radio }
    pub fn radio_off(&mut self) { self.radio = None; }

    /// ObjectClass::Receive_Message behavior inherited by RadioClass.
    pub fn receive_base_message(
        receiver: RadioObjectId,
        from: RadioObjectId,
        message: RadioMessageType,
        parameter: &mut i32,
        host: &mut impl RadioMissionHost,
    ) -> RadioMessageType {
        let _ = parameter;
        match message {
            RadioMessageType::Redraw => {
                host.receive_base_message(receiver, from, RadioMessageType::Redraw, parameter)
            }
            _ => RadioMessageType::Static,
        }
    }

    /// Port of RadioClass::Receive_Message.
    pub fn receive_message(
        &mut self,
        from: RadioObjectId,
        message: RadioMessageType,
        parameter: &mut i32,
        host: &mut impl RadioMissionHost,
    ) -> RadioMessageType {
        self.last_message = message;

        if self.radio == Some(from) && message == RadioMessageType::OverOut {
            let _base_response = host.receive_base_message(self.object_id, from, message, parameter);
            self.radio_off();
            return RadioMessageType::Roger;
        }

        if message == RadioMessageType::Hello && self.strength != 0 {
            if self.radio == Some(from) || self.radio.is_none() {
                self.radio = Some(from);
                return RadioMessageType::Roger;
            }
            return RadioMessageType::Negative;
        }

        host.receive_base_message(self.object_id, from, message, parameter)
    }

    fn transmit_to(
        &mut self,
        message: RadioMessageType,
        parameter: &mut i32,
        target: RadioObjectId,
        network: &mut impl RadioNetwork,
    ) -> RadioMessageType {
        if self.radio == Some(target) && message == RadioMessageType::OverOut {
            self.radio = None;
        }
        network.receive_message(target, self.object_id, message, parameter)
    }

    /// Port of RadioClass::Transmit_Message(message, param, to).
    pub fn transmit_message(
        &mut self,
        message: RadioMessageType,
        parameter: &mut i32,
        target: Option<RadioObjectId>,
        network: &mut impl RadioNetwork,
    ) -> RadioMessageType {
        let Some(target) = target.or(self.contact_with_whom()) else {
            return RadioMessageType::Static;
        };

        if message == RadioMessageType::Hello {
            // The source first closes any previous conversation by sending
            // OVER_OUT to Contact_With_Whom(), then attempts the new handshake.
            let _ = self.transmit_message(
                RadioMessageType::OverOut,
                parameter,
                None,
                network,
            );
            if self.transmit_to(message, parameter, target, network) == RadioMessageType::Roger {
                self.radio = Some(target);
                return RadioMessageType::Roger;
            }
            return RadioMessageType::Negative;
        }

        self.transmit_to(message, parameter, target, network)
    }

    /// The overload without an explicit parameter uses the C++ global LParam.
    pub fn transmit_message_default(
        &mut self,
        message: RadioMessageType,
        target: Option<RadioObjectId>,
        network: &mut impl RadioNetwork,
    ) -> RadioMessageType {
        self.transmit_message(message, &mut self.last_parameter, target, network)
    }

    /// Port of RadioClass::Limbo; contact is closed before base limbo.
    pub fn limbo(
        &mut self,
        host: &mut impl RadioMissionHost,
        network: &mut impl RadioNetwork,
    ) -> bool {
        if !host.is_in_limbo(self.object_id) {
            let _ = self.transmit_message_default(RadioMessageType::OverOut, None, network);
        }
        host.limbo_base(self.object_id)
    }

    pub fn debug_message_text(&self) -> Option<&'static str> {
        RADIO_MESSAGES[self.last_message.id() as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct MockBase {
        limbo: BTreeMap<RadioObjectId, bool>,
        receive_calls: Vec<(RadioObjectId, RadioObjectId, RadioMessageType)>,
        redraw_marks: Vec<RadioObjectId>,
        limbo_calls: Vec<RadioObjectId>,
        base_response: RadioMessageType,
    }

    impl RadioMissionHost for MockBase {
        fn receive_base_message(
            &mut self,
            receiver: RadioObjectId,
            from: RadioObjectId,
            message: RadioMessageType,
            _parameter: &mut i32,
        ) -> RadioMessageType {
            self.receive_calls.push((receiver, from, message));
            if message == RadioMessageType::Redraw {
                self.redraw_marks.push(receiver);
                RadioMessageType::Roger
            } else {
                self.base_response
            }
        }
        fn is_in_limbo(&self, object: RadioObjectId) -> bool {
            self.limbo.get(&object).copied().unwrap_or(false)
        }
        fn limbo_base(&mut self, object: RadioObjectId) -> bool {
            self.limbo_calls.push(object);
            self.limbo.insert(object, true);
            true
        }
    }

    #[derive(Default)]
    struct MockNetwork {
        peers: BTreeMap<RadioObjectId, RadioClass>,
        deliveries: Vec<(RadioObjectId, RadioObjectId, RadioMessageType)>,
        base: MockBase,
    }

    impl RadioNetwork for MockNetwork {
        fn receive_message(
            &mut self,
            receiver: RadioObjectId,
            from: RadioObjectId,
            message: RadioMessageType,
            parameter: &mut i32,
        ) -> RadioMessageType {
            self.deliveries.push((receiver, from, message));
            let mut peer = self.peers.remove(&receiver).expect("peer exists");
            let response = peer.receive_message(from, message, parameter, &mut self.base);
            self.peers.insert(receiver, peer);
            response
        }
    }

    fn id(value: u32) -> RadioObjectId { RadioObjectId(value) }

    #[test]
    fn message_ids_and_debug_strings_preserve_source_order() {
        assert_eq!(RADIO_COUNT, 35);
        assert_eq!(RadioMessageType::OnDepot.id(), 34);
        assert_eq!(RadioMessageType::from_id(33), Some(RadioMessageType::NeedRepair));
        assert_eq!(RadioMessageType::from_id(35), None);
        assert_eq!(RADIO_MESSAGES[0], Some("hisssss"));
        assert_eq!(RADIO_MESSAGES[32], Some("Fancy a little fisticuffs, eh?"));
        assert_eq!(RADIO_MESSAGES[33], None);
        assert_eq!(RADIO_MESSAGES[34], None);
    }

    #[test]
    fn receive_hello_accepts_when_free_and_rejects_competing_contact() {
        let mut host = MockBase::default();
        let mut receiver = RadioClass::new(id(2), 100);
        let mut parameter = 0;
        assert_eq!(receiver.receive_message(id(1), RadioMessageType::Hello, &mut parameter, &mut host), RadioMessageType::Roger);
        assert_eq!(receiver.radio, Some(id(1)));
        assert_eq!(receiver.receive_message(id(3), RadioMessageType::Hello, &mut parameter, &mut host), RadioMessageType::Negative);
        assert_eq!(receiver.last_message, RadioMessageType::Hello);
    }

    #[test]
    fn receive_hello_is_rejected_when_strength_is_zero() {
        let mut host = MockBase::default();
        let mut receiver = RadioClass::new(id(2), 0);
        let mut parameter = 0;
        assert_eq!(receiver.receive_message(id(1), RadioMessageType::Hello, &mut parameter, &mut host), RadioMessageType::Static);
        assert_eq!(receiver.radio, None);
    }

    #[test]
    fn over_out_only_breaks_the_matching_established_contact() {
        let mut host = MockBase::default();
        let mut receiver = RadioClass::new(id(2), 100);
        receiver.radio = Some(id(1));
        let mut parameter = 0;
        assert_eq!(receiver.receive_message(id(3), RadioMessageType::OverOut, &mut parameter, &mut host), RadioMessageType::Static);
        assert_eq!(receiver.radio, Some(id(1)));
        assert_eq!(receiver.receive_message(id(1), RadioMessageType::OverOut, &mut parameter, &mut host), RadioMessageType::Roger);
        assert_eq!(receiver.radio, None);
    }

    #[test]
    fn base_receive_handles_redraw_and_default_messages() {
        let mut host = MockBase::default();
        let mut receiver = RadioClass::new(id(2), 100);
        let mut parameter = 0;
        assert_eq!(receiver.receive_message(id(1), RadioMessageType::Redraw, &mut parameter, &mut host), RadioMessageType::Roger);
        assert_eq!(host.redraw_marks, vec![id(2)]);
        assert_eq!(receiver.receive_message(id(1), RadioMessageType::Attach, &mut parameter, &mut host), RadioMessageType::Static);
    }

    #[test]
    fn transmit_without_explicit_target_uses_current_contact() {
        let mut host = MockBase::default();
        let mut sender = RadioClass::new(id(1), 100);
        assert_eq!(sender.transmit_message_default(RadioMessageType::Roger, None, &mut MockNetwork::default()), RadioMessageType::Static);
        let mut network = MockNetwork::default();
        network.peers.insert(id(2), RadioClass::new(id(2), 100));
        sender.radio = Some(id(2));
        assert_eq!(sender.transmit_message_default(RadioMessageType::Roger, None, &mut network), RadioMessageType::Static);
        assert_eq!(network.deliveries, vec![(id(2), id(1), RadioMessageType::Roger)]);
        let _ = &mut host;
    }

    #[test]
    fn hello_handshake_sends_over_out_to_old_peer_then_negotiates_new_peer() {
        let mut sender = RadioClass::new(id(1), 100);
        sender.radio = Some(id(2));
        let mut network = MockNetwork::default();
        network.peers.insert(id(2), RadioClass::new(id(2), 100));
        network.peers.insert(id(3), RadioClass::new(id(3), 100));
        assert_eq!(sender.transmit_message_default(RadioMessageType::Hello, Some(id(3)), &mut network), RadioMessageType::Roger);
        assert_eq!(sender.radio, Some(id(3)));
        assert_eq!(network.deliveries, vec![
            (id(2), id(1), RadioMessageType::OverOut),
            (id(3), id(1), RadioMessageType::Hello),
        ]);
        assert_eq!(network.peers.get(&id(2)).unwrap().radio, None);
        assert_eq!(network.peers.get(&id(3)).unwrap().radio, Some(id(1)));
    }

    #[test]
    fn hello_handshake_returns_negative_when_remote_rejects_contact() {
        let mut sender = RadioClass::new(id(1), 100);
        let mut peer = RadioClass::new(id(3), 100);
        peer.radio = Some(id(4));
        let mut network = MockNetwork::default();
        network.peers.insert(id(3), peer);
        assert_eq!(sender.transmit_message_default(RadioMessageType::Hello, Some(id(3)), &mut network), RadioMessageType::Negative);
        assert_eq!(sender.radio, None);
    }

    #[test]
    fn sending_over_out_clears_sender_contact_before_remote_delivery() {
        let mut sender = RadioClass::new(id(1), 100);
        sender.radio = Some(id(2));
        let mut peer = RadioClass::new(id(2), 100);
        peer.radio = Some(id(1));
        let mut network = MockNetwork::default();
        network.peers.insert(id(2), peer);
        assert_eq!(sender.transmit_message_default(RadioMessageType::OverOut, None, &mut network), RadioMessageType::Roger);
        assert_eq!(sender.radio, None);
        assert_eq!(network.peers.get(&id(2)).unwrap().radio, None);
    }

    #[test]
    fn limbo_closes_radio_contact_before_delegating_to_base() {
        let mut radio = RadioClass::new(id(1), 100);
        radio.radio = Some(id(2));
        let mut host = MockBase::default();
        let mut network = MockNetwork::default();
        let mut peer = RadioClass::new(id(2), 100);
        peer.radio = Some(id(1));
        network.peers.insert(id(2), peer);
        assert!(radio.limbo(&mut host, &mut network));
        assert_eq!(radio.radio, None);
        assert_eq!(host.limbo_calls, vec![id(1)]);
        assert_eq!(network.peers.get(&id(2)).unwrap().radio, None);
    }

    #[test]
    fn already_limbo_object_does_not_send_over_out_again() {
        let mut radio = RadioClass::new(id(1), 100);
        radio.radio = Some(id(2));
        let mut host = MockBase::default();
        host.limbo.insert(id(1), true);
        assert!(radio.limbo(&mut host, &mut MockNetwork::default()));
        assert_eq!(radio.radio, Some(id(2)));
    }
}
