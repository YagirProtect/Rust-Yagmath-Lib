/// Generates floor/ceil/round for one concrete float specialization.
/// $name names the vector, $type is f32/f64, and $field lists its coordinates.
/// Each operation returns the same specialization and delegates to scalar rounding.
macro_rules! impl_float_rounds {
    ($name:ident, $type:ty, $($field:ident),+) => {
        impl $name<$type> {
            /// Rounds each component toward negative infinity.
            pub fn floor(&self) -> Self {
                Self {
                    $(
                        $field: FloatMath::floor(self.$field),
                    )+
                }
            }

            /// Rounds each component toward positive infinity.
            pub fn ceil(&self) -> Self {
                Self {
                    $(
                        $field: FloatMath::ceil(self.$field),
                    )+
                }
            }

            /// Rounds each component to the nearest integer; ties go away from zero.
            /// The component type is preserved, including NaN and infinities.
            pub fn round(&self) -> Self {
                Self {
                    $(
                        $field: FloatMath::round(self.$field),
                    )+
                }
            }
        }
    };
}

/// Generates unchanged-copy rounding methods for integer vector specializations.
/// $name is followed by a semicolon and a list of concrete scalar types.
/// Integer coordinates are already integral, so no per-field arithmetic is needed.
macro_rules! impl_integer_rounds {
    ($name:ident; $($type:ty),+) => {
        $(
            impl $name<$type> {
                /// Integer components are already rounded; returns an unchanged copy.
                pub fn floor(&self) -> Self {
                    *self
                }

                /// Integer components are already rounded; returns an unchanged copy.
                pub fn ceil(&self) -> Self {
                    *self
                }

                /// Integer components are already rounded; returns an unchanged copy.
                pub fn round(&self) -> Self {
                    *self
                }
            }
        )+
    };
}

/// Attaches rounding support for every built-in numeric specialization of $name.
/// Forwards the $field list to float generation and delegates integer generation
/// to a type-list macro; integer no-op methods only need to copy the vector.
macro_rules! impl_rounds {
    ($name:ident, $($field:ident),+) => {
        super::rounds::impl_float_rounds!($name, f32, $($field),+);
        super::rounds::impl_float_rounds!($name, f64, $($field),+);
        super::rounds::impl_integer_rounds!($name; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
    };
}

pub(super) use {impl_float_rounds, impl_integer_rounds, impl_rounds};
