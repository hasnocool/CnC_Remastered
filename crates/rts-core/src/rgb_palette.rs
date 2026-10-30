//! Red Alert RGB/HSV color models and PaletteClass from RGB.CPP, HSV.CPP,
//! PALETTEC.CPP, RGB.H, HSV.H, and PALETTEC.H.
//!
//! Stored RGB guns are six-bit device values (0..63). Public RGB components
//! expand these values back to eight bits with the original bit replication.
//! Palette writes are host-driven, while RGBClass::Raw_Set/Raw_Color_Prep
//! remain no-ops as they are in the pinned Win32 source.

use std::fmt;

pub const COLOR_COUNT: usize = 256;
pub const PALETTE_BYTES: usize = COLOR_COUNT * 3;
pub const RGB_MAX_VALUE: u8 = 255;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RgbColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl RgbColor {
    /// RGBClass(red, green, blue) stores each supplied 8-bit gun as gun >> 2.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red: red >> 2, green: green >> 2, blue: blue >> 2 }
    }

    /// Construct from the actual 6-bit device-dependent values.
    pub fn from_device_channels(red: u8, green: u8, blue: u8) -> Self {
        Self { red: red & 0x3f, green: green & 0x3f, blue: blue & 0x3f }
    }

    pub fn device_channels(self) -> [u8; 3] { [self.red, self.green, self.blue] }

    pub fn red_component(self) -> u8 { (self.red << 2) | (self.red >> 6) }
    pub fn green_component(self) -> u8 { (self.green << 2) | (self.green >> 6) }
    pub fn blue_component(self) -> u8 { (self.blue << 2) | (self.blue >> 6) }

    /// RGBClass::Adjust masks ratio to eight bits and divides signed
    /// differences by 256 using C++ truncation toward zero.
    pub fn adjust(&mut self, ratio: i32, destination: Self) {
        let ratio = (ratio & 0xff) as i32;
        self.red = adjust_channel(self.red, destination.red, ratio);
        self.green = adjust_channel(self.green, destination.green, ratio);
        self.blue = adjust_channel(self.blue, destination.blue, ratio);
    }

    /// Squared Euclidean distance in the stored six-bit channel space.
    pub fn difference(self, other: Self) -> i32 {
        let dr = self.red as i32 - other.red as i32;
        let dg = self.green as i32 - other.green as i32;
        let db = self.blue as i32 - other.blue as i32;
        dr * dr + dg * dg + db * db
    }

    pub fn to_hsv(self) -> HsvColor {
        let red = self.red_component() as i32;
        let green = self.green_component() as i32;
        let blue = self.blue_component() as i32;

        let value = red.max(green).max(blue);
        let white = red.min(green).min(blue);
        let mut saturation = 0;
        let mut hue = 0;

        if value != 0 {
            saturation = ((value - white) * 255) / value;
        }

        if saturation != 0 {
            let tmp = (value - white) as u32;
            let r1 = (((value - red) as u32 * 255) / tmp) as i32;
            let g1 = (((value - green) as u32 * 255) / tmp) as i32;
            let b1 = (((value - blue) as u32 * 255) / tmp) as i32;

            let hue_index = if value == red {
                if white == green { 5 * 256 + b1 } else { 1 * 256 - g1 }
            } else if value == green {
                if white == blue { 1 * 256 + r1 } else { 3 * 256 - b1 }
            } else if white == red {
                3 * 256 + g1
            } else {
                5 * 256 - r1
            };
            hue = hue_index / 6;
        }

        HsvColor::new(hue as u8, saturation as u8, value as u8)
    }

    /// RGBClass::Set updates the cached CurrentPalette entry. Its old
    /// hardware DAC calls are commented out in the Win32 implementation.
    pub fn set_in_palette(self, palette: &mut PaletteClass, color: usize) {
        palette.colors[color % COLOR_COUNT] = self;
    }
}

