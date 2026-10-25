use super::t_safe_mul::SafeMul;

/// Implements SafeMul for each scalar type in the nonempty $type:ty list.
/// The outer repetition emits one independent trait impl per type.
/// Uses ordinary IEEE floating-point multiplication; NaN/infinity may result.
/// The Safe trait name does not imply a finite-only floating-point operation.
macro_rules! impl_safe_mul {
    ($($type:ty),+) => {
        $(
            impl SafeMul for $type {
                /// Returns ordinary float multiplication, including possible NaN/infinity.
                fn safe_mul(&self, rhs: Self) -> Self {
                    *self * rhs
                }
            }
        )+
    };
}

impl_safe_mul!(f32, f64);
