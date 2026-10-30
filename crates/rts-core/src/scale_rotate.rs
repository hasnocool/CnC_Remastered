//! REDALERT/SPRITE.CPP: GraphicBufferClass::Scale_Rotate.
//!
//! This ports the 24.8 scale, 8-bit angle, table-derived corner geometry and
//! the two original integer Bresenham rasterization cases. Nonzero source
//! pixels overwrite the destination. The C++ implementation performs no
//! clipping; this Rust port retains the same screen coordinate arithmetic but
//! safely skips destination writes outside the supplied framebuffer slice.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point2D { pub x: i32, pub y: i32 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BitmapView<'a> {
    pub width: usize,
    pub height: usize,
    pub data: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScaleRotateReport {
    pub source_samples: u64,
    pub destination_writes: u64,
    pub skipped_destination_writes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScaleRotateError {
    BitmapDimensionsOverflow,
    BitmapDataTooShort { required: usize, supplied: usize },
    InvalidBitmapDimensions,
    SourcePixelOutOfBounds { index: i64, data_length: usize },
    RasterWorkLimitExceeded { requested: u64, maximum: u64 },
}

pub const SINE_TABLE: [i32; 256] = [
    0, 6, 12, 18, 25, 31, 37, 43, 49, 56, 62, 68, 74, 80, 86, 92,
    97, 103, 109, 115, 120, 126, 131, 136, 142, 147, 152, 157, 162, 167, 171, 176,
    180, 185, 189, 193, 197, 201, 205, 209, 212, 216, 219, 222, 225, 228, 231, 233,
    236, 238, 240, 243, 244, 246, 248, 249, 251, 252, 253, 254, 254, 255, 255, 255,
    255, 255, 255, 255, 254, 254, 253, 252, 251, 249, 248, 246, 245, 243, 241, 238,
    236, 234, 231, 228, 225, 222, 219, 216, 213, 209, 205, 201, 198, 194, 189, 185,
    181, 176, 172, 167, 162, 157, 152, 147, 142, 137, 131, 126, 120, 115, 109, 104,
    98, 92, 86, 80, 74, 68, 62, 56, 50, 44, 37, 31, 25, 19, 12, 6,
    0, -5, -12, -18, -24, -30, -37, -43, -49, -55, -61, -67, -73, -79, -85, -91,
    -97, -103, -109, -114, -120, -125, -131, -136, -141, -147, -152, -157, -162, -166, -171, -176,
    -180, -185, -189, -193, -197, -201, -205, -208, -212, -215, -219, -222, -225, -228, -231, -233,
    -236, -238, -240, -242, -244, -246, -248, -249, -250, -252, -253, -254, -254, -255, -255, -255,
    -255, -255, -255, -255, -254, -254, -253, -252, -251, -249, -248, -246, -245, -243, -241, -239,
    -236, -234, -231, -228, -226, -223, -219, -216, -213, -209, -206, -202, -198, -194, -190, -185,
    -181, -177, -172, -167, -162, -158, -153, -148, -142, -137, -132, -126, -121, -115, -110, -104,
    -98, -92, -86, -81, -75, -69, -62, -56, -50, -44, -38, -32, -25, -19, -13, -7,
];

pub const COSINE_TABLE: [i32; 256] = [
    256, 255, 255, 255, 254, 254, 253, 252, 251, 249, 248, 246, 244, 243, 241, 238,
    236, 234, 231, 228, 225, 222, 219, 216, 212, 209, 205, 201, 197, 193, 189, 185,
    181, 176, 171, 167, 162, 157, 152, 147, 142, 137, 131, 126, 120, 115, 109, 103,
    98, 92, 86, 80, 74, 68, 62, 56, 50, 43, 37, 31, 25, 19, 12, 6,
    0, -6, -12, -18, -24, -31, -37, -43, -49, -55, -61, -68, -74, -80, -86, -91,
    -97, -103, -109, -114, -120, -125, -131, -136, -141, -147, -152, -157, -162, -166, -171, -176,
    -180, -185, -189, -193, -197, -201, -205, -209, -212, -216, -219, -222, -225, -228, -231, -233,
    -236, -238, -240, -242, -244, -246, -248, -249, -251, -252, -253, -254, -254, -255, -255, -255,
    -255, -255, -255, -255, -254, -254, -253, -252, -251, -249, -248, -246, -245, -243, -241, -239,
    -236, -234, -231, -228, -225, -222, -219, -216, -213, -209, -205, -202, -198, -194, -190, -185,
    -181, -176, -172, -167, -162, -157, -152, -147, -142, -137, -132, -126, -121, -115, -109, -104,
    -98, -92, -86, -80, -74, -68, -62, -56, -50, -44, -38, -31, -25, -19, -13, -6,
    0, 5, 11, 18, 24, 30, 36, 43, 49, 55, 61, 67, 73, 79, 85, 91,
    97, 103, 108, 114, 120, 125, 131, 136, 141, 146, 151, 156, 161, 166, 171, 176,
    180, 184, 189, 193, 197, 201, 205, 208, 212, 215, 219, 222, 225, 228, 231, 233,
    236, 238, 240, 242, 244, 246, 248, 249, 250, 252, 253, 253, 254, 255, 255, 255,
];

fn write_pixel(
    buffer: &mut [u8],
    screen_index: u32,
    pixel: u8,
    writes: &mut u64,
    skipped: &mut u64,
) {
    if pixel == 0 { return; }
    let index = screen_index as usize;
    if let Some(destination) = buffer.get_mut(index) {
        *destination = pixel;
        *writes += 1;
    } else {
        *skipped += 1;
    }
}

fn read_pixel(bitmap: BitmapView<'_>, index: i64) -> Result<u8, ScaleRotateError> {
    if index < 0 {
        return Err(ScaleRotateError::SourcePixelOutOfBounds {
            index, data_length: bitmap.data.len(),
        });
    }
    bitmap.data.get(index as usize).copied().ok_or(
        ScaleRotateError::SourcePixelOutOfBounds { index, data_length: bitmap.data.len() }
    )
}

fn corner_points(bitmap: BitmapView<'_>, center: Point2D, scale: i32, angle: u8)
    -> (Point2D, Point2D, Point2D)
{
    let angle = angle as usize;
    let cosine = COSINE_TABLE[angle];
    let sine = SINE_TABLE[angle];
    let width_half = scale.wrapping_mul(bitmap.width as i32) >> 1;
    let height_half = scale.wrapping_mul(bitmap.height as i32) >> 1;

    let lc = height_half.wrapping_mul(cosine) >> 8;
    let ws = width_half.wrapping_mul(sine) >> 8;
    let ls = height_half.wrapping_mul(sine) >> 8;
    let wc = width_half.wrapping_mul(cosine) >> 8;

    let p0 = Point2D {
        x: center.x.wrapping_add(lc.wrapping_sub(ws) >> 8),
        y: center.y.wrapping_add(ls.wrapping_neg().wrapping_sub(wc) >> 8),
    };
    let p1 = Point2D {
        x: center.x.wrapping_add(lc.wrapping_add(ws) >> 8),
        y: center.y.wrapping_add(ls.wrapping_neg().wrapping_add(wc) >> 8),
    };
    let p2 = Point2D {
        x: center.x.wrapping_add(lc.wrapping_neg().wrapping_sub(ws) >> 8),
        y: center.y.wrapping_add(ls.wrapping_sub(wc) >> 8),
    };
    (p0, p1, p2)
}

/// Exact integer rasterizer from GraphicBufferClass::Scale_Rotate.
/// max_raster_samples=None preserves the original unbounded loop extent;
/// callers handling untrusted input can provide a finite work limit.
pub fn scale_rotate(
    bitmap: BitmapView<'_>,
    destination: &mut [u8],
    destination_width: usize,
    center: Point2D,
    scale: i32,
    angle: u8,
    max_raster_samples: Option<u64>,
) -> Result<ScaleRotateReport, ScaleRotateError> {
    if bitmap.width == 0 || bitmap.height == 0 || destination_width == 0 {
        return Err(ScaleRotateError::InvalidBitmapDimensions);
    }
    let required = bitmap.width.checked_mul(bitmap.height)
        .ok_or(ScaleRotateError::BitmapDimensionsOverflow)?;
    if bitmap.data.len() < required {
        return Err(ScaleRotateError::BitmapDataTooShort {
            required, supplied: bitmap.data.len(),
        });
    }

    let (p0, p1, p2) = corner_points(bitmap, center, scale, angle);
    let front_dx = p1.x.wrapping_sub(p0.x);
    let front_dy = p1.y.wrapping_sub(p0.y);
    let side_dx = p2.x.wrapping_sub(p0.x);
    let side_dy = p2.y.wrapping_sub(p0.y);

    let mut front_x_step = 1i32;
    let mut front_y_step = destination_width as i32;
    let mut side_x_step = 1i32;
    let mut side_y_step = destination_width as i32;

    let front_dx = front_dx.wrapping_abs() as u64;
    let mut front_dy = front_dy.wrapping_abs() as u64;
    let mut side_dx = side_dx.wrapping_abs() as u64;
    let side_dy = side_dy.wrapping_abs() as u64;

    // The source sets negative direction steps when the edge vector points
    // toward decreasing screen coordinates.
    if p1.y < p0.y { front_y_step = -front_y_step; }
    if p1.x < p0.x { front_x_step = -front_x_step; }
    if p2.y < p0.y { side_y_step = -side_y_step; }
    if p2.x < p0.x { side_x_step = -side_x_step; }

    let start = (p0.x as u32).wrapping_add(
        (destination_width as u32).wrapping_mul(p0.y as u32)
    );
    let mut screen_position = start;
    let mut source_position = 0i64;
    let mut source_x_error = 0u64;
    let mut source_y_error = 0u64;
    let mut front_error = 0u64;
    let mut side_error = 0u64;
    let mut source_samples = 0u64;
    let mut destination_writes = 0u64;
    let mut skipped_destination_writes = 0u64;

    let (outer_steps, inner_steps) = if front_dx > front_dy {
        (side_dy, front_dx)
    } else {
        (side_dx, front_dy)
    };
    let requested = outer_steps.saturating_mul(inner_steps);
    if let Some(maximum) = max_raster_samples {
        if requested > maximum {
            return Err(ScaleRotateError::RasterWorkLimitExceeded { requested, maximum });
        }
    }

    if front_dx > front_dy {
        // Case 1: advance the front edge primarily in X, side edge in Y.
        for _ in 0..side_dy {
            let row_start = screen_position;
            for _ in 0..front_dx {
                let pixel = read_pixel(bitmap, source_position)?;
                source_samples += 1;
                write_pixel(destination, screen_position, pixel,
                    &mut destination_writes, &mut skipped_destination_writes);

                source_x_error = source_x_error.saturating_add(bitmap.width as u64);
                while source_x_error > front_dx {
                    source_position += 1;
                    source_x_error -= front_dx;
                }

                screen_position = screen_position.wrapping_add(front_x_step as u32);
                front_error += front_dy;
                if front_error > front_dx {
                    write_pixel(destination, screen_position, pixel,
                        &mut destination_writes, &mut skipped_destination_writes);
                    front_error -= front_dx;
                    screen_position = screen_position.wrapping_add(front_y_step as u32);
                }
            }

            source_x_error = 0;
            source_position -= bitmap.width as i64 - 1;
            source_y_error += bitmap.height as u64;
            while source_y_error > side_dy {
                source_position += bitmap.width as i64;
                source_y_error -= side_dy;
            }

            front_error = 0;
            screen_position = row_start;
            screen_position = screen_position.wrapping_add(side_y_step as u32);
            side_error += side_dx;
            if side_error > side_dy {
                side_error -= side_dy;
                screen_position = screen_position.wrapping_add(side_x_step as u32);
            }
        }
    } else {
        // Case 2: advance the front edge primarily in Y, side edge in X.
        for _ in 0..side_dx {
            let column_start = screen_position;
            for _ in 0..front_dy {
                let pixel = read_pixel(bitmap, source_position)?;
                source_samples += 1;
                write_pixel(destination, screen_position, pixel,
                    &mut destination_writes, &mut skipped_destination_writes);

                source_x_error = source_x_error.saturating_add(bitmap.width as u64);
                while source_x_error > front_dy {
                    source_position += 1;
                    source_x_error -= front_dy;
                }

                screen_position = screen_position.wrapping_add(front_y_step as u32);
                front_error += front_dx;
                if front_error > front_dy {
                    write_pixel(destination, screen_position, pixel,
                        &mut destination_writes, &mut skipped_destination_writes);
                    front_error -= front_dy;
                    screen_position = screen_position.wrapping_add(front_x_step as u32);
                }
            }

            source_x_error = 0;
            source_position -= bitmap.width as i64 - 1;
            source_y_error += bitmap.height as u64;
            while source_y_error > side_dx {
                source_position += bitmap.width as i64;
                source_y_error -= side_dx;
            }

            screen_position = column_start;
            screen_position = screen_position.wrapping_add(side_x_step as u32);
            side_error += side_dy;
            front_error = 0;
            if side_error > side_dx {
                side_error -= side_dx;
                screen_position = screen_position.wrapping_add(side_y_step as u32);
            }
        }
    }

    Ok(ScaleRotateReport {
        source_samples,
        destination_writes,
        skipped_destination_writes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_tables_retain_legacy_values() {
        assert_eq!(SINE_TABLE[0], 0);
        assert_eq!(SINE_TABLE[64], 255);
        assert_eq!(SINE_TABLE[128], 0);
        assert_eq!(SINE_TABLE[192], -255);
        assert_eq!(COSINE_TABLE[0], 256);
        assert_eq!(COSINE_TABLE[64], 0);
        assert_eq!(COSINE_TABLE[128], -256);
    }

    #[test]
    fn dimensions_and_bitmap_source_are_checked() {
        let bitmap = BitmapView { width: 2, height: 2, data: &[1, 2, 3] };
        assert_eq!(
            scale_rotate(bitmap, &mut [0; 64], 8, Point2D { x: 4, y: 4 }, 256, 0, None),
            Err(ScaleRotateError::BitmapDataTooShort { required: 4, supplied: 3 })
        );
    }

    #[test]
    fn rasterizer_preserves_transparency_and_bounds_destination_writes() {
        let bitmap = BitmapView { width: 2, height: 2, data: &[0, 2, 3, 4] };
        let mut destination = [0u8; 64];
        let report = scale_rotate(
            bitmap, &mut destination, 8, Point2D { x: 4, y: 4 }, 256, 0, Some(100)
        ).unwrap();
        assert!(report.source_samples > 0);
        assert!(report.destination_writes > 0);
        assert!(destination.iter().any(|pixel| *pixel == 2 || *pixel == 3 || *pixel == 4));
        assert_eq!(destination[0], 0);
    }

    #[test]
    fn raster_work_limit_prevents_hostile_dimensions_from_consuming_unbounded_cpu() {
        let bitmap = BitmapView { width: 16, height: 16, data: &[1; 256] };
        assert!(matches!(
            scale_rotate(bitmap, &mut [0; 8], 8, Point2D { x: 0, y: 0 }, 1_000_000, 0, Some(100)),
            Err(ScaleRotateError::RasterWorkLimitExceeded { .. })
        ));
    }
}
