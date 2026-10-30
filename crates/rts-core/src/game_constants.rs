//! Active Tiberian Dawn constants from TIBERIANDAWN/CONST.CPP.
//!
//! Arrays are transcribed in upstream enum/table order. Facing16 is guarded by
//! NEVER, the older Facing32 table by OBSOLETE, and the extra Nuke Lob weapon
//! by PETROGLYPH_EXAMPLE_MOD in the pinned default source; those disabled
//! alternatives are intentionally excluded from the active tables.

use crate::combat::TiberianDawnWarhead;
use crate::coordinate::Coordinate;
use crate::house::RemapTable;

pub const SOURCE_COUNT: usize = 11;
pub const SOURCE_NAMES: [&str; SOURCE_COUNT] = [
    "North", "East", "South", "West",
    "Shipping", "Beach", "Air", "Visible",
    "EnemyBase", "HomeBase", "Ocean"
];

pub const STOPPING_COORD_ABS: [Coordinate; 5] = [
    Coordinate::from_raw(0x00800080), Coordinate::from_raw(0x00400040),
    Coordinate::from_raw(0x004000C0), Coordinate::from_raw(0x00C00040),
    Coordinate::from_raw(0x00C000C0)
];

pub const PIXEL_TO_LEPTON: [u8; 24] = [
    0, 11, 21, 32, 43, 53, 64, 75, 85, 96, 107, 117,
    128, 139, 149, 160, 171, 181, 192, 203, 213, 224, 235, 245
];

pub const ADJACENT_CELL_OFFSETS: [i32; 8] = [-64, -63, 1, 65, 64, 63, -1, -65];
pub const ADJACENT_COORD_OFFSETS: [Coordinate; 8] = [
    Coordinate::from_raw(0xFF000000), Coordinate::from_raw(0xFF000100),
    Coordinate::from_raw(0x00000100), Coordinate::from_raw(0x01000100),
    Coordinate::from_raw(0x01000000), Coordinate::from_raw(0x0100FF00),
    Coordinate::from_raw(0x0000FF00), Coordinate::from_raw(0xFF00FF00)
];

pub const FACING_8: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

pub const FACING_32: [u8; 256] = [
    0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2,
    2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 6, 6,
    6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8,
    8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 10, 10, 10, 10, 10, 10,
    10, 11, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12,
    13, 13, 13, 13, 13, 13, 13, 13, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 17, 17, 17, 17, 17, 17, 17, 17, 17, 18, 18,
    18, 18, 18, 18, 18, 18, 18, 19, 19, 19, 19, 19, 19, 19, 19, 19,
    19, 20, 20, 20, 20, 20, 20, 21, 21, 21, 21, 21, 21, 21, 22, 22,
    22, 22, 22, 22, 22, 23, 23, 23, 23, 23, 23, 23, 24, 24, 24, 24,
    24, 24, 24, 25, 25, 25, 25, 25, 25, 25, 26, 26, 26, 26, 26, 26,
    26, 27, 27, 27, 27, 27, 27, 27, 28, 28, 28, 28, 28, 28, 28, 28,
    29, 29, 29, 29, 29, 29, 29, 29, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 31, 31, 31, 31, 31, 31, 31, 31, 31, 0, 0, 0, 0, 0, 0
];

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BulletType(pub i8);

pub const BULLET_APDS: BulletType = BulletType(2);
pub const BULLET_BULLET: BulletType = BulletType(1);
pub const BULLET_CHEMSPRAY: BulletType = BulletType(9);
pub const BULLET_FLAME: BulletType = BulletType(8);
pub const BULLET_GRENADE: BulletType = BulletType(11);
pub const BULLET_HE: BulletType = BulletType(3);
pub const BULLET_HEADBUTT: BulletType = BulletType(17);
pub const BULLET_HONEST_JOHN: BulletType = BulletType(15);
pub const BULLET_LASER: BulletType = BulletType(12);
pub const BULLET_NAPALM: BulletType = BulletType(10);
pub const BULLET_NONE: BulletType = BulletType(-1);
pub const BULLET_SAM: BulletType = BulletType(6);
pub const BULLET_SNIPER: BulletType = BulletType(0);
pub const BULLET_SPREADFIRE: BulletType = BulletType(16);
pub const BULLET_SSM: BulletType = BulletType(4);
pub const BULLET_SSM2: BulletType = BulletType(5);
pub const BULLET_TOW: BulletType = BulletType(7);
pub const BULLET_TREXBITE: BulletType = BulletType(18);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VocType(pub i8);

