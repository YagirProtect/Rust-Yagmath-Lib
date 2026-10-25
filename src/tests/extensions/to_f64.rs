use crate::extensions::to_f64::t_to_f64::ToF64;
#[test]
fn every_numeric_type_converts_exact_small_values() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            assert_eq!((0 as $type).to_f64(), 0.0);
            assert_eq!((1 as $type).to_f64(), 1.0);
            assert_eq!((42 as $type).to_f64(), 42.0);
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
    assert_eq!((-42_i64).to_f64(), -42.0);
    assert_eq!(1.25_f32.to_f64(), 1.25);
    assert_eq!((-0.0_f64).to_f64().to_bits(), (-0.0_f64).to_bits());
}

#[test]
fn wide_integer_conversion_is_finite_but_can_round() {
    assert_eq!(i64::MIN.to_f64(), -2.0_f64.powi(63));
    assert_eq!(i64::MAX.to_f64(), 2.0_f64.powi(63));
    assert_eq!(u64::MAX.to_f64(), 2.0_f64.powi(64));
    assert_eq!(i128::MIN.to_f64(), -2.0_f64.powi(127));
    assert_eq!(i128::MAX.to_f64(), 2.0_f64.powi(127));
    assert_eq!(u128::MAX.to_f64(), 2.0_f64.powi(128));
    // Integers above 2^53 cannot all be represented distinctly by f64.
    assert_eq!(
        9_007_199_254_740_992_u64.to_f64(),
        9_007_199_254_740_993_u64.to_f64()
    );
}

#[test]
fn float_conversion_preserves_non_finite_values() {
    assert!(f32::NAN.to_f64().is_nan());
    assert!(f64::NAN.to_f64().is_nan());
    assert_eq!(f32::INFINITY.to_f64(), f64::INFINITY);
    assert_eq!(f32::NEG_INFINITY.to_f64(), f64::NEG_INFINITY);
    assert_eq!(f64::INFINITY.to_f64(), f64::INFINITY);
    assert_eq!(f64::NEG_INFINITY.to_f64(), f64::NEG_INFINITY);
}
