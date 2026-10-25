//! The scalar multiplication policy used by vector * and *=.

/// Saturating built-in integer multiplication; ordinary IEEE float multiplication.
pub trait SafeMul {
    /// Integers saturate at their bounds; floats use regular multiplication.
    fn safe_mul(&self, rhs: Self) -> Self;
}