pub const VOC_BAZOOKA: VocType = VocType(33);
pub const VOC_DINOATK1: VocType = VocType(106);
pub const VOC_FLAMER1: VocType = VocType(40);
pub const VOC_LASER: VocType = VocType(54);
pub const VOC_MGUN11: VocType = VocType(50);
pub const VOC_MGUN2: VocType = VocType(51);
pub const VOC_MINI: VocType = VocType(45);
pub const VOC_NONE: VocType = VocType(-1);
pub const VOC_RIFLE: VocType = VocType(41);
pub const VOC_ROCKET1: VocType = VocType(58);
pub const VOC_ROCKET2: VocType = VocType(59);
pub const VOC_SNIPER: VocType = VocType(57);
pub const VOC_TANK1: VocType = VocType(65);
pub const VOC_TANK2: VocType = VocType(66);
pub const VOC_TANK3: VocType = VocType(67);
pub const VOC_TANK4: VocType = VocType(68);
pub const VOC_TOSS: VocType = VocType(73);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnimType(pub i8);

pub const ANIM_CHEM_N: AnimType = AnimType(22);
pub const ANIM_FLAME_N: AnimType = AnimType(14);
pub const ANIM_GUN_N: AnimType = AnimType(50);
pub const ANIM_MUZZLE_FLASH: AnimType = AnimType(34);
pub const ANIM_NONE: AnimType = AnimType(-1);

