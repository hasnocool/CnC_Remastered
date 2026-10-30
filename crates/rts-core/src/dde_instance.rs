//! Win32 DDEML instance wrapper from REDALERT/DDE.CPP and
//! TIBERIANDAWN/DDE.CPP. The pinned implementations intentionally disable
//! initialization/registration and callback dispatch, but keep the raw
//! DdeUninitialize, callback toggle, connect/disconnect and client POKE calls.

pub const DDE_POKE_TIMEOUT_MS: u32 = 60 * 1000;
pub const DDE_POKE_TOPIC: &str = "POKE TOPIC";
pub const DDE_POKE_ITEM: &str = "POKE ITEM";
pub const DDE_SYSTEM_TOPIC: &str = "System";

pub const DDE_ADVISE_CONNECT: i32 = -1;
pub const DDE_ADVISE_DISCONNECT: i32 = -2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DdeCallbackEvent {
    Register,
    Unregister,
    AdviseData,
    TransactionComplete,
    Disconnect,
    Connect,
    Poke,
    Other(u32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DdeCallbackReply {
    Null,
    Ack,
    NotProcessed,
    Connected,
}

pub trait DdeInstanceHost {
    fn dde_uninitialize(&mut self, instance_id: u32) -> bool;
    fn dde_connect(&mut self, instance_id: u32, service: &str, topic: &str) -> Option<u64>;
    fn dde_disconnect(&mut self, conversation: u64) -> bool;
    fn dde_client_poke(
        &mut self,
        conversation: u64,
        topic: &str,
        format_is_cf_text: bool,
        payload: &[u8],
        timeout_ms: u32,
    ) -> bool;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DdeInstance {
    instance_id: u32,
    conversation: Option<u64>,
    process_pokes: bool,
    ascii_name: String,
    local_initialization_attempted: bool,
    server_registration_enabled: bool,
}

impl DdeInstance {
    /// Instance_Class(name1, name2) returns immediately in both pinned files.
    /// Arguments are discarded and the Win32 DDEML ID remains static zero.
    pub fn new(name1: &str, name2: &str) -> Self {
        let _ = (name1, name2);
        Self {
            instance_id: 0,
            conversation: None,
            process_pokes: false,
            ascii_name: String::new(),
            local_initialization_attempted: false,
            server_registration_enabled: false,
        }
    }

    pub const fn instance_id(&self) -> u32 { self.instance_id }
    pub const fn process_pokes(&self) -> bool { self.process_pokes }
    pub const fn conversation(&self) -> Option<u64> { self.conversation }
    pub const fn local_initialization_attempted(&self) -> bool {
        self.local_initialization_attempted
    }
    pub const fn server_registration_enabled(&self) -> bool {
        self.server_registration_enabled
    }
    pub fn ascii_name(&self) -> &str { &self.ascii_name }

    /// Enable_Callback simply stores and returns the requested flag.
    pub fn enable_callback(&mut self, enabled: bool) -> bool {
        self.process_pokes = enabled;
        self.process_pokes
    }

    /// Register_Server is wholly inside #if (0) and therefore has no active
    /// implementation in the pinned Win32 builds.
    pub const fn register_server_is_active() -> bool { false }

    /// Test_Server_Running is just Open_Poke_Connection followed by Close.
    pub fn test_server_running(
        &mut self,
        host: &mut impl DdeInstanceHost,
        service: &str,
    ) -> bool {
        if self.open_poke_connection(host, service) {
            self.close_poke_connection(host);
            true
        } else {
            false
        }
    }

    pub fn open_poke_connection(
        &mut self,
        host: &mut impl DdeInstanceHost,
        service: &str,
    ) -> bool {
        self.conversation = host.dde_connect(self.instance_id, service, DDE_POKE_TOPIC);
        self.conversation.is_some()
    }

    /// Close_Poke_Connection returns TRUE when already disconnected. When a
    /// conversation exists it clears the handle before making DdeDisconnect.
    pub fn close_poke_connection(&mut self, host: &mut impl DdeInstanceHost) -> bool {
        let Some(conversation) = self.conversation.take() else { return true };
        host.dde_disconnect(conversation)
    }

    /// Poke_Server uses CF_TEXT / XTYP_POKE and a fixed 60-second timeout.
    pub fn poke_server(
        &self,
        host: &mut impl DdeInstanceHost,
        payload: &[u8],
        format_is_cf_text: bool,
    ) -> bool {
        let Some(conversation) = self.conversation else { return false };
        host.dde_client_poke(
            conversation,
            DDE_POKE_TOPIC,
            format_is_cf_text,
            payload,
            DDE_POKE_TIMEOUT_MS,
        )
    }

    /// dde_callback returns NULL immediately in the pinned source. Its
    /// XTYP_CONNECT, XTYP_DISCONNECT and XTYP_POKE dispatcher is under #if (0).
    pub const fn callback(
        &self,
        _event: DdeCallbackEvent,
        _format_is_cf_text: bool,
        _topic: Option<&str>,
        _service: Option<&str>,
        _payload: Option<&[u8]>,
    ) -> DdeCallbackReply {
        DdeCallbackReply::Null
    }

    /// Destructor invokes DdeUninitialize(id_inst), even though construction
    /// returned before the DdeInitialize call in the current sources.
    pub fn shutdown(&mut self, host: &mut impl DdeInstanceHost) -> bool {
        self.conversation = None;
        host.dde_uninitialize(self.instance_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeHost {
        uninitialized: Vec<u32>,
        connects: Vec<(u32, String, String)>,
        disconnects: Vec<u64>,
        pokes: Vec<(u64, String, bool, Vec<u8>, u32)>,
        connect_result: Option<u64>,
        disconnect_result: bool,
        poke_result: bool,
    }

    impl DdeInstanceHost for FakeHost {
        fn dde_uninitialize(&mut self, id: u32) -> bool {
            self.uninitialized.push(id);
            true
        }
        fn dde_connect(&mut self, id: u32, service: &str, topic: &str) -> Option<u64> {
            self.connects.push((id, service.to_string(), topic.to_string()));
            self.connect_result
        }
        fn dde_disconnect(&mut self, conversation: u64) -> bool {
            self.disconnects.push(conversation);
            self.disconnect_result
        }
        fn dde_client_poke(
            &mut self,
            conversation: u64,
            topic: &str,
            format_is_cf_text: bool,
            payload: &[u8],
            timeout_ms: u32,
        ) -> bool {
            self.pokes.push((
                conversation, topic.to_string(), format_is_cf_text, payload.to_vec(), timeout_ms,
            ));
            self.poke_result
        }
    }

    #[test]
    fn constructor_preserves_disabled_ddeml_initialization_and_callback_state() {
        let instance = DdeInstance::new("LOCAL", "REMOTE");
        assert_eq!(instance.instance_id(), 0);
        assert_eq!(instance.ascii_name(), "");
        assert!(!instance.local_initialization_attempted());
        assert!(!instance.server_registration_enabled());
        assert!(!instance.process_pokes());
        assert!(!DdeInstance::register_server_is_active());
    }

    #[test]
    fn callback_toggle_echoes_requested_value() {
        let mut instance = DdeInstance::new("LOCAL", "REMOTE");
        assert!(instance.enable_callback(true));
        assert!(instance.process_pokes());
        assert!(!instance.enable_callback(false));
    }

    #[test]
    fn open_and_close_use_instance_service_and_poke_topic() {
        let mut host = FakeHost { connect_result: Some(77), disconnect_result: true, ..FakeHost::default() };
        let mut instance = DdeInstance::new("LOCAL", "REMOTE");
        assert!(instance.open_poke_connection(&mut host, "WCHAT"));
        assert_eq!(instance.conversation(), Some(77));
        assert_eq!(host.connects, vec![(0, "WCHAT".into(), DDE_POKE_TOPIC.into())]);
        assert!(instance.close_poke_connection(&mut host));
        assert_eq!(instance.conversation(), None);
        assert_eq!(host.disconnects, vec![77]);
        assert!(instance.close_poke_connection(&mut host)); // null conversation succeeds
    }

    #[test]
    fn test_server_running_opens_then_closes_only_on_success() {
        let mut host = FakeHost { connect_result: Some(9), disconnect_result: true, ..FakeHost::default() };
        let mut instance = DdeInstance::new("LOCAL", "REMOTE");
        assert!(instance.test_server_running(&mut host, "WCHAT"));
        assert_eq!(host.disconnects, vec![9]);
        host.connect_result = None;
        assert!(!instance.test_server_running(&mut host, "MISSING"));
        assert_eq!(host.disconnects, vec![9]);
    }

    #[test]
    fn poke_uses_cf_text_topic_and_fixed_timeout() {
        let mut host = FakeHost { connect_result: Some(12), poke_result: true, ..FakeHost::default() };
        let mut instance = DdeInstance::new("LOCAL", "REMOTE");
        assert!(instance.open_poke_connection(&mut host, "WCHAT"));
        assert!(instance.poke_server(&mut host, b"packet", true));
        assert_eq!(
            host.pokes,
            vec![(12, DDE_POKE_TOPIC.into(), true, b"packet".to_vec(), 60_000)]
        );
    }

    #[test]
    fn active_callback_is_always_null_even_when_pokes_enabled() {
        let mut instance = DdeInstance::new("LOCAL", "REMOTE");
        instance.enable_callback(true);
        assert_eq!(
            instance.callback(DdeCallbackEvent::Poke, true, Some(DDE_POKE_TOPIC), None, Some(b"payload")),
            DdeCallbackReply::Null
        );
        assert_eq!(
            instance.callback(DdeCallbackEvent::Connect, true, Some(DDE_POKE_TOPIC), Some("LOCAL"), None),
            DdeCallbackReply::Null
        );
    }

    #[test]
    fn destructor_uninitializes_static_zero_instance_id() {
        let mut host = FakeHost::default();
        let mut instance = DdeInstance::new("LOCAL", "REMOTE");
        assert!(instance.shutdown(&mut host));
        assert_eq!(host.uninitialized, vec![0]);
    }
}
