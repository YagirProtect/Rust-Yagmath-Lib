/// Adds length and length_squared to an existing vector named by $name.
/// $field lists all coordinates to include. Each is converted through ToF64;
/// hypot avoids premature overflow in length, while squared length sums directly.
macro_rules! impl_length {
    ($name:ident, $($field:ident),+) => {
        impl<T: ToF64> $name<T> {
            /// Returns the sum of squared components as f64, without a square root.
            /// Large values may overflow to infinity; NaN components propagate.
            pub fn length_squared(&self) -> f64 {
                let mut sum = 0.0_f64;

                $(
                    let value = self.$field.to_f64();
                    sum += value * value;
                )+

                sum
            }

            /// Euclidean magnitude, computed by accumulating hypot in f64.
            /// This avoids premature square overflow/underflow. A magnitude outside
            /// f64's range is infinite; hypot may return infinity even with NaN inputs.
            /// Components convert before calculation, so wide integers can lose precision.
            pub fn length(&self) -> f64 {
                let mut length = 0.0_f64;

                $(
                    length = length.hypot(self.$field.to_f64());
                )+

                length
            }
        }
    };
}

pub(super) use impl_length;