pub const WEAPON_COUNT: usize = 25;
pub const WEAPON_NAMES: [&str; WEAPON_COUNT] = [
    "WEAPON_RIFLE", "WEAPON_CHAIN_GUN", "WEAPON_PISTOL", "WEAPON_M16", "WEAPON_DRAGON",
    "WEAPON_FLAMETHROWER", "WEAPON_FLAME_TONGUE", "WEAPON_CHEMSPRAY", "WEAPON_GRENADE", "WEAPON_75MM",
    "WEAPON_105MM", "WEAPON_120MM", "WEAPON_TURRET_GUN", "WEAPON_MAMMOTH_TUSK", "WEAPON_MLRS",
    "WEAPON_155MM", "WEAPON_M60MG", "WEAPON_TOMAHAWK", "WEAPON_TOW_TWO", "WEAPON_NAPALM",
    "WEAPON_OBELISK_LASER", "WEAPON_NIKE", "WEAPON_HONEST_JOHN", "WEAPON_STEG", "WEAPON_TREX"
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeaponTypeClass {
    pub fires: BulletType,
    pub attack: u8,
    pub rof: u8,
    pub range: i32,
    pub sound: VocType,
    pub animation: AnimType,
}

pub const WEAPONS: [WeaponTypeClass; WEAPON_COUNT] = [
    WeaponTypeClass { fires: BULLET_SNIPER, attack: 125, rof: 40, range: 1408, sound: VOC_SNIPER, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_SPREADFIRE, attack: 25, rof: 50, range: 1024, sound: VOC_MINI, animation: ANIM_GUN_N },
    WeaponTypeClass { fires: BULLET_BULLET, attack: 1, rof: 7, range: 448, sound: VOC_RIFLE, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_BULLET, attack: 15, rof: 20, range: 512, sound: VOC_MGUN2, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_TOW, attack: 30, rof: 60, range: 1024, sound: VOC_BAZOOKA, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_FLAME, attack: 35, rof: 50, range: 512, sound: VOC_FLAMER1, animation: ANIM_FLAME_N },
    WeaponTypeClass { fires: BULLET_FLAME, attack: 50, rof: 50, range: 512, sound: VOC_FLAMER1, animation: ANIM_FLAME_N },
    WeaponTypeClass { fires: BULLET_CHEMSPRAY, attack: 80, rof: 70, range: 512, sound: VOC_FLAMER1, animation: ANIM_CHEM_N },
    WeaponTypeClass { fires: BULLET_GRENADE, attack: 50, rof: 60, range: 832, sound: VOC_TOSS, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_APDS, attack: 25, rof: 60, range: 1024, sound: VOC_TANK2, animation: ANIM_MUZZLE_FLASH },
    WeaponTypeClass { fires: BULLET_APDS, attack: 30, rof: 50, range: 1216, sound: VOC_TANK3, animation: ANIM_MUZZLE_FLASH },
    WeaponTypeClass { fires: BULLET_APDS, attack: 40, rof: 80, range: 1216, sound: VOC_TANK4, animation: ANIM_MUZZLE_FLASH },
    WeaponTypeClass { fires: BULLET_APDS, attack: 40, rof: 60, range: 1536, sound: VOC_TANK4, animation: ANIM_MUZZLE_FLASH },
    WeaponTypeClass { fires: BULLET_SSM, attack: 75, rof: 80, range: 1280, sound: VOC_ROCKET1, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_SSM2, attack: 75, rof: 80, range: 1536, sound: VOC_ROCKET1, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_HE, attack: 150, rof: 65, range: 1536, sound: VOC_TANK1, animation: ANIM_MUZZLE_FLASH },
    WeaponTypeClass { fires: BULLET_BULLET, attack: 15, rof: 30, range: 1024, sound: VOC_MGUN11, animation: ANIM_GUN_N },
    WeaponTypeClass { fires: BULLET_SSM, attack: 60, rof: 35, range: 1920, sound: VOC_ROCKET2, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_SSM, attack: 60, rof: 40, range: 1664, sound: VOC_ROCKET2, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_NAPALM, attack: 100, rof: 20, range: 1152, sound: VOC_NONE, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_LASER, attack: 200, rof: 90, range: 1920, sound: VOC_LASER, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_SAM, attack: 50, rof: 50, range: 1920, sound: VOC_ROCKET2, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_HONEST_JOHN, attack: 100, rof: 200, range: 2560, sound: VOC_ROCKET1, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_HEADBUTT, attack: 100, rof: 30, range: 384, sound: VOC_DINOATK1, animation: ANIM_NONE },
    WeaponTypeClass { fires: BULLET_TREXBITE, attack: 155, rof: 30, range: 384, sound: VOC_DINOATK1, animation: ANIM_NONE },
];

pub const WARHEAD_COUNT: usize = 12;
pub const WARHEAD_NAMES: [&str; WARHEAD_COUNT] = [
    "WARHEAD_SA", "WARHEAD_HE", "WARHEAD_AP", "WARHEAD_FIRE",
    "WARHEAD_LASER", "WARHEAD_PB", "WARHEAD_FIST", "WARHEAD_FOOT",
    "WARHEAD_HOLLOW_POINT", "WARHEAD_SPORE", "WARHEAD_HEADBUTT", "WARHEAD_FEEDME"
];

/// Exact Warheads[] data, indexed by the Tiberian Dawn WarheadType enum.
pub const TIBERIAN_DAWN_WARHEADS: [TiberianDawnWarhead; WARHEAD_COUNT] = [
    TiberianDawnWarhead { id: 0, spread_factor: 2, is_wall_destroyer: false, is_wood_destroyer: false, is_tiberium_destroyer: false, modifiers: [0x100, 0x80, 0x90, 0x40, 0x40] },
    TiberianDawnWarhead { id: 1, spread_factor: 6, is_wall_destroyer: true, is_wood_destroyer: true, is_tiberium_destroyer: true, modifiers: [0xE0, 0xC0, 0x90, 0x40, 0x100] },
    TiberianDawnWarhead { id: 2, spread_factor: 6, is_wall_destroyer: true, is_wood_destroyer: true, is_tiberium_destroyer: false, modifiers: [0x40, 0xC0, 0xC0, 0x100, 0x80] },
    TiberianDawnWarhead { id: 3, spread_factor: 8, is_wall_destroyer: false, is_wood_destroyer: true, is_tiberium_destroyer: true, modifiers: [0xE0, 0x100, 0xB0, 0x40, 0x80] },
    TiberianDawnWarhead { id: 4, spread_factor: 4, is_wall_destroyer: false, is_wood_destroyer: false, is_tiberium_destroyer: false, modifiers: [0x100, 0x100, 0x100, 0x100, 0x100] },
    TiberianDawnWarhead { id: 5, spread_factor: 7, is_wall_destroyer: true, is_wood_destroyer: true, is_tiberium_destroyer: true, modifiers: [0x100, 0x100, 0xC0, 0xC0, 0xC0] },
    TiberianDawnWarhead { id: 6, spread_factor: 4, is_wall_destroyer: false, is_wood_destroyer: false, is_tiberium_destroyer: false, modifiers: [0x100, 0x20, 0x20, 0x10, 0x10] },
    TiberianDawnWarhead { id: 7, spread_factor: 4, is_wall_destroyer: false, is_wood_destroyer: false, is_tiberium_destroyer: false, modifiers: [0x100, 0x20, 0x20, 0x10, 0x10] },
    TiberianDawnWarhead { id: 8, spread_factor: 4, is_wall_destroyer: false, is_wood_destroyer: false, is_tiberium_destroyer: false, modifiers: [0x100, 0x8, 0x8, 0x8, 0x8] },
    TiberianDawnWarhead { id: 9, spread_factor: 255, is_wall_destroyer: false, is_wood_destroyer: false, is_tiberium_destroyer: false, modifiers: [0x100, 0x1, 0x1, 0x1, 0x1] },
    TiberianDawnWarhead { id: 10, spread_factor: 1, is_wall_destroyer: true, is_wood_destroyer: true, is_tiberium_destroyer: false, modifiers: [0x100, 0xC0, 0x80, 0x20, 0x8] },
    TiberianDawnWarhead { id: 11, spread_factor: 1, is_wall_destroyer: true, is_wood_destroyer: true, is_tiberium_destroyer: false, modifiers: [0x100, 0xC0, 0x80, 0x20, 0x8] },
];

pub fn warhead_by_id(id: i8) -> Option<&'static TiberianDawnWarhead> {
    usize::try_from(id).ok().and_then(|index| TIBERIAN_DAWN_WARHEADS.get(index))
}

