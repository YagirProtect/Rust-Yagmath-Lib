/// Adds in-place normalization for the f32 and f64 versions of $name.
/// $field lists the coordinates to assign. The f64 implementation delegates to
/// normalized; f32 computes in f64 and casts back only after validating length.
macro_rules! impl_normalize {
    ($name:ident, $($field:ident),+) => {
        impl $name<f64> {
            /// Normalizes in place. Returns None without mutation when normalized()
            /// fails (zero/non-finite length). Available only for float components.
            pub fn normalize(&mut self) -> Option<()> {
                let normalized = self.normalized()?;

                $(
                    self.$field = normalized.$field;
                )+

                Some(())
            }
        }

        impl $name<f32> {
            /// Normalizes in place using an f64 length, then rounds back to f32.
            /// Returns None without mutation for zero/non-finite length.
            pub fn normalize(&mut self) -> Option<()> {
                let length = self.length();

                if length == 0.0 || !length.is_finite() {
                    return None;
                }

                $(
                    self.$field = (f64::from(self.$field) / length) as f32;
                )+

                Some(())
            }
        }
    };
}

pub(super) use impl_normalize;
