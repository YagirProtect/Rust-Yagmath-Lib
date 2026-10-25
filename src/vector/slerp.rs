/// Adds spherical direction interpolation and linear magnitude interpolation.
/// $name is the existing vector type; $field is its complete coordinate list.
/// A normalized tangent defines the arc together with the start direction.
/// Antipodal inputs need a deterministic perpendicular because their arc is ambiguous.
/// Lengths are interpolated relative to a common scale to avoid unnecessary overflow.
macro_rules! impl_slerp {
    ($name:ident, $($field:ident),+) => {
        impl<T: ToF64> $name<T> {
            /// Interpolates direction along the shortest arc and length linearly.
            /// t is clamped to [0, 1]. Unit inputs stay unit length within precision.
            /// Inputs need not be normalized; components convert to f64.
            /// See slerp_unclamped for zero vectors and opposite directions.
            pub fn slerp<U: ToF64>(&self, other: $name<U>, t: f64) -> $name<f64> {
                self.slerp_unclamped(other, t.clamp(0.0, 1.0))
            }

            /// Like slerp, but extrapolates both angle and length outside [0, 1].
            /// An extrapolated negative length reverses the resulting direction.
            /// Zero/non-finite vectors, non-finite t or result overflow return NaNs.
            /// This validation applies even at t=0 or t=1; valid endpoints are exact.
            /// For exactly opposite directions the arc is ambiguous: a deterministic
            /// perpendicular is chosen by projecting the least-aligned positive
            /// coordinate axis (ties use field order) onto the perpendicular plane.
            /// This choice can change abruptly near opposite directions.
            /// Wide integers may lose precision when converted to f64.
            pub fn slerp_unclamped<U: ToF64>(&self, other: $name<U>, t: f64) -> $name<f64> {
                let invalid = || $name { $($field: { let _ = stringify!($field); f64::NAN },)+ };
                let start = $name { $($field: self.$field.to_f64(),)+ };
                let end = $name { $($field: other.$field.to_f64(),)+ };
                let mut start_scale = 0.0_f64;
                let mut end_scale = 0.0_f64;
                if !t.is_finite() { return invalid(); }
                $(
                    if !start.$field.is_finite() || !end.$field.is_finite() {
                        return invalid();
                    }
                    start_scale = start_scale.max(start.$field.abs());
                    end_scale = end_scale.max(end.$field.abs());
                )+
                if start_scale == 0.0 || end_scale == 0.0 { return invalid(); }
                if t == 0.0 { return start; }
                if t == 1.0 { return end; }
                if true $(&& start.$field == end.$field)+ { return start; }

                // Normalize scaled inputs so huge/subnormal magnitudes remain usable.
                let a = $name { $($field: start.$field / start_scale,)+ };
                let b = $name { $($field: end.$field / end_scale,)+ };
                let a_length = a.length();
                let b_length = b.length();
                let a = $name { $($field: a.$field / a_length,)+ };
                let b = $name { $($field: b.$field / b_length,)+ };
                let angle = a.angle_between_rad(b);

                let direction = if angle == 0.0 {
                    a
                } else {
                    // Near opposite vectors use their sum; near parallel vectors
                    // use their difference, retaining the small change of direction.
                    let sign = if a.dot(b) < 0.0 { 1.0 } else { -1.0 };
                    let mut tangent = $name { $($field: b.$field + sign * a.$field,)+ };
                    if tangent.length() == 0.0 {
                        let mut least = f64::INFINITY;
                        let mut axis = "";
                        $(
                            if a.$field.abs() < least {
                                least = a.$field.abs();
                                axis = stringify!($field);
                            }
                        )+
                        let _ = least;
                        tangent = $name { $($field: if axis == stringify!($field) { 1.0 } else { 0.0 },)+ };
                    }
                    // Remove the component along a to get the arc's tangent.
                    let along = tangent.dot(a);
                    tangent = $name { $($field: tangent.$field - along * a.$field,)+ };
                    let mut scale = 0.0_f64;
                    $(scale = scale.max(tangent.$field.abs());)+
                    if scale == 0.0 { return invalid(); }
                    tangent = $name { $($field: tangent.$field / scale,)+ };
                    let length = tangent.length();
                    let (sin, cos) = FloatMath::sin_cos(angle * t);
                    $name { $($field: a.$field * cos + (tangent.$field / length) * sin,)+ }
                };

                // Interpolate lengths relative to a common scale. The true lengths
                // may overflow f64 even when all result components are representable.
                let scale = start_scale.max(end_scale);
                let start_length = (start_scale / scale) * a_length;
                let end_length = (end_scale / scale) * b_length;
                let length = start_length + (end_length - start_length) * t;
                let result = $name { $($field: (direction.$field * length) * scale,)+ };
                $(if !result.$field.is_finite() { return invalid(); })+
                result
            }
        }
    };
}

pub(super) use impl_slerp;
