use crate::extensions::add::t_safe_add::SafeAdd;

#[test]
fn signed_addition_saturates_in_both_directions() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            assert_eq!(<$type>::MAX.safe_add(1), <$type>::MAX);
            assert_eq!(<$type>::MIN.safe_add(-1), <$type>::MIN);
            assert_eq!(<$type>::MIN.safe_add(<$type>::MAX), -1);
            assert_eq!((-3 as $type).safe_add(5), 2);
            assert_eq!((0 as $type).safe_add(0), 0);
        )+};
    }
    check_types!(i8, i16, i32, i64, i128, isize);
}

#[test]
fn unsigned_addition_saturates_at_maximum() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            assert_eq!(<$type>::MAX.safe_add(1), <$type>::MAX);
            assert_eq!(<$type>::MAX.safe_add(<$type>::MAX), <$type>::MAX);
            assert_eq!(<$type>::MAX.safe_add(0), <$type>::MAX);
            assert_eq!((3 as $type).safe_add(5), 8);
        )+};
    }
    check_types!(u8, u16, u32, u64, u128, usize);
}

#[test]
fn float_addition_preserves_ieee_results() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            assert_eq!((1.5 as $type).safe_add(-2.0), -0.5);
            assert_eq!(<$type>::MAX.safe_add(<$type>::MAX), <$type>::INFINITY);
            assert_eq!(<$type>::MIN.safe_add(<$type>::MIN), <$type>::NEG_INFINITY);
            assert!(<$type>::INFINITY.safe_add(<$type>::NEG_INFINITY).is_nan());
            assert!(<$type>::NAN.safe_add(1.0).is_nan());
        )+};
    }
    check_types!(f32, f64);
}
