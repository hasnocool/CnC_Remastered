//! Tiberian Dawn HouseTypeClass catalogue from TIBERIANDAWN/HDATA.CPP.
//!
//! The array order is identical to HouseTypeClass::Pointers in the upstream
//! source. RemapTable stores the identity of each upstream palette table;
//! the actual 256-entry palette bytes remain owned by the rendering subsystem.

pub const HOUSE_NONE: i8 = -1;
pub const HOUSE_COUNT: usize = 10;

pub const COLOR_GOOD: u8 = 180;
pub const COLOR_BRIGHT_GOOD: u8 = 176;
pub const COLOR_BAD: u8 = 123;
pub const COLOR_BRIGHT_BAD: u8 = 127;
pub const COLOR_NEUTRAL: u8 = 205;
pub const COLOR_BRIGHT_NEUTRAL: u8 = 202;

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HousesType {
    Good = 0,
    Bad = 1,
    Neutral = 2,
    JurassicPark = 3,
    Multi1 = 4,
    Multi2 = 5,
    Multi3 = 6,
    Multi4 = 7,
    Multi5 = 8,
    Multi6 = 9,
}

impl HousesType {
    pub const ALL: [Self; HOUSE_COUNT] = [
        Self::Good,
        Self::Bad,
        Self::Neutral,
        Self::JurassicPark,
        Self::Multi1,
        Self::Multi2,
        Self::Multi3,
        Self::Multi4,
        Self::Multi5,
        Self::Multi6,
    ];

    pub const fn ordinal(self) -> usize {
        self as usize
    }

    pub const fn from_ordinal(value: usize) -> Option<Self> {
        match value {
            0 => Some(Self::Good),
            1 => Some(Self::Bad),
            2 => Some(Self::Neutral),
            3 => Some(Self::JurassicPark),
            4 => Some(Self::Multi1),
            5 => Some(Self::Multi2),
            6 => Some(Self::Multi3),
            7 => Some(Self::Multi4),
            8 => Some(Self::Multi5),
            9 => Some(Self::Multi6),
            _ => None,
        }
    }
}

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerColorType {
    Gold = 0,
    LightBlue = 1,
    Red = 2,
    Green = 3,
    Orange = 4,
    Blue = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RemapTable {
    Gold,
    LightBlue,
    Red,
    Green,
    Orange,
    Blue,
    None,
}

/// Symbolic localization resource used by TYPE.H's FullName integer field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HouseDisplayName {
    Gdi,
    Nod,
    Civilian,
    JurassicPark,
}

/// Rust representation of the upstream HouseTypeClass data record.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HouseTypeClass {
    pub house: HousesType,
    pub ini_name: &'static str,
    pub full_name: HouseDisplayName,
    /// Fixed four-byte C++ Suffix[4], including its trailing NUL.
    pub suffix: [u8; 4],
    pub lemon: u32,
    pub color: u8,
    pub bright_color: u8,
    pub remap_table: RemapTable,
    pub remap_color: PlayerColorType,
    pub prefix: char,
    pub firepower_bias: f32,
    pub groundspeed_bias: f32,
    pub airspeed_bias: f32,
    pub armor_bias: f32,
    pub rate_of_fire_bias: f32,
    pub cost_bias: f32,
    pub build_speed_bias: f32,
}

impl HouseTypeClass {
    /// Port of the original constructor, including three-byte suffix copy
    /// and the seven default 1.0 gameplay biases.
    pub fn new(
        house: HousesType,
        ini_name: &'static str,
        full_name: HouseDisplayName,
        suffix: &str,
        lemon: u32,
        color: u8,
        bright_color: u8,
        remap_color: PlayerColorType,
        remap_table: RemapTable,
        prefix: char,
    ) -> Self {
        let mut suffix_bytes = [0_u8; 4];
        let count = suffix.len().min(3);
        suffix_bytes[..count].copy_from_slice(&suffix.as_bytes()[..count]);

        Self {
            house,
            ini_name,
            full_name,
            suffix: suffix_bytes,
            lemon,
            color,
            bright_color,
            remap_table,
            remap_color,
            prefix,
            firepower_bias: 1.0,
            groundspeed_bias: 1.0,
            airspeed_bias: 1.0,
            armor_bias: 1.0,
            rate_of_fire_bias: 1.0,
            cost_bias: 1.0,
            build_speed_bias: 1.0,
        }
    }

    pub fn suffix_str(&self) -> &str {
        let end = self.suffix.iter().position(|byte| *byte == 0).unwrap_or(4);
        // The constructor only copies the ASCII suffix values from the source.
        std::str::from_utf8(&self.suffix[..end]).unwrap_or("")
    }
}

/// Mutable view over the source's static house-pointer catalogue.
///
/// Constructing this catalogue gives the pre-One_Time static values.
/// Call `one_time` at the same initialization point as the engine to apply
/// the Jurassic Park radar color override.
#[derive(Debug, Clone, PartialEq)]
pub struct HouseCatalog {
    entries: [HouseTypeClass; HOUSE_COUNT],
}

