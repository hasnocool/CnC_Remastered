//! Tiberian Dawn UnitTrackerClass from TIBERIANDAWN/UTRACKER.CPP.
//!
//! Win32 long is 32 bits. Network conversion mutates the same counter array
//! in-place and is guarded by the original InNetworkFormat state bit.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitTrackerError {
    NegativeUnitCount(i32),
    AllocationFailed,
    UnitIndexOutOfRange { index: i32, count: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitTrackerClass {
    totals: Vec<i32>,
    in_network_format: bool,
}

impl UnitTrackerClass {
    pub fn new(unit_count: i32) -> Result<Self, UnitTrackerError> {
        let count = usize::try_from(unit_count)
            .map_err(|_| UnitTrackerError::NegativeUnitCount(unit_count))?;
        let mut totals = Vec::new();
        totals.try_reserve_exact(count).map_err(|_| UnitTrackerError::AllocationFailed)?;
        totals.resize(count, 0_i32);
        Ok(Self { totals, in_network_format: false })
    }

    pub fn unit_count(&self) -> usize { self.totals.len() }
    pub const fn in_network_format(&self) -> bool { self.in_network_format }

    pub fn increment_unit_total(&mut self, unit_type: i32) -> Result<i32, UnitTrackerError> {
        let index = self.index(unit_type)?;
        self.totals[index] = self.totals[index].wrapping_add(1);
        Ok(self.totals[index])
    }

    pub fn decrement_unit_total(&mut self, unit_type: i32) -> Result<i32, UnitTrackerError> {
        let index = self.index(unit_type)?;
        self.totals[index] = self.totals[index].wrapping_sub(1);
        Ok(self.totals[index])
    }

    fn index(&self, unit_type: i32) -> Result<usize, UnitTrackerError> {
        let index = usize::try_from(unit_type).map_err(|_| UnitTrackerError::UnitIndexOutOfRange {
            index: unit_type, count: self.totals.len(),
        })?;
        if index >= self.totals.len() {
            return Err(UnitTrackerError::UnitIndexOutOfRange { index: unit_type, count: self.totals.len() });
        }
        Ok(index)
    }

    /// Get_Unit_Total returns zero for an invalid index or null storage.
    pub fn get_unit_total(&self, unit_type: i32) -> i32 {
        usize::try_from(unit_type).ok()
            .and_then(|index| self.totals.get(index).copied())
            .unwrap_or(0)
    }

    /// Safe slice counterpart to Get_All_Totals.
    pub fn get_all_totals(&self) -> &[i32] { &self.totals }
    pub fn get_all_totals_mut(&mut self) -> &mut [i32] { &mut self.totals }

    /// Clear_Unit_Total clears the backing memory but does not alter the
    /// InNetworkFormat flag in the original source.
    pub fn clear_unit_total(&mut self) {
        self.totals.fill(0);
    }

    /// htonl for every signed 32-bit counter, preserving the raw bit pattern.
    pub fn to_network_format(&mut self) {
        if !self.in_network_format {
            for value in &mut self.totals {
                let raw = u32::from_ne_bytes(value.to_ne_bytes());
                *value = i32::from_ne_bytes(raw.to_be().to_ne_bytes());
            }
        }
        self.in_network_format = true;
    }

    /// ntohl for every signed 32-bit counter, preserving the raw bit pattern.
    pub fn to_pc_format(&mut self) {
        if self.in_network_format {
            for value in &mut self.totals {
                let raw = u32::from_ne_bytes(value.to_ne_bytes());
                *value = i32::from_ne_bytes(raw.from_be().to_ne_bytes());
            }
        }
        self.in_network_format = false;
    }

    pub fn network_bytes(&self) -> Vec<u8> {
        self.totals.iter().flat_map(|value| value.to_ne_bytes()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_allocates_zeroed_32_bit_counter_array() {
        let tracker = UnitTrackerClass::new(4).unwrap();
        assert_eq!(tracker.unit_count(), 4);
        assert_eq!(tracker.get_all_totals(), &[0, 0, 0, 0]);
        assert!(!tracker.in_network_format());
    }

    #[test]
    fn negative_count_is_rejected_and_zero_count_is_valid() {
        assert_eq!(UnitTrackerClass::new(-1).err(), Some(UnitTrackerError::NegativeUnitCount(-1)));
        let empty = UnitTrackerClass::new(0).unwrap();
        assert_eq!(empty.unit_count(), 0);
        assert_eq!(empty.get_all_totals(), &[]);
    }

    #[test]
    fn increment_decrement_and_get_total_track_each_type() {
        let mut tracker = UnitTrackerClass::new(3).unwrap();
        assert_eq!(tracker.increment_unit_total(1), Ok(1));
        assert_eq!(tracker.increment_unit_total(1), Ok(2));
        assert_eq!(tracker.decrement_unit_total(1), Ok(1));
        assert_eq!(tracker.get_unit_total(0), 0);
        assert_eq!(tracker.get_unit_total(1), 1);
        assert_eq!(tracker.get_unit_total(2), 0);
    }

    #[test]
    fn invalid_get_returns_zero_and_mutations_report_bounds_error() {
        let mut tracker = UnitTrackerClass::new(2).unwrap();
        assert_eq!(tracker.get_unit_total(-1), 0);
        assert_eq!(tracker.get_unit_total(2), 0);
        assert_eq!(
            tracker.increment_unit_total(-1),
            Err(UnitTrackerError::UnitIndexOutOfRange { index: -1, count: 2 })
        );
        assert_eq!(
            tracker.decrement_unit_total(2),
            Err(UnitTrackerError::UnitIndexOutOfRange { index: 2, count: 2 })
        );
    }

    #[test]
    fn clear_zeros_all_counters_without_changing_format_state() {
        let mut tracker = UnitTrackerClass::new(3).unwrap();
        tracker.increment_unit_total(0).unwrap();
        tracker.increment_unit_total(2).unwrap();
        tracker.to_network_format();
        tracker.clear_unit_total();
        assert_eq!(tracker.get_all_totals(), &[0, 0, 0]);
        assert!(tracker.in_network_format());
    }

    #[test]
    fn network_conversion_uses_big_endian_bytes_and_is_idempotent() {
        let mut tracker = UnitTrackerClass::new(2).unwrap();
        tracker.get_all_totals_mut()[0] = 0x0102_0304;
        tracker.get_all_totals_mut()[1] = -2;
        tracker.to_network_format();
        assert!(tracker.in_network_format());
        assert_eq!(&tracker.network_bytes()[..4], &[1, 2, 3, 4]);
        let once = tracker.get_all_totals().to_vec();
        tracker.to_network_format();
        assert_eq!(tracker.get_all_totals(), once.as_slice());
    }

    #[test]
    fn pc_conversion_round_trips_positive_and_negative_counter_bits() {
        let mut tracker = UnitTrackerClass::new(3).unwrap();
        tracker.get_all_totals_mut().copy_from_slice(&[0x0102_0304, -1, i32::MIN]);
        let original = tracker.get_all_totals().to_vec();
        tracker.to_network_format();
        tracker.to_pc_format();
        assert_eq!(tracker.get_all_totals(), original.as_slice());
        assert!(!tracker.in_network_format());
    }

    #[test]
    fn repeated_pc_conversion_is_a_no_op() {
        let mut tracker = UnitTrackerClass::new(1).unwrap();
        tracker.get_all_totals_mut()[0] = 0x0102_0304;
        tracker.to_pc_format();
        assert_eq!(tracker.get_unit_total(0), 0x0102_0304);
        assert!(!tracker.in_network_format());
    }

    #[test]
    fn counter_arithmetic_wraps_32_bit_storage() {
        let mut tracker = UnitTrackerClass::new(1).unwrap();
        tracker.get_all_totals_mut()[0] = i32::MAX;
        assert_eq!(tracker.increment_unit_total(0), Ok(i32::MIN));
        assert_eq!(tracker.decrement_unit_total(0), Ok(i32::MAX));
    }
}
