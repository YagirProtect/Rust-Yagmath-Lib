use super::{ConversionErrorKind, ConversionMode, ConvertFrom, RoundingMode};

/// Implements ConvertFrom for one integer source and multiple integer targets.
/// $source/$target are type fragments, separated by a semicolon and target list.
/// TryFrom checks the range; failed Clamp selects MIN/MAX from the source sign.
/// Wrap uses Rust integer casts, and RoundingMode is irrelevant for integer inputs.
macro_rules! impl_integer_to_integer {
    ($source:ty; $($target:ty),+) => {$(
        impl ConvertFrom<$source> for $target {
            /// Applies the generated integer-source conversion policy.
            /// Rounding is ignored because the source is already an integer.
            fn convert_from(value: &$source, mode: ConversionMode, _: RoundingMode)
                -> Result<Self, ConversionErrorKind>
            {
                match mode {
                    ConversionMode::Wrap => Ok(*value as $target),
                    ConversionMode::Checked => <$target>::try_from(*value)
                        .map_err(|_| ConversionErrorKind::OutOfRange),
                    ConversionMode::Clamp => match <$target>::try_from(*value) {
                        Ok(value) => Ok(value),
                        Err(_) => {
                            // Unsigned sources never take the negative branch,
                            // even when u128's high bit becomes negative after as i128.
                            let negative = <$source>::MIN != 0 && (*value as i128) < 0;
                            Ok(if negative { <$target>::MIN } else { <$target>::MAX })
                        }
                    },
                }
            }
        }
    )+};
}

/// Implements one integer-to-float conversion pair.
/// $source/$target are scalar types; $mantissa_bits is 24 for f32 or 53 for f64.
/// Checked verifies significant binary digits instead of casting the float back,
/// because saturating float-to-int casts could hide precision loss at integer limits.
macro_rules! impl_integer_to_float {
    ($source:ty, $target:ty, $mantissa_bits:expr) => {
        impl ConvertFrom<$source> for $target {
            /// Applies the generated integer-source conversion policy.
            /// Rounding is ignored because the source is already an integer.
            fn convert_from(
                value: &$source,
                mode: ConversionMode,
                _: RoundingMode,
            ) -> Result<Self, ConversionErrorKind> {
                if mode == ConversionMode::Wrap {
                    return Err(ConversionErrorKind::UnsupportedMode);
                }
                let result = *value as $target;
                if mode == ConversionMode::Clamp {
                    return Ok(result.clamp(<$target>::MIN, <$target>::MAX));
                }
                if !result.is_finite() {
                    return Err(ConversionErrorKind::OutOfRange);
                }

                let negative = <$source>::MIN != 0 && (*value as i128) < 0;
                let magnitude = if negative {
                    (*value as i128).unsigned_abs()
                } else {
                    *value as u128
                };
                // Trailing zero bits do not consume float mantissa precision.
                let significant_bits =
                    (128 - magnitude.leading_zeros()).saturating_sub(magnitude.trailing_zeros());
                if significant_bits > $mantissa_bits {
                    return Err(ConversionErrorKind::PrecisionLoss);
                }
                Ok(result)
            }
        }
    };
}

/// Builds the conversion implementations for each listed integer source type.
/// Each source gets all 12 integer targets and both float targets; this macro only
/// wires together the two policy-generating macros defined above.
macro_rules! impl_integer_sources {
    ($($source:ty),+) => {$(
        impl_integer_to_integer!($source; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
        impl_integer_to_float!($source, f32, 24);
        impl_integer_to_float!($source, f64, 53);
    )+};
}

impl_integer_sources!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
