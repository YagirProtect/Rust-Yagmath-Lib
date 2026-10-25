use super::t_safe_sub::SafeSub;

/// Implements SafeSub for each scalar type in the nonempty $type:ty list.
/// The outer repetition emits one independent trait impl per type.
/// Uses ordinary IEEE floating-point subtraction; NaN/infinity may result.
/// The Safe trait name does not imply a finite-only floating-point operation.
macro_rules! impl_safe_sub {
    ($($type:ty),+) => {
        $(
            impl SafeSub for $type {
                /// Returns ordinary float subtraction; there is no finite-result validation.
                fn safe_sub(&self, rhs: Self) -> Self {
                    *self - rhs
                }
            }
        )+
    };
}

impl_safe_sub!(f32, f64);
