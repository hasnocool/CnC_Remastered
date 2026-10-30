//! REDALERT/UDPADDR.CPP currently exposes only a disabled broadcast-address
//! discovery entry point. The dialog and address enumeration are entirely
//! inside #if (0), so the active function always returns false.

pub fn get_broadcast_addresses() -> bool { false }

#[cfg(test)]
mod tests {
    #[test]
    fn disabled_broadcast_discovery_reports_failure() {
        assert!(!super::get_broadcast_addresses());
    }
}
