use super::t_safe_sub::SafeSub;

/// Implements SafeSub for each scalar type in the nonempty $type:ty list.
/// The outer repetition emits one independent trait impl per type.
/// Uses saturating_sub, so integer subtraction clamps at MIN/MAX
/// in both debug and release rather than wrapping or panicking.
macro_rules! impl_safe_sub {
    ($($type:ty),+) => {
        $(
            impl SafeSub for $type {
                /// Returns saturating integer subtraction without modifying the borrowed left operand.
                fn safe_sub(&self, rhs: Self) -> Self {
                    self.saturating_sub(rhs)
                }
            }
        )+
    };
}

impl_safe_sub!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
