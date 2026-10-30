//! IBM 256-colour palette operations from Tiberian Dawn PALETTE.CPP and
//! MORPHPAL.CPP. Palette channels are stored as 768 consecutive DAC values.

pub const PALETTE_BYTES: usize = 768;
pub type Palette = [u8; PALETTE_BYTES];

/// Copy a complete palette, matching the observable IBM Set_Palette path.
#[inline]
pub fn set_palette(source: &Palette, current: &mut Palette) {
    current.copy_from_slice(source);
}

/// Set one RGB triplet and publish the resulting palette.
pub fn set_palette_color(
    palette: Option<&mut Palette>,
    color: i32,
    rgb: [u8; 3],
    current: &mut Palette,
) -> bool {
    let Some(palette) = palette else { return false };
    if !(0..256).contains(&color) {
        return false;
    }
    let start = color as usize * 3;
    palette[start..start + 3].copy_from_slice(&rgb);
    current.copy_from_slice(palette);
    true
}

/// Determine the legacy per-frame time quantum and channel step.
pub fn determine_bump_rate(target: &Palette, current: &Palette, delay: u32) -> (i16, u16) {
    let diff = target.iter().zip(current.iter())
        .map(|(&a, &b)| (a as i32 - b as i32).unsigned_abs())
        .max().unwrap_or(0);
    let mut ticks = if diff == 0 { 0 } else {
        (((delay as u64) << 8) / diff as u64).min(0x7fff) as i32
    };
    let base = ticks;
    let mut rate = 1u32;
    while rate <= diff && ticks < 256 {
        ticks += base;
        rate += 1;
    }
    (ticks as i16, rate as u16)
}

/// Advance every differing channel by at most step, without overshooting.
pub fn bump_palette(target: &Palette, current: &mut Palette, step: u16) -> bool {
    let mut changed = false;
    for i in 0..PALETTE_BYTES {
        let from = current[i] as i32;
        let to = target[i] as i32;
        if from == to { continue; }
        changed = true;
        let amount = step as i32;
        current[i] = if from < to {
            (from + amount).min(to) as u8
        } else {
            (from - amount).max(to) as u8
        };
    }
    changed
}

/// Produce successive palette states for Fade_Palette_To.
/// The caller controls timing and callbacks; each frame is one Bump_Palette.
pub fn fade_palette_frames(target: &Palette, start: &Palette, delay: u32) -> Vec<Palette> {
    let (_ticks_per_step, step) = determine_bump_rate(target, start, delay);
    let mut current = *start;
    let mut frames = Vec::new();
    while bump_palette(target, &mut current, step) {
        frames.push(current);
    }
    frames
}

/// One MORPHPAL Palette_To_Palette update using the legacy signed-byte
/// channel differences and 8-bit fixed-point SCALE arithmetic.
pub fn morph_palette_frame(
    source: &Palette,
    destination: &Palette,
    current_time: u32,
    delay: u32,
) -> (Palette, bool) {
    let mut output = [0u8; PALETTE_BYTES];
    let mut change = false;
    for i in 0..PALETTE_BYTES {
        if current_time < delay {
            let d = (destination[i] & 63) as i8;
            let s = (source[i] & 63) as i8;
            let diff = d.wrapping_sub(s) as i32;
            if diff != 0 { change = true; }
            let scaled = if delay == 0 { 0 } else {
                ((((diff.wrapping_shl(8) / delay as i32) as u32)
                    .wrapping_mul(current_time)) >> 8) as i32
            };
            output[i] = ((s as i32 + scaled) as i8) as u8;
        } else {
            output[i] = destination[i] & 63;
            change = false;
        }
    }
    (output, change)
}
