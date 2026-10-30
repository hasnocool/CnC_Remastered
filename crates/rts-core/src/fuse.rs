//! FuseClass from both REDALERT/FUSE.CPP and TIBERIANDAWN/FUSE.CPP.
//!
//! The two translation units implement the same timer/arming/proximity
//! algorithm and the same eight-byte raw save format:
//! Timer:u8, Arming:u8, HeadTo:u32 little-endian, Proximity:i16 little-endian.

use crate::coordinate::Coordinate;

pub const FUSE_SAVE_SIZE: usize = 8;
pub const FUSE_CLOSE_DISTANCE: u32 = 0x0010;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FuseVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FuseClass {
    pub variant: FuseVariant,
    pub timer: u8,
    pub arming: u8,
    pub head_to: Coordinate,
    pub proximity: i16,
}

impl FuseClass {
    pub const fn new(variant: FuseVariant) -> Self {
        Self { variant, timer: 0, arming: 0, head_to: Coordinate::ZERO, proximity: 0 }
    }

    /// Arm_Fuse clamps the maximum lifetime to at least the arming time,
    /// then stores both countdowns in their legacy unsigned-byte fields.
    pub fn arm_fuse(&mut self, location: Coordinate, target: Coordinate, time_to: i32, arming: i32) {
        let maximum = time_to.max(arming).min(0xff);
        let arming = arming.min(0xff);
        self.timer = maximum as u8;
        self.arming = arming as u8;
        self.head_to = target;
        self.proximity = Coordinate::distance(location, target) as i16;
    }

    /// Fuse_Checkup always decrements Timer, even during the arming period.
    /// Once armed, the fuse triggers on timer expiry, within 0x10 leptons of
    /// the target, or when it moves away while still inside ICON_LEPTON_W.
    ///
    /// ICON_LEPTON_W is supplied by the caller because it is defined in the
    /// game icon headers rather than in either FUSE translation unit.
    pub fn fuse_checkup(
        &mut self,
        new_location: Coordinate,
        icon_lepton_width: u32,
    ) -> bool {
        if self.timer != 0 { self.timer -= 1; }

        if self.arming != 0 {
            self.arming -= 1;
        } else {
            if self.timer == 0 { return true; }

            let proximity = Coordinate::distance(new_location, self.head_to) as i32;
            if proximity < FUSE_CLOSE_DISTANCE as i32 { return true; }

            if proximity < icon_lepton_width as i32 && proximity > self.proximity as i32 {
                return true;
            }
            self.proximity = proximity as i16;
        }
        false
    }

    pub const fn fuse_target(&self) -> Coordinate { self.head_to }

    /// Fuse_Write writes the four fields sequentially with no padding.
    pub fn fuse_write(&self) -> [u8; FUSE_SAVE_SIZE] {
        let mut bytes = [0u8; FUSE_SAVE_SIZE];
        bytes[0] = self.timer;
        bytes[1] = self.arming;
        bytes[2..6].copy_from_slice(&self.head_to.raw().to_le_bytes());
        bytes[6..8].copy_from_slice(&self.proximity.to_le_bytes());
        bytes
    }

    /// Fuse_Read restores the exact Timer/Arming/HeadTo/Proximity byte order.
    pub fn fuse_read(&mut self, bytes: &[u8]) -> Result<(), FuseReadError> {
        if bytes.len() < FUSE_SAVE_SIZE {
            return Err(FuseReadError::Truncated { supplied: bytes.len(), required: FUSE_SAVE_SIZE });
        }
        self.timer = bytes[0];
        self.arming = bytes[1];
        self.head_to = Coordinate::from_raw(u32::from_le_bytes(bytes[2..6].try_into().unwrap()));
        self.proximity = i16::from_le_bytes(bytes[6..8].try_into().unwrap());
        Ok(())
    }

