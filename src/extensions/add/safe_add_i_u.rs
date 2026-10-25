//! SafeAdd implementations for built-in integers, using saturating_add.

use crate::extensions::add::t_safe_add::SafeAdd;

/// Implements SafeAdd for each scalar type in the nonempty $type:ty list.
/// The outer repetition emits one independent trait impl per type.
/// Uses saturating_add, so integer addition clamps at MIN/MAX
/// in both debug and release rather than wrapping or panicking.
macro_rules! impl_safe_add {
    ($($type:ty),+) => {
        $(
            impl SafeAdd for $type {
                /// Returns saturating integer addition without modifying the borrowed left operand.
                fn safe_add(&self, rhs: Self) -> Self {
                    self.saturating_add(rhs)
                }
            }
        )+
    };
}

impl_safe_add!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);