//! Red Alert RadioClass from REDALERT/RADIO.CPP.
//!
//! Red Alert stores the three most recent distinct received messages for its
//! cheat-key debugger. Peer delivery and MissionClass behavior are supplied
//! by the host; contact negotiation and state transitions live here.

pub const RADIO_COUNT: usize = 34;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RedAlertRadioMessage {
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
    Cannot = 30,
    AllDone = 31,
    NeedRepair = 32,
    OnDepot = 33,
}

impl Default for RedAlertRadioMessage {
    fn default() -> Self { Self::Static }
}

impl RedAlertRadioMessage {
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
            28 => Self::AttackThis, 29 => Self::Reload, 30 => Self::Cannot,
            31 => Self::AllDone, 32 => Self::NeedRepair, 33 => Self::OnDepot,
            _ => return None,
        })
    }
    pub const fn id(self) -> u8 { self as u8 }
}

pub const RED_ALERT_RADIO_MESSAGES: [&str; RADIO_COUNT] = [
    "static (no message)",
    "Roger.",
    "Come in.",
    "Over and out.",
    "Requesting transport.",
    "Attach to transport.",
    "I've got a delivery for you.",
    "I'm performing load/unload maneuver. Be careful.",
    "I'm clear.",
    "You are clear to unload. Driving away now.",
    "Am unable to comply.",
    "I'm starting construction now... act busy.",
    "I've finished construction. You are free.",
    "We bumped, redraw yourself please.",
    "I'm trying to load up now.",
    "May I become a passenger?",
    "Are you ready to receive shipment?",
    "Are you trying to become a passenger?",
    "Move to location X.",
    "Do you need to move?",
    "All right already. Now what?",
    "I'm a passenger now.",
    "Backup into refinery now.",
    "Run away!",
    "Tether established.",
    "Tether broken.",
    "Repair one step.",
    "Are you prepared to fight?",
    "Attack this target please.",
    "Reload one step.",
    "Circumstances prevent success.",
    "All done with the request.",
    "Do you need service depot work?",
    "Are you sitting on service depot?",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RedAlertRadioId(pub u32);

pub trait RedAlertMissionHost {
    fn is_active(&self, object: RedAlertRadioId) -> bool;
    fn is_in_limbo(&self, object: RedAlertRadioId) -> bool;
    fn receive_mission_message(
        &mut self,
        receiver: RedAlertRadioId,
        from: RedAlertRadioId,
        message: RedAlertRadioMessage,
        parameter: &mut i32,
    ) -> RedAlertRadioMessage;
    fn mission_limbo(&mut self, object: RedAlertRadioId) -> bool;
}

pub trait RedAlertRadioNetwork {
    fn receive_message(
        &mut self,
        receiver: RedAlertRadioId,
        from: RedAlertRadioId,
        message: RedAlertRadioMessage,
        parameter: &mut i32,
    ) -> RedAlertRadioMessage;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedAlertRadioDebugSnapshot {
    pub history: [Option<RedAlertRadioMessage>; 3],
    pub contact: Option<RedAlertRadioId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertRadioClass {
    pub object_id: RedAlertRadioId,
    pub strength: u32,
    pub radio: Option<RedAlertRadioId>,
    /// Old[0] is most recent, Old[2] is the oldest retained distinct message.
    pub old: [Option<RedAlertRadioMessage>; 3],
    pub last_parameter: i32,
}

impl RedAlertRadioClass {
    pub const fn new(object_id: RedAlertRadioId, strength: u32) -> Self {
        Self {
            object_id, strength, radio: None,
            old: [None, None, None],
            last_parameter: 0,
        }
    }

    pub const fn in_radio_contact(&self) -> bool { self.radio.is_some() }
    pub const fn contact_with_whom(&self) -> Option<RedAlertRadioId> { self.radio }
    pub fn radio_off(&mut self) { self.radio = None; }

    fn record_message(&mut self, message: RedAlertRadioMessage) {
        if self.old[0] != Some(message) {
            self.old[2] = self.old[1];
            self.old[1] = self.old[0];
            self.old[0] = Some(message);
        }
    }

    /// Port of RadioClass::Receive_Message.
    pub fn receive_message(
        &mut self,
        from: RedAlertRadioId,
        message: RedAlertRadioMessage,
        parameter: &mut i32,
        host: &mut impl RedAlertMissionHost,
    ) -> RedAlertRadioMessage {
        debug_assert!(host.is_active(self.object_id));
        self.record_message(message);

        if self.radio == Some(from) && message == RedAlertRadioMessage::OverOut {
            let _ = host.receive_mission_message(self.object_id, from, message, parameter);
            self.radio_off();
            return RedAlertRadioMessage::Roger;
        }

        if message == RedAlertRadioMessage::Hello && self.strength != 0 {
            if self.radio == Some(from) || self.radio.is_none() {
                self.radio = Some(from);
                return RedAlertRadioMessage::Roger;
            }
            return RedAlertRadioMessage::Negative;
        }

        host.receive_mission_message(self.object_id, from, message, parameter)
    }

    fn transmit_to(
        &mut self,
        message: RedAlertRadioMessage,
        parameter: &mut i32,
        target: RedAlertRadioId,
        network: &mut impl RedAlertRadioNetwork,
    ) -> RedAlertRadioMessage {
        if self.radio == Some(target) && message == RedAlertRadioMessage::OverOut {
            self.radio = None;
        }
        network.receive_message(target, self.object_id, message, parameter)
    }

    /// Explicit-parameter Transmit_Message overload.
    pub fn transmit_message(
        &mut self,
        message: RedAlertRadioMessage,
        parameter: &mut i32,
        target: Option<RedAlertRadioId>,
        network: &mut impl RedAlertRadioNetwork,
    ) -> RedAlertRadioMessage {
        let Some(target) = target.or(self.contact_with_whom()) else {
            return RedAlertRadioMessage::Static;
        };

        if message == RedAlertRadioMessage::Hello {
            let _ = self.transmit_message(RedAlertRadioMessage::OverOut, parameter, None, network);
            if self.transmit_to(message, parameter, target, network) == RedAlertRadioMessage::Roger {
                self.radio = Some(target);
                return RedAlertRadioMessage::Roger;
            }
            return RedAlertRadioMessage::Negative;
        }
        self.transmit_to(message, parameter, target, network)
    }

    /// Overload that uses the game-global LParam supplied by the caller.
    pub fn transmit_message_with_lparam(
        &mut self,
        message: RedAlertRadioMessage,
        global_lparam: &mut i32,
        target: Option<RedAlertRadioId>,
        network: &mut impl RedAlertRadioNetwork,
    ) -> RedAlertRadioMessage {
        self.transmit_message(message, global_lparam, target, network)
    }

    pub fn limbo(
        &mut self,
        host: &mut impl RedAlertMissionHost,
        network: &mut impl RedAlertRadioNetwork,
    ) -> bool {
        debug_assert!(host.is_active(self.object_id));
        if !host.is_in_limbo(self.object_id) {
            let _ = self.transmit_message_with_lparam(
                RedAlertRadioMessage::OverOut,
                &mut self.last_parameter,
                None,
                network,
            );
        }
        host.mission_limbo(self.object_id)
    }

    pub fn debug_snapshot(&self) -> RedAlertRadioDebugSnapshot {
        RedAlertRadioDebugSnapshot { history: self.old, contact: self.radio }
    }

    pub fn debug_message_text(&self, age: usize) -> Option<&'static str> {
        self.old.get(age)
            .and_then(|message| message.map(|value| RED_ALERT_RADIO_MESSAGES[value.id() as usize]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct MissionHost {
        active: BTreeMap<RedAlertRadioId, bool>,
        limbo: BTreeMap<RedAlertRadioId, bool>,
        receive_calls: Vec<(RedAlertRadioId, RedAlertRadioId, RedAlertRadioMessage)>,
        limbo_calls: Vec<RedAlertRadioId>,
        response: RedAlertRadioMessage,
    }

    impl RedAlertMissionHost for MissionHost {
        fn is_active(&self, object: RedAlertRadioId) -> bool {
            self.active.get(&object).copied().unwrap_or(true)
        }
        fn is_in_limbo(&self, object: RedAlertRadioId) -> bool {
            self.limbo.get(&object).copied().unwrap_or(false)
        }
        fn receive_mission_message(
            &mut self, receiver: RedAlertRadioId, from: RedAlertRadioId,
            message: RedAlertRadioMessage, _parameter: &mut i32,
        ) -> RedAlertRadioMessage {
            self.receive_calls.push((receiver, from, message));
            if message == RedAlertRadioMessage::Redraw {
                RedAlertRadioMessage::Roger
            } else {
                self.response
            }
        }
        fn mission_limbo(&mut self, object: RedAlertRadioId) -> bool {
            self.limbo_calls.push(object);
            self.limbo.insert(object, true);
            true
        }
    }

    #[derive(Default)]
    struct Network {
        peers: BTreeMap<RedAlertRadioId, RedAlertRadioClass>,
        deliveries: Vec<(RedAlertRadioId, RedAlertRadioId, RedAlertRadioMessage)>,
        mission: MissionHost,
    }

    impl RedAlertRadioNetwork for Network {
        fn receive_message(
            &mut self, receiver: RedAlertRadioId, from: RedAlertRadioId,
            message: RedAlertRadioMessage, parameter: &mut i32,
        ) -> RedAlertRadioMessage {
            self.deliveries.push((receiver, from, message));
            let mut peer = self.peers.remove(&receiver).expect("radio peer exists");
            let result = peer.receive_message(from, message, parameter, &mut self.mission);
            self.peers.insert(receiver, peer);
            result
        }
    }

    fn id(value: u32) -> RedAlertRadioId { RedAlertRadioId(value) }

    #[test]
    fn red_alert_message_ids_and_texts_have_34_entries() {
        assert_eq!(RADIO_COUNT, 34);
        assert_eq!(RedAlertRadioMessage::Cannot.id(), 30);
        assert_eq!(RedAlertRadioMessage::AllDone.id(), 31);
        assert_eq!(RedAlertRadioMessage::NeedRepair.id(), 32);
        assert_eq!(RedAlertRadioMessage::OnDepot.id(), 33);
        assert_eq!(RedAlertRadioMessage::from_id(34), None);
        assert_eq!(RED_ALERT_RADIO_MESSAGES[0], "static (no message)");
        assert_eq!(RED_ALERT_RADIO_MESSAGES[30], "Circumstances prevent success.");
        assert_eq!(RED_ALERT_RADIO_MESSAGES[33], "Are you sitting on service depot?");
    }

    #[test]
    fn receive_history_shifts_only_when_message_differs_from_most_recent() {
        let mut radio = RedAlertRadioClass::new(id(1), 100);
        let mut mission = MissionHost::default();
        let mut parameter = 0;
        for message in [
            RedAlertRadioMessage::Hello,
            RedAlertRadioMessage::Hello,
            RedAlertRadioMessage::Roger,
            RedAlertRadioMessage::OverOut,
        ] {
            let _ = radio.receive_message(id(2), message, &mut parameter, &mut mission);
        }
        assert_eq!(radio.old, [
            Some(RedAlertRadioMessage::OverOut),
            Some(RedAlertRadioMessage::Roger),
            Some(RedAlertRadioMessage::Hello),
        ]);
        assert_eq!(radio.debug_message_text(1), Some("Roger."));
    }

    #[test]
    fn receive_hello_accepts_available_radio_and_rejects_competing_peer() {
        let mut radio = RedAlertRadioClass::new(id(1), 100);
        let mut host = MissionHost::default();
        let mut parameter = 0;
        assert_eq!(radio.receive_message(id(2), RedAlertRadioMessage::Hello, &mut parameter, &mut host), RedAlertRadioMessage::Roger);
        assert_eq!(radio.radio, Some(id(2)));
        assert_eq!(radio.receive_message(id(3), RedAlertRadioMessage::Hello, &mut parameter, &mut host), RedAlertRadioMessage::Negative);
    }

    #[test]
    fn receive_hello_needs_nonzero_strength() {
        let mut radio = RedAlertRadioClass::new(id(1), 0);
        let mut host = MissionHost::default();
        let mut parameter = 0;
        assert_eq!(radio.receive_message(id(2), RedAlertRadioMessage::Hello, &mut parameter, &mut host), RedAlertRadioMessage::Static);
        assert_eq!(radio.radio, None);
    }

    #[test]
    fn over_out_only_closes_matching_peer_and_delegates_to_mission() {
        let mut radio = RedAlertRadioClass::new(id(1), 100);
        radio.radio = Some(id(2));
        let mut host = MissionHost::default();
        let mut parameter = 0;
        assert_eq!(radio.receive_message(id(3), RedAlertRadioMessage::OverOut, &mut parameter, &mut host), RedAlertRadioMessage::Static);
        assert_eq!(radio.radio, Some(id(2)));
        assert_eq!(radio.receive_message(id(2), RedAlertRadioMessage::OverOut, &mut parameter, &mut host), RedAlertRadioMessage::Roger);
        assert_eq!(radio.radio, None);
    }

    #[test]
    fn transmit_without_contact_returns_static() {
        let mut radio = RedAlertRadioClass::new(id(1), 100);
        let mut lparam = 0;
        let result = radio.transmit_message_with_lparam(
            RedAlertRadioMessage::Roger, &mut lparam, None, &mut Network::default(),
        );
        assert_eq!(result, RedAlertRadioMessage::Static);
    }

    #[test]
    fn hello_handshake_releases_old_peer_then_establishes_new_peer() {
        let mut sender = RedAlertRadioClass::new(id(1), 100);
        sender.radio = Some(id(2));
        let mut network = Network::default();
        let mut old_peer = RedAlertRadioClass::new(id(2), 100);
        old_peer.radio = Some(id(1));
        network.peers.insert(id(2), old_peer);
        network.peers.insert(id(3), RedAlertRadioClass::new(id(3), 100));
        let mut parameter = 0;
        assert_eq!(sender.transmit_message(
            RedAlertRadioMessage::Hello, &mut parameter, Some(id(3)), &mut network,
        ), RedAlertRadioMessage::Roger);
        assert_eq!(sender.radio, Some(id(3)));
        assert_eq!(network.deliveries, vec![
            (id(2), id(1), RedAlertRadioMessage::OverOut),
            (id(3), id(1), RedAlertRadioMessage::Hello),
        ]);
        assert_eq!(network.peers.get(&id(2)).unwrap().radio, None);
        assert_eq!(network.peers.get(&id(3)).unwrap().radio, Some(id(1)));
    }

    #[test]
    fn failed_hello_returns_negative_without_setting_contact() {
        let mut sender = RedAlertRadioClass::new(id(1), 100);
        let mut peer = RedAlertRadioClass::new(id(2), 100);
        peer.radio = Some(id(3));
        let mut network = Network::default();
        network.peers.insert(id(2), peer);
        let mut parameter = 0;
        assert_eq!(sender.transmit_message(
            RedAlertRadioMessage::Hello, &mut parameter, Some(id(2)), &mut network,
        ), RedAlertRadioMessage::Negative);
        assert_eq!(sender.radio, None);
    }

    #[test]
    fn over_out_clears_sender_contact_before_remote_delivery() {
        let mut sender = RedAlertRadioClass::new(id(1), 100);
        sender.radio = Some(id(2));
        let mut peer = RedAlertRadioClass::new(id(2), 100);
        peer.radio = Some(id(1));
        let mut network = Network::default();
        network.peers.insert(id(2), peer);
        let mut parameter = 0;
        assert_eq!(sender.transmit_message(
            RedAlertRadioMessage::OverOut, &mut parameter, None, &mut network,
        ), RedAlertRadioMessage::Roger);
        assert_eq!(sender.radio, None);
        assert_eq!(network.peers.get(&id(2)).unwrap().radio, None);
    }

    #[test]
    fn limbo_sends_over_out_before_mission_limbo() {
        let mut sender = RedAlertRadioClass::new(id(1), 100);
        sender.radio = Some(id(2));
        let mut network = Network::default();
        let mut peer = RedAlertRadioClass::new(id(2), 100);
        peer.radio = Some(id(1));
        network.peers.insert(id(2), peer);
        let mut host = MissionHost::default();
        assert!(sender.limbo(&mut host, &mut network));
        assert_eq!(sender.radio, None);
        assert_eq!(host.limbo_calls, vec![id(1)]);
        assert_eq!(network.peers.get(&id(2)).unwrap().radio, None);
    }

    #[test]
    fn already_limbo_radio_skips_over_out() {
        let mut sender = RedAlertRadioClass::new(id(1), 100);
        sender.radio = Some(id(2));
        let mut host = MissionHost::default();
        host.limbo.insert(id(1), true);
        let mut network = Network::default();
        assert!(sender.limbo(&mut host, &mut network));
        assert_eq!(sender.radio, Some(id(2)));
        assert!(network.deliveries.is_empty());
    }
}
