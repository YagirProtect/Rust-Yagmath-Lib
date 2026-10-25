use super::t_safe_mul::SafeMul;

/// Implements SafeMul for each scalar type in the nonempty $type:ty list.
/// The outer repetition emits one independent trait impl per type.
/// Uses saturating_mul, so integer multiplication clamps at MIN/MAX
/// in both debug and release rather than wrapping or panicking.
macro_rules! impl_safe_mul {
    ($($type:ty),+) => {
        $(
            impl SafeMul for $type {
                /// Returns saturating integer multiplication without modifying the borrowed left operand.
                fn safe_mul(&self, rhs: Self) -> Self {
                    self.saturating_mul(rhs)
                }
            }
        )+
    };
}

impl_safe_mul!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
