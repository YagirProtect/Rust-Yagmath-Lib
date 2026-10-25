use crate::vector::{Vector2, Vector3, Vector4};

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
}

#[test]
fn clamp_length_preserves_direction_in_every_dimension() {
    let a = Vector2::new(3, -4);
    let r: Vector2<f64> = a.clamp_length_max(2.5).unwrap();
    close(r.x, 1.5);
    close(r.y, -2.0);
    close(r.length(), 2.5);
    assert_eq!(a, Vector2::new(3, -4));

    let r = Vector3::new(-2, 3, -6).clamp_length_max(3.5).unwrap();
    close(r.x, -1.0);
    close(r.y, 1.5);
    close(r.z, -3.0);
    close(r.length(), 3.5);

    let r = Vector4::new(1, -2, 2, -4).clamp_length_max(2.5).unwrap();
    close(r.x, 0.5);
    close(r.y, -1.0);
    close(r.z, 1.0);
    close(r.w, -2.0);
    close(r.length(), 2.5);
}

#[test]
fn clamp_length_supports_all_numeric_types_without_integer_truncation() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let r: Vector2<f64> = Vector2::new(3 as $type, 4 as $type).clamp_length_max(2.5).unwrap();
            close(r.x, 1.5); close(r.y, 2.0);
            let r: Vector3<f64> = Vector3::new(2 as $type, 3 as $type, 6 as $type).clamp_length_max(3.5).unwrap();
            close(r.x, 1.0); close(r.y, 1.5); close(r.z, 3.0);
            let r: Vector4<f64> = Vector4::new(1 as $type, 2 as $type, 2 as $type, 4 as $type).clamp_length_max(2.5).unwrap();
            close(r.x, 0.5); close(r.y, 1.0); close(r.z, 1.0); close(r.w, 2.0);
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn short_equal_and_zero_vectors_are_not_extended() {
    let a = Vector2::new(3.0, -4.0);
    assert_eq!(a.clamp_length_max(5.0), Some(a));
    assert_eq!(a.clamp_length_max(10.0), Some(a));
    let b = Vector3::new(0.5, -0.25, 0.125);
    assert_eq!(b.clamp_length_max(10.0), Some(b));
    let c = Vector4::new(1.0, -2.0, 2.0, -4.0);
    assert_eq!(c.clamp_length_max(5.0), Some(c));
    assert_eq!(
        Vector2::<i32>::zero().clamp_length_max(0.0),
        Some(Vector2::zero())
    );
    assert_eq!(
        Vector3::<u8>::zero().clamp_length_max(1.0),
        Some(Vector3::zero())
    );
    assert_eq!(
        Vector4::<f32>::zero().clamp_length_max(10.0),
        Some(Vector4::zero())
    );
}

#[test]
fn zero_limit_returns_zero_for_nonzero_vectors() {
    for max in [0.0, -0.0] {
        assert_eq!(
            Vector2::new(3, -4).clamp_length_max(max),
            Some(Vector2::zero())
        );
        assert_eq!(
            Vector3::new(1, 2, 3).clamp_length_max(max),
            Some(Vector3::zero())
        );
        assert_eq!(
            Vector4::new(1, 2, 3, 4).clamp_length_max(max),
            Some(Vector4::zero())
        );
    }
}

#[test]
fn invalid_limits_and_components_return_none() {
    for max in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(Vector2::new(3, 4).clamp_length_max(max).is_none());
        assert!(Vector3::<f32>::zero().clamp_length_max(max).is_none());
        assert!(Vector4::<i32>::zero().clamp_length_max(max).is_none());
    }
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(Vector2::new(invalid, 0.0).clamp_length_max(1.0).is_none());
        assert!(
            Vector3::new(1.0, invalid, 3.0)
                .clamp_length_max(1.0)
                .is_none()
        );
        assert!(
            Vector4::new(1.0, 2.0, 3.0, invalid)
                .clamp_length_max(0.0)
                .is_none()
        );
    }
}

#[test]
fn very_large_finite_vectors_can_be_limited_even_if_length_overflows() {
    let v = Vector2::new(f64::MAX, -f64::MAX);
    assert_eq!(v.length(), f64::INFINITY);
    let r = v.clamp_length_max(10.0).unwrap();
    close(r.length(), 10.0);
    close(r.x, 10.0 * std::f64::consts::FRAC_1_SQRT_2);
    close(r.y, -r.x);

    let r = Vector4::new(f64::MAX, f64::MAX, -f64::MAX, -f64::MAX)
        .clamp_length_max(2.0)
        .unwrap();
    assert_eq!(r, Vector4::new(1.0, 1.0, -1.0, -1.0));
}

#[test]
fn very_small_limits_and_components_do_not_collapse_unnecessarily() {
    let r = Vector2::new(3e-200_f64, -4e-200)
        .clamp_length_max(2.5e-200)
        .unwrap();
    close(r.x / 1e-200, 1.5);
    close(r.y / 1e-200, -2.0);

    let tiny = f64::from_bits(1);
    let r = Vector2::new(1.0, 0.0).clamp_length_max(tiny).unwrap();
    assert_eq!(r, Vector2::new(tiny, 0.0));
    let v = Vector2::new(tiny, 0.0);
    assert_eq!(v.clamp_length_max(1.0), Some(v));
}
