use crate::bounds::{Bounds2, Bounds3};
use crate::vector::{Vector2, Vector3};
use std::collections::HashSet;

#[test]
fn bounds_support_every_numeric_type() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let a = Bounds2::new(Vector2::new(1 as $type, 2 as $type), Vector2::new(4 as $type, 8 as $type)).unwrap();
            assert!(a.is_valid());
            assert_eq!(a.center(), Some(Vector2::new(2.5, 5.0)));
            assert_eq!(a.size(), Some(Vector2::new(3.0, 6.0)));
            assert!(a.contains(Vector2::new(2 as $type, 3 as $type)));
            let b = Bounds3::new(Vector3::new(1 as $type, 2 as $type, 3 as $type),
                Vector3::new(4 as $type, 8 as $type, 9 as $type)).unwrap();
            assert!(b.is_valid());
            assert_eq!(b.center(), Some(Vector3::new(2.5, 5.0, 6.0)));
            assert_eq!(b.size(), Some(Vector3::new(3.0, 6.0, 6.0)));
            assert!(Bounds2::<$type>::default().contains(Vector2::zero()));
            assert!(Bounds3::<$type>::default().contains(Vector3::zero()));
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn constructors_reject_reversed_and_non_finite_bounds() {
    assert!(Bounds2::new(Vector2::new(2, 0), Vector2::new(1, 1)).is_none());
    assert!(Bounds2::new(Vector2::new(0, 2), Vector2::new(1, 1)).is_none());
    for axis in 0..3 {
        let mut min = [0; 3];
        min[axis] = 2;
        assert!(
            Bounds3::new(Vector3::new(min[0], min[1], min[2]), Vector3::new(1, 1, 1)).is_none()
        );
    }
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(Bounds2::new(Vector2::new(invalid, 0.0), Vector2::new(1.0, 1.0)).is_none());
        assert!(Bounds2::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, invalid)).is_none());
        assert!(
            Bounds3::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, invalid, 1.0)).is_none()
        );
    }
}

#[test]
fn invalid_public_fields_are_handled_without_panicking() {
    let mut bounds = Bounds2::new(Vector2::new(0, 0), Vector2::new(1, 1)).unwrap();
    bounds.min.x = 2;
    assert!(!bounds.is_valid());
    assert_eq!(bounds.center(), None);
    assert_eq!(bounds.size(), None);
    assert!(!bounds.contains(Vector2::new(0, 0)));
    let valid = Bounds2::default();
    assert!(!bounds.intersects(&valid));
    assert!(!valid.intersects(&bounds));
    let original = bounds;
    assert_eq!(bounds.encapsulate(Vector2::new(-1, 2)), None);
    assert_eq!(bounds, original);
}

#[test]
fn midpoint_avoids_overflow_and_preserves_tiny_equal_values() {
    let bounds = Bounds2::new(
        Vector2::new(-f64::MAX, f64::MAX),
        Vector2::new(f64::MAX, f64::MAX),
    )
    .unwrap();
    assert_eq!(bounds.center(), Some(Vector2::new(0.0, f64::MAX)));
    assert_eq!(bounds.size(), Some(Vector2::new(f64::INFINITY, 0.0)));
    let tiny = f64::from_bits(1);
    let point = Vector2::new(tiny, -tiny);
    let bounds = Bounds2::new(point, point).unwrap();
    assert_eq!(bounds.center(), Some(point));
    assert_eq!(bounds.size(), Some(Vector2::zero()));
    let bounds = Bounds3::new(Vector3::new(i8::MIN, 0, 0), Vector3::new(i8::MAX, 0, 0)).unwrap();
    assert_eq!(bounds.center(), Some(Vector3::new(-0.5, 0.0, 0.0)));
    assert_eq!(bounds.size(), Some(Vector3::new(255.0, 0.0, 0.0)));
}

#[test]
fn bounds_are_copy_comparable_and_hashable() {
    let bounds = Bounds3::new(Vector3::new(0, 0, 0), Vector3::new(1, 2, 3)).unwrap();
    let copied = bounds;
    assert_eq!(bounds, copied);
    let mut set = HashSet::new();
    assert!(set.insert(bounds));
    assert!(!set.insert(copied));
    assert!(set.contains(&bounds));
}
