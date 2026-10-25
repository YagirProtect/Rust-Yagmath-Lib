/// Adds a magnitude limit to $name using all coordinates in $field.
/// Short vectors return unchanged after f64 conversion. Longer vectors are scaled
/// before normalization so huge/subnormal coordinates can still produce a usable direction.
macro_rules! impl_clamp_length {
    ($name:ident, $($field:ident),+) => {
        impl<T: crate::extensions::to_f64::t_to_f64::ToF64> $name<T> {
            /// Returns a vector whose length is at most max, within float precision.
            /// Shorter vectors are unchanged after conversion to f64.
            /// Returns None for negative/non-finite limits or non-finite components.
            pub fn clamp_length_max(&self, max: f64) -> Option<$name<f64>> {
                if max < 0.0 || !max.is_finite() {
                    return None;
                }

                let vector = $name::<f64> {
                    $(
                        $field: self.$field.to_f64(),
                    )+
                };
                $(
                    if !vector.$field.is_finite() {
                        return None;
                    }
                )+

                if vector.length() <= max {
                    return Some(vector);
                }
                if max == 0.0 {
                    return Some($name::<f64>::zero());
                }

                // Scaling avoids overflow in large lengths and loss of precision
                // when normalizing components close to the minimum f64 magnitude.
                let mut scale = 0.0_f64;
                $(
                    scale = scale.max(vector.$field.abs());
                )+
                let mut scaled_length = 0.0_f64;
                $(
                    scaled_length = FloatMath::hypot(scaled_length, vector.$field / scale);
                )+

                Some($name::<f64> {
                    $(
                        $field: ((vector.$field / scale) / scaled_length) * max,
                    )+
                })
            }
        }
    };
}

pub(super) use impl_clamp_length;
