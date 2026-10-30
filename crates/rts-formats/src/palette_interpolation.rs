//! Palette interpolation support from both variants' INTERPAL.CPP.
//!
//! Red Alert actively builds the 256x256 midpoint-color remap table. The
//! corresponding Tiberian Dawn creation loop is guarded by #if (0), but
//! still clears InterpolationPaletteChanged. Interpolate_2X_Scale is a no-op
//! in both pinned Win32 builds because its implementation is under #if (0).

use std::fmt;

pub const PALETTE_COLOR_COUNT: usize = 256;
pub const PALETTE_RGB_BYTES: usize = PALETTE_COLOR_COUNT * 3;
pub const INTERPOLATION_TABLE_BYTES: usize = PALETTE_COLOR_COUNT * PALETTE_COLOR_COUNT;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterpolationVariant {
    RedAlert,
    TiberianDawn,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InterpolationError {
    PaletteTooShort { supplied: usize, required: usize },
    TableTooShort { supplied: usize, required: usize },
}

impl fmt::Display for InterpolationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for InterpolationError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaletteInterpolation {
    pub table: Box<[u8; INTERPOLATION_TABLE_BYTES]>,
    pub interpolation_palette: Option<Box<[u8; PALETTE_RGB_BYTES]>>,
    pub changed: bool,
    pub copy_type: i32,
}

impl Default for PaletteInterpolation {
    fn default() -> Self {
        Self {
            table: Box::new([0; INTERPOLATION_TABLE_BYTES]),
            interpolation_palette: None,
            changed: true,
            copy_type: 0,
        }
    }
}

impl PaletteInterpolation {
    pub fn new() -> Self { Self::default() }

    pub fn table(&self) -> &[u8; INTERPOLATION_TABLE_BYTES] { &self.table }
    pub fn table_mut(&mut self) -> &mut [u8; INTERPOLATION_TABLE_BYTES] { &mut self.table }

    pub fn set_interpolation_palette(&mut self, palette: &[u8]) -> Result<(), InterpolationError> {
        if palette.len() < PALETTE_RGB_BYTES {
            return Err(InterpolationError::PaletteTooShort {
                supplied: palette.len(),
                required: PALETTE_RGB_BYTES,
            });
        }
        let mut copy = Box::new([0; PALETTE_RGB_BYTES]);
        copy.copy_from_slice(&palette[..PALETTE_RGB_BYTES]);
        self.interpolation_palette = Some(copy);
        self.changed = true;
        Ok(())
    }

    /// Read_Interpolation_Palette: if the named resource exists, copy its
    /// 65,536 bytes and clear the changed flag. A missing file leaves state
    /// untouched, just as the source does.
    pub fn read_interpolation_palette(&mut self, file_data: Option<&[u8]>) -> Result<bool, InterpolationError> {
        let Some(data) = file_data else { return Ok(false) };
        if data.len() < INTERPOLATION_TABLE_BYTES {
            return Err(InterpolationError::TableTooShort {
                supplied: data.len(),
                required: INTERPOLATION_TABLE_BYTES,
            });
        }
        self.table.copy_from_slice(&data[..INTERPOLATION_TABLE_BYTES]);
        self.changed = false;
        Ok(true)
    }

    /// Write_Interpolation_Palette writes only when the named output resource
    /// does not already exist. Some(existing_bytes) means no write is made.
    pub fn write_interpolation_palette(&self, existing_file: Option<&[u8]>) -> Option<Vec<u8>> {
        existing_file.is_none().then(|| self.table.to_vec())
    }

    /// Create_Palette_Interpolation_Table. Red Alert computes each midpoint
    /// and performs the original exhaustive 256-color RGB least-distance
    /// search. Tiberian Dawn compiles the table-generation loop out and only
    /// resets the global changed flag.
    pub fn create_palette_interpolation_table(
        &mut self,
        variant: InterpolationVariant,
    ) -> Result<bool, InterpolationError> {
        if variant == InterpolationVariant::RedAlert {
            let palette = self.interpolation_palette.as_ref()
                .ok_or(InterpolationError::PaletteTooShort { supplied: 0, required: PALETTE_RGB_BYTES })?;

            for first in 0..PALETTE_COLOR_COUNT {
                let first_at = first * 3;
                let first_rgb = [palette[first_at], palette[first_at + 1], palette[first_at + 2]];

                for second in 0..PALETTE_COLOR_COUNT {
                    let second_at = second * 3;
                    let middle = [
                        (first_rgb[0] as u16 + palette[second_at] as u16) >> 1,
                        (first_rgb[1] as u16 + palette[second_at + 1] as u16) >> 1,
                        (first_rgb[2] as u16 + palette[second_at + 2] as u16) >> 1,
                    ];

                    let mut closest_distance = 500_000i32;
                    let mut closest_index = 0usize;
                    for candidate in 0..PALETTE_COLOR_COUNT {
                        let at = candidate * 3;
                        let dr = palette[at] as i32 - middle[0] as i32;
                        let dg = palette[at + 1] as i32 - middle[1] as i32;
                        let db = palette[at + 2] as i32 - middle[2] as i32;
                        let distance = dr * dr + dg * dg + db * db;
                        if distance < closest_distance {
                            closest_distance = distance;
                            closest_index = candidate;
                        }
                    }
                    self.table[first * PALETTE_COLOR_COUNT + second] = closest_index as u8;
                }
            }
        }
        self.changed = false;
        Ok(variant == InterpolationVariant::RedAlert)
    }

    /// Increase_Palette_Luminance modifies all 256 RGB entries in place.
    /// The original uses 32-bit unsigned intermediate arithmetic and then
    /// MIN(cap, (int)value) before truncating back to one byte.
    pub fn increase_palette_luminance(
        palette: &mut [u8],
        red_percentage: i32,
        green_percentage: i32,
        blue_percentage: i32,
        cap: i32,
    ) -> Result<(), InterpolationError> {
        if palette.len() < PALETTE_RGB_BYTES {
            return Err(InterpolationError::PaletteTooShort {
                supplied: palette.len(),
                required: PALETTE_RGB_BYTES,
            });
        }
        let percentages = [red_percentage, green_percentage, blue_percentage];
        for index in 0..PALETTE_COLOR_COUNT {
            let at = index * 3;
            for channel in 0..3 {
                let value = palette[at + channel] as u32;
                let increased = value.wrapping_add(
                    value.wrapping_mul(percentages[channel] as u32) / 100,
                );
                let clamped = cap.min(increased as i32);
                palette[at + channel] = clamped as u8;
            }
        }
        Ok(())
    }

    /// Interpolate_2X_Scale is deliberately empty in both variants in the
    /// pinned sources (the rendering implementation is fully under #if (0)).
    pub fn interpolate_2x_scale(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gray_palette() -> Vec<u8> {
        let mut palette = vec![0u8; PALETTE_RGB_BYTES];
        for index in 0..256 {
            palette[index * 3] = index as u8;
            palette[index * 3 + 1] = index as u8;
            palette[index * 3 + 2] = index as u8;
        }
        palette
    }

    #[test]
    fn red_alert_builds_midpoint_table_using_nearest_palette_color() {
        let mut interpolation = PaletteInterpolation::new();
        interpolation.set_interpolation_palette(&gray_palette()).unwrap();
        assert!(interpolation.changed);
        assert!(interpolation.create_palette_interpolation_table(InterpolationVariant::RedAlert).unwrap());
        assert!(!interpolation.changed);
        assert_eq!(interpolation.table[0], 0);
        assert_eq!(interpolation.table[1], 1);
        assert_eq!(interpolation.table[255], 128);
        assert_eq!(interpolation.table[255 * 256 + 255], 255);
    }

    #[test]
    fn tiberian_dawn_keeps_existing_table_and_only_clears_changed_flag() {
        let mut interpolation = PaletteInterpolation::new();
        interpolation.table[7 * 256 + 9] = 123;
        interpolation.set_interpolation_palette(&gray_palette()).unwrap();
        assert!(!interpolation.create_palette_interpolation_table(InterpolationVariant::TiberianDawn).unwrap());
        assert!(!interpolation.changed);
        assert_eq!(interpolation.table[7 * 256 + 9], 123);
    }

    #[test]
    fn read_and_write_follow_file_existence_rules() {
        let mut interpolation = PaletteInterpolation::new();
        let bytes = vec![0x5a; INTERPOLATION_TABLE_BYTES];
        assert!(interpolation.read_interpolation_palette(Some(&bytes)).unwrap());
        assert!(!interpolation.changed);
        assert_eq!(interpolation.table[0], 0x5a);
        assert!(interpolation.write_interpolation_palette(None).is_some());
        assert_eq!(interpolation.write_interpolation_palette(Some(&bytes)), None);
        assert!(!interpolation.read_interpolation_palette(None).unwrap());
    }

    #[test]
    fn luminance_increase_scales_and_caps_each_rgb_channel() {
        let mut palette = vec![100u8; PALETTE_RGB_BYTES];
        PaletteInterpolation::increase_palette_luminance(&mut palette, 50, 100, 300, 200).unwrap();
        assert_eq!(&palette[..3], &[150, 200, 255]);
        assert!(PaletteInterpolation::increase_palette_luminance(&mut [0; 12], 20, 20, 20, 63).is_ok());
    }

    #[test]
    fn invalid_palette_and_table_lengths_are_rejected() {
        let mut interpolation = PaletteInterpolation::new();
        assert!(matches!(
            interpolation.set_interpolation_palette(&[0; 10]),
            Err(InterpolationError::PaletteTooShort { .. })
        ));
        assert!(matches!(
            interpolation.read_interpolation_palette(Some(&[0; 100])),
            Err(InterpolationError::TableTooShort { .. })
        ));
    }
}
