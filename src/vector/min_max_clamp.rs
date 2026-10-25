// Like float min/max, prefer the other value if exactly one value is NaN.
/// Chooses a minimum without converting or copying either operand.
/// If exactly one operand is unordered with itself (NaN), return the other;
/// both NaNs remain NaN. T must supply the intended PartialOrd behavior.
pub(super) fn component_min<T: PartialOrd>(a: T, b: T) -> T {
    if a <= b || b.partial_cmp(&b).is_none() {
        a
    } else {
        b
    }
}

/// Chooses a maximum with the same unordered/NaN policy as component_min.
/// Keeping comparison in T preserves full-width integer coordinates.
pub(super) fn component_max<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b || b.partial_cmp(&b).is_none() {
        a
    } else {
        b
    }
}

/// Adds componentwise comparison/clamping to $name for the $field list.
/// Uses PartialOrd directly rather than converting through f64, preserving integers.
/// Clamp validates every component before consuming fields into the result.
macro_rules! impl_min_max_clamp {
    ($name:ident, $($field:ident),+) => {
        impl<T: PartialOrd> $name<T> {
            /// Returns the smaller value in each component.
            /// If exactly one float component is NaN, uses the other value.
            pub fn min(self, rhs: Self) -> Self {
                Self {
                    $(
                        $field: super::min_max_clamp::component_min(self.$field, rhs.$field),
                    )+
                }
            }

            /// Returns the larger value in each component.
            /// If exactly one float component is NaN, uses the other value.
            pub fn max(self, rhs: Self) -> Self {
                Self {
                    $(
                        $field: super::min_max_clamp::component_max(self.$field, rhs.$field),
                    )+
                }
            }

            /// Limits each component to its inclusive bounds.
            /// Returns None for reversed/incomparable bounds or NaN components.
            pub fn clamp(self, min: Self, max: Self) -> Option<Self> {
                $(
                    if !(min.$field <= max.$field)
                        || self.$field.partial_cmp(&self.$field).is_none()
                    {
                        return None;
                    }
                )+

                Some(Self {
                    $(
                        $field: if self.$field < min.$field {
                            min.$field
                        } else if self.$field > max.$field {
                            max.$field
                        } else {
                            self.$field
                        },
                    )+
                })
            }
        }
    };
}

pub(super) use impl_min_max_clamp;
