use super::{Math, remap::interpolate};
use crate::extensions::to_f64::t_to_f64::ToF64;

impl Math {
    /// Interpolates from..to with zero slope at both ends. t is clamped to [0, 1].
    /// NaN propagates. Infinite endpoints can produce NaN/infinity.
    /// Numeric inputs convert to f64 and may lose precision.
    pub fn smooth_step<T: ToF64>(from: T, to: T, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        interpolate(from.to_f64(), to.to_f64(), t * t * (3.0 - 2.0 * t))
    }
}
