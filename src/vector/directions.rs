use super::{Vector2, Vector3, Vector4};
use core::ops::Neg;

// Числовые типы предоставляют значения 0 и 1 через этот трейт.
/// Zero and one for direction constructors, implemented for all 14 numeric types.
/// Negative directions additionally require Neg<Output = Self>.
pub trait DirectionScalar {
    /// Zero for the zero vector and unused direction coordinates.
    const ZERO: Self;
    /// Positive unit value for axis directions.
    const ONE: Self;
}

impl<T: DirectionScalar> Vector2<T> {
    /// Returns the zero vector; every component is zero.
    pub fn zero() -> Self {
        Self::new(T::ZERO, T::ZERO)
    }

    /// Returns the positive Y axis; other components, including W, are zero.
    pub fn up() -> Self {
        Self::new(T::ZERO, T::ONE)
    }

    /// Returns the positive X axis; other components, including W, are zero.
    pub fn right() -> Self {
        Self::new(T::ONE, T::ZERO)
    }
}

// Neg нужен для отрицательных направлений. У u8, u32 и т. д. его нет.
impl<T: DirectionScalar + Neg<Output = T>> Vector2<T> {
    /// Returns the negative Y axis. Requires a signed scalar.
    pub fn down() -> Self {
        Self::new(T::ZERO, -T::ONE)
    }

    /// Returns the negative X axis. Requires a signed scalar.
    pub fn left() -> Self {
        Self::new(-T::ONE, T::ZERO)
    }
}

impl<T: DirectionScalar> Vector3<T> {
    /// Returns the zero vector; every component is zero.
    pub fn zero() -> Self {
        Self::new(T::ZERO, T::ZERO, T::ZERO)
    }

    /// Returns the positive Y axis; other components, including W, are zero.
    pub fn up() -> Self {
        Self::new(T::ZERO, T::ONE, T::ZERO)
    }

    /// Returns the positive X axis; other components, including W, are zero.
    pub fn right() -> Self {
        Self::new(T::ONE, T::ZERO, T::ZERO)
    }

    /// Returns the positive Z axis; W is zero in 4D. Available in 3D/4D.
    pub fn forward() -> Self {
        Self::new(T::ZERO, T::ZERO, T::ONE)
    }
}

impl<T: DirectionScalar + Neg<Output = T>> Vector3<T> {
    /// Returns the negative Y axis. Requires a signed scalar.
    pub fn down() -> Self {
        Self::new(T::ZERO, -T::ONE, T::ZERO)
    }

    /// Returns the negative X axis. Requires a signed scalar.
    pub fn left() -> Self {
        Self::new(-T::ONE, T::ZERO, T::ZERO)
    }

    /// Returns the negative Z axis; W is zero in 4D. Requires a signed scalar.
    pub fn backward() -> Self {
        Self::new(T::ZERO, T::ZERO, -T::ONE)
    }
}

// У всех направлений Vector4 компонент W равен нулю.
impl<T: DirectionScalar> Vector4<T> {
    /// Returns the zero vector; every component is zero.
    pub fn zero() -> Self {
        Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO)
    }

    /// Returns the positive Y axis; other components, including W, are zero.
    pub fn up() -> Self {
        Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO)
    }

    /// Returns the positive X axis; other components, including W, are zero.
    pub fn right() -> Self {
        Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO)
    }

    /// Returns the positive Z axis; W is zero in 4D. Available in 3D/4D.
    pub fn forward() -> Self {
        Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO)
    }
}

impl<T: DirectionScalar + Neg<Output = T>> Vector4<T> {
    /// Returns the negative Y axis. Requires a signed scalar.
    pub fn down() -> Self {
        Self::new(T::ZERO, -T::ONE, T::ZERO, T::ZERO)
    }

    /// Returns the negative X axis. Requires a signed scalar.
    pub fn left() -> Self {
        Self::new(-T::ONE, T::ZERO, T::ZERO, T::ZERO)
    }

    /// Returns the negative Z axis; W is zero in 4D. Requires a signed scalar.
    pub fn backward() -> Self {
        Self::new(T::ZERO, T::ZERO, -T::ONE, T::ZERO)
    }
}

// Единственный макрос: задаёт 0 и 1 для перечисленных числовых типов.
/// Implements DirectionScalar for each comma-separated scalar $type.
/// $type:ty accepts type syntax, not a field identifier. Repetition emits one impl
/// per numeric type, supplying typed ZERO and ONE for direction constructors.
macro_rules! impl_direction_scalar {
    ($($type:ty),+) => {
        $(
            impl DirectionScalar for $type {
                const ZERO: Self = 0 as $type;
                const ONE: Self = 1 as $type;
            }
        )+
    };
}

impl_direction_scalar!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);
