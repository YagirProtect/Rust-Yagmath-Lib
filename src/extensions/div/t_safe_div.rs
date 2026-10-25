//! The scalar division policy used by vector /.

/// Checked division for built-in numbers. Integers truncate toward zero and
/// reject signed MIN / -1. Custom implementations define their own guarantees.
pub trait SafeDiv: Sized {
    /// Returns None on division by zero or overflow.
    /// Floating-point implementations also reject non-finite inputs and results.
    fn safe_div(&self, rhs: Self) -> Option<Self>;
}
