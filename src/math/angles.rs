use super::Math;
use crate::extensions::to_f64::t_to_f64::ToF64;
use std::f64::consts::TAU;

impl Math {
    /// Shortest signed angular difference in radians, in (-PI, PI].
    /// Exactly opposite directions choose +PI. Non-finite inputs return NaN.
    pub fn delta_angle_rad<T: ToF64>(current: T, target: T) -> f64 {
        angle_delta(current.to_f64(), target.to_f64(), TAU)
    }

    /// Shortest signed angular difference in degrees, in `(-180, 180]`.
    /// Inputs may be f32/f64 or another ToF64 type; output is always f64.
    /// Exact half-turns choose +180. Non-finite inputs return NaN.
    /// Reduction is performed directly in degrees, without a radians round trip.
    ///
    /// ```
    /// use yagmath::Math;
    /// assert_eq!(Math::delta_angle_deg(350.0_f32, 10.0), 20.0);
    /// assert_eq!(Math::delta_angle_deg(10.0_f64, 350.0), -20.0);
    /// ```
    pub fn delta_angle_deg<T: ToF64>(current: T, target: T) -> f64 {
        angle_delta(current.to_f64(), target.to_f64(), 360.0)
    }

    /// Interpolates along the shortest arc in radians; t is clamped to [0, 1].
    /// The result is not wrapped: e.g. 350 to 10 degrees passes through 360.
    /// NaN/non-finite angles can produce NaN. Inputs convert to f64.
    pub fn lerp_angle_rad<T: ToF64>(from: T, to: T, t: f64) -> f64 {
        let delta = Self::delta_angle_rad(from.to_f64(), to.to_f64());
        from.to_f64() + delta * t.clamp(0.0, 1.0)
    }

    /// Shortest-arc interpolation in degrees, with t clamped to `[0, 1]`.
    /// Output is f64 and not wrapped: 350 toward 10 passes through 360.
    /// NaN/non-finite inputs can propagate as NaN.
    pub fn lerp_angle_deg<T: ToF64>(from: T, to: T, t: f64) -> f64 {
        let from = from.to_f64();
        from + Self::delta_angle_deg(from, to.to_f64()) * t.clamp(0.0, 1.0)
    }

    /// Advances along the shortest arc, in radians, without overshooting.
    /// Negative steps do nothing; invalid angles/NaN steps return NaN.
    /// The result is not wrapped to a single revolution.
    pub fn move_towards_angle_rad<T: ToF64>(current: T, target: T, max_delta: f64) -> f64 {
        let current = current.to_f64();
        let delta = Self::delta_angle_rad(current, target.to_f64());
        current + Self::move_towards(0.0, delta, max_delta)
    }

    /// Advances along the shortest arc in degrees without overshooting.
    /// max_delta and the f64 result are degrees. Negative steps do nothing;
    /// invalid angles/NaN steps return NaN. Output is not wrapped.
    pub fn move_towards_angle_deg<T: ToF64>(current: T, target: T, max_delta: f64) -> f64 {
        let current = current.to_f64();
        let delta = Self::delta_angle_deg(current, target.to_f64());
        current + Self::move_towards(0.0, delta, max_delta)
    }

    /// Smooth damping along the shortest arc, in radians.
    /// velocity is in radians/second. Other rules match smooth_damp.
    pub fn smooth_damp_angle_rad<T: ToF64>(
        current: T,
        target: T,
        velocity: &mut f64,
        smooth_time: f64,
        max_speed: f64,
        delta_time: f64,
    ) -> f64 {
        let current = current.to_f64();
        let target = current + Self::delta_angle_rad(current, target.to_f64());
        Self::smooth_damp(
            current,
            target,
            velocity,
            smooth_time,
            max_speed,
            delta_time,
        )
    }

    /// Smooth damping along the shortest arc in degrees.
    /// velocity and max_speed are degrees/second; times are seconds and output
    /// is f64 degrees. Validation and velocity preservation match smooth_damp.
    /// Output is not wrapped into a single revolution.
    pub fn smooth_damp_angle_deg<T: ToF64>(
        current: T,
        target: T,
        velocity: &mut f64,
        smooth_time: f64,
        max_speed: f64,
        delta_time: f64,
    ) -> f64 {
        let current = current.to_f64();
        let target = current + Self::delta_angle_deg(current, target.to_f64());
        Self::smooth_damp(
            current,
            target,
            velocity,
            smooth_time,
            max_speed,
            delta_time,
        )
    }
}

/// Computes the shortest signed difference for a finite, positive turn period.
/// Prefer direct subtraction to retain tiny deltas near the current angle.
/// If it overflows, reduce each finite input first. Signed remainder avoids
/// losing a tiny negative delta by rounding period + delta back to period.
/// The negative half-turn is remapped to the positive half-turn consistently.
fn angle_delta(current: f64, target: f64, period: f64) -> f64 {
    if !current.is_finite() || !target.is_finite() {
        return f64::NAN;
    }
    let difference = target - current;
    let delta = if difference.is_finite() {
        difference % period
    } else {
        ((target % period) - (current % period)) % period
    };
    let half = period * 0.5;
    if delta > half {
        delta - period
    } else if delta <= -half {
        delta + period
    } else {
        delta
    }
}
