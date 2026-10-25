/// Generates tuple-like Display for an existing vector $name.
/// $first is formatted without a separator; remaining $field entries get a leading
/// comma and space. Forwarding the Formatter preserves requested scalar formatting.
macro_rules! impl_display {
    ($name:ident, $first:ident $(, $field:ident)*) => {
        impl<T: std::fmt::Display> std::fmt::Display for $name<T> {
            /// Formats the first coordinate, then comma-separated remaining coordinates.
            /// Reuse the caller Formatter so scalar precision/format settings are forwarded.
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("(")?;
                std::fmt::Display::fmt(&self.$first, f)?;
                $(
                    f.write_str(", ")?;
                    std::fmt::Display::fmt(&self.$field, f)?;
                )*
                f.write_str(")")
            }
        }
    };
}

pub(super) use impl_display;