    pub fn from_save_bytes(variant: FuseVariant, bytes: &[u8]) -> Result<Self, FuseReadError> {
        let mut fuse = Self::new(variant);
        fuse.fuse_read(bytes)?;
        Ok(fuse)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FuseReadError {
    Truncated { supplied: usize, required: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn coord(x: i16, y: i16) -> Coordinate { Coordinate::from_leptons(x, y) }

    #[test]
    fn constructor_zeros_all_fields_for_both_variants() {
        let ra = FuseClass::new(FuseVariant::RedAlert);
        let td = FuseClass::new(FuseVariant::TiberianDawn);
        assert_eq!((ra.timer, ra.arming, ra.head_to, ra.proximity), (0, 0, Coordinate::ZERO, 0));
        assert_eq!((td.timer, td.arming, td.head_to, td.proximity), (0, 0, Coordinate::ZERO, 0));
    }

    #[test]
    fn arm_fuse_enforces_lifetime_at_least_arming_and_byte_limits() {
        let mut fuse = FuseClass::new(FuseVariant::RedAlert);
        fuse.arm_fuse(coord(0, 0), coord(1000, 0), 10, 20);
        assert_eq!((fuse.timer, fuse.arming), (20, 20));
        assert_eq!(fuse.fuse_target(), coord(1000, 0));
        assert_eq!(fuse.proximity, 1000);

        fuse.arm_fuse(coord(0, 0), coord(1, 2), 1000, 300);
        assert_eq!((fuse.timer, fuse.arming), (255, 255));
    }

    #[test]
    fn timer_runs_during_arming_but_detonation_waits_until_armed() {
        let mut fuse = FuseClass::new(FuseVariant::TiberianDawn);
        fuse.arm_fuse(coord(0, 0), coord(1000, 0), 2, 2);
        assert!(!fuse.fuse_checkup(coord(500, 0), 256));
        assert_eq!((fuse.timer, fuse.arming), (1, 1));
        assert!(!fuse.fuse_checkup(coord(500, 0), 256));
        assert_eq!((fuse.timer, fuse.arming), (0, 0));
        assert!(fuse.fuse_checkup(coord(500, 0), 256));
    }

    #[test]
    fn timer_expiry_and_close_target_trigger_detonation() {
        let mut timed = FuseClass::new(FuseVariant::RedAlert);
        timed.arm_fuse(coord(0, 0), coord(1000, 0), 1, 0);
        assert!(timed.fuse_checkup(coord(500, 0), 256));

        let mut close = FuseClass::new(FuseVariant::RedAlert);
        close.arm_fuse(coord(0, 0), coord(1000, 0), 50, 0);
        assert!(close.fuse_checkup(coord(985, 0), 256));
    }

    #[test]
    fn proximity_increase_inside_icon_width_triggers_proximity_fuse() {
        let mut fuse = FuseClass::new(FuseVariant::TiberianDawn);
        fuse.arm_fuse(coord(0, 0), coord(1000, 0), 50, 0);
        assert!(!fuse.fuse_checkup(coord(900, 0), 256));
        assert_eq!(fuse.proximity, 100);
        assert!(fuse.fuse_checkup(coord(800, 0), 256));
    }

    #[test]
    fn increasing_proximity_outside_icon_width_does_not_trigger() {
        let mut fuse = FuseClass::new(FuseVariant::TiberianDawn);
        fuse.arm_fuse(coord(0, 0), coord(1000, 0), 50, 0);
        assert!(!fuse.fuse_checkup(coord(500, 0), 256));
        assert!(!fuse.fuse_checkup(coord(400, 0), 256));
        assert_eq!(fuse.proximity, 600);
    }

    #[test]
    fn save_format_is_exactly_eight_little_endian_bytes() {
        let mut fuse = FuseClass::new(FuseVariant::RedAlert);
        fuse.timer = 0x12;
        fuse.arming = 0x34;
        fuse.head_to = Coordinate::from_raw(0x1234_5678);
        fuse.proximity = -2;
        assert_eq!(fuse.fuse_write(), [0x12, 0x34, 0x78, 0x56, 0x34, 0x12, 0xfe, 0xff]);

        let restored = FuseClass::from_save_bytes(FuseVariant::TiberianDawn, &fuse.fuse_write()).unwrap();
        assert_eq!(restored.timer, 0x12);
        assert_eq!(restored.arming, 0x34);
        assert_eq!(restored.head_to, Coordinate::from_raw(0x1234_5678));
        assert_eq!(restored.proximity, -2);
    }

    #[test]
    fn truncated_save_data_is_rejected_without_partial_mutation() {
        let mut fuse = FuseClass::new(FuseVariant::RedAlert);
        fuse.timer = 9;
        assert_eq!(
            fuse.fuse_read(&[1, 2, 3]),
            Err(FuseReadError::Truncated { supplied: 3, required: 8 })
        );
        assert_eq!(fuse.timer, 9);
    }
}
