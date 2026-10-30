//! Red Alert UnitTrackerClass from REDALERT/UTRACKER.CPP.
//!
//! The REDALERT translation unit defines htonl/ntohl as zero-return macros
//! whenever WIN32 is absent. The build policy is explicit here: Win32 uses
//! network byte order, while the DOS branch replaces every converted counter
//! with zero and cannot recover its original value on To_PC_Format.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertTrackerPlatform {
    Win32,
    Dos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedAlertTrackerError {
    NegativeUnitCount(i32),
    AllocationFailed,
    UnitIndexOutOfRange { index: i32, count: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedAlertUnitTracker {
    totals: Vec<i32>,
    unit_count: usize,
    in_network_format: bool,
    platform: RedAlertTrackerPlatform,
}

impl RedAlertUnitTracker {
    pub fn new(unit_count: i32, platform: RedAlertTrackerPlatform) -> Result<Self, RedAlertTrackerError> {
        let count = usize::try_from(unit_count)
            .map_err(|_| RedAlertTrackerError::NegativeUnitCount(unit_count))?;
        let mut totals = Vec::new();
        totals.try_reserve_exact(count).map_err(|_| RedAlertTrackerError::AllocationFailed)?;
        totals.resize(count, 0_i32);
        Ok(Self { totals, unit_count: count, in_network_format: false, platform })
    }

    pub const fn unit_count(&self) -> usize { self.unit_count }
    pub const fn in_network_format(&self) -> bool { self.in_network_format }
    pub const fn platform(&self) -> RedAlertTrackerPlatform { self.platform }

    fn index(&self, unit_type: i32) -> Result<usize, RedAlertTrackerError> {
        let index = usize::try_from(unit_type).map_err(|_| RedAlertTrackerError::UnitIndexOutOfRange {
            index: unit_type, count: self.unit_count,
        })?;
        if index >= self.unit_count {
            return Err(RedAlertTrackerError::UnitIndexOutOfRange { index: unit_type, count: self.unit_count });
        }
        Ok(index)
    }

    pub fn increment_unit_total(&mut self, unit_type: i32) -> Result<i32, RedAlertTrackerError> {
        let index = self.index(unit_type)?;
        self.totals[index] = self.totals[index].wrapping_add(1);
        Ok(self.totals[index])
    }

    pub fn decrement_unit_total(&mut self, unit_type: i32) -> Result<i32, RedAlertTrackerError> {
        let index = self.index(unit_type)?;
        self.totals[index] = self.totals[index].wrapping_sub(1);
        Ok(self.totals[index])
    }

    pub fn get_unit_total(&self, unit_type: i32) -> i32 {
        usize::try_from(unit_type).ok()
            .and_then(|index| self.totals.get(index).copied())
            .unwrap_or(0)
    }

    pub fn get_all_totals(&self) -> &[i32] { &self.totals }
    pub fn get_all_totals_mut(&mut self) -> &mut [i32] { &mut self.totals }

    /// Mirrors memset(UnitTotals, 0, UnitCount * sizeof(long)).
    /// As in the source it does not reset InNetworkFormat.
    pub fn clear_unit_total(&mut self) { self.totals.fill(0); }

    /// Win32 calls htonl; the non-Win32 translation unit expands htonl(x) to
    /// literal zero. The conversion-state flag is set in either configuration.
    pub fn to_network_format(&mut self) {
        if !self.in_network_format {
            match self.platform {
                RedAlertTrackerPlatform::Win32 => {
                    for total in &mut self.totals {
                        let raw = u32::from_ne_bytes(total.to_ne_bytes());
                        *total = i32::from_ne_bytes(raw.to_be().to_ne_bytes());
                    }
                }
                RedAlertTrackerPlatform::Dos => self.totals.fill(0),
            }
        }
        self.in_network_format = true;
    }

    /// Win32 calls ntohl; DOS expands ntohl(x) to zero as well.
    pub fn to_pc_format(&mut self) {
        if self.in_network_format {
            match self.platform {
                RedAlertTrackerPlatform::Win32 => {
                    for total in &mut self.totals {
                        let raw = u32::from_ne_bytes(total.to_ne_bytes());
                        *total = i32::from_ne_bytes(raw.from_be().to_ne_bytes());
                    }
                }
                RedAlertTrackerPlatform::Dos => self.totals.fill(0),
            }
        }
        self.in_network_format = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_zeroes_the_declared_number_of_32_bit_totals() {
        let tracker = RedAlertUnitTracker::new(4, RedAlertTrackerPlatform::Win32).unwrap();
        assert_eq!(tracker.unit_count(), 4);
        assert_eq!(tracker.get_all_totals(), &[0; 4]);
        assert!(!tracker.in_network_format());
    }

    #[test]
    fn negative_unit_count_is_rejected() {
        assert!(matches!(
            RedAlertUnitTracker::new(-1, RedAlertTrackerPlatform::Dos),
            Err(RedAlertTrackerError::NegativeUnitCount(-1))
        ));
    }

    #[test]
    fn increment_decrement_clear_and_get_keep_source_counter_semantics() {
        let mut tracker = RedAlertUnitTracker::new(2, RedAlertTrackerPlatform::Win32).unwrap();
        assert_eq!(tracker.increment_unit_total(0), Ok(1));
        assert_eq!(tracker.increment_unit_total(0), Ok(2));
        assert_eq!(tracker.decrement_unit_total(0), Ok(1));
        assert_eq!(tracker.get_unit_total(0), 1);
        assert_eq!(tracker.get_unit_total(-1), 0);
        assert_eq!(tracker.get_unit_total(2), 0);
        tracker.clear_unit_total();
        assert_eq!(tracker.get_all_totals(), &[0, 0]);
    }

    #[test]
    fn win32_network_conversion_roundtrips_signed_raw_counter_bits() {
        let mut tracker = RedAlertUnitTracker::new(3, RedAlertTrackerPlatform::Win32).unwrap();
        tracker.get_all_totals_mut().copy_from_slice(&[0x0102_0304, -1, i32::MIN]);
        let original = tracker.get_all_totals().to_vec();
        tracker.to_network_format();
        assert!(tracker.in_network_format());
        assert_eq!(tracker.get_all_totals()[0].to_ne_bytes(), [1, 2, 3, 4]);
        tracker.to_network_format();
        tracker.to_pc_format();
        assert_eq!(tracker.get_all_totals(), original.as_slice());
        tracker.to_pc_format();
        assert!(!tracker.in_network_format());
    }

    #[test]
    fn dos_network_macros_replace_every_counter_with_zero() {
        let mut tracker = RedAlertUnitTracker::new(3, RedAlertTrackerPlatform::Dos).unwrap();
        tracker.get_all_totals_mut().copy_from_slice(&[12, -7, 0x1020_3040]);
        tracker.to_network_format();
        assert!(tracker.in_network_format());
        assert_eq!(tracker.get_all_totals(), &[0, 0, 0]);
        tracker.to_pc_format();
        assert!(!tracker.in_network_format());
        assert_eq!(tracker.get_all_totals(), &[0, 0, 0]);
    }

    #[test]
    fn dos_conversion_guard_prevents_repeated_macro_execution_until_pc_format() {
        let mut tracker = RedAlertUnitTracker::new(1, RedAlertTrackerPlatform::Dos).unwrap();
        tracker.get_all_totals_mut()[0] = 42;
        tracker.to_network_format();
        tracker.get_all_totals_mut()[0] = 99;
        tracker.to_network_format();
        assert_eq!(tracker.get_unit_total(0), 99);
        tracker.to_pc_format();
        assert_eq!(tracker.get_unit_total(0), 0);
    }

    #[test]
    fn clear_keeps_the_conversion_guard_unchanged() {
        let mut tracker = RedAlertUnitTracker::new(2, RedAlertTrackerPlatform::Win32).unwrap();
        tracker.to_network_format();
        tracker.clear_unit_total();
        assert!(tracker.in_network_format());
    }

    #[test]
    fn invalid_mutating_indices_return_bounds_errors() {
        let mut tracker = RedAlertUnitTracker::new(1, RedAlertTrackerPlatform::Win32).unwrap();
        assert!(matches!(tracker.increment_unit_total(-1), Err(RedAlertTrackerError::UnitIndexOutOfRange { .. })));
        assert!(matches!(tracker.decrement_unit_total(1), Err(RedAlertTrackerError::UnitIndexOutOfRange { .. })));
    }
}
