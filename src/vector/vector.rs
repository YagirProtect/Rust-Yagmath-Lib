use crate::float_math::FloatMath;
use crate::extensions::to_f64::t_to_f64::ToF64;
use crate::extensions::mul::t_safe_mul::SafeMul;
use crate::extensions::add::t_safe_add::SafeAdd;
use crate::extensions::div::t_safe_div::SafeDiv;
use crate::extensions::sub::t_safe_sub::SafeSub;

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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

        impl<T: Default> Default for $name<T> {
            /// Defaults each field separately; no arithmetic trait bound is needed.
            fn default() -> Self {
                Self {
                    $(
                        $field: T::default(),
                    )+
                }
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
