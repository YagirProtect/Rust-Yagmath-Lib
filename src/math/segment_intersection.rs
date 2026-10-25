use super::{Math, remap::interpolate};
use crate::extensions::to_f64::t_to_f64::ToF64;
use crate::vector::Vector2;

/// The nonempty intersection of two closed 2D line segments.
/// Coordinates use f64; equality compares stored values exactly.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SegmentIntersection2 {
    /// One common point, including an endpoint touch or a zero-length segment.
    Point(Vector2<f64>),
    /// A common nonzero-length segment. Endpoints follow the first input
    /// segment's direction; reversing that input reverses these endpoints.
    Overlap {
        /// First overlap endpoint along the first input segment.
        start: Vector2<f64>,
        /// Last overlap endpoint along the first input segment.
        end: Vector2<f64>,
    },
}

impl Math {
    /// Intersects closed 2D segments a_start..a_end and b_start..b_end.
    /// Returns Point for a crossing/touch, Overlap for a collinear shared segment,
    /// or None if disjoint. Zero-length segments are treated as points.
    /// Each segment may use a different ToF64 scalar type.
    ///
    /// Non-finite inputs or scaling that loses a nonzero coordinate return None.
    /// Coordinates convert to f64; wide integer precision can be lost. Comparisons
    /// use no tolerance: nearly parallel/collinear cases remain subject to floating
    /// rounding and determinant underflow. This is not an exact geometry predicate.
    /// Scaling each axis by a power of two reduces intermediate overflow.
    ///
    /// ```
    /// use yagmath::{Math, SegmentIntersection2, vector::Vector2};
    /// let hit = Math::segment_intersection(
    ///     Vector2::new(0, 0), Vector2::new(4, 4),
    ///     Vector2::new(0, 4), Vector2::new(4, 0),
    /// );
    /// assert_eq!(hit, Some(SegmentIntersection2::Point(Vector2::new(2.0, 2.0))));
    /// ```
    pub fn segment_intersection<A: ToF64, B: ToF64>(
        a_start: Vector2<A>,
        a_end: Vector2<A>,
        b_start: Vector2<B>,
        b_end: Vector2<B>,
    ) -> Option<SegmentIntersection2> {
        let original = [
            convert(a_start),
            convert(a_end),
            convert(b_start),
            convert(b_end),
        ];
        if original
            .iter()
            .any(|p| !p.x.is_finite() || !p.y.is_finite())
        {
            return None;
        }
        // Cheap rejection also prevents unrelated points from becoming coincident
        // through a later, lossy float operation.
        let [a, b, c, d] = original;
        if a.x.min(b.x) > c.x.max(d.x)
            || c.x.min(d.x) > a.x.max(b.x)
            || a.y.min(b.y) > c.y.max(d.y)
            || c.y.min(d.y) > a.y.max(b.y)
        {
            return None;
        }
        let x_scale = power_of_two_scale(original.iter().fold(0.0_f64, |m, p| m.max(p.x.abs())));
        let y_scale = power_of_two_scale(original.iter().fold(0.0_f64, |m, p| m.max(p.y.abs())));
        let scaled = original.map(|p| Vector2::new(p.x / x_scale, p.y / y_scale));
        if original
            .iter()
            .zip(&scaled)
            .any(|(p, q)| (p.x != 0.0 && q.x == 0.0) || (p.y != 0.0 && q.y == 0.0))
        {
            return None;
        }
        let [a, b, c, d] = scaled;
        if original[0] == original[1] {
            return on_segment(a, c, d).then_some(SegmentIntersection2::Point(original[0]));
        }
        if original[2] == original[3] {
            return on_segment(c, a, b).then_some(SegmentIntersection2::Point(original[2]));
        }
        let r = difference(b, a);
        let s = difference(d, c);
        let q = difference(c, a);
        let denominator = r.cross(s);
        if denominator == 0.0 {
            if q.cross(r) != 0.0 || q.cross(s) != 0.0 {
                return None;
            }
            return collinear_overlap(original, r.x.abs() >= r.y.abs());
        }
        // a + t*r = c + u*s. Check the numerators against the signed
        // denominator before division, avoiding overflowing off-segment ratios.
        let t_numerator = q.cross(s);
        let u_numerator = q.cross(r);
        if !in_unit_interval(t_numerator, denominator)
            || !in_unit_interval(u_numerator, denominator)
        {
            return None;
        }
        // Preserve original endpoints exactly when the intersection lies on one.
        let point = if t_numerator == 0.0 {
            original[0]
        } else if t_numerator == denominator {
            original[1]
        } else if u_numerator == 0.0 {
            original[2]
        } else if u_numerator == denominator {
            original[3]
        } else {
            let t = t_numerator / denominator;
            Vector2::new(
                interpolate(original[0].x, original[1].x, t),
                interpolate(original[0].y, original[1].y, t),
            )
        };
        Some(SegmentIntersection2::Point(point))
    }
}

