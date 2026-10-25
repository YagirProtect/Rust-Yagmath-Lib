//! The scalar addition policy used by vector + and +=.

/// Saturating built-in integer addition; ordinary IEEE floating-point addition.
/// Float results can be NaN/infinite. Custom types define their own policy.
pub trait SafeAdd{
    /// Returns a new sum without modifying self.
    fn safe_add(&self, rhs:Self) -> Self;
}
