use crate::extensions::sub::t_safe_sub::SafeSub;

#[test]
fn integers_saturate_at_their_bounds() {
    macro_rules! check_signed {
        ($($type:ty),+) => {$(
            assert_eq!(<$type>::MIN.safe_sub(1), <$type>::MIN);
            assert_eq!(<$type>::MAX.safe_sub(-1), <$type>::MAX);
            assert_eq!((5 as $type).safe_sub(3), 2);
        )+};
    }
    macro_rules! check_unsigned {
        ($($type:ty),+) => {$(
            assert_eq!((0 as $type).safe_sub(1), 0);
            assert_eq!((5 as $type).safe_sub(3), 2);
        )+};
    }
    check_signed!(i8, i16, i32, i64, i128, isize);
    check_unsigned!(u8, u16, u32, u64, u128, usize);
}

#[test]
fn floats_use_regular_subtraction() {
    assert_eq!(5.5_f32.safe_sub(2.0), 3.5);
    assert_eq!(5.5_f64.safe_sub(2.0), 3.5);
    assert_eq!(f32::MIN.safe_sub(f32::MAX), f32::NEG_INFINITY);
    assert_eq!(f64::MIN.safe_sub(f64::MAX), f64::NEG_INFINITY);
}
