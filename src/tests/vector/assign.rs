use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn assign_operators_change_all_components_for_every_numeric_type() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let mut a = Vector2::new(2 as $type, 3 as $type);
            a += Vector2::new(1 as $type, 2 as $type);
            assert_eq!(a, Vector2::new(3 as $type, 5 as $type));
            a -= Vector2::new(1 as $type, 2 as $type);
            assert_eq!(a, Vector2::new(2 as $type, 3 as $type));
            a *= Vector2::new(3 as $type, 4 as $type);
            assert_eq!(a, Vector2::new(6 as $type, 12 as $type));
            a *= 2 as $type;
            assert_eq!(a, Vector2::new(12 as $type, 24 as $type));

            let mut a = Vector3::new(2 as $type, 3 as $type, 4 as $type);
            a += Vector3::new(1 as $type, 2 as $type, 3 as $type);
            assert_eq!(a, Vector3::new(3 as $type, 5 as $type, 7 as $type));
            a -= Vector3::new(1 as $type, 2 as $type, 3 as $type);
            assert_eq!(a, Vector3::new(2 as $type, 3 as $type, 4 as $type));
            a *= Vector3::new(3 as $type, 4 as $type, 5 as $type);
            assert_eq!(a, Vector3::new(6 as $type, 12 as $type, 20 as $type));
            a *= 2 as $type;
            assert_eq!(a, Vector3::new(12 as $type, 24 as $type, 40 as $type));

            let mut a = Vector4::new(2 as $type, 3 as $type, 4 as $type, 5 as $type);
            a += Vector4::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
            assert_eq!(a, Vector4::new(3 as $type, 5 as $type, 7 as $type, 9 as $type));
            a -= Vector4::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
            assert_eq!(a, Vector4::new(2 as $type, 3 as $type, 4 as $type, 5 as $type));
            a *= Vector4::new(3 as $type, 4 as $type, 5 as $type, 6 as $type);
            assert_eq!(a, Vector4::new(6 as $type, 12 as $type, 20 as $type, 30 as $type));
            a *= 2 as $type;
            assert_eq!(a, Vector4::new(12 as $type, 24 as $type, 40 as $type, 60 as $type));
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn signed_assign_operators_saturate_instead_of_panicking() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let mut a = Vector2::new(<$type>::MAX, <$type>::MIN);
            a += Vector2::new(1, -1);
            assert_eq!(a, Vector2::new(<$type>::MAX, <$type>::MIN));
            a -= Vector2::new(-1, 1);
            assert_eq!(a, Vector2::new(<$type>::MAX, <$type>::MIN));
            a *= Vector2::new(2, 2);
            assert_eq!(a, Vector2::new(<$type>::MAX, <$type>::MIN));
            let mut a = Vector3::new(<$type>::MIN, <$type>::MAX, 0);
            a *= -1;
            assert_eq!(a, Vector3::new(<$type>::MAX, -<$type>::MAX, 0));
        )+};
    }
    check_types!(i8, i16, i32, i64, i128, isize);
}

#[test]
fn unsigned_assign_operators_saturate_and_handle_zero() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let mut a = Vector2::new(<$type>::MAX, 0);
            a += Vector2::new(1, 0);
            assert_eq!(a, Vector2::new(<$type>::MAX, 0));
            a -= Vector2::new(0, 1);
            assert_eq!(a, Vector2::new(<$type>::MAX, 0));
            a *= 2;
            assert_eq!(a, Vector2::new(<$type>::MAX, 0));
            a *= 0;
            assert_eq!(a, Vector2::new(0, 0));
        )+};
    }
    check_types!(u8, u16, u32, u64, u128, usize);
}

#[test]
fn float_assign_operators_preserve_fractional_and_ieee_results() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let mut a = Vector2::new(1.5 as $type, -2.5 as $type);
            a += Vector2::new(0.5, 1.0);
            assert_eq!(a, Vector2::new(2.0, -1.5));
            a -= Vector2::new(0.5, 1.0);
            assert_eq!(a, Vector2::new(1.5, -2.5));
            a *= Vector2::new(-2.0, 0.5);
            assert_eq!(a, Vector2::new(-3.0, -1.25));
            a *= 0.5;
            assert_eq!(a, Vector2::new(-1.5, -0.625));

            let mut a = Vector2::new(<$type>::MAX, <$type>::MIN);
            a *= 2.0;
            assert_eq!(a, Vector2::new(<$type>::INFINITY, <$type>::NEG_INFINITY));
            a *= 0.0;
            assert!(a.x.is_nan() && a.y.is_nan());
        )+};
    }
    check_types!(f32, f64);
}

#[test]
fn vector_assign_does_not_require_copy_components() {
    use crate::extensions::add::t_safe_add::SafeAdd;
    use crate::extensions::mul::t_safe_mul::SafeMul;
    use crate::extensions::sub::t_safe_sub::SafeSub;

    #[derive(Debug, PartialEq)]
    struct Number(i32);

    impl SafeAdd for Number {
        fn safe_add(&self, rhs: Self) -> Self {
            Self(self.0.saturating_add(rhs.0))
        }
    }
    impl SafeSub for Number {
        fn safe_sub(&self, rhs: Self) -> Self {
            Self(self.0.saturating_sub(rhs.0))
        }
    }
    impl SafeMul for Number {
        fn safe_mul(&self, rhs: Self) -> Self {
            Self(self.0.saturating_mul(rhs.0))
        }
    }

    let mut a = Vector2::new(Number(2), Number(3));
    a += Vector2::new(Number(1), Number(2));
    assert_eq!(a, Vector2::new(Number(3), Number(5)));
    a -= Vector2::new(Number(1), Number(2));
    assert_eq!(a, Vector2::new(Number(2), Number(3)));
    a *= Vector2::new(Number(3), Number(4));
    assert_eq!(a, Vector2::new(Number(6), Number(12)));
}
