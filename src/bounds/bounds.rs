use crate::extensions::to_f64::t_to_f64::ToF64;
use crate::vector::{Vector2, Vector3};

// Avoid overflow for opposite-sign extremes, while preserving tiny equal values.
/// Computes a midpoint from already ordered, converted corners.
/// Across zero, halve each endpoint before adding to avoid subtraction overflow.
/// For same-sign inputs, min + (max-min)/2 also preserves tiny equal values.
fn midpoint(min: f64, max: f64) -> f64 {
    if min <= 0.0 && max >= 0.0 {
        min * 0.5 + max * 0.5
    } else {
        min + (max - min) * 0.5
    }
}

/// Defines one closed axis-aligned bounds type and its geometric methods.
/// $name names the bounds, $vector names its corner vector, and $field lists axes.
/// For example: create_bounds!(Bounds3, Vector3, x, y, z).
/// Public corners can be edited, so queries revalidate their invariants.
macro_rules! create_bounds {
    ($name:ident, $vector:ident, $($field:ident),+) => {
        /// Closed axis-aligned bounds stored as minimum and maximum corners.
        /// Fields are public; methods revalidate them. Touching counts as intersection.
        /// Size is geometric (max - min), not an integer cell count.
        /// Equality/hash compare corners when the scalar supports them.
        /// Default uses scalar defaults for both corners (zero bounds for numbers).
        /// With the `serde` feature, deserialization restores corners without validation;
        /// call `is_valid` to validate untrusted numeric data.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name<T> {
            /// Inclusive minimum corner. Direct changes can invalidate the bounds.
            pub min: $vector<T>,
            /// Inclusive maximum corner. Direct changes can invalidate the bounds.
            pub max: $vector<T>,
        }

        impl<T: Default> Default for $name<T> {
            /// Defaults both corners independently. For numeric scalars this creates
            /// a zero-size region at the origin, not an empty-region sentinel.
            fn default() -> Self {
                Self {
                    min: $vector::default(),
                    max: $vector::default(),
                }
            }
        }

        impl<T: PartialOrd + ToF64> $name<T> {
            /// Creates bounds from inclusive corners. Rejects reversed bounds,
            /// NaN, and infinite coordinates.
            pub fn new(min: $vector<T>, max: $vector<T>) -> Option<Self> {
                let bounds = Self { min, max };
                if bounds.is_valid() {
                    Some(bounds)
                } else {
                    None
                }
            }

            /// Creates inclusive bounds from two corners in any order.
            /// Each axis is sorted independently, without converting coordinates.
            /// Returns None for NaN or infinite coordinates.
            pub fn from_start_end(mut start: $vector<T>, mut end: $vector<T>) -> Option<Self> {
                $(
                    if start.$field > end.$field {
                        std::mem::swap(&mut start.$field, &mut end.$field);
                    }
                )+
                Self::new(start, end)
            }

            /// Creates bounds from a center and full geometric size, not half-size.
            /// Returns f64 coordinates to preserve fractional corners for integer inputs.
            /// Rejects negative/non-finite sizes, non-finite centers and corner overflow.
            pub fn from_center_size(center: $vector<T>, size: $vector<T>) -> Option<$name<f64>> {
                let center = $vector::<f64> {
                    $(
                        $field: center.$field.to_f64(),
                    )+
                };
                let half_size = $vector::<f64> {
                    $(
                        $field: {
                            let size = size.$field.to_f64();
                            if size < 0.0 || !size.is_finite() {
                                return None;
                            }
                            size * 0.5
                        },
                    )+
                };

                $name::<f64>::new(
                    $vector {
                        $(
                            $field: center.$field - half_size.$field,
                        )+
                    },
                    $vector {
                        $(
                            $field: center.$field + half_size.$field,
                        )+
                    },
                )
            }

            /// Public fields may be edited directly; checks their current validity.
            pub fn is_valid(&self) -> bool {
                $(
                    if !(self.min.$field <= self.max.$field)
                        || !self.min.$field.to_f64().is_finite()
                        || !self.max.$field.to_f64().is_finite()
                    {
                        return false;
                    }
                )+
                true
            }

            /// Returns the midpoint, or None if public fields contain invalid bounds.
            pub fn center(&self) -> Option<$vector<f64>> {
                if !self.is_valid() {
                    return None;
                }
                Some($vector {
                    $(
                        $field: midpoint(self.min.$field.to_f64(), self.max.$field.to_f64()),
                    )+
                })
            }

            /// Returns geometric max - min, without adding a cell to inclusive bounds.
            /// Returns None for invalid bounds; very large f64 sizes can be infinite.
            pub fn size(&self) -> Option<$vector<f64>> {
                if !self.is_valid() {
                    return None;
                }
                Some($vector {
                    $(
                        $field: self.max.$field.to_f64() - self.min.$field.to_f64(),
                    )+
                })
            }

            /// Includes points on any edge/corner. Invalid bounds contain no points.
            pub fn contains(&self, point: $vector<T>) -> bool {
                if !self.is_valid() {
                    return false;
                }
                $(
                    if !(point.$field >= self.min.$field && point.$field <= self.max.$field) {
                        return false;
                    }
                )+
                true
            }

            /// Returns the nearest point in the closed bounds, preserving the scalar type.
            /// A point already inside the bounds is returned unchanged.
            /// Returns None for invalid bounds or non-finite point coordinates.
            pub fn closest_point(&self, point: $vector<T>) -> Option<$vector<T>>
            where
                T: Copy,
            {
                if !self.is_valid() {
                    return None;
                }
                $(
                    if !point.$field.to_f64().is_finite() {
                        return None;
                    }
                )+
                point.clamp(self.min, self.max)
            }

            /// Touching edges/corners intersect. Invalid bounds never intersect.
            pub fn intersects(&self, other: &Self) -> bool {
                if !self.is_valid() || !other.is_valid() {
                    return false;
                }
                $(
                    if !(self.min.$field <= other.max.$field && self.max.$field >= other.min.$field) {
                        return false;
                    }
                )+
                true
            }

            /// Expands the bounds to include a point. Returns None without mutation
            /// for invalid bounds or non-finite points.
            pub fn encapsulate(&mut self, point: $vector<T>) -> Option<()>
            where
                T: Copy,
            {
                if !self.is_valid() {
                    return None;
                }
                $(
                    if !point.$field.to_f64().is_finite() {
                        return None;
                    }
                )+
                $(
                    if point.$field < self.min.$field {
                        self.min.$field = point.$field;
                    }
                    if point.$field > self.max.$field {
                        self.max.$field = point.$field;
                    }
                )+
                Some(())
            }
        }
    };
}

create_bounds!(Bounds2, Vector2, x, y);
create_bounds!(Bounds3, Vector3, x, y, z);
