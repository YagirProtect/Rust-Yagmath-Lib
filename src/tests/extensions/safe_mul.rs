use crate::extensions::mul::t_safe_mul::SafeMul;

#[test]
fn integer_multiplication_saturates() {
    macro_rules! check_signed {
        ($($type:ty),+) => {$(
            assert_eq!(<$type>::MAX.safe_mul(2), <$type>::MAX);
            assert_eq!(<$type>::MIN.safe_mul(2), <$type>::MIN);
            assert_eq!(<$type>::MIN.safe_mul(-1), <$type>::MAX);
            assert_eq!(<$type>::MAX.safe_mul(0), 0);
            assert_eq!((-3 as $type).safe_mul(4), -12);
        )+};
    }
    macro_rules! check_unsigned {
        ($($type:ty),+) => {$(
            assert_eq!(<$type>::MAX.safe_mul(2), <$type>::MAX);
            assert_eq!(<$type>::MAX.safe_mul(0), 0);
            assert_eq!((3 as $type).safe_mul(4), 12);
        )+};
    }
    check_signed!(i8, i16, i32, i64, i128, isize);
    check_unsigned!(u8, u16, u32, u64, u128, usize);
}

#[test]
fn floats_use_regular_multiplication() {
    macro_rules! check_float {
        ($($type:ty),+) => {$(
            assert_eq!((1.5 as $type).safe_mul(-2.0), -3.0);
            assert_eq!(<$type>::MAX.safe_mul(2.0), <$type>::INFINITY);
            assert_eq!(<$type>::MIN.safe_mul(2.0), <$type>::NEG_INFINITY);
            assert!(<$type>::NAN.safe_mul(1.0).is_nan());
        )+};
    }
    check_float!(f32, f64);
}
