use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn subnormal_vectors_with_multiple_components_produce_unit_directions() {
    let tiny = f64::from_bits(1);
    let v = Vector2::new(tiny, -tiny).normalized().unwrap();
    assert!((v.length() - 1.0).abs() < 1e-12);
    assert!((v.x - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-12);
    assert!((v.y + std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-12);

    let v = Vector3::new(tiny, tiny, -tiny).normalized().unwrap();
    assert!((v.length() - 1.0).abs() < 1e-12);
    assert!((v.x - 1.0 / 3.0_f64.sqrt()).abs() < 1e-12);
    assert!((v.y - v.x).abs() < 1e-12);
    assert!((v.z + v.x).abs() < 1e-12);

    let v = Vector4::new(tiny, tiny, -tiny, -tiny).normalized().unwrap();
    for (actual, expected) in [(v.x, 0.5), (v.y, 0.5), (v.z, -0.5), (v.w, -0.5)] {
        assert!((actual - expected).abs() < 1e-12);
    }
    assert!((v.length() - 1.0).abs() < 1e-12);
}

#[test]
fn normalized_returns_f64_and_preserves_original() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let original = Vector2::new(3 as $type, 4 as $type);
            let result: Vector2<f64> = original.normalized().unwrap();
            assert!((result.x - 0.6).abs() < 1e-12);
            assert!((result.y - 0.8).abs() < 1e-12);
            assert!((result.length() - 1.0).abs() < 1e-12);
            assert_eq!(original, Vector2::new(3 as $type, 4 as $type));
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );

    let v = Vector3::new(-2, 3, -6);
    let result: Vector3<f64> = v.normalized().unwrap();
    assert!((result.x + 2.0 / 7.0).abs() < 1e-12);
    assert!((result.y - 3.0 / 7.0).abs() < 1e-12);
    assert!((result.z + 6.0 / 7.0).abs() < 1e-12);
    assert!((result.length() - 1.0).abs() < 1e-12);
    assert_eq!(v, Vector3::new(-2, 3, -6));

    let v = Vector4::new(1, -2, 2, -4);
    let result: Vector4<f64> = v.normalized().unwrap();
    assert!((result.x - 0.2).abs() < 1e-12);
    assert!((result.y + 0.4).abs() < 1e-12);
    assert!((result.z - 0.4).abs() < 1e-12);
    assert!((result.w + 0.8).abs() < 1e-12);
    assert!((result.length() - 1.0).abs() < 1e-12);
    assert_eq!(v, Vector4::new(1, -2, 2, -4));
}

#[test]
fn normalized_rejects_zero_non_finite_and_overflowing_lengths() {
    assert!(Vector2::new(0, 0).normalized().is_none());
    assert!(Vector3::new(0_u8, 0, 0).normalized().is_none());
    assert!(
        Vector4::new(0.0_f32, -0.0, 0.0, -0.0)
            .normalized()
            .is_none()
    );
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(Vector2::new(invalid, 1.0).normalized().is_none());
        assert!(Vector3::new(1.0, invalid, 3.0).normalized().is_none());
        assert!(Vector4::new(1.0, 2.0, 3.0, invalid).normalized().is_none());
    }
    assert!(Vector2::new(f64::MAX, f64::MAX).normalized().is_none());
}

#[test]
fn normalized_handles_small_large_and_minimum_integer_components() {
    for scale in [1e200_f64, 1e-200] {
        let result = Vector2::new(3.0 * scale, -4.0 * scale)
            .normalized()
            .unwrap();
        assert!((result.x - 0.6).abs() < 1e-12);
        assert!((result.y + 0.8).abs() < 1e-12);
    }
    assert_eq!(
        Vector2::new(f64::from_bits(1), 0.0).normalized(),
        Some(Vector2::new(1.0, 0.0))
    );
    assert_eq!(
        Vector2::new(i8::MIN, 0).normalized(),
        Some(Vector2::new(-1.0, 0.0))
    );
    assert_eq!(
        Vector2::new(f32::MAX, 0.0).normalized(),
        Some(Vector2::new(1.0, 0.0))
    );
}
