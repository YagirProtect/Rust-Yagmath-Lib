/// Adds distance queries to $name using every coordinate in $field.
/// Convert before subtracting so integer differences cannot overflow their source type.
/// The hypot and direct-square implementations deliberately have different float limits.
macro_rules! impl_distance {
    ($name:ident, $($field:ident),+) => {
        impl<T: ToF64> $name<T> {
            /// Euclidean distance, accumulating component differences with hypot.
            /// Components convert to f64 before subtraction; integer subtraction
            /// cannot overflow. Float differences/results can overflow to infinity.
            pub fn distance(&self, rhs: Self) -> f64 {
                let mut distance = 0.0_f64;

                $(
                    let delta = self.$field.to_f64() - rhs.$field.to_f64();
                    distance = FloatMath::hypot(distance, delta);
                )+



                distance
            }
            
            /// Sum of squared component differences, without taking a square root.
            /// Useful for radius comparisons. Uses f64; squares may overflow or
            /// underflow even when distance itself remains representable.
            pub fn distance_squared(&self, rhs: Self) -> f64 {
                let mut sum = 0.0_f64;
            
                $(
                    let delta = self.$field.to_f64() - rhs.$field.to_f64();
                    sum += delta * delta;
                )+
            
                sum
            }
        }
        
    };
}

pub(super) use impl_distance;
