use crate::bounds::{Bounds2, Bounds3};
use crate::vector::{Vector2, Vector3};

#[test]
fn closest_point_2d_handles_inside_edges_and_corners() {
    let bounds = Bounds2::new(Vector2::new(0, 0), Vector2::new(10, 20)).unwrap();
    for point in [
        Vector2::new(5, 10),
        Vector2::new(0, 0),
        Vector2::new(10, 20),
        Vector2::new(0, 10),
    ] {
        assert_eq!(bounds.closest_point(point), Some(point));
    }
    for (point, expected) in [
        (Vector2::new(-5, 10), Vector2::new(0, 10)),
        (Vector2::new(15, 10), Vector2::new(10, 10)),
        (Vector2::new(5, -10), Vector2::new(5, 0)),
        (Vector2::new(5, 30), Vector2::new(5, 20)),
        (Vector2::new(-5, 30), Vector2::new(0, 20)),
        (Vector2::new(15, -10), Vector2::new(10, 0)),
    ] {
        assert_eq!(bounds.closest_point(point), Some(expected));
        assert!(bounds.contains(expected));
    }
}

#[test]
fn closest_point_3d_handles_each_face_edges_and_corners() {
    let bounds = Bounds3::new(Vector3::new(0, 0, 0), Vector3::new(10, 20, 30)).unwrap();
    for (point, expected) in [
        (Vector3::new(5, 10, 15), Vector3::new(5, 10, 15)),
        (Vector3::new(-1, 10, 15), Vector3::new(0, 10, 15)),
        (Vector3::new(11, 10, 15), Vector3::new(10, 10, 15)),
        (Vector3::new(5, -1, 15), Vector3::new(5, 0, 15)),
        (Vector3::new(5, 21, 15), Vector3::new(5, 20, 15)),
        (Vector3::new(5, 10, -1), Vector3::new(5, 10, 0)),
        (Vector3::new(5, 10, 31), Vector3::new(5, 10, 30)),
        (Vector3::new(-1, 25, 15), Vector3::new(0, 20, 15)),
        (Vector3::new(-1, 25, 40), Vector3::new(0, 20, 30)),
    ] {
        let result = bounds.closest_point(point).unwrap();
        assert_eq!(result, expected);
        assert!(bounds.contains(result));
        assert_eq!(bounds.closest_point(result), Some(result));
    }
}

#[test]
fn closest_point_preserves_all_numeric_types() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let bounds = Bounds2::new(Vector2::new(2 as $type, 3 as $type), Vector2::new(8 as $type, 9 as $type)).unwrap();
            let result: Vector2<$type> = bounds.closest_point(Vector2::new(1 as $type, 10 as $type)).unwrap();
            assert_eq!(result, Vector2::new(2 as $type, 9 as $type));
            let bounds = Bounds3::new(Vector3::new(2 as $type, 3 as $type, 4 as $type),
                Vector3::new(8 as $type, 9 as $type, 10 as $type)).unwrap();
            let result: Vector3<$type> = bounds.closest_point(Vector3::new(1 as $type, 10 as $type, 5 as $type)).unwrap();
            assert_eq!(result, Vector3::new(2 as $type, 9 as $type, 5 as $type));
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn closest_point_preserves_fractions_wide_integer_precision_and_inputs() {
    let bounds = Bounds2::new(Vector2::new(-0.5_f32, 0.25), Vector2::new(1.5, 2.75)).unwrap();
    let original = bounds;
    let point = Vector2::new(-1.25_f32, 1.125);
    assert_eq!(bounds.closest_point(point), Some(Vector2::new(-0.5, 1.125)));
    assert_eq!(bounds, original);
    assert_eq!(point, Vector2::new(-1.25, 1.125));

    let bounds = Bounds2::new(Vector2::new(u128::MAX - 1, 0), Vector2::new(u128::MAX, 1)).unwrap();
    assert_eq!(
        bounds.closest_point(Vector2::new(u128::MAX - 2, 0)),
        Some(Vector2::new(u128::MAX - 1, 0))
    );
    let bounds =
        Bounds3::new(Vector3::new(i128::MIN, 0, 1), Vector3::new(i128::MAX, 1, 2)).unwrap();
    assert_eq!(
        bounds.closest_point(Vector3::new(i128::MIN, i128::MAX, i128::MIN)),
        Some(Vector3::new(i128::MIN, 1, 1))
    );
}

#[test]
fn degenerate_bounds_return_the_only_available_point_or_plane() {
    let bounds = Bounds2::new(Vector2::new(3, 4), Vector2::new(3, 4)).unwrap();
    assert_eq!(
        bounds.closest_point(Vector2::new(100, -100)),
        Some(Vector2::new(3, 4))
    );
    let bounds = Bounds3::new(Vector3::new(0, 0, 5), Vector3::new(10, 20, 5)).unwrap();
    assert_eq!(
        bounds.closest_point(Vector3::new(5, 10, -50)),
        Some(Vector3::new(5, 10, 5))
    );
}

#[test]
fn invalid_bounds_and_points_return_none_without_mutation() {
    let invalid = Bounds2 {
        min: Vector2::new(10, 0),
        max: Vector2::new(0, 10),
    };
    assert_eq!(invalid.closest_point(Vector2::new(5, 5)), None);
    let bounds =
        Bounds3::new(Vector3::new(0.0_f64, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0)).unwrap();
    let original = bounds;
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            bounds
                .closest_point(Vector3::new(invalid, 0.5, 0.5))
                .is_none()
        );
        assert!(
            bounds
                .closest_point(Vector3::new(0.5, invalid, 0.5))
                .is_none()
        );
        assert!(
            bounds
                .closest_point(Vector3::new(0.5, 0.5, invalid))
                .is_none()
        );
        assert_eq!(bounds, original);
    }
}
