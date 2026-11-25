use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

// The generated implementations resolve their imports in this module.
use super::{VectorComponent, component::component_for_field};
use crate::conversion::{ConversionError, ConversionMode, ConvertFrom, RoundingMode};
use crate::extensions::{SafeAdd, SafeDiv, SafeMul, SafeSub, ToF64};
use crate::float_math::FloatMath;

/// Defines one public vector type and attaches its feature implementations.
/// $name is the struct identifier; $field is a nonempty comma-separated list of
/// component identifiers. For example: create_vector!(Vector3, x, y, z).
/// Each feature macro receives the same identifiers; no runtime dispatch is involved.
macro_rules! create_vector {
    ($name:ident, $($field:ident),+) => {
        /// A fixed-dimensional vector with public, generic scalar components.
        ///
        /// `new` has no numeric bounds. `Default` defaults each component.
        /// `Copy`, `Clone`, equality and hashing follow the scalar's capabilities;
        /// float vectors support `PartialEq`, not `Eq` or `Hash`.
        /// Display prints `(x, y, ...)`.
        ///
        /// # Operators
        ///
        /// `+`, `-`, `*` are componentwise, with saturating built-in integers and
        /// ordinary float arithmetic. Scalar `*` is supported on the right.
        /// Vector/scalar `/` returns `Option<Self>` and fails if any component
        /// division fails. `+=`, `-=`, vector/scalar `*=` are supported; `/=` is not.
        /// For `+` and `+=`, the right-hand vector can also have fewer dimensions
        /// with the same component type. Only shared coordinates are added.
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name<T> {
            $(
                #[doc = concat!("The `", stringify!($field), "` component.")]
                pub $field: T,
            )+
        }

        impl<T> $name<T> {
            /// Constructs a vector from components in field order, without validation.
            pub fn new($($field: T),+) -> Self {
                Self { $($field),+ }
            }
        }

        super::basic_operations::impl_basic_operations!($name, $($field),+);
        super::display::impl_display!($name, $($field),+);
        super::convert::impl_convert!($name, $($field),+);
        super::length::impl_length!($name, $($field),+);
        super::clamp_length::impl_clamp_length!($name, $($field),+);
        super::normalize::impl_normalize!($name, $($field),+);
        super::normalized::impl_normalized!($name, $($field),+);
        super::distance::impl_distance!($name, $($field),+);
        super::dot::impl_dot!($name, $($field),+);
        super::lerp::impl_lerp!($name, $($field),+);
        super::min_max_clamp::impl_min_max_clamp!($name, $($field),+);
        super::rounds::impl_rounds!($name, $($field),+);
        super::reflect::impl_reflect!($name, $($field),+);
        super::move_towards::impl_move_towards!($name, $($field),+);
        super::angle::impl_angle!($name, $($field),+);
        super::slerp::impl_slerp!($name, $($field),+);
    };
}

create_vector!(Vector2, x, y);
create_vector!(Vector3, x, y, z);
create_vector!(Vector4, x, y, z, w);
