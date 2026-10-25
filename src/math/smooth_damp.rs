use super::Math;
use crate::extensions::to_f64::t_to_f64::ToF64;

impl Math {
    /// Spring-like smoothing toward target, updating velocity for the next call.
    /// delta_time and smooth_time are seconds, max_speed is units/second;
    /// use f64::INFINITY for no speed limit. smooth_time is at least 0.0001.
    /// A zero delta_time leaves current and velocity unchanged.
    /// Negative times/speed, NaN or non-finite inputs (except max_speed=+inf)
    /// return NaN without changing velocity. Intermediate overflow does likewise.
    /// Inputs convert to f64; wide integers may lose precision.
    ///
    /// ```
    /// use yagmath::Math;
    ///
    /// let mut position = 0.0;
    /// let mut velocity = 0.0; // Keep this between frames.
    /// position = Math::smooth_damp(
    ///     position, 10.0, &mut velocity,
    ///     0.3, f64::INFINITY, 1.0 / 60.0,
    /// );
    /// assert!(position > 0.0 && position < 10.0);
    /// ```
    pub fn smooth_damp<T: ToF64>(
        current: T,
        target: T,
        velocity: &mut f64,
        smooth_time: f64,
        max_speed: f64,
        delta_time: f64,
    ) -> f64 {
        let current = current.to_f64();
        let target = target.to_f64();
        if !current.is_finite()
            || !target.is_finite()
            || !velocity.is_finite()
            || !smooth_time.is_finite()
            || smooth_time < 0.0
            || !delta_time.is_finite()
            || delta_time < 0.0
            || max_speed.is_nan()
            || max_speed < 0.0
        {
            return f64::NAN;
        }
        if delta_time == 0.0 {
            return current;
        }

        let smooth_time = smooth_time.max(0.0001);
        let omega = 2.0 / smooth_time;
        let x = omega * delta_time;
        // Rational approximation of exp(-x). These coefficients approximate the
        // decay of the spring response; they are not unit-conversion constants.
        let decay = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
        let max_change = max_speed * smooth_time;
        let change = (current - target).clamp(-max_change, max_change);
        let adjusted_target = current - change;
        let temp = (*velocity + omega * change) * delta_time;
        let mut next_velocity = (*velocity - omega * temp) * decay;
        let mut output = adjusted_target + (change + temp) * decay;
        if !output.is_finite() || !next_velocity.is_finite() {
            return f64::NAN;
        }

        // Stop at the goal even when an incoming velocity would carry us past it.
        if (target > current && output > target)
            || (target < current && output < target)
            || current == target
        {
            output = target;
            next_velocity = 0.0;
        }
        *velocity = next_velocity;
        output
    }
}