impl Default for HouseCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl HouseCatalog {
    pub fn new() -> Self {
        let neutral = (COLOR_NEUTRAL, COLOR_BRIGHT_NEUTRAL);
        let entries = [
            HouseTypeClass::new(
                HousesType::Good, "GoodGuy", HouseDisplayName::Gdi, "GDI", 0,
                COLOR_GOOD, COLOR_BRIGHT_GOOD, PlayerColorType::Gold, RemapTable::Gold, 'G',
            ),
            HouseTypeClass::new(
                HousesType::Bad, "BadGuy", HouseDisplayName::Nod, "NOD", 0,
                COLOR_BAD, COLOR_BRIGHT_BAD, PlayerColorType::LightBlue, RemapTable::LightBlue, 'B',
            ),
            HouseTypeClass::new(
                HousesType::Neutral, "Neutral", HouseDisplayName::Civilian, "CIV", 0,
                neutral.0, neutral.1, PlayerColorType::Gold, RemapTable::None, 'C',
            ),
            HouseTypeClass::new(
                HousesType::JurassicPark, "Special", HouseDisplayName::JurassicPark, "JP", 0,
                neutral.0, neutral.1, PlayerColorType::Gold, RemapTable::None, 'J',
            ),
            HouseTypeClass::new(
                HousesType::Multi1, "Multi1", HouseDisplayName::Civilian, "MP1", 0,
                neutral.0, neutral.1, PlayerColorType::LightBlue, RemapTable::LightBlue, 'M',
            ),
            HouseTypeClass::new(
                HousesType::Multi2, "Multi2", HouseDisplayName::Civilian, "MP2", 0,
                neutral.0, neutral.1, PlayerColorType::Orange, RemapTable::Orange, 'M',
            ),
            HouseTypeClass::new(
                HousesType::Multi3, "Multi3", HouseDisplayName::Civilian, "MP3", 0,
                neutral.0, neutral.1, PlayerColorType::Green, RemapTable::Green, 'M',
            ),
            HouseTypeClass::new(
                HousesType::Multi4, "Multi4", HouseDisplayName::Civilian, "MP4", 0,
                neutral.0, neutral.1, PlayerColorType::Blue, RemapTable::Blue, 'M',
            ),
            HouseTypeClass::new(
                HousesType::Multi5, "Multi5", HouseDisplayName::Civilian, "MP5", 0,
                neutral.0, neutral.1, PlayerColorType::Gold, RemapTable::Gold, 'M',
            ),
            HouseTypeClass::new(
                HousesType::Multi6, "Multi6", HouseDisplayName::Civilian, "MP6", 0,
                neutral.0, neutral.1, PlayerColorType::Red, RemapTable::Red, 'M',
            ),
        ];
        Self { entries }
    }

    /// HouseTypeClass::From_Name; null and unknown names map to HOUSE_NONE.
    pub fn from_name(name: Option<&str>) -> Option<HousesType> {
        let name = name?;
        Self::new()
            .entries
            .iter()
            .find(|entry| entry.ini_name.eq_ignore_ascii_case(name))
            .map(|entry| entry.house)
    }

    /// Bounds-safe counterpart to HouseTypeClass::As_Reference.
    pub fn as_reference(&self, house: HousesType) -> &HouseTypeClass {
        &self.entries[house.ordinal()]
    }

    pub fn as_reference_by_id(&self, house: i32) -> Option<&HouseTypeClass> {
        let ordinal = usize::try_from(house).ok()?;
        self.entries.get(ordinal)
    }

    pub fn as_reference_mut(&mut self, house: HousesType) -> &mut HouseTypeClass {
        &mut self.entries[house.ordinal()]
    }

    pub fn entries(&self) -> &[HouseTypeClass; HOUSE_COUNT] {
        &self.entries
    }