pub const GROUND_COST_S1: u8 = 0x00;
pub const GROUND_COST_S2: u8 = 0x40;
pub const GROUND_COST_S3: u8 = 0x70;
pub const GROUND_COST_S4: u8 = 0xA0;
pub const GROUND_COST_S5: u8 = 0xC0;
pub const GROUND_COST_S6: u8 = 0xFF;

pub const LAND_COUNT: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroundRadarColor {
    PaletteIndex(u8),
    Blue,
    DarkGray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LandType {
    Clear, Road, Water, Rock, Wall, Tiberium, Beach,
}

pub const LAND_TYPES: [LandType; LAND_COUNT] = [
    LandType::Clear, LandType::Road, LandType::Water, LandType::Rock,
    LandType::Wall, LandType::Tiberium, LandType::Beach,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroundType {
    pub radar_color: GroundRadarColor,
    /// Foot, Tracked, Harvester, Wheeled, Winged, Hover, Float/Build.
    pub movement_costs: [u8; 7],
    pub is_buildable: bool,
}

pub const GROUND: [GroundType; LAND_COUNT] = [
    GroundType { radar_color: GroundRadarColor::PaletteIndex(66), movement_costs: [112, 112, 112, 160, 255, 192, 0], is_buildable: true },
    GroundType { radar_color: GroundRadarColor::PaletteIndex(68), movement_costs: [192, 160, 160, 160, 255, 192, 0], is_buildable: true },
    GroundType { radar_color: GroundRadarColor::Blue, movement_costs: [0, 0, 0, 0, 255, 192, 255], is_buildable: false },
    GroundType { radar_color: GroundRadarColor::DarkGray, movement_costs: [0, 0, 0, 0, 255, 0, 0], is_buildable: false },
    GroundType { radar_color: GroundRadarColor::DarkGray, movement_costs: [0, 0, 0, 0, 255, 0, 0], is_buildable: false },
    GroundType { radar_color: GroundRadarColor::PaletteIndex(143), movement_costs: [112, 112, 112, 160, 255, 192, 0], is_buildable: false },
    GroundType { radar_color: GroundRadarColor::PaletteIndex(66), movement_costs: [112, 112, 112, 160, 255, 192, 0], is_buildable: false },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheaterDataType {
    /// Full name, INI name, and three-character filename identifier.
    pub names: [&'static str; 3],
}

pub const THEATER_COUNT: usize = 4;
pub const THEATERS: [TheaterDataType; THEATER_COUNT] = [
    TheaterDataType { names: ["DESERT", "DESERT", "DES"] },
    TheaterDataType { names: ["JUNGLE", "JUNGLE", "JUN"] },
    TheaterDataType { names: ["TEMPERATE", "TEMPERAT", "TEM"] },
    TheaterDataType { names: ["WINTER", "WINTER", "WIN"] },
];

pub const REMAP_GOLD: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
    176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
    208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
    224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
];

pub const REMAP_RED: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
    127, 126, 125, 124, 122, 46, 120, 47, 125, 124, 123, 122, 42, 121, 120, 120,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
    208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
    224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
];

pub const REMAP_BLUE: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
    2, 119, 118, 135, 136, 138, 112, 12, 118, 135, 136, 137, 138, 139, 114, 112,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
    208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
    224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
];

pub const REMAP_ORANGE: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
    24, 25, 26, 27, 29, 31, 46, 47, 26, 27, 28, 29, 30, 31, 43, 47,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
    208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
    224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
];

