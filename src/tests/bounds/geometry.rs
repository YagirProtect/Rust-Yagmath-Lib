use crate::bounds::{Bounds2, Bounds3};
use crate::vector::{Vector2, Vector3};

#[test]
fn contains_includes_corners_edges_faces_and_zero_sized_bounds() {
    let bounds = Bounds2::new(Vector2::new(0, 0), Vector2::new(10, 20)).unwrap();
    for point in [
        Vector2::new(0, 0),
        Vector2::new(10, 20),
        Vector2::new(0, 20),
        Vector2::new(10, 0),
        Vector2::new(5, 10),
    ] {
        assert!(bounds.contains(point));
    }
    for point in [
        Vector2::new(-1, 10),
        Vector2::new(11, 10),
        Vector2::new(5, -1),
        Vector2::new(5, 21),
    ] {
        assert!(!bounds.contains(point));
    }
    let bounds = Bounds3::new(Vector3::new(-1, -2, -3), Vector3::new(1, 2, 3)).unwrap();
    assert!(bounds.contains(Vector3::new(-1, -2, -3)));
    assert!(bounds.contains(Vector3::new(1, 2, 3)));
    assert!(bounds.contains(Vector3::new(0, 0, 3)));
    assert!(!bounds.contains(Vector3::new(0, 0, 4)));
    let point = Bounds3::new(Vector3::new(1, 2, 3), Vector3::new(1, 2, 3)).unwrap();
    assert!(point.contains(Vector3::new(1, 2, 3)));
    assert!(!point.contains(Vector3::new(1, 2, 4)));
}

#[test]
fn intersections_include_touching_and_containment() {
    let a = Bounds2::new(Vector2::new(0, 0), Vector2::new(10, 10)).unwrap();
    for b in [
        Bounds2::new(Vector2::new(5, 5), Vector2::new(15, 15)).unwrap(),
        Bounds2::new(Vector2::new(2, 2), Vector2::new(3, 3)).unwrap(),
        Bounds2::new(Vector2::new(10, 10), Vector2::new(20, 20)).unwrap(),
        Bounds2::new(Vector2::new(5, 5), Vector2::new(5, 5)).unwrap(),
    ] {
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }
    let b = Bounds2::new(Vector2::new(11, 0), Vector2::new(20, 10)).unwrap();
    assert!(!a.intersects(&b));
    assert!(!b.intersects(&a));

    let a = Bounds3::new(Vector3::new(0, 0, 0), Vector3::new(10, 10, 10)).unwrap();
    for axis in 0..3 {
        let mut min = [0; 3];
        min[axis] = 11;
        let b = Bounds3::new(
            Vector3::new(min[0], min[1], min[2]),
            Vector3::new(20, 20, 20),
        )
        .unwrap();
        assert!(!a.intersects(&b));
        assert!(!b.intersects(&a));
    }
    let touching = Bounds3::new(Vector3::new(10, 10, 10), Vector3::new(20, 20, 20)).unwrap();
    assert!(a.intersects(&touching));
    assert!(a.intersects(&a));
}

#[test]
fn encapsulate_expands_only_necessary_components() {
    let mut bounds = Bounds2::new(Vector2::new(0, 0), Vector2::new(10, 20)).unwrap();
    assert_eq!(bounds.encapsulate(Vector2::new(-5, 30)), Some(()));
    assert_eq!(bounds.min, Vector2::new(-5, 0));
    assert_eq!(bounds.max, Vector2::new(10, 30));
    assert!(bounds.contains(Vector2::new(-5, 30)));
    let original = bounds;
    assert_eq!(bounds.encapsulate(Vector2::new(5, 10)), Some(()));
    assert_eq!(bounds, original);

    let mut bounds = Bounds3::<i8>::default();
    bounds
        .encapsulate(Vector3::new(i8::MIN, 4, i8::MAX))
        .unwrap();
    assert_eq!(bounds.min, Vector3::new(i8::MIN, 0, 0));
    assert_eq!(bounds.max, Vector3::new(0, 4, i8::MAX));
    bounds
        .encapsulate(Vector3::new(i8::MAX, -4, i8::MIN))
        .unwrap();
    assert_eq!(bounds.min, Vector3::new(i8::MIN, -4, i8::MIN));
    assert_eq!(bounds.max, Vector3::new(i8::MAX, 4, i8::MAX));
}

#[test]
fn non_finite_points_are_rejected_without_partial_expansion() {
    let mut bounds =
        Bounds3::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0)).unwrap();
    let original = bounds;
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let point = Vector3::new(-10.0, 10.0, invalid);
        assert!(!bounds.contains(point));
        assert_eq!(bounds.encapsulate(point), None);
        assert_eq!(bounds, original);
    }
}

#[test]
fn containment_uses_exact_integer_comparisons_for_wide_types() {
    let mut bounds =
        Bounds2::new(Vector2::new(u128::MAX - 1, 0), Vector2::new(u128::MAX, 1)).unwrap();
    assert!(!bounds.contains(Vector2::new(u128::MAX - 2, 0)));
    assert!(bounds.contains(Vector2::new(u128::MAX - 1, 0)));
    bounds.encapsulate(Vector2::new(u128::MAX - 2, 2)).unwrap();
    assert_eq!(bounds.min.x, u128::MAX - 2);
    assert_eq!(bounds.max.y, 2);
}
