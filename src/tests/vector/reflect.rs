use crate::vector::{Vector2, Vector3, Vector4};

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
}

#[test]
fn reflection_inverts_the_normal_component_in_every_dimension() {
    let a = Vector2::new(3, -4);
    assert_eq!(
        a.reflected(Vector2::new(0, 1)),
        Some(Vector2::new(3.0, 4.0))
    );
    assert_eq!(
        a.reflected(Vector2::new(1, 0)),
        Some(Vector2::new(-3.0, -4.0))
    );
    assert_eq!(a, Vector2::new(3, -4));
    assert_eq!(
        Vector3::new(1, 2, 3).reflected(Vector3::new(0, 0, 1)),
        Some(Vector3::new(1.0, 2.0, -3.0))
    );
    assert_eq!(
        Vector4::new(1, 2, 3, 4).reflected(Vector4::new(0, 0, 0, 1)),
        Some(Vector4::new(1.0, 2.0, 3.0, -4.0))
    );
}

#[test]
fn reflection_supports_every_numeric_type_and_mixed_normal_types() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let r: Vector2<f64> = Vector2::new(3 as $type, 4 as $type).reflected(Vector2::new(0, 1)).unwrap();
            close(r.x, 3.0); close(r.y, -4.0);
            let r: Vector3<f64> = Vector3::new(1 as $type, 2 as $type, 3 as $type).reflected(Vector3::new(0.0_f32, 0.0, 2.0)).unwrap();
            close(r.x, 1.0); close(r.y, 2.0); close(r.z, -3.0);
            let r: Vector4<f64> = Vector4::new(1 as $type, 2 as $type, 3 as $type, 4 as $type).reflected(Vector4::new(0, 0, 0, 1)).unwrap();
            close(r.x, 1.0); close(r.y, 2.0); close(r.z, 3.0); close(r.w, -4.0);
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn reflection_handles_arbitrary_planes_and_preserves_length() {
    let a = Vector3::new(3.0_f64, -2.0, 5.0);
    let n = Vector3::new(1.0_f64, 2.0, 3.0);
    let r = a.reflected(n).unwrap();
    close(r.x, 1.0);
    close(r.y, -6.0);
    close(r.z, -1.0);
    close(r.length(), a.length());
    close(r.dot(n), -a.dot(n));
    let back = r.reflected(n).unwrap();
    close(back.x, a.x);
    close(back.y, a.y);
    close(back.z, a.z);

    let r = Vector2::new(1, 0).reflected(Vector2::new(1, 1)).unwrap();
    close(r.x, 0.0);
    close(r.y, -1.0);
    let r = Vector4::new(1, 2, 3, 4)
        .reflected(Vector4::new(1, 1, 1, 1))
        .unwrap();
    close(r.x, -4.0);
    close(r.y, -3.0);
    close(r.z, -2.0);
    close(r.w, -1.0);
}

#[test]
fn normal_magnitude_and_sign_do_not_change_reflection() {
    for y in [
        1.0,
        -1.0,
        10.0,
        -10.0,
        f64::MAX,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
    ] {
        assert_eq!(
            Vector2::new(3, -4).reflected(Vector2::new(0.0, y)),
            Some(Vector2::new(3.0, 4.0))
        );
    }
    let huge = Vector3::new(f64::MAX, f64::MAX, f64::MAX);
    let result = Vector3::new(1, 0, 0).reflected(huge).unwrap();
    close(result.x, 1.0 / 3.0);
    close(result.y, -2.0 / 3.0);
    close(result.z, -2.0 / 3.0);
}

#[test]
fn zero_and_tangent_vectors_are_unchanged() {
    assert_eq!(
        Vector2::new(0, 0).reflected(Vector2::new(0, 1)),
        Some(Vector2::zero())
    );
    assert_eq!(
        Vector3::new(0, 0, 0).reflected(Vector3::new(0, 1, 0)),
        Some(Vector3::zero())
    );
    assert_eq!(
        Vector4::new(0, 0, 0, 0).reflected(Vector4::new(0, 1, 0, 0)),
        Some(Vector4::zero())
    );
    assert_eq!(
        Vector3::new(1, 0, 2).reflected(Vector3::new(0, 1, 0)),
        Some(Vector3::new(1.0, 0.0, 2.0))
    );
}

#[test]
fn invalid_inputs_return_none() {
    assert!(Vector2::new(1, 2).reflected(Vector2::<i32>::zero()).is_none());
    assert!(
        Vector3::new(0, 0, 0)
            .reflected(Vector3::<i32>::zero())
            .is_none()
    );
    assert!(
        Vector4::new(1, 2, 3, 4)
            .reflected(Vector4::<f64>::zero())
            .is_none()
    );
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            Vector2::new(invalid, 1.0)
                .reflected(Vector2::new(0, 1))
                .is_none()
        );
        assert!(
            Vector3::new(1, 2, 3)
                .reflected(Vector3::new(0.0, invalid, 1.0))
                .is_none()
        );
        assert!(
            Vector4::new(1.0, 2.0, 3.0, invalid)
                .reflected(Vector4::new(0, 0, 0, 1))
                .is_none()
        );
    }
}

#[test]
fn reflect_mutates_float_vectors_without_changing_scalar_type() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let mut a = Vector2::new(3 as $type, -4 as $type);
            assert_eq!(a.reflect(Vector2::new(0, 1)), Some(()));
            assert_eq!(a, Vector2::new(3.0, 4.0));
            let mut a = Vector3::new(1 as $type, 2 as $type, 3 as $type);
            assert_eq!(a.reflect(Vector3::new(0, 0, 1)), Some(()));
            assert_eq!(a, Vector3::new(1.0, 2.0, -3.0));
            let mut a = Vector4::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
            assert_eq!(a.reflect(Vector4::new(0, 0, 0, 1)), Some(()));
            assert_eq!(a, Vector4::new(1.0, 2.0, 3.0, -4.0));
        )+};
    }
    check_types!(f32, f64);
}

#[test]
fn failed_reflect_preserves_source_including_on_f32_overflow() {
    let mut a = Vector2::new(3.0_f64, -4.0);
    let original = a;
    assert_eq!(a.reflect(Vector2::<i32>::zero()), None);
    assert_eq!(a, original);
    let mut b = Vector3::new(1.0_f32, 2.0, 3.0);
    let original = b;
    assert_eq!(b.reflect(Vector3::new(0.0, f64::NAN, 1.0)), None);
    assert_eq!(b, original);

    let mut a = Vector2::new(f32::MAX, f32::MAX);
    let original = a;
    assert_eq!(
        a.reflect(Vector2::new(1.0_f64, -(2.0_f64.sqrt() - 1.0))),
        None
    );
    assert_eq!(a, original);
}

#[test]
fn reflection_avoids_intermediate_overflow_and_preserves_subnormal_values() {
    let a = Vector2::new(f64::MAX, -f64::MAX);
    assert_eq!(
        a.reflected(Vector2::new(0, 1)),
        Some(Vector2::new(f64::MAX, f64::MAX))
    );
    let tiny = f64::from_bits(1);
    assert_eq!(
        Vector2::new(tiny, -tiny).reflected(Vector2::new(0, 1)),
        Some(Vector2::new(tiny, tiny))
    );
    assert!(
        Vector2::new(f64::MAX, f64::MAX)
            .reflected(Vector2::new(1.0_f64, -(2.0_f64.sqrt() - 1.0)))
            .is_none()
    );
}
