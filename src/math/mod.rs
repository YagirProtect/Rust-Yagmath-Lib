//! Helpers accessed through [`Math`]: range mapping, smoothing, angles and 2D intersections.
//! Inputs convert to f64. `_rad` means radians, `_deg` degrees; times are seconds.
//! Scalar methods return numbers/NaN; checked inverse lerp returns Option.
//! Segment intersection returns an optional point/overlap classification.

mod inverse_lerp;
mod remap;
mod move_towards;
mod smooth_step;
mod ping_pong;
mod angles;
mod smooth_damp;
mod segment_intersection;

pub use segment_intersection::SegmentIntersection2;

/// Scalar math and small geometry operations, called as associated functions.
///
/// ```
/// use yagmath::Math;
///
/// assert_eq!(Math::inverse_lerp(10, 20, 15), 0.5);
/// assert_eq!(Math::inverse_lerp(10, 20, 25), 1.0);
/// assert_eq!(Math::inverse_lerp_unclamped(10, 20, 25), 1.5);
/// assert_eq!(Math::inverse_lerp(10, 10, 15), 0.0);
/// assert_eq!(Math::checked_inverse_lerp(10, 10, 15), None);
/// ```
pub struct Math;
