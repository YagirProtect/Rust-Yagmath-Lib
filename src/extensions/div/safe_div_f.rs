use super::t_safe_div::SafeDiv;

/// Implements SafeDiv for each scalar type in the nonempty $type:ty list.
/// The outer repetition emits one independent trait impl per type.
/// Rejects zero (including -0), NaN/infinite inputs, and non-finite quotients.
/// Finite underflow to zero is accepted; it is not treated as overflow.
macro_rules! impl_safe_div {
    ($($type:ty),+) => {
        $(
            impl SafeDiv for $type {
                /// Returns a finite quotient of finite inputs with a nonzero divisor.
                /// Zero, NaN/infinity and quotient overflow fail; underflow to zero is accepted.
                fn safe_div(&self, rhs: Self) -> Option<Self> {
                    if rhs == 0.0 || !self.is_finite() || !rhs.is_finite() {
                        return None;
                    }

                    let result = *self / rhs;
                    if result.is_finite() {
                        Some(result)
                    } else {
                        None
                    }
                }
            }
        )+
    };
}

impl_safe_div!(f32, f64);
