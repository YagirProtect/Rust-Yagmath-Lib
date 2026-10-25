use super::Math;
use crate::extensions::to_f64::t_to_f64::ToF64;

impl Math {
    /// Triangle wave from 0 to length and back, also for negative values.
    /// Zero length returns 0 for finite values. Negative/non-finite lengths or
    /// non-finite values return NaN. Inputs convert to f64 and may lose precision.
    pub fn ping_pong<T: ToF64>(value: T, length: T) -> f64 {
        let value = value.to_f64();
        let length = length.to_f64();
        if !value.is_finite() || !length.is_finite() || length < 0.0 {
            return f64::NAN;
        }
        if length == 0.0 {
            return 0.0;
        }
        let period = length * 2.0;
        if period.is_infinite() {
            // With such a long period any finite value is within one excursion.
            let value = value.abs();
            return if value <= length {
                value
            } else {
                length - (value - length)
            };
        }
        let phase = value.rem_euclid(period);
        if phase <= length {
            phase
        } else {
            period - phase
        }
    }
}