pub const REMAP_GREEN: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
    5, 165, 166, 167, 159, 142, 140, 199, 166, 167, 157, 3, 159, 143, 142, 141,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
    208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
    224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
];

pub const REMAP_LT_BLUE: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
    161, 200, 201, 202, 204, 205, 206, 12, 201, 202, 203, 204, 205, 115, 198, 114,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
    208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
    224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
];

pub const REMAP_NONE: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
    176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207,
    208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
    224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
];

pub const fn remap_table(table: RemapTable) -> &'static [u8; 256] {
    match table {
        RemapTable::Gold => &REMAP_GOLD,
        RemapTable::Red => &REMAP_RED,
        RemapTable::Blue => &REMAP_BLUE,
        RemapTable::Orange => &REMAP_ORANGE,
        RemapTable::Green => &REMAP_GREEN,
        RemapTable::LightBlue => &REMAP_LT_BLUE,
        RemapTable::None => &REMAP_NONE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_name_and_stopping_coordinate_tables_match_source() {
        assert_eq!(SOURCE_NAMES, [
            "North", "East", "South", "West", "Shipping", "Beach",
            "Air", "Visible", "EnemyBase", "HomeBase", "Ocean",
        ]);
        assert_eq!(STOPPING_COORD_ABS.map(Coordinate::raw), [
            0x0080_0080, 0x0040_0040, 0x0040_00C0, 0x00C0_0040, 0x00C0_00C0,
        ]);
    }

    #[test]
    fn pixel_and_adjacent_coordinate_tables_preserve_source_values() {
        assert_eq!(PIXEL_TO_LEPTON.len(), 24);
        assert_eq!(&PIXEL_TO_LEPTON[..8], &[0x00, 0x0B, 0x15, 0x20, 0x2B, 0x35, 0x40, 0x4B]);
        assert_eq!(PIXEL_TO_LEPTON[23], 0xF5);
        assert_eq!(ADJACENT_CELL_OFFSETS, [-64, -63, 1, 65, 64, 63, -1, -65]);
        assert_eq!(ADJACENT_COORD_OFFSETS.map(Coordinate::raw), [
            0xFF00_0000, 0xFF00_0100, 0x0000_0100, 0x0100_0100,
            0x0100_0000, 0x0100_FF00, 0x0000_FF00, 0xFF00_FF00,
        ]);
    }

    #[test]
    fn active_facing_lookup_tables_keep_all_256_entries() {
        assert_eq!(FACING_8.len(), 256);
        assert_eq!(FACING_32.len(), 256);
        assert_eq!(FACING_8[0], 0);
        assert_eq!(FACING_8[16], 1);
        assert_eq!(FACING_8[240], 0);
        assert_eq!(FACING_32[5], 1);
        assert_eq!(FACING_32[248], 31);
        assert_eq!(FACING_32[255], 0);
    }

    #[test]
    fn weapon_catalogue_matches_source_order_and_profile_values() {
        assert_eq!(WEAPONS.len(), 25);
        assert_eq!(WEAPON_NAMES[0], "WEAPON_RIFLE");
        assert_eq!(WEAPON_NAMES[24], "WEAPON_TREX");
        assert_eq!(WEAPONS[0], WeaponTypeClass {
            fires: BULLET_SNIPER, attack: 125, rof: 40, range: 0x0580,
            sound: VOC_SNIPER, animation: ANIM_NONE,
        });
        assert_eq!(WEAPONS[1].fires, BULLET_SPREADFIRE);
        assert_eq!(WEAPONS[15].attack, 150);
        assert_eq!(WEAPONS[24].attack, 155);
    }

    #[test]
    fn warhead_profiles_match_upstream_combat_data() {
        assert_eq!(TIBERIAN_DAWN_WARHEADS.len(), 12);
        assert_eq!(WARHEAD_NAMES[0], "WARHEAD_SA");
        assert_eq!(WARHEAD_NAMES[1], "WARHEAD_HE");
        assert!(warhead_by_id(1).unwrap().is_wall_destroyer);
        assert_eq!(warhead_by_id(1).unwrap().spread_factor, 6);
        assert_eq!(warhead_by_id(1).unwrap().modifiers, [0xE0, 0xC0, 0x90, 0x40, 0x100]);
        assert!(!warhead_by_id(11).unwrap().is_tiberium_destroyer);
        assert_eq!(warhead_by_id(-1), None);
        assert_eq!(warhead_by_id(12), None);
    }

    #[test]
    fn ground_profiles_keep_source_cost_order_and_buildability() {
        assert_eq!(LAND_TYPES.len(), 7);
        assert_eq!(GROUND[0].radar_color, GroundRadarColor::PaletteIndex(66));
        assert_eq!(GROUND[0].movement_costs, [0x70, 0x70, 0x70, 0xA0, 0xFF, 0xC0, 0x00]);
        assert!(GROUND[0].is_buildable);
        assert!(GROUND[1].is_buildable);
        assert_eq!(GROUND[2].radar_color, GroundRadarColor::Blue);
        assert_eq!(GROUND[3].radar_color, GroundRadarColor::DarkGray);
        assert!(!GROUND[6].is_buildable);
    }

    #[test]
    fn theater_names_keep_full_ini_and_filename_forms() {
        assert_eq!(THEATERS.len(), 4);
        assert_eq!(THEATERS[0].names, ["DESERT", "DESERT", "DES"]);
        assert_eq!(THEATERS[2].names, ["TEMPERATE", "TEMPERAT", "TEM"]);
        assert_eq!(THEATERS[3].names, ["WINTER", "WINTER", "WIN"]);
    }

    #[test]
    fn identity_and_colored_remap_tables_preserve_exact_bytes() {
        let identity: [u8; 256] = core::array::from_fn(|index| index as u8);
        assert_eq!(REMAP_GOLD, identity);
        assert_eq!(REMAP_NONE, identity);
        for table in [
            RemapTable::Red, RemapTable::Blue, RemapTable::Orange,
            RemapTable::Green, RemapTable::LightBlue,
        ] {
            assert_eq!(remap_table(table).len(), 256);
            assert_ne!(remap_table(table), &REMAP_NONE);
        }
        assert_eq!(REMAP_RED[176], 127);
        assert_eq!(REMAP_BLUE[176], 2);
        assert_eq!(REMAP_LT_BLUE[176], 161);
    }
}
