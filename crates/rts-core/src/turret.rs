//! TurretClass constructors/destructor from REDALERT/TURRET.CPP and TURRET.H.
//!
//! The translation unit defines only an empty default constructor, an empty
//! destructor, and the parameterized constructor forwarding UnitType and
//! HousesType to DriveClass. The vehicle-driving/weapon behavior remains in
//! the separate DriveClass/derived-class translation units.

pub trait DriveClassConstruction<UnitType, HouseType> {
    fn drive_default() -> Self;
    fn drive_with_class_and_house(class_id: UnitType, house: HouseType) -> Self;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TurretClass<DriveState> {
    pub drive: DriveState,
}

impl<DriveState: Default> Default for TurretClass<DriveState> {
    /// The C++ default TurretClass constructor body is empty; the base class
    /// is default-constructed by the compiler before the body executes.
    fn default() -> Self {
        Self { drive: DriveState::default() }
    }
}

impl<DriveState> TurretClass<DriveState> {
    pub const fn from_drive_state(drive: DriveState) -> Self {
        Self { drive }
    }

    /// The C++ destructor body is empty. Rust drops the contained base state
    /// normally, preserving ownership semantics of the host implementation.
    pub fn destroy(self) {}
}

impl<DriveState> TurretClass<DriveState> {
    /// Parameterized TurretClass constructor: delegate all initialization to
    /// DriveClass(class_id, house), then execute the empty derived constructor.
    pub fn with_class_and_house<UnitType, HouseType>(
        class_id: UnitType,
        house: HouseType,
        drive_constructor: impl FnOnce(UnitType, HouseType) -> DriveState,
    ) -> Self {
        Self { drive: drive_constructor(class_id, house) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct DriveState {
        class_id: u16,
        house: u8,
        constructed_by_parameterized_base: bool,
    }

    impl DriveClassConstruction<u16, u8> for DriveState {
        fn drive_default() -> Self { Self::default() }
        fn drive_with_class_and_house(class_id: u16, house: u8) -> Self {
            Self { class_id, house, constructed_by_parameterized_base: true }
        }
    }

    #[test]
    fn default_constructor_only_default_constructs_the_base() {
        let turret = TurretClass::<DriveState>::default();
        assert_eq!(turret.drive, DriveState::default());
    }

    #[test]
    fn parameterized_constructor_forwards_class_and_house_to_drive_base() {
        let turret = TurretClass::with_class_and_house(7u16, 3u8, |class_id, house| {
            DriveState::drive_with_class_and_house(class_id, house)
        });
        assert_eq!(turret.drive.class_id, 7);
        assert_eq!(turret.drive.house, 3);
        assert!(turret.drive.constructed_by_parameterized_base);
    }

    #[test]
    fn explicit_empty_destructor_drops_the_derived_wrapper() {
        let turret = TurretClass::<DriveState>::default();
        turret.destroy();
    }
}
