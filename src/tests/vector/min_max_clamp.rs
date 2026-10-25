use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn min_and_max_select_components_independently_in_every_dimension() {
    let a = Vector2::new(2, 10);
    let b = Vector2::new(5, 3);
    assert_eq!(a.min(b), Vector2::new(2, 3));
    assert_eq!(a.max(b), Vector2::new(5, 10));

    let a = Vector3::new(2, 10, -5);
    let b = Vector3::new(5, 3, -8);
    assert_eq!(a.min(b), Vector3::new(2, 3, -8));
    assert_eq!(a.max(b), Vector3::new(5, 10, -5));

    let a = Vector4::new(2, 10, -5, 7);
    let b = Vector4::new(5, 3, -8, 9);
    assert_eq!(a.min(b), Vector4::new(2, 3, -8, 7));
    assert_eq!(a.max(b), Vector4::new(5, 10, -5, 9));
    assert_eq!(a.min(b), b.min(a));
    assert_eq!(a.max(b), b.max(a));
    assert_eq!(a.min(a), a);
    assert_eq!(a.max(a), a);
}

#[test]
fn clamp_limits_only_components_outside_their_bounds() {
    let original = Vector2::new(-5, 120);
    assert_eq!(
        original.clamp(Vector2::new(0, 0), Vector2::new(100, 80)),
        Some(Vector2::new(0, 80))
    );
    assert_eq!(original, Vector2::new(-5, 120));

    assert_eq!(
        Vector3::new(-5, 4, 120).clamp(Vector3::new(0, 1, 2), Vector3::new(10, 8, 20)),
        Some(Vector3::new(0, 4, 20))
    );
    assert_eq!(
        Vector4::new(-5, 4, 120, 7).clamp(Vector4::new(0, 1, 2, 7), Vector4::new(10, 8, 20, 7)),
        Some(Vector4::new(0, 4, 20, 7))
    );
}

#[test]
fn bounds_methods_support_all_numeric_types_without_converting_to_f64() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let a = Vector2::new(2 as $type, 10 as $type);
            let b = Vector2::new(5 as $type, 3 as $type);
            assert_eq!(a.min(b), Vector2::new(2 as $type, 3 as $type));
            assert_eq!(a.max(b), Vector2::new(5 as $type, 10 as $type));
            assert_eq!(a.clamp(Vector2::new(3 as $type, 1 as $type), Vector2::new(8 as $type, 9 as $type)),
                Some(Vector2::new(3 as $type, 9 as $type)));
            assert_eq!(Vector3::<$type>::zero().clamp(Vector3::zero(), Vector3::zero()), Some(Vector3::zero()));
            assert_eq!(Vector4::<$type>::zero().clamp(Vector4::zero(), Vector4::zero()), Some(Vector4::zero()));
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );

    let a = Vector2::new(u128::MAX, u128::MAX - 1);
    let b = Vector2::new(u128::MAX - 1, u128::MAX);
    assert_eq!(a.min(b), Vector2::new(u128::MAX - 1, u128::MAX - 1));
    assert_eq!(a.max(b), Vector2::new(u128::MAX, u128::MAX));
    assert_eq!(
        a.clamp(b, Vector2::new(u128::MAX, u128::MAX)),
        Some(Vector2::new(u128::MAX, u128::MAX))
    );
}

#[test]
fn clamp_accepts_boundaries_and_rejects_reversed_bounds_in_each_component() {
    let v = Vector4::new(i32::MIN, i32::MAX, 0, 1);
    assert_eq!(v.clamp(v, v), Some(v));
    for axis in 0..4 {
        let mut bounds = [10; 4];
        bounds[axis] = -1;
        assert!(
            Vector4::new(0, 0, 0, 0)
                .clamp(
                    Vector4::zero(),
                    Vector4::new(bounds[0], bounds[1], bounds[2], bounds[3])
                )
                .is_none()
        );
    }
    assert!(
        Vector2::new(1, 2)
            .clamp(Vector2::new(2, 0), Vector2::new(1, 3))
            .is_none()
    );
    assert!(
        Vector3::new(1, 2, 3)
            .clamp(Vector3::new(0, 0, 4), Vector3::new(2, 3, 3))
            .is_none()
    );
}

#[test]
fn float_bounds_preserve_fractions_and_handle_infinity() {
    assert_eq!(
        Vector2::new(-0.5_f32, 1.5).clamp(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0)),
        Some(Vector2::new(0.0, 1.0))
    );
    let v = Vector3::new(f64::NEG_INFINITY, 0.25, f64::INFINITY);
    assert_eq!(
        v.clamp(Vector3::new(-1.0, 0.0, -1.0), Vector3::new(1.0, 1.0, 1.0)),
        Some(Vector3::new(-1.0, 0.25, 1.0))
    );
    assert_eq!(
        v.clamp(
            Vector3::new(f64::NEG_INFINITY, 0.0, 0.0),
            Vector3::new(0.0, 1.0, f64::INFINITY)
        ),
        Some(v)
    );
}

#[test]
fn float_min_max_ignore_one_nan_and_clamp_rejects_nan() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let a = Vector2::new(<$type>::NAN, 2.0);
            let b = Vector2::new(1.0, <$type>::NAN);
            assert_eq!(a.min(b), Vector2::new(1.0, 2.0));
            assert_eq!(b.min(a), Vector2::new(1.0, 2.0));
            assert_eq!(a.max(b), Vector2::new(1.0, 2.0));
            assert_eq!(b.max(a), Vector2::new(1.0, 2.0));
            assert!(a.min(a).x.is_nan());
            assert!(a.max(a).x.is_nan());
            assert!(a.clamp(Vector2::new(0.0, 0.0), Vector2::new(3.0, 3.0)).is_none());
            assert!(Vector2::new(1.0, 2.0).clamp(a, Vector2::new(3.0, 3.0)).is_none());
            assert!(Vector2::new(1.0, 2.0).clamp(Vector2::new(0.0, 0.0), b).is_none());
        )+};
    }
    check_types!(f32, f64);
}
