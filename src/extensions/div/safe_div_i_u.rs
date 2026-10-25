use super::t_safe_div::SafeDiv;

/// Implements SafeDiv for each scalar type in the nonempty $type:ty list.
/// The outer repetition emits one independent trait impl per type.
/// Delegates to checked_div: zero divisors and signed MIN / -1 return None.
/// Integer results truncate toward zero; no float conversion occurs.
macro_rules! impl_safe_div {
    ($($type:ty),+) => {
        $(
            impl SafeDiv for $type {
                /// Returns the truncated quotient, or None for zero or signed division overflow.
                fn safe_div(&self, rhs: Self) -> Option<Self> {
                    self.checked_div(rhs)
                }
            }
        )+
    };
}

impl_safe_div!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
