//! Projectile/flying-object movement from both variants' FLY.CPP.
//!
//! The 24-pixel/256-lepton cell scale makes PIXEL_LEPTON_W equal to 10 by
//! integer division. Coordinates use the shared lookup-table Coord_Move port.

use crate::coordinate::Coordinate;
use crate::facing::DirType;

pub const PIXEL_LEPTON_W: i32 = 256 / 24;
pub const HIGH_COORD_MASK: u32 = 0xC000_C000;
pub const MPH_IMMOBILE: u16 = 0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ImpactType { None = 0, Normal = 1, Edge = 2 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FlyVariant { RedAlert, TiberianDawn }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FlyState {
    pub speed_accum: u32,
    pub speed_add: u16,
}

impl Default for FlyState {
    fn default() -> Self { Self::new() }
}

impl FlyState {
    pub const fn new() -> Self {
        Self { speed_accum: 0, speed_add: MPH_IMMOBILE }
    }

    /// Red Alert's fixed(speed, 256) multiplication followed by rounded
    /// fixed-to-cardinal conversion.
    pub fn fly_speed_red_alert(&mut self, speed: i32, maximum: u16) {
        let product = (maximum as u64).wrapping_mul(speed as u64);
        self.speed_add = ((product.wrapping_add(128)) / 256) as u16;
    }

    /// Tiberian Dawn passes the fixed-point speed value directly to
    /// Fixed_To_Cardinal(base, fixed), whose assembly computes
    /// (base * fixed + 0x8000) >> 16.
    pub fn fly_speed_tiberian_dawn(&mut self, speed_fixed: u32, maximum: u16) {
        let product = (maximum as u64).wrapping_mul(speed_fixed as u64);
        self.speed_add = (product.wrapping_add(0x8000) >> 16) as u16;
    }

    pub fn speed(&self) -> u16 { self.speed_add }

    /// Advance a projectile by its accumulated movement. The C++ Coord_Move
    /// argument is an unsigned short, so the quantized distance is truncated
    /// to 16 bits at that call boundary.
    pub fn physics(&mut self, coord: &mut Coordinate, facing: DirType) -> ImpactType {
        if self.speed_add == MPH_IMMOBILE { return ImpactType::None; }

        let actual = (self.speed_add as i32).wrapping_add(self.speed_accum as i32);
        let remainder = actual % PIXEL_LEPTON_W;
        let quotient = actual / PIXEL_LEPTON_W;
        self.speed_accum = remainder as u32;
        let quantized = actual.wrapping_sub(remainder);

        if quotient == 0 { return ImpactType::None; }

        let old = *coord;
        let new_coord = coord.moved(facing, quantized as u16);
        if new_coord == *coord { return ImpactType::None; }

        *coord = new_coord;
        if new_coord.raw() & HIGH_COORD_MASK != 0 {
            *coord = old;
            return ImpactType::Edge;
        }
        ImpactType::Normal
    }

    pub fn physics_for_variant(&mut self, coord: &mut Coordinate, facing: DirType, _variant: FlyVariant) -> ImpactType {
        self.physics(coord, facing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_alert_speed_uses_fraction_of_256_with_rounding() {
        let mut fly = FlyState::new();
        fly.fly_speed_red_alert(128, 100);
        assert_eq!(fly.speed(), 50);
        fly.fly_speed_red_alert(255, 100);
        assert_eq!(fly.speed(), 100);
    }

    #[test]
    fn tiberian_dawn_uses_fixed_to_cardinal_assembly_rounding() {
        let mut fly = FlyState::new();
        fly.fly_speed_tiberian_dawn(0x8000, 100);
        assert_eq!(fly.speed(), 50);
        fly.fly_speed_tiberian_dawn(0x1_0000, 100);
        assert_eq!(fly.speed(), 100);
    }

    #[test]
    fn movement_accumulates_until_a_whole_pixel_quantum() {
        let mut fly = FlyState { speed_accum: 0, speed_add: 9 };
        let mut coord = Coordinate::from_leptons(0x2000, 0x2000);
        assert_eq!(fly.physics(&mut coord, DirType::EAST), ImpactType::None);
        assert_eq!(fly.speed_accum, 9);
        assert_eq!(coord, Coordinate::from_leptons(0x2000, 0x2000));
        assert_eq!(fly.physics(&mut coord, DirType::EAST), ImpactType::Normal);
        assert_eq!(fly.speed_accum, 8);
        assert_eq!(coord, Coordinate::from_leptons(0x2009, 0x2000));
    }

    #[test]
    fn edge_collision_restores_previous_coordinate() {
        let mut fly = FlyState { speed_accum: 0, speed_add: 10 };
        let mut coord = Coordinate::from_leptons(0x3fff, 0x1000);
        let old = coord;
        assert_eq!(fly.physics(&mut coord, DirType::EAST), ImpactType::Edge);
        assert_eq!(coord, old);
    }

    #[test]
    fn immobile_fly_does_not_accumulate_or_move() {
        let mut fly = FlyState::new();
        let mut coord = Coordinate::from_leptons(0x1000, 0x1000);
        assert_eq!(fly.physics(&mut coord, DirType::NORTH), ImpactType::None);
        assert_eq!(fly.speed_accum, 0);
        assert_eq!(coord, Coordinate::from_leptons(0x1000, 0x1000));
    }
}
