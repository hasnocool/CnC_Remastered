//! Exact REDALERT/FACE.CPP desired-facing functions and FACE.H DirType operators.

pub type DirType = u8;

pub const DIR_MIN: DirType = 0;
pub const DIR_N: DirType = 0;
pub const DIR_NE: DirType = 1 << 5;
pub const DIR_E: DirType = 2 << 5;
pub const DIR_SE: DirType = 3 << 5;
pub const DIR_S: DirType = 4 << 5;
pub const DIR_SW: DirType = 5 << 5;
pub const DIR_W: DirType = 6 << 5;
pub const DIR_NW: DirType = 7 << 5;
pub const DIR_MAX: DirType = 255;

pub const fn direction_add(left: DirType, right: DirType) -> DirType {
    left.wrapping_add(right)
}

pub const fn direction_add_int(left: DirType, right: i32) -> DirType {
    left.wrapping_add(right as u8)
}

pub const fn direction_sub(left: DirType, right: DirType) -> DirType {
    left.wrapping_sub(right)
}

pub const fn direction_sub_int(left: DirType, right: i32) -> DirType {
    left.wrapping_sub(right as u8)
}

/// Exact Desired_Facing8 from REDALERT/FACE.CPP. North is zero and angles
/// increase clockwise in 32-unit octant steps. The source's coincident-point
/// result is retained (0x20).
pub fn desired_facing8(x1: i32, y1: i32, x2: i32, y2: i32) -> DirType {
    let mut index = 0u32;

    let dx_signed = x2.wrapping_sub(x1);
    let mut dx = dx_signed.unsigned_abs();
    if dx_signed < 0 { index |= 0x00c0; }

    let dy_signed = y1.wrapping_sub(y2);
    let mut dy = dy_signed.unsigned_abs();
    if dy_signed < 0 {
        index ^= 0x0040;
    }

    let (bigger, smaller) = if dx < dy { (dy, dx) } else { (dx, dy) };

    if (bigger.wrapping_add(1) / 2) <= smaller {
        index += 0x0020;
        return index as u8;
    }

    let mut adder = index & 0x0040;
    if dx == bigger { adder ^= 0x0040; }
    index += adder;
    index as u8
}

/// Exact Desired_Facing256 from REDALERT/FACE.CPP. The source returns 0xff
/// when source and destination coordinates are identical.
pub fn desired_facing256(src_x: i32, src_y: i32, dst_x: i32, dst_y: i32) -> DirType {
    let mut composite = 0u32;

    let dx_signed = dst_x.wrapping_sub(src_x);
    let dx = dx_signed.unsigned_abs();
    if dx_signed < 0 { composite |= 0x00c0; }

    let dy_signed = src_y.wrapping_sub(dst_y);
    let dy = dy_signed.unsigned_abs();
    if dy_signed < 0 { composite ^= 0x0040; }

    if dx == 0 && dy == 0 { return 0xff; }

    let (bigger, smaller) = if dx < dy { (dy, dx) } else { (dx, dy) };
    // C++ computes smaller * 32U in a 32-bit unsigned value.
    let mut fraction = smaller.wrapping_mul(32) / bigger;
    let mut adder = composite & 0x0040;
    if dx > dy { adder ^= 0x0040; }
    if adder != 0 {
        fraction = adder.wrapping_sub(fraction).wrapping_sub(1);
    }
    composite = composite.wrapping_add(fraction);
    (composite & 0x00ff) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_constants_and_wrapping_operators_match_face_header() {
        assert_eq!([DIR_N, DIR_NE, DIR_E, DIR_SE, DIR_S, DIR_SW, DIR_W, DIR_NW],
            [0, 32, 64, 96, 128, 160, 192, 224]);
        assert_eq!(direction_add(250, 10), 4);
        assert_eq!(direction_add_int(250, 10), 4);
        assert_eq!(direction_sub(2, 10), 248);
        assert_eq!(direction_sub_int(2, 10), 248);
    }

    #[test]
    fn desired_facing8_preserves_cardinal_diagonal_and_coincident_cases() {
        assert_eq!(desired_facing8(0, 0, 0, -100), DIR_N);
        assert_eq!(desired_facing8(0, 0, 100, 0), DIR_E);
        assert_eq!(desired_facing8(0, 0, 0, 100), DIR_S);
        assert_eq!(desired_facing8(0, 0, -100, 0), DIR_W);
        assert_eq!(desired_facing8(0, 0, 100, -100), DIR_NE);
        assert_eq!(desired_facing8(0, 0, 0, 0), DIR_NE);
    }

    #[test]
    fn desired_facing256_preserves_quadrants_and_identical_point_sentinel() {
        assert_eq!(desired_facing256(0, 0, 0, -100), 0);
        assert_eq!(desired_facing256(0, 0, 100, 0), 64);
        assert_eq!(desired_facing256(0, 0, 0, 100), 128);
        assert_eq!(desired_facing256(0, 0, -100, 0), 192);
        assert_eq!(desired_facing256(0, 0, 100, -100), 32);
        assert_eq!(desired_facing256(10, 10, 10, 10), 0xff);
    }
}
