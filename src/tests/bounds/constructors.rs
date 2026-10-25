use crate::bounds::{Bounds2, Bounds3};
use crate::vector::{Vector2, Vector3};

#[test]
fn start_end_sorts_each_axis_independently() {
    let a = Vector2::new(10, -2);
    let b = Vector2::new(-5, 20);
    let bounds = Bounds2::from_start_end(a, b).unwrap();
    assert_eq!(bounds.min, Vector2::new(-5, -2));
    assert_eq!(bounds.max, Vector2::new(10, 20));
    assert_eq!(Bounds2::from_start_end(b, a), Some(bounds));
    assert!(bounds.contains(a) && bounds.contains(b));

    let bounds = Bounds3::from_start_end(Vector3::new(10, -2, 8), Vector3::new(-5, 20, 3)).unwrap();
    assert_eq!(bounds.min, Vector3::new(-5, -2, 3));
    assert_eq!(bounds.max, Vector3::new(10, 20, 8));

    let point = Vector3::new(1, 2, 3);
    let bounds = Bounds3::from_start_end(point, point).unwrap();
    assert_eq!(bounds.min, point);
    assert_eq!(bounds.max, point);
}

#[test]
fn both_constructors_support_all_numeric_types() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let bounds = Bounds2::from_start_end(Vector2::new(10 as $type, 0 as $type), Vector2::new(0 as $type, 20 as $type)).unwrap();
            assert_eq!(bounds.min, Vector2::new(0 as $type, 0 as $type));
            assert_eq!(bounds.max, Vector2::new(10 as $type, 20 as $type));
            let bounds = Bounds3::from_start_end(Vector3::new(10 as $type, 0 as $type, 5 as $type),
                Vector3::new(0 as $type, 20 as $type, 3 as $type)).unwrap();
            assert_eq!(bounds.min, Vector3::new(0 as $type, 0 as $type, 3 as $type));
            assert_eq!(bounds.max, Vector3::new(10 as $type, 20 as $type, 5 as $type));

            let bounds: Bounds2<f64> = Bounds2::from_center_size(Vector2::new(5 as $type, 10 as $type),
                Vector2::new(3 as $type, 6 as $type)).unwrap();
            assert_eq!(bounds.min, Vector2::new(3.5, 7.0));
            assert_eq!(bounds.max, Vector2::new(6.5, 13.0));
            assert_eq!(bounds.center(), Some(Vector2::new(5.0, 10.0)));
            assert_eq!(bounds.size(), Some(Vector2::new(3.0, 6.0)));

            let bounds: Bounds3<f64> = Bounds3::from_center_size(Vector3::new(5 as $type, 10 as $type, 2 as $type),
                Vector3::new(3 as $type, 6 as $type, 5 as $type)).unwrap();
            assert_eq!(bounds.min, Vector3::new(3.5, 7.0, -0.5));
            assert_eq!(bounds.max, Vector3::new(6.5, 13.0, 4.5));
            assert_eq!(bounds.center(), Some(Vector3::new(5.0, 10.0, 2.0)));
            assert_eq!(bounds.size(), Some(Vector3::new(3.0, 6.0, 5.0)));
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn zero_size_and_negative_centers_are_valid() {
    let point = Vector2::new(-5, -10);
    let bounds = Bounds2::from_center_size(point, Vector2::new(0, 0)).unwrap();
    assert_eq!(bounds.min, Vector2::new(-5.0, -10.0));
    assert_eq!(bounds.max, bounds.min);
    assert_eq!(bounds.size(), Some(Vector2::zero()));
    let bounds =
        Bounds3::from_center_size(Vector3::new(-5, -10, -2), Vector3::new(3, 6, 0)).unwrap();
    assert_eq!(bounds.min, Vector3::new(-6.5, -13.0, -2.0));
    assert_eq!(bounds.max, Vector3::new(-3.5, -7.0, -2.0));
}

#[test]
fn negative_sizes_are_rejected_in_every_axis() {
    for axis in 0..2 {
        let mut size = [1; 2];
        size[axis] = -1;
        assert!(
            Bounds2::from_center_size(Vector2::new(0, 0), Vector2::new(size[0], size[1])).is_none()
        );
    }
    for axis in 0..3 {
        let mut size = [1; 3];
        size[axis] = -1;
        assert!(
            Bounds3::from_center_size(
                Vector3::new(0, 0, 0),
                Vector3::new(size[0], size[1], size[2])
            )
            .is_none()
        );
    }
}

#[test]
fn constructors_reject_non_finite_centers_sizes_and_corners() {
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            Bounds2::from_start_end(Vector2::new(invalid, 0.0), Vector2::new(1.0, 1.0)).is_none()
        );
        assert!(
            Bounds3::from_start_end(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, invalid))
                .is_none()
        );
        assert!(
            Bounds2::from_center_size(Vector2::new(invalid, 0.0), Vector2::new(1.0, 1.0)).is_none()
        );
        assert!(
            Bounds2::from_center_size(Vector2::new(0.0, 0.0), Vector2::new(1.0, invalid)).is_none()
        );
        assert!(
            Bounds3::from_center_size(Vector3::new(0.0, invalid, 0.0), Vector3::new(1.0, 1.0, 1.0))
                .is_none()
        );
        assert!(
            Bounds3::from_center_size(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, invalid))
                .is_none()
        );
    }
    assert!(
        Bounds2::from_center_size(Vector2::new(f64::MAX, 0.0), Vector2::new(f64::MAX, 1.0))
            .is_none()
    );
    assert!(
        Bounds3::from_center_size(
            Vector3::new(0.0, 0.0, f64::MIN),
            Vector3::new(1.0, 1.0, f64::MAX)
        )
        .is_none()
    );
}

#[test]
fn corner_sorting_preserves_wide_integer_precision_and_extrema() {
    let bounds =
        Bounds2::from_start_end(Vector2::new(u128::MAX, 0), Vector2::new(u128::MAX - 1, 1))
            .unwrap();
    assert_eq!(bounds.min, Vector2::new(u128::MAX - 1, 0));
    assert_eq!(bounds.max, Vector2::new(u128::MAX, 1));
    let bounds = Bounds3::from_start_end(
        Vector3::new(i128::MAX, i128::MIN, 1),
        Vector3::new(i128::MIN, i128::MAX, 0),
    )
    .unwrap();
    assert_eq!(bounds.min, Vector3::new(i128::MIN, i128::MIN, 0));
    assert_eq!(bounds.max, Vector3::new(i128::MAX, i128::MAX, 1));
}

#[test]
fn center_size_handles_large_finite_sizes_and_tiny_centers() {
    let bounds =
        Bounds2::from_center_size(Vector2::new(0.0, 0.0), Vector2::new(f64::MAX, f64::MAX))
            .unwrap();
    assert_eq!(bounds.min, Vector2::new(-f64::MAX * 0.5, -f64::MAX * 0.5));
    assert_eq!(bounds.max, Vector2::new(f64::MAX * 0.5, f64::MAX * 0.5));
    assert_eq!(bounds.center(), Some(Vector2::zero()));
    assert_eq!(bounds.size(), Some(Vector2::new(f64::MAX, f64::MAX)));
    let tiny = f64::from_bits(1);
    let bounds = Bounds2::from_center_size(Vector2::new(tiny, -tiny), Vector2::zero()).unwrap();
    assert_eq!(bounds.center(), Some(Vector2::new(tiny, -tiny)));
}
