//! SafeAdd implementations for f32/f64, including ordinary NaN/infinity propagation.

use crate::extensions::add::t_safe_add::SafeAdd;

/// Implements SafeAdd for each scalar type in the nonempty $type:ty list.
/// The outer repetition emits one independent trait impl per type.
/// Uses ordinary IEEE floating-point addition; NaN/infinity may result.
/// The Safe trait name does not imply a finite-only floating-point operation.
macro_rules! impl_safe_add {
    ($($type:ty),+) => {
        $(
            impl SafeAdd for $type {
                /// Returns ordinary float addition; NaN and infinity follow IEEE arithmetic.
                fn safe_add(&self, rhs: Self) -> Self {
                    self + rhs
                }
            }
        )+
    };
}


impl_safe_add!(f32, f64);