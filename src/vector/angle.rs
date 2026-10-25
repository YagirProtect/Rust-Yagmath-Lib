/// Adds an unsigned angle query for $name using the listed $field coordinates.
/// Scale and normalize both inputs, then use the half-angle identity
/// angle = 2 * atan2(length(a-b), length(a+b)). This retains small-angle precision
/// better than acos(dot(a,b)) and applies in all supported dimensions.
macro_rules! impl_angle {
    ($name:ident, $($field:ident),+) => {
        impl<T: ToF64> $name<T> {
            /// Returns the unsigned angle between two directions, in radians [0, PI].
            /// Inputs need not be normalized. Zero vectors or non-finite components
            /// return NaN. Components convert to f64; wide integers may lose precision.
            /// Use .to_degrees() on the result for degrees.
            pub fn angle_between_rad<U: ToF64>(&self, other: $name<U>) -> f64 {
                let a = $name { $($field: self.$field.to_f64(),)+ };
                let b = $name { $($field: other.$field.to_f64(),)+ };
                let mut a_scale = 0.0_f64;
                let mut b_scale = 0.0_f64;
                $(
                    if !a.$field.is_finite() || !b.$field.is_finite() {
                        return f64::NAN;
                    }
                    a_scale = a_scale.max(a.$field.abs());
                    b_scale = b_scale.max(b.$field.abs());
                )+
                if a_scale == 0.0 || b_scale == 0.0 {
                    return f64::NAN;
                }

                // Scale before normalization to handle huge and subnormal inputs.
                let a = $name { $($field: a.$field / a_scale,)+ };
                let b = $name { $($field: b.$field / b_scale,)+ };
                let a_length = a.length();
                let b_length = b.length();
                let mut difference_length = 0.0_f64;
                let mut sum_length = 0.0_f64;
                $(
                    {
                        let a = a.$field / a_length;
                        let b = b.$field / b_length;
                        difference_length = difference_length.hypot(a - b);
                        sum_length = sum_length.hypot(a + b);
                    }
                )+

                // Half-angle formula keeps precision near both 0 and PI,
                // where acos of the dot product loses accuracy.
                2.0 * difference_length.atan2(sum_length)
            }

            /// Unsigned angle between directions in degrees `[0, 180]`.
            /// Same normalization, mixed-type support and NaN rules as angle_between_rad.
            pub fn angle_between_deg<U: ToF64>(&self, other: $name<U>) -> f64 {
                self.angle_between_rad(other).to_degrees()
            }
        }
    };
}

pub(super) use impl_angle;
