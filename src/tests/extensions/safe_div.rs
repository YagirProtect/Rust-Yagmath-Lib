use crate::extensions::div::t_safe_div::SafeDiv;

#[test]
fn integers_handle_zero_and_overflow() {
    macro_rules! check_signed {
        ($($type:ty),+) => {$(
            assert_eq!((10 as $type).safe_div(0), None);
            assert_eq!(<$type>::MIN.safe_div(-1), None);
            assert_eq!((10 as $type).safe_div(3), Some(3));
            assert_eq!((-10 as $type).safe_div(3), Some(-3));
        )+};
    }
    macro_rules! check_unsigned {
        ($($type:ty),+) => {$(
            assert_eq!((10 as $type).safe_div(0), None);
            assert_eq!((10 as $type).safe_div(3), Some(3));
            assert_eq!((0 as $type).safe_div(2), Some(0));
        )+};
    }
    check_signed!(i8, i16, i32, i64, i128, isize);
    check_unsigned!(u8, u16, u32, u64, u128, usize);
}

#[test]
fn floats_require_finite_inputs_and_results() {
    macro_rules! check_float {
        ($($type:ty),+) => {$(
            assert_eq!((5.0 as $type).safe_div(2.0), Some(2.5));
            assert_eq!((1.0 as $type).safe_div(0.0), None);
            assert_eq!((1.0 as $type).safe_div(-0.0), None);
            assert_eq!((0.0 as $type).safe_div(0.0), None);
            assert_eq!(<$type>::MAX.safe_div(0.5), None);
            assert_eq!(<$type>::INFINITY.safe_div(1.0), None);
            assert_eq!((1.0 as $type).safe_div(<$type>::INFINITY), None);
            assert_eq!(<$type>::NAN.safe_div(1.0), None);
            assert_eq!((1.0 as $type).safe_div(<$type>::NAN), None);
        )+};
    }
    check_float!(f32, f64);
}
