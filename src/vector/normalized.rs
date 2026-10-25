/// Generates nonmutating unit-vector conversion for $name and its $field list.
/// Returns f64 components and rejects zero/non-finite lengths. Subnormal lengths
/// use a scaled computation to avoid dividing by an inaccurately rounded length.
macro_rules! impl_normalized {
    ($name:ident, $($field:ident),+) => {
        impl<T: ToF64> $name<T> {
            /// Returns a unit-length f64 copy without changing self.
            /// Returns None if length() is zero or non-finite, including overflowing
            /// magnitudes of finite components. Handles representable subnormal lengths.
            pub fn normalized(&self) -> Option<$name<f64>> {
                let length = self.length();

                if length == 0.0 || !length.is_finite() {
                    return None;
                }

                // Subnormal lengths lose too much precision. Scale the components
                // before calculating their ratios so the direction remains unit length.
                if length < f64::MIN_POSITIVE {
                    let mut scale = 0.0_f64;
                    $(
                        scale = scale.max(self.$field.to_f64().abs());
                    )+
                    let mut scaled_length = 0.0_f64;
                    $(
                        scaled_length = scaled_length.hypot(self.$field.to_f64() / scale);
                    )+
                    return Some($name::<f64> {
                        $(
                            $field: (self.$field.to_f64() / scale) / scaled_length,
                        )+
                    });
                }

                Some($name::<f64> {
                    $(
                        $field: self.$field.to_f64() / length,
                    )+
                })
            }
        }
    };
}

pub(super) use impl_normalized;
