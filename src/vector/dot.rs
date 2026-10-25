/// Generates the scalar product for $name and the listed $field coordinates.
/// Products and their sum use f64; no normalization or angle clamping is performed.
macro_rules! impl_dot {
    ($name:ident, $($field:ident),+) => {
        
        impl <T: ToF64> $name<T> {
            /// Sum of component products, after conversion to f64.
            /// For unit vectors this is the cosine of their angle; otherwise the
            /// magnitudes also contribute. NaN/infinity propagate; sums may overflow.
            pub fn dot(&self, rhs: Self) -> f64{
                let mut sum = 0.0_f64;
                
                $(
                    sum += self.$field.to_f64() * rhs.$field.to_f64();
                )+
                
                
                sum
            }
        }
    };
}

pub(super) use impl_dot;
