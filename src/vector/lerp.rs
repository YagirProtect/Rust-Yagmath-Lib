/// Adds clamped and unclamped componentwise interpolation to $name.
/// $field is the coordinate list. Each output uses a + (b-a)*t in f64;
/// this intentionally does not normalize the result or suppress IEEE overflow.
macro_rules! impl_lerp {
    ($name:ident, $($field:ident),+) => {
        impl<T: ToF64> $name<T> {
            /// Componentwise a + (b - a) * t in f64; allows extrapolation.
            /// Does not preserve direction length. Non-finite inputs and intermediate
            /// overflow propagate, even at endpoints. Wide integers may lose precision.
            pub fn lerp_unclamped(&self, rhs: Self, t: f64) -> $name<f64> {
                $name::<f64> {
                    $(
                        $field: {
                            let a = self.$field.to_f64();
                            let b = rhs.$field.to_f64();

                            a + (b - a) * t
                        },
                    )+
                }
            }
            
            /// Componentwise linear interpolation with t clamped to [0, 1].
            /// NaN t propagates. Uses the same arithmetic/overflow rules as lerp_unclamped.
            pub fn lerp(&self, rhs: Self, t: f64) -> $name<f64> {
                let t = t.clamp(0.0, 1.0);
                $name::<f64> {
                    $(
                        $field: {
                            let a = self.$field.to_f64();
                            let b = rhs.$field.to_f64();

                            a + (b - a) * t
                        },
                    )+
                }
            }
        }
    }
}
pub(super) use impl_lerp;