fn adjust_channel(source: u8, destination: u8, ratio: i32) -> u8 {
    let difference = destination as i32 - source as i32;
    (source as i32 + (difference * ratio) / 256) as u8
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HsvColor {
    hue: u8,
    saturation: u8,
    value: u8,
}

impl HsvColor {
    pub const fn new(hue: u8, saturation: u8, value: u8) -> Self {
        Self { hue, saturation, value }
    }

    pub fn hue_component(self) -> u8 { self.hue }
    pub fn saturation_component(self) -> u8 { self.saturation }
    pub fn value_component(self) -> u8 { self.value }

    /// HSVClass::Adjust uses linear component interpolation with ratio & 255.
    /// Hue does not wrap around the 0/255 boundary in the legacy code.
    pub fn adjust(&mut self, ratio: i32, destination: Self) {
        let ratio = ratio & 0xff;
        self.value = adjust_channel(self.value, destination.value, ratio);
        self.saturation = adjust_channel(self.saturation, destination.saturation, ratio);
        self.hue = adjust_channel(self.hue, destination.hue, ratio);
    }

    pub fn difference(self, other: Self) -> i32 {
        let dh = (self.hue as i32 - other.hue as i32).abs();
        let ds = (self.saturation as i32 - other.saturation as i32).abs();
        let dv = (self.value as i32 - other.value as i32).abs();
        dh * dh + ds * ds + dv * dv
    }

    /// HSVClass::operator RGBClass reproduces the source's six-sector lookup
    /// arithmetic, including its modulo-255 hue fraction and >>2 RGB storage.
    pub fn to_rgb(self) -> RgbColor {
        let hue = self.hue as u32 * 6;
        let fraction = hue % 255;
        let saturation = self.saturation as u32;
        let value = self.value as u32;

        let mut values = [0u32; 7];
        values[1] = value;
        values[2] = value;

        let tmp = (saturation * fraction) / 255;
        values[3] = (value * (255 - tmp)) / 255;
        values[4] = (value * (255 - saturation)) / 255;
        values[5] = values[4];

        let tmp = 255 - (saturation * (255 - fraction)) / 255;
        values[6] = (value * tmp) / 255;

        let mut index = hue / 255;
        index += if index > 4 { u32::MAX - 3 } else { 2 };
        let red = values[index as usize];

        index += if index > 4 { u32::MAX - 3 } else { 2 };
        let blue = values[index as usize];

        index += if index > 4 { u32::MAX - 3 } else { 2 };
        let green = values[index as usize];

        RgbColor::new(red as u8, green as u8, blue as u8)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaletteClass {
    colors: [RgbColor; COLOR_COUNT],
}

impl Default for PaletteClass {
    fn default() -> Self { Self { colors: [RgbColor::default(); COLOR_COUNT] } }
}

impl PaletteClass {
    pub fn new() -> Self { Self::default() }

    /// PaletteClass(RGBClass const&) fills all 256 palette entries.
    pub fn filled(color: RgbColor) -> Self { Self { colors: [color; COLOR_COUNT] } }

    pub fn from_device_bytes(bytes: &[u8]) -> Result<Self, PaletteError> {
        if bytes.len() < PALETTE_BYTES {
            return Err(PaletteError::PaletteTooShort { supplied: bytes.len(), required: PALETTE_BYTES });
        }
        let mut colors = [RgbColor::default(); COLOR_COUNT];
        for (index, color) in colors.iter_mut().enumerate() {
            let at = index * 3;
            *color = RgbColor::from_device_channels(bytes[at], bytes[at + 1], bytes[at + 2]);
        }
        Ok(Self { colors })
    }

    /// Returns the 768 stored device-channel bytes, not expanded 8-bit RGB.
    pub fn to_device_bytes(&self) -> [u8; PALETTE_BYTES] {
        let mut result = [0u8; PALETTE_BYTES];
        for (index, color) in self.colors.iter().enumerate() {
            let at = index * 3;
            result[at..at + 3].copy_from_slice(&color.device_channels());
        }
        result
    }

    pub fn color(&self, index: usize) -> RgbColor { self.colors[index % COLOR_COUNT] }
    pub fn set_color(&mut self, index: usize, color: RgbColor) {
        self.colors[index % COLOR_COUNT] = color;
    }
    pub fn colors(&self) -> &[RgbColor; COLOR_COUNT] { &self.colors }
    pub fn colors_mut(&mut self) -> &mut [RgbColor; COLOR_COUNT] { &mut self.colors }

    pub fn adjust_to_black(&mut self, ratio: i32) {
        let black = RgbColor::default();
        for color in &mut self.colors { color.adjust(ratio, black); }
    }

    pub fn adjust_toward(&mut self, ratio: i32, destination: &Self) {
        for (color, target) in self.colors.iter_mut().zip(destination.colors.iter()) {
            color.adjust(ratio, *target);
        }
    }

    pub fn partial_adjust_to_black(&mut self, ratio: i32, lookup: &[u8]) -> Result<(), PaletteError> {
        if lookup.len() < COLOR_COUNT {
            return Err(PaletteError::LookupTooShort { supplied: lookup.len(), required: COLOR_COUNT });
        }
        let black = RgbColor::default();
        for (index, color) in self.colors.iter_mut().enumerate() {
            if lookup[index] != 0 { color.adjust(ratio, black); }
        }
        Ok(())
    }

    pub fn partial_adjust_toward(
        &mut self,
        ratio: i32,
        destination: &Self,
        lookup: &[u8],
    ) -> Result<(), PaletteError> {
        if lookup.len() < COLOR_COUNT {
            return Err(PaletteError::LookupTooShort { supplied: lookup.len(), required: COLOR_COUNT });
        }
        for index in 0..COLOR_COUNT {
            if lookup[index] != 0 {
                self.colors[index].adjust(ratio, destination.colors[index]);
            }
        }
        Ok(())
    }

    /// PaletteClass::Closest_Color uses RGBClass::Difference over all entries
    /// and keeps the first index on equal distances.
    pub fn closest_color(&self, color: RgbColor) -> usize {
        let mut closest = 0usize;
        let mut best: Option<i32> = None;
        for (index, entry) in self.colors.iter().enumerate() {
            let difference = color.difference(*entry);
            if best.is_none_or(|value| difference < value) {
                best = Some(difference);
                closest = index;
            }
        }
        closest
    }

    /// PaletteClass::Set(time, callback) blocks until the duration expires.
    /// Intermediate palettes are derived from the original CurrentPalette,
    /// not from the previous intermediate frame. A host callback can pump
    /// messages/audio and advance the underlying system tick.
    pub fn set_with_host(
        &self,
        current_palette: &mut Self,
        duration_ticks: u32,
        host: &mut impl PaletteFadeHost,
        mut callback: Option<&mut dyn FnMut()>,
    ) {
        let original = current_palette.clone();
        let start_tick = host.system_tick();

        if duration_ticks != 0 {
            loop {
                let remaining = duration_ticks.saturating_sub(host.system_tick().wrapping_sub(start_tick));
                if remaining == 0 { break; }

                let elapsed = duration_ticks - remaining;
                let adjust = ((elapsed as u64 * 256) / duration_ticks as u64) as i32;
                let mut intermediate = original.clone();
                intermediate.adjust_toward(adjust, self);

                let bytes = intermediate.to_device_bytes();
                host.set_palette(&bytes);
                *current_palette = intermediate;

                if let Some(callback) = callback.as_deref_mut() { callback(); }

                // Match the source's holdtime loop: do not calculate another
                // palette while the countdown value remains unchanged.
                let holdtime = remaining;
                let mut observed = remaining;
                while observed == holdtime && holdtime != 0 {
                    if let Some(callback) = callback.as_deref_mut() { callback(); }
                    observed = duration_ticks.saturating_sub(host.system_tick().wrapping_sub(start_tick));
                }
            }
        }

        // Final palette is written exactly, regardless of intermediate rounding.
        let bytes = self.to_device_bytes();
        host.set_palette(&bytes);
        *current_palette = self.clone();
    }
}

pub trait PaletteFadeHost {
    /// Monotonic 32-bit system tick count.
    fn system_tick(&mut self) -> u32;
    /// Win32 Set_Palette path; receives the complete raw 6-bit palette.
    fn set_palette(&mut self, palette: &[u8; PALETTE_BYTES]);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaletteError {
    PaletteTooShort { supplied: usize, required: usize },
    LookupTooShort { supplied: usize, required: usize },
}

impl fmt::Display for PaletteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for PaletteError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakePaletteHost {
        tick: u32,
        calls: Vec<[u8; PALETTE_BYTES]>,
        advance_per_query: u32,
    }

    impl PaletteFadeHost for FakePaletteHost {
        fn system_tick(&mut self) -> u32 {
            let value = self.tick;
            self.tick = self.tick.wrapping_add(self.advance_per_query);
            value
        }
        fn set_palette(&mut self, palette: &[u8; PALETTE_BYTES]) {
            self.calls.push(*palette);
        }
    }

    #[test]
    fn rgb_constructor_and_device_components_match_six_bit_storage() {
        let color = RgbColor::new(255, 128, 64);
        assert_eq!(color.device_channels(), [63, 32, 16]);
        assert_eq!((color.red_component(), color.green_component(), color.blue_component()), (255, 130, 65));
        assert_eq!(RgbColor::from_device_channels(63, 32, 16), color);
    }

    #[test]
    fn rgb_adjust_and_difference_use_device_channel_values() {
        let mut color = RgbColor::from_device_channels(63, 0, 10);
        color.adjust(128, RgbColor::default());
        assert_eq!(color.device_channels(), [32, 0, 5]);
        assert_eq!(RgbColor::default().difference(RgbColor::from_device_channels(3, 4, 0)), 25);
        color.adjust(256, RgbColor::default());
        assert_eq!(color.device_channels(), [32, 0, 5]);
    }

    #[test]
    fn rgb_hsv_conversions_keep_legacy_sector_arithmetic() {
        let red = RgbColor::new(255, 0, 0);
        let hsv = red.to_hsv();
        assert_eq!(hsv.value_component(), 255);
        assert_eq!(hsv.saturation_component(), 255);
        let roundtrip = hsv.to_rgb();
        assert_eq!(roundtrip.red_component(), 255);
        assert_eq!(roundtrip.green_component(), 0);
        assert_eq!(roundtrip.blue_component(), 0);

        let gray = RgbColor::new(128, 128, 128).to_hsv();
        assert_eq!(gray.hue_component(), 0);
        assert_eq!(gray.saturation_component(), 0);
        assert_eq!(gray.value_component(), 130);
    }

    #[test]
    fn hsv_adjust_and_difference_are_componentwise_not_circular() {
        let mut from = HsvColor::new(250, 20, 10);
        from.adjust(128, HsvColor::new(10, 40, 30));
        assert_eq!(from, HsvColor::new(130, 30, 20));
        assert_eq!(HsvColor::new(0, 0, 0).difference(HsvColor::new(3, 4, 0)), 25);
    }

    #[test]
    fn palette_serializes_raw_channels_and_supports_color_remapping() {
        let color = RgbColor::new(252, 64, 16);
        let palette = PaletteClass::filled(color);
        let bytes = palette.to_device_bytes();
        assert_eq!(bytes.len(), PALETTE_BYTES);
        assert_eq!(&bytes[..3], &[63, 16, 4]);
        assert_eq!(PaletteClass::from_device_bytes(&bytes).unwrap(), palette);
        assert_eq!(palette.closest_color(color), 0);
    }

    #[test]
    fn palette_adjustments_and_partial_lut_follow_rgb_adjust() {
        let white = RgbColor::new(255, 255, 255);
        let mut source = PaletteClass::filled(white);
        source.adjust_to_black(128);
        assert_eq!(source.color(0).device_channels(), [32, 32, 32]);

        let destination = PaletteClass::filled(RgbColor::new(0, 128, 255));
        source.adjust_toward(255, &destination);
        assert_eq!(source, destination);

        let mut partial = PaletteClass::filled(white);
        let mut lut = [0u8; COLOR_COUNT];
        lut[9] = 1;
        partial.partial_adjust_to_black(255, &lut).unwrap();
        assert_eq!(partial.color(8), white);
        assert_eq!(partial.color(9), RgbColor::default());
    }

    #[test]
    fn rgb_set_updates_current_palette_entry_without_direct_hardware_access() {
        let mut current = PaletteClass::default();
        RgbColor::new(255, 0, 0).set_in_palette(&mut current, 10);
        assert_eq!(current.color(10).red_component(), 255);
        assert_eq!(current.color(9), RgbColor::default());
    }

    #[test]
    fn timed_palette_set_uses_original_palette_and_finishes_exactly() {
        let from = PaletteClass::filled(RgbColor::default());
        let target = PaletteClass::filled(RgbColor::new(255, 255, 255));
        let mut host = FakePaletteHost { advance_per_query: 1, ..FakePaletteHost::default() };
        let mut current = from.clone();
        let mut callbacks = 0usize;
        target.set_with_host(&mut current, 3, &mut host, Some(&mut || callbacks += 1));
        assert_eq!(current, target);
        assert_eq!(host.calls.last().unwrap().as_slice(), target.to_device_bytes().as_slice());
        assert!(!host.calls.is_empty());
        assert!(callbacks > 0);
    }

    #[test]
    fn zero_duration_sets_palette_directly() {
        let target = PaletteClass::filled(RgbColor::new(1, 2, 3));
        let mut current = PaletteClass::default();
        let mut host = FakePaletteHost::default();
        target.set_with_host(&mut current, 0, &mut host, None);
        assert_eq!(current, target);
        assert_eq!(host.calls.len(), 1);
    }
}
