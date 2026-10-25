use crate::extensions::to_f64::t_to_f64::ToF64;

/// Internal scalar interpolation interface; its containing module is crate-private.
/// T is the endpoint type; calculation and output use f64.
pub trait Lerp<T> {
    /// Interpolates two ToF64 values with a + (b-a)*t, without clamping.
    /// This internal blanket implementation permits extrapolation and IEEE overflow;
    /// it is not exported as a downstream scalar extension.
    fn lerp(&self, other: T, t: f64) -> f64;
}


impl<T: ToF64> Lerp<T> for T {
    /// Interpolates two ToF64 values with a + (b-a)*t, without clamping.
    /// This internal blanket implementation permits extrapolation and IEEE overflow;
    /// it is not exported as a downstream scalar extension.
    fn lerp(&self, other: Self, t: f64) -> f64 {
        let a = self.to_f64();
        let b = other.to_f64();

        a + (b - a) * t
    }
}