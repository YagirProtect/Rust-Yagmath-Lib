use super::{Vector2, Vector3};
use crate::extensions::to_f64::t_to_f64::ToF64;

impl<T: ToF64> Vector2<T> {
    /// Returns the shortest signed angle to other in radians `[-PI, PI]`.
    /// Positive is counterclockwise in an X-right/Y-up plane. Inputs need not be
    /// normalized; mixed numeric types work. An exact half-turn chooses +PI.
    /// Zero/non-finite directions return NaN. Wide integers may lose precision.
    /// The determinant sign is subject to floating rounding/underflow.
    ///
    /// ```
    /// use yagmath::vector::Vector2;
    /// let right = Vector2::<i32>::right();
    /// let up = Vector2::<i32>::up();
    /// assert_eq!(right.signed_angle_rad(up).to_degrees(), 90.0);
    /// assert_eq!(up.signed_angle_rad(right).to_degrees(), -90.0);
    /// ```
    pub fn signed_angle_rad<U: ToF64>(&self, other: Vector2<U>) -> f64 {
        let Some(a) = scaled2(Vector2::new(self.x.to_f64(), self.y.to_f64())) else {
            return f64::NAN;
        };
        let Some(b) = scaled2(other) else {
            return f64::NAN;
        };
        let angle = a.angle_between_rad(b);
        if a.cross(b) < 0.0 { -angle } else { angle }
    }

    /// Signed angle in degrees `[-180, 180]`, counterclockwise positive.
    /// Exact half-turns choose +180; invalid directions return NaN.
    /// All other rules match signed_angle_rad.
    pub fn signed_angle_deg<U: ToF64>(&self, other: Vector2<U>) -> f64 {
        self.signed_angle_rad(other).to_degrees()
    }
}

impl<T: ToF64> Vector3<T> {
    /// Returns the angle_between_rad magnitude with sign chosen by
    /// `axis.dot(self.cross(other))`, in radians `[-PI, PI]`.
    /// Positive follows the right-hand rule around axis. Axis need not be unit.
    /// This uses the full 3D angle: it does not project inputs onto the plane
    /// perpendicular to axis. Axis only determines the sign, as in Unity SignedAngle.
    /// A zero sign chooses the positive angle, including an exact half-turn.
    /// Zero/non-finite vectors or axis return NaN. Mixed numeric types work;
    /// wide integers and nearly degenerate orientation signs can lose precision.
    ///
    /// ```
    /// use yagmath::vector::Vector3;
    /// let right = Vector3::<i32>::right();
    /// let up = Vector3::<i32>::up();
    /// assert_eq!(right.signed_angle_rad(up, Vector3::<i32>::forward()).to_degrees(), 90.0);
    /// assert_eq!(right.signed_angle_rad(up, Vector3::<i32>::backward()).to_degrees(), -90.0);
    /// ```
    pub fn signed_angle_rad<U: ToF64, A: ToF64>(&self, other: Vector3<U>, axis: Vector3<A>) -> f64 {
        let Some(a) = scaled3(Vector3::new(
            self.x.to_f64(),
            self.y.to_f64(),
            self.z.to_f64(),
        )) else {
            return f64::NAN;
        };
        let Some(b) = scaled3(other) else {
            return f64::NAN;
        };
        let Some(axis) = scaled3(axis) else {
            return f64::NAN;
        };
        let angle = a.angle_between_rad(b);
        if axis.dot(a.cross(b)) < 0.0 {
            -angle
        } else {
            angle
        }
    }

    /// Full 3D angle in degrees `[-180, 180]`, with axis selecting its sign.
    /// No projection is performed. Exact half-turns choose +180; invalid
    /// directions/axis return NaN. All other rules match signed_angle_rad.
    pub fn signed_angle_deg<U: ToF64, A: ToF64>(&self, other: Vector3<U>, axis: Vector3<A>) -> f64 {
        self.signed_angle_rad(other, axis).to_degrees()
    }
}

/// Converts and validates a 2D direction, then divides by its largest absolute
/// component. A positive scale preserves orientation without overflowing products.
/// Zero/non-finite inputs fail; a unit length is not required for sign detection.
fn scaled2<T: ToF64>(v: Vector2<T>) -> Option<Vector2<f64>> {
    let (x, y) = (v.x.to_f64(), v.y.to_f64());
    let scale = x.abs().max(y.abs());
    if !x.is_finite() || !y.is_finite() || scale == 0.0 {
        return None;
    }
    Some(Vector2::new(x / scale, y / scale))
}

/// Scales a nonzero finite 3D direction for bounded cross/triple products.
/// Axis scaling does not change the sign of the scalar triple product.
/// Individual tiny relative coordinates can still underflow in f64 arithmetic.
fn scaled3<T: ToF64>(v: Vector3<T>) -> Option<Vector3<f64>> {
    let (x, y, z) = (v.x.to_f64(), v.y.to_f64(), v.z.to_f64());
    let scale = x.abs().max(y.abs()).max(z.abs());
    if !x.is_finite() || !y.is_finite() || !z.is_finite() || scale == 0.0 {
        return None;
    }
    Some(Vector3::new(x / scale, y / scale, z / scale))
}
