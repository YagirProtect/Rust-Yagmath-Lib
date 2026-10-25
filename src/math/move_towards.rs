use super::Math;
use crate::extensions::to_f64::t_to_f64::ToF64;

impl Math {
    /// Advances by at most max_delta without overshooting. Negative steps do nothing.
    /// Non-finite endpoints or a NaN step return NaN; +infinity snaps to target.
    /// Integer inputs convert to f64 and may lose precision.
    pub fn move_towards<T: ToF64>(current: T, target: T, max_delta: f64) -> f64 {
        let current = current.to_f64();
        let target = target.to_f64();
        if !current.is_finite() || !target.is_finite() || max_delta.is_nan() {
            return f64::NAN;
        }
        if max_delta <= 0.0 || current == target {
            return current;
        }
        if max_delta == f64::INFINITY || (target - current).abs() <= max_delta {
            return target;
        }
        if target > current {
            current + max_delta
        } else {
            current - max_delta
        }
    }
}
