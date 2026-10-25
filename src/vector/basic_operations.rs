/// Generates componentwise operator and assignment trait implementations.
/// $name names an existing generic vector; $field lists its components.
/// Each implementation requires only its own SafeAdd/Sub/Mul/Div trait.
/// Scalar multiplication/division also require Copy to reuse rhs for every field.
/// Division propagates the first failed component with ?, returning no partial vector.
macro_rules! impl_basic_operations {
    ($name:ident, $($field:ident),+) => {
        impl<T> core::ops::AddAssign for $name<T>
        where
            T: crate::extensions::add::t_safe_add::SafeAdd,
        {
            /// Adds corresponding fields using SafeAdd; each assignment replaces one field.
            fn add_assign(&mut self, rhs: Self) {
                $(
                    self.$field = self.$field.safe_add(rhs.$field);
                )+
            }
        }

        impl<T> core::ops::SubAssign for $name<T>
        where
            T: crate::extensions::sub::t_safe_sub::SafeSub,
        {
            /// Subtracts corresponding fields using SafeSub, preserving the scalar policy.
            fn sub_assign(&mut self, rhs: Self) {
                $(
                    self.$field = self.$field.safe_sub(rhs.$field);
                )+
            }
        }

        impl<T> core::ops::MulAssign for $name<T>
        where
            T: crate::extensions::mul::t_safe_mul::SafeMul,
        {
            /// Multiplies corresponding fields; this is componentwise, not a dot product.
            fn mul_assign(&mut self, rhs: Self) {
                $(
                    self.$field = self.$field.safe_mul(rhs.$field);
                )+
            }
        }

        impl<T> core::ops::MulAssign<T> for $name<T>
        where
            T: crate::extensions::mul::t_safe_mul::SafeMul + Copy,
        {
            /// Multiplies every field by the same copied scalar rhs.
            fn mul_assign(&mut self, rhs: T) {
                $(
                    self.$field = self.$field.safe_mul(rhs);
                )+
            }
        }

        impl<T> core::ops::Add for $name<T>
        where
            T: crate::extensions::add::t_safe_add::SafeAdd,
        {
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

        impl<T> core::ops::Sub for $name<T>
        where
            T: crate::extensions::sub::t_safe_sub::SafeSub,
        {
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

        impl<T> core::ops::Mul for $name<T>
        where
            T: crate::extensions::mul::t_safe_mul::SafeMul,
        {
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

        impl<T> core::ops::Div for $name<T>
        where
            T: crate::extensions::div::t_safe_div::SafeDiv,
        {
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

        impl<T> core::ops::Mul<T> for $name<T>
        where
            T: crate::extensions::mul::t_safe_mul::SafeMul + Copy,
        {
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

        impl<T> core::ops::Div<T> for $name<T>
        where
            T: crate::extensions::div::t_safe_div::SafeDiv + Copy,
        {
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
