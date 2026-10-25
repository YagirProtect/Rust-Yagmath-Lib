use crate::Math;

#[test]
fn ordinary_inverse_lerp_returns_numbers_for_every_type() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let t: f64 = Math::inverse_lerp(0 as $type, 10 as $type, 5 as $type);
            assert_eq!(t, 0.5);
            assert_eq!(Math::inverse_lerp(0 as $type, 10 as $type, 20 as $type), 1.0);
            assert_eq!(Math::inverse_lerp_unclamped(0 as $type, 10 as $type, 20 as $type), 2.0);
            assert_eq!(Math::inverse_lerp(10 as $type, 0 as $type, 5 as $type), 0.5);
            assert_eq!(Math::inverse_lerp(10 as $type, 0 as $type, 20 as $type), 0.0);
            assert_eq!(Math::inverse_lerp(5 as $type, 5 as $type, 10 as $type), 0.0);
            assert_eq!(Math::inverse_lerp_unclamped(5 as $type, 5 as $type, 10 as $type), 0.0);
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
    assert_eq!(Math::inverse_lerp(10, 20, 5), 0.0);
    assert_eq!(Math::inverse_lerp_unclamped(10, 20, 5), -0.5);
}

#[test]
fn ordinary_methods_document_non_finite_and_degenerate_results() {
    assert!(Math::inverse_lerp(0.0, 1.0, f64::NAN).is_nan());
    assert!(Math::inverse_lerp_unclamped(0.0, 1.0, f64::NAN).is_nan());
    assert_eq!(Math::inverse_lerp(0.0, 1.0, f64::INFINITY), 1.0);
    assert_eq!(Math::inverse_lerp(0.0, 1.0, f64::NEG_INFINITY), 0.0);
    assert_eq!(
        Math::inverse_lerp_unclamped(0.0, 1.0, f64::INFINITY),
        f64::INFINITY
    );
    assert_eq!(Math::inverse_lerp(0.0, f64::INFINITY, 0.5), 0.0);
    assert!(Math::inverse_lerp(0.0, f64::INFINITY, f64::INFINITY).is_nan());
    assert_eq!(Math::inverse_lerp(1.0, 1.0, f64::NAN), 0.0);
    assert_eq!(Math::inverse_lerp(f64::INFINITY, f64::INFINITY, 10.0), 0.0);
    assert_eq!(Math::inverse_lerp(-0.0, 0.0, 0.0), 0.0);
}

#[test]
fn ordinary_methods_keep_extreme_finite_calculations_stable() {
    assert_eq!(Math::inverse_lerp(-f64::MAX, f64::MAX, 0.0), 0.5);
    assert_eq!(Math::inverse_lerp_unclamped(-f64::MAX, 0.0, f64::MAX), 2.0);
    let tiny = f64::from_bits(1);
    assert_eq!(Math::inverse_lerp(0.0, tiny, f64::MAX), 1.0);
    assert_eq!(
        Math::inverse_lerp_unclamped(0.0, tiny, f64::MAX),
        f64::INFINITY
    );
    assert_eq!(Math::inverse_lerp(0.0, tiny, -f64::MAX), 0.0);
    assert_eq!(
        Math::inverse_lerp_unclamped(0.0, tiny, -f64::MAX),
        f64::NEG_INFINITY
    );
    assert_eq!(Math::inverse_lerp_unclamped(0.0, tiny * 2.0, tiny), 0.5);
}

#[test]
fn inverse_lerp_supports_all_numeric_types() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            assert_eq!(Math::checked_inverse_lerp(0 as $type, 10 as $type, 0 as $type), Some(0.0));
            assert_eq!(Math::checked_inverse_lerp(0 as $type, 10 as $type, 5 as $type), Some(0.5));
            assert_eq!(Math::checked_inverse_lerp(0 as $type, 10 as $type, 10 as $type), Some(1.0));
            assert_eq!(Math::checked_inverse_lerp(0 as $type, 10 as $type, 20 as $type), Some(1.0));
            assert_eq!(Math::checked_inverse_lerp_unclamped(0 as $type, 10 as $type, 20 as $type), Some(2.0));
            assert_eq!(Math::checked_inverse_lerp_unclamped(10 as $type, 0 as $type, 5 as $type), Some(0.5));
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn clamped_and_unclamped_differ_outside_the_range() {
    assert_eq!(Math::checked_inverse_lerp(10, 20, 5), Some(0.0));
    assert_eq!(Math::checked_inverse_lerp(10, 20, 25), Some(1.0));
    assert_eq!(Math::checked_inverse_lerp_unclamped(10, 20, 5), Some(-0.5));
    assert_eq!(Math::checked_inverse_lerp_unclamped(10, 20, 25), Some(1.5));
    assert_eq!(Math::checked_inverse_lerp(-20, -10, -15), Some(0.5));
    assert_eq!(
        Math::checked_inverse_lerp_unclamped(-20, -10, -25),
        Some(-0.5)
    );
}