    /// HouseTypeClass::One_Time: change the JP owner's radar colors to Nod red.
    pub fn one_time(&mut self) {
        let jp = self.as_reference_mut(HousesType::JurassicPark);
        jp.color = COLOR_BAD;
        jp.bright_color = COLOR_BRIGHT_BAD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointer_table_order_matches_the_ten_upstream_house_ids() {
        let catalog = HouseCatalog::new();
        assert_eq!(catalog.entries().len(), HOUSE_COUNT);
        for (index, house) in HousesType::ALL.into_iter().enumerate() {
            assert_eq!(catalog.entries()[index].house, house);
            assert_eq!(house.ordinal(), index);
        }
        assert_eq!(HousesType::from_ordinal(HOUSE_COUNT), None);
    }

    #[test]
    fn canonical_ini_names_resolve_case_insensitively() {
        assert_eq!(HouseCatalog::from_name(Some("GoodGuy")), Some(HousesType::Good));
        assert_eq!(HouseCatalog::from_name(Some("badguy")), Some(HousesType::Bad));
        assert_eq!(HouseCatalog::from_name(Some("MULTI6")), Some(HousesType::Multi6));
        assert_eq!(HouseCatalog::from_name(Some("not-a-house")), None);
        assert_eq!(HouseCatalog::from_name(None), None);
    }

    #[test]
    fn canonical_names_suffixes_and_prefixes_match_source() {
        let catalog = HouseCatalog::new();
        let expected = [
            ("GoodGuy", "GDI", 'G'),
            ("BadGuy", "NOD", 'B'),
            ("Neutral", "CIV", 'C'),
            ("Special", "JP", 'J'),
            ("Multi1", "MP1", 'M'),
            ("Multi2", "MP2", 'M'),
            ("Multi3", "MP3", 'M'),
            ("Multi4", "MP4", 'M'),
            ("Multi5", "MP5", 'M'),
            ("Multi6", "MP6", 'M'),
        ];
        for (house, (name, suffix, prefix)) in HousesType::ALL.into_iter().zip(expected) {
            let entry = catalog.as_reference(house);
            assert_eq!(entry.ini_name, name);
            assert_eq!(entry.suffix_str(), suffix);
            assert_eq!(entry.suffix[3], 0);
            assert_eq!(entry.prefix, prefix);
        }
    }

    #[test]
    fn remap_table_and_player_color_pairs_match_all_houses() {
        let catalog = HouseCatalog::new();
        let expected = [
            (RemapTable::Gold, PlayerColorType::Gold),
            (RemapTable::LightBlue, PlayerColorType::LightBlue),
            (RemapTable::None, PlayerColorType::Gold),
            (RemapTable::None, PlayerColorType::Gold),
            (RemapTable::LightBlue, PlayerColorType::LightBlue),
            (RemapTable::Orange, PlayerColorType::Orange),
            (RemapTable::Green, PlayerColorType::Green),
            (RemapTable::Blue, PlayerColorType::Blue),
            (RemapTable::Gold, PlayerColorType::Gold),
            (RemapTable::Red, PlayerColorType::Red),
        ];
        for (house, expected) in HousesType::ALL.into_iter().zip(expected) {
            let entry = catalog.as_reference(house);
            assert_eq!((entry.remap_table, entry.remap_color), expected);
        }
    }

    #[test]
    fn radar_colors_follow_gdi_nod_and_neutral_source_constants() {
        let catalog = HouseCatalog::new();
        let good = catalog.as_reference(HousesType::Good);
        let bad = catalog.as_reference(HousesType::Bad);
        let neutral = catalog.as_reference(HousesType::Neutral);
        assert_eq!((good.color, good.bright_color), (180, 176));
        assert_eq!((bad.color, bad.bright_color), (123, 127));
        assert_eq!((neutral.color, neutral.bright_color), (205, 202));
        for house in [
            HousesType::Multi1, HousesType::Multi2, HousesType::Multi3,
            HousesType::Multi4, HousesType::Multi5, HousesType::Multi6,
        ] {
            let entry = catalog.as_reference(house);
            assert_eq!((entry.color, entry.bright_color), (205, 202));
        }
    }

    #[test]
    fn constructor_bias_defaults_and_lemon_frequency_are_zero_and_one() {
        let catalog = HouseCatalog::new();
        for entry in catalog.entries() {
            assert_eq!(entry.lemon, 0);
            assert_eq!(entry.firepower_bias, 1.0);
            assert_eq!(entry.groundspeed_bias, 1.0);
            assert_eq!(entry.airspeed_bias, 1.0);
            assert_eq!(entry.armor_bias, 1.0);
            assert_eq!(entry.rate_of_fire_bias, 1.0);
            assert_eq!(entry.cost_bias, 1.0);
            assert_eq!(entry.build_speed_bias, 1.0);
        }
    }

    #[test]
    fn one_time_recolors_jurassic_park_and_is_idempotent() {
        let mut catalog = HouseCatalog::new();
        let before = catalog.as_reference(HousesType::JurassicPark);
        assert_eq!((before.color, before.bright_color), (205, 202));

        catalog.one_time();
        let after = catalog.as_reference(HousesType::JurassicPark);
        assert_eq!((after.color, after.bright_color), (COLOR_BAD, COLOR_BRIGHT_BAD));

        catalog.one_time();
        let repeated = catalog.as_reference(HousesType::JurassicPark);
        assert_eq!((repeated.color, repeated.bright_color), (COLOR_BAD, COLOR_BRIGHT_BAD));
    }

    #[test]
    fn reference_lookup_rejects_invalid_numeric_ids() {
        let catalog = HouseCatalog::new();
        assert_eq!(catalog.as_reference_by_id(-1), None);
        assert_eq!(catalog.as_reference_by_id(HOUSE_COUNT as i32), None);
        assert_eq!(catalog.as_reference_by_id(0).unwrap().house, HousesType::Good);
    }
}
