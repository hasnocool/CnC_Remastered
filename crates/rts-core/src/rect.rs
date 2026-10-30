//! Rectangle geometry ported from REDALERT/RECT.H and RECT.CPP.

/// Westwood rectangle with the same origin-plus-size representation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }

    /// Port of Rect::Is_Valid: width and height must both be positive.
    pub const fn is_valid(self) -> bool {
        self.width > 0 && self.height > 0
    }

    /// Port of Rect::Size using 32-bit wrapping arithmetic.
    pub const fn size(self) -> i32 {
        self.width.wrapping_mul(self.height)
    }

    /// Port of Rect::Intersect.
    ///
    /// Returns the zero-sized invalid rectangle when either input is invalid
    /// or the intersection is empty. Optional anchor coordinates are adjusted
    /// only when an intersection is successfully produced, matching the C++
    /// pointer parameters.
    pub fn intersect(
        self,
        rectangle: Self,
        anchor_x: Option<&mut i32>,
        anchor_y: Option<&mut i32>,
    ) -> Self {
        let invalid = Self::default();
        if !self.is_valid() || !rectangle.is_valid() {
            return invalid;
        }

        let mut result = rectangle;

        // Left clipping.
        if result.x < self.x {
            result.width = result.width.wrapping_sub(self.x.wrapping_sub(result.x));
            result.x = self.x;
        }
        if result.width < 1 {
            return invalid;
        }

        // Top clipping.
        if result.y < self.y {
            result.height = result.height.wrapping_sub(self.y.wrapping_sub(result.y));
            result.y = self.y;
        }
        if result.height < 1 {
            return invalid;
        }

        // Right clipping. Preserve the source's strict edge comparisons.
        let result_right = result.x.wrapping_add(result.width);
        let self_right = self.x.wrapping_add(self.width);
        if result_right > self_right {
            result.width = result.width.wrapping_sub(result_right.wrapping_sub(self_right));
        }
        if result.width < 1 {
            return invalid;
        }

        // Bottom clipping.
        let result_bottom = result.y.wrapping_add(result.height);
        let self_bottom = self.y.wrapping_add(self.height);
        if result_bottom > self_bottom {
            result.height = result.height.wrapping_sub(result_bottom.wrapping_sub(self_bottom));
        }
        if result.height < 1 {
            return invalid;
        }

        if let Some(x) = anchor_x {
            *x = x.wrapping_sub(result.x.wrapping_sub(self.x));
        }
        if let Some(y) = anchor_y {
            *y = y.wrapping_sub(result.y.wrapping_sub(self.y));
        }

        result
    }

    /// Standard containment check using the same right/bottom-exclusive edges
    /// used by the source intersection implementation.
    pub fn contains(self, x: i32, y: i32) -> bool {
        self.is_valid()
            && x >= self.x
            && y >= self.y
            && x < self.x.wrapping_add(self.width)
            && y < self.y.wrapping_add(self.height)
    }
}

/// Port of the free Union(rect1, rect2) function.
///
/// Note: the original implementation adds one to width/height only in the
/// branch that extends the right/bottom edge. This is unusual for half-open
/// rectangles but is preserved as observable source behavior.
pub fn union(rect1: Rect, rect2: Rect) -> Rect {
    if rect1.is_valid() {
        if rect2.is_valid() {
            let mut result = rect1;

            if result.x > rect2.x {
                result.width = result.width.wrapping_add(result.x.wrapping_sub(rect2.x));
                result.x = rect2.x;
            }
            if result.y > rect2.y {
                result.height = result.height.wrapping_add(result.y.wrapping_sub(rect2.y));
                result.y = rect2.y;
            }

            let result_right = result.x.wrapping_add(result.width);
            let rect2_right = rect2.x.wrapping_add(rect2.width);
            if result_right < rect2_right {
                result.width = rect2_right.wrapping_sub(result.x).wrapping_add(1);
            }

            let result_bottom = result.y.wrapping_add(result.height);
            let rect2_bottom = rect2.y.wrapping_add(rect2.height);
            if result_bottom < rect2_bottom {
                result.height = rect2_bottom.wrapping_sub(result.y).wrapping_add(1);
            }

            return result;
        }
        return rect1;
    }
    rect2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validity_and_area_match_source() {
        assert!(Rect::new(1, 2, 3, 4).is_valid());
        assert!(!Rect::new(1, 2, 0, 4).is_valid());
        assert!(!Rect::new(1, 2, 3, -1).is_valid());
        assert_eq!(Rect::new(0, 0, 7, 9).size(), 63);
    }

    #[test]
    fn intersection_clips_all_four_edges() {
        let bounds = Rect::new(10, 20, 100, 80);
        let candidate = Rect::new(0, 5, 150, 120);
        let mut x = 40;
        let mut y = 50;

        let result = bounds.intersect(candidate, Some(&mut x), Some(&mut y));

        assert_eq!(result, Rect::new(10, 20, 100, 80));
        assert_eq!(x, 30);
        assert_eq!(y, 35);
    }

    #[test]
    fn intersection_returns_invalid_on_disjoint_rectangles() {
        let bounds = Rect::new(0, 0, 10, 10);
        let disjoint = Rect::new(10, 0, 5, 5);
        assert_eq!(bounds.intersect(disjoint, None, None), Rect::default());
    }

    #[test]
    fn failed_intersection_does_not_adjust_anchor() {
        let bounds = Rect::new(0, 0, 10, 10);
        let disjoint = Rect::new(50, 50, 2, 2);
        let mut x = 9;
        let mut y = 11;
        assert_eq!(
            bounds.intersect(disjoint, Some(&mut x), Some(&mut y)),
            Rect::default()
        );
        assert_eq!((x, y), (9, 11));
    }

    #[test]
    fn union_preserves_original_extension_plus_one_rule() {
        let result = union(Rect::new(0, 0, 10, 10), Rect::new(5, 5, 10, 10));
        assert_eq!(result, Rect::new(0, 0, 16, 16));
    }

    #[test]
    fn union_returns_valid_operand_when_other_is_invalid() {
        let a = Rect::new(1, 2, 3, 4);
        assert_eq!(union(a, Rect::default()), a);
        assert_eq!(union(Rect::default(), a), a);
    }
}
