/// Adds straight-line movement to $name over the listed $field coordinates.
/// The limit applies to the Euclidean step, not to each coordinate separately.
/// Scale endpoints when subtraction/length overflows or a subnormal norm loses precision.
macro_rules! impl_move_towards {
    ($name:ident, $($field:ident),+) => {
        impl<T: ToF64> $name<T> {
            /// Moves toward target by at most max_delta in Euclidean distance.
            /// Returns a new f64 vector; the original is unchanged. Negative steps
            /// do nothing; +infinity snaps to target. Non-finite components or a
            /// NaN step return a vector of NaNs. Wide integers may lose precision.
            pub fn move_towards<U: ToF64>(&self, target: $name<U>, max_delta: f64) -> $name<f64> {
                let current = $name { $($field: self.$field.to_f64(),)+ };
                let target = $name { $($field: target.$field.to_f64(),)+ };
                if max_delta.is_nan() $(|| !current.$field.is_finite() || !target.$field.is_finite())+ {
                    return $name { $($field: { let _ = stringify!($field); f64::NAN },)+ };
                }
                if max_delta <= 0.0 { return current; }
                if max_delta == f64::INFINITY { return target; }

                let mut delta = $name { $($field: target.$field - current.$field,)+ };
                let mut length = delta.length();
                if length == 0.0 || length <= max_delta { return target; }
                if !length.is_finite() || length < f64::MIN_POSITIVE {
                    // Scale endpoints before subtracting for extreme finite inputs.
                    let mut scale = 0.0_f64;
                    $(scale = scale.max(current.$field.abs()).max(target.$field.abs());)+
                    delta = $name { $($field: target.$field / scale - current.$field / scale,)+ };
                    length = delta.length();
                }
                $name { $($field: current.$field + (delta.$field / length) * max_delta,)+ }
            }
        }
    };
}

pub(super) use impl_move_towards;
