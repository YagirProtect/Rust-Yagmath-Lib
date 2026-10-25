/// Generates borrowed vector conversion methods for $name and its $field list.
/// The destination scalar implements `ConvertFrom<T>`. A scalar error is wrapped
/// with the failing field name; ? stops before constructing a partial output vector.
macro_rules! impl_convert {
    ($name:ident, $($field:ident),+) => {
        impl<T> $name<T> {
            /// Converts every component, truncating fractional float-to-integer values.
            /// Checked rejects overflow/float precision loss; Clamp saturates;
            /// Wrap supports integer-to-integer conversions only.
            pub fn convert<U>(&self, mode: crate::conversion::ConversionMode)
                -> Result<$name<U>, crate::conversion::ConversionError>
            where
                U: crate::conversion::ConvertFrom<T>,
            {
                self.convert_with_rounding(mode, crate::conversion::RoundingMode::Truncate)
            }

            /// Rounding is applied only for float-to-integer conversions, before
            /// range checking. Round uses halfway-away-from-zero behavior.
            pub fn convert_with_rounding<U>(
                &self,
                mode: crate::conversion::ConversionMode,
                rounding: crate::conversion::RoundingMode,
            ) -> Result<$name<U>, crate::conversion::ConversionError>
            where
                U: crate::conversion::ConvertFrom<T>,
            {
                Ok($name {
                    $(
                        $field: U::convert_from(&self.$field, mode, rounding)
                            .map_err(|kind| crate::conversion::ConversionError {
                                component: super::component::component_for_field!($field),
                                kind,
                            })?,
                    )+
                })
            }
        }
    };
}

pub(super) use impl_convert;
