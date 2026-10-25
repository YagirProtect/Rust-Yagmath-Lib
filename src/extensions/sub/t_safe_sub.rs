//! The scalar subtraction policy used by vector - and -=.

/// Saturating built-in integer subtraction; ordinary IEEE float subtraction.
pub trait SafeSub {
    /// Returns a new difference without modifying self. Float overflow is infinite.
    fn safe_sub(&self, rhs: Self) -> Self;
}
