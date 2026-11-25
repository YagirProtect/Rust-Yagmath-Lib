use crate::extensions::ToF64;

/// Adds linear interpolation to numeric scalars and custom types that implement ToF64.
pub trait Lerp<T> {
    /// Interpolates toward `other` with `t` clamped to [0, 1].
    fn lerp(&self, other: T, t: f64) -> f64 {
        self.lerp_unclamped(other, t.clamp(0.0, 1.0))
    }

    /// Interpolates using `a + (b - a) * t` in f64.
    /// Values of `t` outside [0, 1] extrapolate. NaN and overflow can propagate,
    /// and converting wide integers to f64 can lose precision.
    fn lerp_unclamped(&self, other: T, t: f64) -> f64;
}

impl<T: ToF64> Lerp<T> for T {
    fn lerp_unclamped(&self, other: T, t: f64) -> f64 {
        let a = self.to_f64();
        let b = other.to_f64();

        a + (b - a) * t
    }
}