/// Converts one endpoint through ToF64; validation follows after all conversions.
fn convert<T: ToF64>(p: Vector2<T>) -> Vector2<f64> {
    Vector2::new(p.x.to_f64(), p.y.to_f64())
}

/// Returns b-a for already scaled coordinates, where subtraction cannot overflow.
fn difference(b: Vector2<f64>, a: Vector2<f64>) -> Vector2<f64> {
    Vector2::new(b.x - a.x, b.y - a.y)
}

/// Picks the largest power of two <= a positive finite maximum, or 1 for zero.
/// Power-of-two scaling avoids introducing general division rounding for ordinary
/// values. Subnormal maxima use their highest set fraction bit as the scale.
fn power_of_two_scale(maximum: f64) -> f64 {
    if maximum == 0.0 {
        return 1.0;
    }
    let bits = maximum.to_bits();
    let exponent = bits & 0x7ff0_0000_0000_0000;
    if exponent != 0 {
        f64::from_bits(exponent)
    } else {
        f64::from_bits(1_u64 << (63 - bits.leading_zeros()))
    }
}

/// Tests a point against a closed segment using scaled coordinates and exact
/// float comparisons. Also handles a segment whose two endpoints are equal.
fn on_segment(p: Vector2<f64>, a: Vector2<f64>, b: Vector2<f64>) -> bool {
    p.x >= a.x.min(b.x)
        && p.x <= a.x.max(b.x)
        && p.y >= a.y.min(b.y)
        && p.y <= a.y.max(b.y)
        && difference(p, a).cross(difference(b, a)) == 0.0
}

/// Tests whether numerator/denominator is in `[0,1]` without dividing.
/// The caller supplies a nonzero finite denominator of either sign.
fn in_unit_interval(numerator: f64, denominator: f64) -> bool {
    if denominator > 0.0 {
        numerator >= 0.0 && numerator <= denominator
    } else {
        numerator <= 0.0 && numerator >= denominator
    }
}

/// Intersects collinear segments on a nonconstant axis, using original endpoints
/// so overlap endpoints retain their converted coordinates without interpolation.
/// The returned endpoint order follows the first input segment.
fn collinear_overlap(points: [Vector2<f64>; 4], use_x: bool) -> Option<SegmentIntersection2> {
    let coordinate = |p: Vector2<f64>| if use_x { p.x } else { p.y };
    let [a, b, c, d] = points;
    let forward = coordinate(a) <= coordinate(b);
    let (a_low, a_high) = if forward { (a, b) } else { (b, a) };
    let (b_low, b_high) = if coordinate(c) <= coordinate(d) {
        (c, d)
    } else {
        (d, c)
    };
    let low = if coordinate(a_low) >= coordinate(b_low) {
        a_low
    } else {
        b_low
    };
    let high = if coordinate(a_high) <= coordinate(b_high) {
        a_high
    } else {
        b_high
    };
    if coordinate(low) > coordinate(high) {
        return None;
    }
    if coordinate(low) == coordinate(high) {
        return Some(SegmentIntersection2::Point(low));
    }
    let (start, end) = if forward { (low, high) } else { (high, low) };
    Some(SegmentIntersection2::Overlap { start, end })
}
