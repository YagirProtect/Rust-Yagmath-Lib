/// Generates mutating reflection for a concrete float vector specialization.
/// $name is the vector, $type is f32/f64, and $field lists the coordinates.
/// Compute an f64 reflected copy, validate every cast, then assign all fields.
/// A failed calculation/cast leaves the original vector untouched.
macro_rules! impl_reflect_in_place {
    ($name:ident, $type:ty, $($field:ident),+) => {
        impl $name<$type> {
            /// Reflects in place, automatically normalizing the surface normal.
            /// Returns None without mutation for invalid inputs or result overflow.
            pub fn reflect<N: ToF64>(
                &mut self,
                normal: $name<N>,
            ) -> Option<()> {
                let result = self.reflected(normal)?;
                $(
                    let $field = result.$field as $type;
                    if !$field.is_finite() {
                        return None;
                    }
                )+
                $(
                    self.$field = $field;
                )+
                Some(())
            }
        }
    };
}

/// Generates reflected for $name and delegates f32/f64 mutation generation.
/// $field lists the coordinates. Normal and input are scaled independently;
/// reflection is computed on scaled components and restored afterward.
/// The normal is normalized internally and a zero normal is rejected.
macro_rules! impl_reflect {
    ($name:ident, $($field:ident),+) => {
        impl<T: ToF64> $name<T> {
            /// Reflects across a line/plane/hyperplane through the origin.
            /// The surface normal is normalized automatically; its sign is irrelevant.
            /// Returns None for zero/non-finite normals, non-finite components,
            /// or result overflow. The original vector is unchanged.
            pub fn reflected<N: ToF64>(
                &self,
                normal: $name<N>,
            ) -> Option<$name<f64>> {
                let vector = $name::<f64> {
                    $($field: self.$field.to_f64(),)+
                };
                let normal = $name::<f64> {
                    $($field: normal.$field.to_f64(),)+
                };
                let mut normal_scale = 0.0_f64;
                let mut vector_scale = 0.0_f64;
                $(
                    if !vector.$field.is_finite() || !normal.$field.is_finite() {
                        return None;
                    }
                    normal_scale = normal_scale.max(normal.$field.abs());
                    vector_scale = vector_scale.max(vector.$field.abs());
                )+
                if normal_scale == 0.0 {
                    return None;
                }

                // Scale before normalization so huge/subnormal normals remain usable.
                let normal = $name::<f64> {
                    $($field: normal.$field / normal_scale,)+
                }.normalized()?;
                if vector_scale == 0.0 {
                    return Some(vector);
                }
                // Reflect a scaled vector to avoid overflow in the dot product.
                let scaled = $name::<f64> {
                    $($field: vector.$field / vector_scale,)+
                };
                let factor = 2.0 * scaled.dot(normal);
                let result = $name::<f64> {
                    $(
                        $field: (scaled.$field - factor * normal.$field) * vector_scale,
                    )+
                };
                $(
                    if !result.$field.is_finite() {
                        return None;
                    }
                )+
                Some(result)
            }
        }

        super::reflect::impl_reflect_in_place!($name, f32, $($field),+);
        super::reflect::impl_reflect_in_place!($name, f64, $($field),+);
    };
}

pub(super) use {impl_reflect, impl_reflect_in_place};
