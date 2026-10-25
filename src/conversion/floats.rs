use super::{ConversionErrorKind, ConversionMode, ConvertFrom, RoundingMode};

/// Implements one floating source type against each listed integer target.
/// Round in f64 first; Checked then validates an exclusive upper range bound.
/// Clamp uses Rust saturating casts, but NaN is rejected before casting.
/// Wrap is unsupported for floats, and integer precision-loss checks are unnecessary
/// after the caller has explicitly selected the desired rounding rule.
macro_rules! impl_float_to_integer {
    ($source:ty; $($target:ty),+) => {$(
        impl ConvertFrom<$source> for $target {
            /// Applies the generated floating-source conversion policy.
            /// Float-to-integer rounds before checking; float-to-float checks representation.
            fn convert_from(value: &$source, mode: ConversionMode, rounding: RoundingMode)
                -> Result<Self, ConversionErrorKind>
            {
                if mode == ConversionMode::Wrap {
                    return Err(ConversionErrorKind::UnsupportedMode);
                }
                let value = rounding.apply(f64::from(*value));
                if value.is_nan() {
                    return Err(ConversionErrorKind::NonFinite);
                }
                if mode == ConversionMode::Clamp {
                    // Rust float-to-integer casts saturate, including infinities.
                    return Ok(value as $target);
                }
                if !value.is_finite() {
                    return Err(ConversionErrorKind::NonFinite);
                }

                let lower = <$target>::MIN as f64;
                // Use an exclusive power-of-two bound, not MAX as f64:
                // wide integer maxima round upward when represented by f64.
                let upper = (<$target>::MAX / 2 + 1) as f64 * 2.0;
                if value < lower || value >= upper {
                    return Err(ConversionErrorKind::OutOfRange);
                }
                Ok(value as $target)
            }
        }
    )+};
}

/// Implements conversion from a float source to each listed float target.
/// Checked rejects non-finite values and verifies exact finite representation via
/// an f64 comparison. Clamp permits rounding and replaces infinities with bounds.
/// RoundingMode is ignored because it is reserved for float-to-integer conversion.
macro_rules! impl_float_to_float {
    ($source:ty; $($target:ty),+) => {$(
        impl ConvertFrom<$source> for $target {
            /// Applies the generated floating-source conversion policy.
            /// Float-to-integer rounds before checking; float-to-float checks representation.
            fn convert_from(value: &$source, mode: ConversionMode, _: RoundingMode)
                -> Result<Self, ConversionErrorKind>
            {
                if mode == ConversionMode::Wrap {
                    return Err(ConversionErrorKind::UnsupportedMode);
                }
                if value.is_nan() {
                    return Err(ConversionErrorKind::NonFinite);
                }
                let result = *value as $target;
                if mode == ConversionMode::Clamp {
                    return Ok(result.clamp(<$target>::MIN, <$target>::MAX));
                }
                if !value.is_finite() {
                    return Err(ConversionErrorKind::NonFinite);
                }
                if !result.is_finite() {
                    return Err(ConversionErrorKind::OutOfRange);
                }
                if f64::from(result) != f64::from(*value) {
                    return Err(ConversionErrorKind::PrecisionLoss);
                }
                Ok(result)
            }
        }
    )+};
}

impl_float_to_integer!(f32; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_float_to_integer!(f64; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_float_to_float!(f32; f32, f64);
impl_float_to_float!(f64; f32, f64);
