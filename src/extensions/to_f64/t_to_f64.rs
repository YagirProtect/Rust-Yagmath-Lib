//! The numeric conversion bound for geometric methods.

/// Supplies an f64 representation for geometry. Built-in integers use Rust casts;
/// more than 53 significant binary digits can lose precision. Float NaN/infinity
/// are preserved. Implementing this does not automatically implement arithmetic traits.
pub trait ToF64 {
    /// Returns the scalar as f64 without changing it; conversion can be lossy.
    fn to_f64(&self) -> f64;
}

/// Implements ToF64 for every comma-separated scalar $type.
/// Each generated method borrows its scalar and returns a Rust `as f64` cast.
/// This supplies geometric conversion, not the stricter Checked conversion policy.
macro_rules! impl_to_f64 {
    ($($type:ty),+) => {
        $(
            impl ToF64 for $type {
                /// Performs the built-in numeric cast; wide integer precision can be lost.
                fn to_f64(&self) -> f64 {
                    *self as f64
                }
            }
        )+
    };
}

impl_to_f64!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
);
