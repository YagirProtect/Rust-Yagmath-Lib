use super::Math;
use crate::extensions::to_f64::t_to_f64::ToF64;

impl Math {
    /// Maps between ranges without clamping. Reversed ranges are allowed.
    /// Equal input bounds return out_min. Inputs convert to f64 (possibly lossy).
    /// NaN/infinity may propagate; an overflowing result is infinite.
    pub fn remap<I: ToF64, O: ToF64>(
        value: I,
        in_min: I,
        in_max: I,
        out_min: O,
        out_max: O,
    ) -> f64 {
        let t = Self::inverse_lerp_unclamped(in_min, in_max, value);
        interpolate(out_min.to_f64(), out_max.to_f64(), t)
    }

    /// Like remap, but limits the position in the input range to [0, 1].
    pub fn remap_clamped<I: ToF64, O: ToF64>(
        value: I,
        in_min: I,
        in_max: I,
        out_min: O,
        out_max: O,
    ) -> f64 {
        let t = Self::inverse_lerp(in_min, in_max, value);
        interpolate(out_min.to_f64(), out_max.to_f64(), t)
    }
}

/// Interpolates f64 endpoints for remap and smooth_step.
/// Return exact endpoints at t=0/1. For an interior t and opposite signs,
/// use weighted endpoints so end-start cannot overflow unnecessarily.
/// Extrapolation uses the direct formula and may overflow.
pub(super) fn interpolate(start: f64, end: f64, t: f64) -> f64 {
    if t == 0.0 {
        return start;
    }
    if t == 1.0 {
        return end;
    }
    if (0.0..=1.0).contains(&t) {
        // Opposite signs can overflow when subtracting even with finite endpoints.
        if start.is_sign_negative() != end.is_sign_negative() {
            return start * (1.0 - t) + end * t;
        }
    }
    start + (end - start) * t
}
