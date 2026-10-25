//! Private float operations selected by Cargo features, independent of the caller.
//! Explicit trait calls ensure no_std tests exercise libm even with a std test runner.

/// Operations absent from core's float API that the geometry implementation uses.
/// This trait is private to the crate; public numeric trait bounds are unchanged.
pub(crate) trait FloatMath: Sized {
    /// Stable Euclidean magnitude of two components, including extreme values.
    fn hypot(self, other: Self) -> Self;
    /// Sine and cosine of a radian angle, returned in that order.
    fn sin_cos(self) -> (Self, Self);
    /// Quadrant-aware arctangent of self / other.
    fn atan2(self, other: Self) -> Self;
    /// Rounds toward negative infinity, preserving the scalar type.
    fn floor(self) -> Self;
    /// Rounds toward positive infinity, preserving the scalar type.
    fn ceil(self) -> Self;
    /// Rounds to the nearest integer with halfway values away from zero.
    fn round(self) -> Self;
    /// Removes the fractional part toward zero.
    fn trunc(self) -> Self;
    /// Euclidean remainder, matching std's treatment of negative values and zero.
    fn rem_euclid(self, other: Self) -> Self;
}

/// Chooses std when available, otherwise libm. The final branch only permits a
/// single clear compile_error in lib.rs for an unsupported feature combination.
macro_rules! backend {
    ($std:expr, $libm:expr) => {{
        #[cfg(feature = "std")]
        {
            $std
        }
        #[cfg(all(not(feature = "std"), feature = "libm"))]
        {
            $libm
        }
        #[cfg(not(any(feature = "std", feature = "libm")))]
        {
            unreachable!("a float backend must be enabled")
        }
    }};
}

/// Implements the same backend policy for f32/f64 using their respective libm
/// functions. No indirect calls, allocations, or additional public bounds result.
macro_rules! impl_float_math {
    ($type:ty, $hypot:ident, $sincos:ident, $atan2:ident,
     $floor:ident, $ceil:ident, $round:ident, $trunc:ident) => {
        impl FloatMath for $type {
            /// Delegates to the selected stable magnitude implementation.
            fn hypot(self, other: Self) -> Self {
                backend!(<$type>::hypot(self, other), libm::$hypot(self, other))
            }
            /// Uses radian arguments on both backends.
            fn sin_cos(self) -> (Self, Self) {
                backend!(<$type>::sin_cos(self), libm::$sincos(self))
            }
            /// Preserves the y, x argument order of the inherent float method.
            fn atan2(self, other: Self) -> Self {
                backend!(<$type>::atan2(self, other), libm::$atan2(self, other))
            }
            /// Rounds down through the selected backend.
            fn floor(self) -> Self {
                backend!(<$type>::floor(self), libm::$floor(self))
            }
            /// Rounds up through the selected backend.
            fn ceil(self) -> Self {
                backend!(<$type>::ceil(self), libm::$ceil(self))
            }
            /// Both implementations break ties away from zero.
            fn round(self) -> Self {
                backend!(<$type>::round(self), libm::$round(self))
            }
            /// Both implementations preserve the sign of zero.
            fn trunc(self) -> Self {
                backend!(<$type>::trunc(self), libm::$trunc(self))
            }
            /// Reproduces std's formula without confusing Euclidean remainder
            /// with libm's IEEE remainder function, which has different semantics.
            fn rem_euclid(self, other: Self) -> Self {
                backend!(<$type>::rem_euclid(self, other), {
                    let remainder = self % other;
                    if remainder < 0.0 {
                        remainder + other.abs()
                    } else {
                        remainder
                    }
                })
            }
        }
    };
}

impl_float_math!(f64, hypot, sincos, atan2, floor, ceil, round, trunc);
impl_float_math!(f32, hypotf, sincosf, atan2f, floorf, ceilf, roundf, truncf);
