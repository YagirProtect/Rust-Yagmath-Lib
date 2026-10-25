use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn length_squared_supports_every_numeric_type_and_dimension() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let squared: f64 = Vector2::new(3 as $type, 4 as $type).length_squared();
            assert_eq!(squared, 25.0);
            assert_eq!(Vector3::new(2 as $type, 3 as $type, 6 as $type).length_squared(), 49.0);
            assert_eq!(Vector4::new(1 as $type, 2 as $type, 2 as $type, 4 as $type).length_squared(), 25.0);
            assert_eq!(Vector2::<$type>::zero().length_squared(), 0.0);
            assert_eq!(Vector3::<$type>::zero().length_squared(), 0.0);
            assert_eq!(Vector4::<$type>::zero().length_squared(), 0.0);
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn length_squared_converts_before_multiplication_and_preserves_fractions() {
    assert_eq!(Vector2::new(i8::MIN, i8::MAX).length_squared(), 32513.0);
    assert_eq!(Vector2::new(u8::MAX, u8::MAX).length_squared(), 130050.0);
    assert_eq!(Vector2::new(-0.5_f32, 1.5).length_squared(), 2.5);
    let v = Vector3::new(-0.5_f64, 1.5, -2.0);
    assert_eq!(v.length_squared(), 6.5);
    assert!((v.length_squared() - v.length().powi(2)).abs() < 1e-12);
    assert_eq!(v, Vector3::new(-0.5, 1.5, -2.0));
}

#[test]
fn length_squared_handles_float_limits_without_panicking() {
    assert_eq!(Vector2::new(f64::MAX, 0.0).length_squared(), f64::INFINITY);
    assert_eq!(Vector2::new(f64::from_bits(1), 0.0).length_squared(), 0.0);
    assert_eq!(
        Vector3::new(1.0, f64::NEG_INFINITY, 2.0).length_squared(),
        f64::INFINITY
    );
    assert!(
        Vector4::new(1.0, 2.0, 3.0, f64::NAN)
            .length_squared()
            .is_nan()
    );
}

#[test]
fn length_uses_every_component_and_ignores_sign() {
    assert_eq!(Vector2::new(3, -4).length(), 5.0);
    assert_eq!(Vector3::new(-2, 3, -6).length(), 7.0);
    assert_eq!(Vector4::new(1, -2, 2, -4).length(), 5.0);
    assert_eq!(Vector2::new(0, 0).length(), 0.0);
    assert_eq!(Vector3::new(0, 0, 0).length(), 0.0);
    assert_eq!(Vector4::new(0, 0, 0, 0).length(), 0.0);
}

#[test]
fn length_converts_every_numeric_type_before_calculation() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let length: f64 = Vector2::new(3 as $type, 4 as $type).length();
            assert_eq!(length, 5.0);
            assert_eq!(Vector3::new(2 as $type, 3 as $type, 6 as $type).length(), 7.0);
            assert_eq!(Vector4::new(1 as $type, 2 as $type, 2 as $type, 4 as $type).length(), 5.0);
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
    assert_eq!(Vector2::new(i8::MIN, 0).length(), 128.0);
    assert_eq!(Vector2::new(u8::MAX, 0).length(), 255.0);
    assert!((Vector2::new(1, 1).length() - std::f64::consts::SQRT_2).abs() < 1e-15);
}

#[test]
fn length_avoids_intermediate_overflow_and_underflow() {
    for scale in [1e200_f64, 1e-200] {
        let length = Vector2::new(3.0 * scale, 4.0 * scale).length();
        assert!((length / (5.0 * scale) - 1.0).abs() < 1e-12);
    }
    let tiny = f64::from_bits(1);
    assert_eq!(Vector2::new(tiny, 0.0).length(), tiny);
    assert_eq!(Vector2::new(f64::MAX, 0.0).length(), f64::MAX);
    assert_eq!(Vector2::new(f64::MAX, f64::MAX).length(), f64::INFINITY);
}

#[test]
fn length_reports_non_finite_components() {
    assert!(Vector2::new(f64::NAN, 1.0).length().is_nan());
    assert_eq!(
        Vector3::new(1.0, f64::INFINITY, 2.0).length(),
        f64::INFINITY
    );
    assert_eq!(
        Vector4::new(1.0, 2.0, 3.0, f64::NEG_INFINITY).length(),
        f64::INFINITY
    );
}
