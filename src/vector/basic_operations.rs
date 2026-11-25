use super::{Vector2, Vector3, Vector4};
use crate::extensions::SafeAdd;
use core::ops::{Add, AddAssign};

/// Generates componentwise operator and assignment trait implementations.
/// $name names an existing generic vector; $field lists its components.
/// Each implementation requires only its own SafeAdd/Sub/Mul/Div trait.
/// Scalar multiplication/division also require Copy to reuse rhs for every field.
/// Division propagates the first failed component with ?, returning no partial vector.
macro_rules! impl_basic_operations {
    ($name:ident, $($field:ident),+) => {
        impl<T: SafeAdd> AddAssign for $name<T> {
            /// Adds corresponding fields using SafeAdd; each assignment replaces one field.
            fn add_assign(&mut self, rhs: Self) {
                $(
                    self.$field = self.$field.safe_add(rhs.$field);
                )+
            }
        }

        impl<T: SafeSub> SubAssign for $name<T> {
            /// Subtracts corresponding fields using SafeSub, preserving the scalar policy.
            fn sub_assign(&mut self, rhs: Self) {
                $(
                    self.$field = self.$field.safe_sub(rhs.$field);
                )+
            }
        }

        impl<T: SafeMul> MulAssign for $name<T> {
            /// Multiplies corresponding fields; this is componentwise, not a dot product.
            fn mul_assign(&mut self, rhs: Self) {
                $(
                    self.$field = self.$field.safe_mul(rhs.$field);
                )+
            }
        }

        impl<T: SafeMul + Copy> MulAssign<T> for $name<T> {
            /// Multiplies every field by the same copied scalar rhs.
            fn mul_assign(&mut self, rhs: T) {
                $(
                    self.$field = self.$field.safe_mul(rhs);
                )+
            }
        }

        impl<T: SafeAdd> Add for $name<T> {
           type Output = Self;
           /// Builds a vector from pairwise sums using the scalar SafeAdd policy.
           fn add(self, rhs: Self) -> Self {
               Self {
                   $(
                       $field: self.$field.safe_add(rhs.$field),
                   )+
               }
           }
        }

        impl<T: SafeSub> Sub for $name<T> {
           type Output = Self;
           /// Builds a vector from pairwise differences using the scalar SafeSub policy.
           fn sub(self, rhs: Self) -> Self {
               Self {
                   $(
                       $field: self.$field.safe_sub(rhs.$field),
                   )+
               }
           }
        }

        impl<T: SafeMul> Mul for $name<T> {
            type Output = Self;
            /// Builds a vector from pairwise products using the scalar SafeMul policy.
            fn mul(self, rhs: Self) -> Self {
                Self {
                    $(
                        $field: self.$field.safe_mul(rhs.$field),
                    )+
                }
            }
        }

        impl<T: SafeDiv> Div for $name<T> {
           type Output = Option<Self>;
           /// Divides corresponding fields. The first None propagates through ?,
           /// so a failure never returns a partially computed vector.
           fn div(self, rhs: Self) -> Self::Output {
               Some(Self {
                   $(
                       $field: self.$field.safe_div(rhs.$field)?,
                   )+
               })
           }
        }

        impl<T: SafeMul + Copy> Mul<T> for $name<T> {
            type Output = Self;
            /// Builds a vector by multiplying each field by the copied scalar rhs.
            fn mul(self, rhs: T) -> Self {
                Self {
                    $(
                        $field: self.$field.safe_mul(rhs),
                    )+
                }
            }
        }

        impl<T: SafeDiv + Copy> Div<T> for $name<T> {
            type Output = Option<Self>;
            /// Divides every field by the same copied scalar. Any failed scalar
            /// division propagates as None for the whole vector.
            fn div(self, rhs: T) -> Option<Self> {
                Some(Self {
                    $(
                        $field: self.$field.safe_div(rhs)?,
                    )+
                })
            }
        }
    };
}

pub(super) use impl_basic_operations;

/// Adds a smaller right-hand vector to a larger left-hand vector.
/// The field list contains the shared coordinates. Other coordinates keep their values.
macro_rules! impl_add_smaller {
    ($larger:ident, $smaller:ident, $($field:ident),+) => {
        impl<T: SafeAdd> Add<$smaller<T>> for $larger<T> {
            type Output = Self;

            /// Adds the shared coordinates and returns a vector of the left-hand dimension.
            fn add(mut self, rhs: $smaller<T>) -> Self::Output {
                self += rhs;
                self
            }
        }

        impl<T: SafeAdd> AddAssign<$smaller<T>> for $larger<T> {
            /// Updates the shared coordinates using the scalar addition policy.
            fn add_assign(&mut self, rhs: $smaller<T>) {
                $(
                    self.$field = self.$field.safe_add(rhs.$field);
                )+
            }
        }
    };
}

impl_add_smaller!(Vector3, Vector2, x, y);
impl_add_smaller!(Vector4, Vector2, x, y);
impl_add_smaller!(Vector4, Vector3, x, y, z);
