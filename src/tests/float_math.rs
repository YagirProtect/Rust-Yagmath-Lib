use crate::float_math::FloatMath;

#[test]
fn magnitude_backend_handles_finite_extremes_and_special_values() {
    for (x, y) in [
        (3.0_f64, 4.0),
        (1.0, 1.0),
        (1e300, -1e300),
        (1e-300, 1e-300),
        (f64::from_bits(1), 0.0),
    ] {
        let result = FloatMath::hypot(x, y);
        let reference = x.hypot(y);
        assert!(result.is_finite());
        assert!((result - reference).abs() <= reference * 8.0 * f64::EPSILON);
    }
    assert_eq!(FloatMath::hypot(f64::MAX, f64::MAX), f64::INFINITY);
    assert_eq!(FloatMath::hypot(f64::INFINITY, f64::NAN), f64::INFINITY);
    assert!(FloatMath::hypot(f64::NAN, 1.0).is_nan());
}

#[test]
fn rounding_backends_preserve_ties_and_signed_zero_for_both_scalar_types() {
    macro_rules! check {
        ($type:ty) => {
            for x in [
                -2.5 as $type,
                -0.5,
                -0.0,
                0.0,
                0.5,
                2.5,
                <$type>::MAX,
                <$type>::INFINITY,
                <$type>::NEG_INFINITY,
            ] {
                assert_eq!(FloatMath::floor(x).to_bits(), x.floor().to_bits());
                assert_eq!(FloatMath::ceil(x).to_bits(), x.ceil().to_bits());
                assert_eq!(FloatMath::round(x).to_bits(), x.round().to_bits());
                assert_eq!(FloatMath::trunc(x).to_bits(), x.trunc().to_bits());
            }
            assert!(FloatMath::round(<$type>::NAN).is_nan());
        };
    }
    check!(f32);
    check!(f64);
}

#[test]
fn euclidean_remainder_backend_matches_sign_and_boundary_semantics() {
    for x in [
        -10.0_f64,
        -4.0,
        -f64::EPSILON,
        -0.0,
        0.0,
        f64::EPSILON,
        10.0,
    ] {
        for period in [-4.0_f64, 4.0] {
            assert_eq!(
                FloatMath::rem_euclid(x, period).to_bits(),
                x.rem_euclid(period).to_bits()
            );
        }
    }
    assert!(FloatMath::rem_euclid(1.0_f64, 0.0).is_nan());
    assert!(FloatMath::rem_euclid(f64::INFINITY, 4.0).is_nan());
    assert!(FloatMath::rem_euclid(f64::NAN, 4.0).is_nan());
}

#[test]
fn trigonometry_backend_preserves_quadrants_and_large_angle_reduction() {
    for angle in [-1e300_f64, -core::f64::consts::PI, -0.0, 0.5, 1e300] {
        let (sin, cos) = FloatMath::sin_cos(angle);
        let (expected_sin, expected_cos) = angle.sin_cos();
        assert!((sin - expected_sin).abs() < 1e-14);
        assert!((cos - expected_cos).abs() < 1e-14);
        assert!((sin * sin + cos * cos - 1.0).abs() < 1e-14);
    }
    for y in [-1.0_f64, -0.0, 0.0, 1.0] {
        for x in [-1.0_f64, -0.0, 0.0, 1.0] {
            assert!((FloatMath::atan2(y, x) - y.atan2(x)).abs() < 1e-14);
        }
    }
    let (sin, cos) = FloatMath::sin_cos(f64::INFINITY);
    assert!(sin.is_nan() && cos.is_nan());
}