#[test]
fn reversed_ranges_preserve_start_and_end_meanings() {
    assert_eq!(Math::checked_inverse_lerp(20, 10, 20), Some(0.0));
    assert_eq!(Math::checked_inverse_lerp(20, 10, 10), Some(1.0));
    assert_eq!(Math::checked_inverse_lerp(20, 10, 15), Some(0.5));
    assert_eq!(Math::checked_inverse_lerp(20, 10, 25), Some(0.0));
    assert_eq!(Math::checked_inverse_lerp(20, 10, 5), Some(1.0));
    assert_eq!(Math::checked_inverse_lerp_unclamped(20, 10, 25), Some(-0.5));
    assert_eq!(Math::checked_inverse_lerp_unclamped(20, 10, 5), Some(1.5));
}

#[test]
fn invalid_inputs_return_none() {
    for value in [-1, 0, 1] {
        assert_eq!(Math::checked_inverse_lerp(0, 0, value), None);
        assert_eq!(Math::checked_inverse_lerp_unclamped(0, 0, value), None);
    }
    assert_eq!(Math::checked_inverse_lerp(-0.0, 0.0, 0.0), None);
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(Math::checked_inverse_lerp(invalid, 1.0, 0.5), None);
        assert_eq!(Math::checked_inverse_lerp(0.0, invalid, 0.5), None);
        assert_eq!(Math::checked_inverse_lerp(0.0, 1.0, invalid), None);
        assert_eq!(
            Math::checked_inverse_lerp_unclamped(invalid, 1.0, 0.5),
            None
        );
        assert_eq!(
            Math::checked_inverse_lerp_unclamped(0.0, invalid, 0.5),
            None
        );
        assert_eq!(
            Math::checked_inverse_lerp_unclamped(0.0, 1.0, invalid),
            None
        );
    }
}

#[test]
fn integer_subtraction_happens_after_conversion() {
    assert_eq!(
        Math::checked_inverse_lerp(i8::MIN, i8::MAX, 0),
        Some(128.0 / 255.0)
    );
    assert_eq!(
        Math::checked_inverse_lerp_unclamped(255_u8, 0, 127),
        Some(128.0 / 255.0)
    );
    assert_eq!(
        Math::checked_inverse_lerp(i128::MIN, i128::MAX, 0),
        Some(0.5)
    );
    // Adjacent wide integers may become equal in f64; no division by zero occurs.
    assert_eq!(
        Math::checked_inverse_lerp_unclamped(u128::MAX - 1, u128::MAX, u128::MAX),
        None
    );
}

#[test]
fn intermediate_float_overflow_does_not_destroy_valid_results() {
    assert_eq!(
        Math::checked_inverse_lerp(-f64::MAX, f64::MAX, 0.0),
        Some(0.5)
    );
    assert_eq!(
        Math::checked_inverse_lerp_unclamped(f64::MAX, -f64::MAX, 0.0),
        Some(0.5)
    );
    assert_eq!(
        Math::checked_inverse_lerp_unclamped(-f64::MAX, 0.0, f64::MAX),
        Some(2.0)
    );
    assert_eq!(
        Math::checked_inverse_lerp_unclamped(0.0, f64::MAX, -f64::MAX),
        Some(-1.0)
    );
}

#[test]
fn tiny_ranges_and_unrepresentable_results_are_handled() {
    let tiny = f64::from_bits(1);
    assert_eq!(
        Math::checked_inverse_lerp_unclamped(0.0, tiny * 2.0, tiny),
        Some(0.5)
    );
    assert_eq!(
        Math::checked_inverse_lerp_unclamped(0.0, tiny, tiny),
        Some(1.0)
    );
    assert_eq!(
        Math::checked_inverse_lerp_unclamped(0.0, tiny, f64::MAX),
        None
    );
    assert_eq!(Math::checked_inverse_lerp(0.0, tiny, f64::MAX), Some(1.0));
    assert_eq!(Math::checked_inverse_lerp(tiny, 0.0, f64::MAX), Some(0.0));
}
