use crate::{Plane, vector::Vector3};

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
}

fn vector_close(actual: Vector3<f64>, expected: Vector3<f64>) {
    close(actual.x, expected.x);
    close(actual.y, expected.y);
    close(actual.z, expected.z);
}

#[test]
fn normal_point_constructor_and_signed_distance() {
    let plane = Plane::from_normal_point(Vector3::new(0, 5, 0), Vector3::new(0, 10, 0)).unwrap();
    assert_eq!(plane.normal(), Vector3::new(0.0, 1.0, 0.0));
    assert_eq!(plane.distance(), -10.0);
    close(plane.signed_distance(Vector3::new(0, 13, 0)), 3.0);
    close(plane.signed_distance(Vector3::new(0, 8, 0)), -2.0);
    close(plane.signed_distance(Vector3::new(3, 10, -4)), 0.0);
    let plane =
        Plane::from_normal_point(Vector3::new(1.0, 2.0, 3.0), Vector3::new(2, -4, 6)).unwrap();
    close(plane.normal().length(), 1.0);
    close(plane.signed_distance(Vector3::new(2, -4, 6)), 0.0);
}

#[test]
fn constructors_reject_invalid_geometry() {
    assert!(Plane::from_normal_point(Vector3::new(0, 0, 0), Vector3::new(0, 0, 0)).is_none());
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            Plane::from_normal_point(Vector3::new(bad, 0.0, 0.0), Vector3::new(0, 0, 0)).is_none()
        );
        assert!(
            Plane::from_normal_point(Vector3::new(0, 1, 0), Vector3::new(0.0, bad, 0.0)).is_none()
        );
        assert!(
            Plane::from_points(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, bad, 0.0)
            )
            .is_none()
        );
    }
    assert!(
        Plane::from_points(
            Vector3::new(0, 0, 0),
            Vector3::new(0, 0, 0),
            Vector3::new(1, 1, 1)
        )
        .is_none()
    );
    assert!(
        Plane::from_points(
            Vector3::new(0, 0, 0),
            Vector3::new(1, 1, 1),
            Vector3::new(2, 2, 2)
        )
        .is_none()
    );
    assert!(
        Plane::from_normal_point(
            Vector3::new(1, 1, 1),
            Vector3::new(f64::MAX, f64::MAX, f64::MAX)
        )
        .is_none()
    );
}

#[test]
fn three_points_define_orientation() {
    let a = Vector3::new(0, 0, 3);
    let b = Vector3::new(1, 0, 3);
    let c = Vector3::new(0, 1, 3);
    let plane = Plane::from_points(a, b, c).unwrap();
    assert_eq!(plane.normal(), Vector3::new(0.0, 0.0, 1.0));
    assert_eq!(plane.distance(), -3.0);
    for point in [a, b, c] {
        close(plane.signed_distance(point), 0.0);
    }
    assert_eq!(Plane::from_points(a, c, b).unwrap(), plane.flipped());
}

#[test]
fn closest_point_and_vector_projection_have_different_offsets() {
    let plane = Plane::from_normal_point(Vector3::new(0, 1, 0), Vector3::new(0, 10, 0)).unwrap();
    vector_close(
        plane.closest_point(Vector3::new(2, 13, 4)),
        Vector3::new(2.0, 10.0, 4.0),
    );
    vector_close(
        plane.closest_point(Vector3::new(2, 8, 4)),
        Vector3::new(2.0, 10.0, 4.0),
    );
    vector_close(
        plane.project_vector(Vector3::new(2, 13, 4)),
        Vector3::new(2.0, 0.0, 4.0),
    );
    let plane = Plane::from_normal_point(Vector3::new(1, 2, 3), Vector3::new(4, 5, 6)).unwrap();
    let point = Vector3::new(-2.0, 7.0, 1.0);
    let closest = plane.closest_point(point);
    close(plane.signed_distance(closest), 0.0);
    close(point.distance(closest), plane.signed_distance(point).abs());
    close(plane.normal().dot(plane.project_vector(point)), 0.0);
    vector_close(plane.closest_point(closest), closest);
}

#[test]
fn flipped_plane_preserves_geometry_and_reverses_sign() {
    let plane = Plane::from_normal_point(Vector3::new(1, 2, 3), Vector3::new(4, 5, 6)).unwrap();
    let flipped = plane.flipped();
    let point = Vector3::new(10, 20, 30);
    close(
        flipped.signed_distance(point),
        -plane.signed_distance(point),
    );
    vector_close(flipped.closest_point(point), plane.closest_point(point));
    vector_close(flipped.project_vector(point), plane.project_vector(point));
    assert_eq!(flipped.flipped(), plane);
    assert_ne!(plane, flipped);
    let copied = plane;
    assert_eq!(copied, plane);
}

#[test]
fn huge_and_subnormal_geometry() {
    for scale in [f64::MAX, f64::from_bits(1)] {
        let plane =
            Plane::from_normal_point(Vector3::new(scale, scale, 0.0), Vector3::new(0, 0, 0))
                .unwrap();
        close(plane.normal().length(), 1.0);
        close(plane.signed_distance(Vector3::new(1, -1, 0)), 0.0);
        let plane = Plane::from_points(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(scale, 0.0, 0.0),
            Vector3::new(0.0, scale, 0.0),
        )
        .unwrap();
        assert_eq!(plane.normal(), Vector3::new(0.0, 0.0, 1.0));
    }
    let plane = Plane::from_points(
        Vector3::new(-f64::MAX, 0.0, 0.0),
        Vector3::new(f64::MAX, 0.0, 0.0),
        Vector3::new(0.0, f64::MAX, 0.0),
    )
    .unwrap();
    assert_eq!(plane.normal(), Vector3::new(0.0, 0.0, 1.0));
    let plane = Plane::from_normal_point(Vector3::new(1, 1, 1), Vector3::new(0, 0, 0)).unwrap();
    let projected = plane.closest_point(Vector3::new(f64::MAX, f64::MAX, f64::MAX));
    assert!(projected.x.is_finite() && projected.y.is_finite() && projected.z.is_finite());
    assert!((projected.x / f64::MAX).abs() < 1e-14);
}

#[test]
fn invalid_queries_and_projection_overflow() {
    let plane = Plane::from_normal_point(Vector3::new(0, 1, 0), Vector3::new(0, 0, 0)).unwrap();
    assert!(
        plane
            .signed_distance(Vector3::new(0.0, f64::NAN, 0.0))
            .is_nan()
    );
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let v = plane.closest_point(Vector3::new(1.0, bad, 0.0));
        assert!(v.x.is_nan() && v.y.is_nan() && v.z.is_nan());
        let v = plane.project_vector(Vector3::new(1.0, bad, 0.0));
        assert!(v.x.is_nan() && v.y.is_nan() && v.z.is_nan());
    }
    let plane = Plane::from_normal_point(Vector3::new(1, -2, 0), Vector3::new(0, 0, 0)).unwrap();
    let v = plane.project_vector(Vector3::new(f64::MAX, f64::MAX, 0.0));
    assert!(v.x.is_nan() && v.y.is_nan() && v.z.is_nan());
}

#[test]
fn all_numeric_input_types_are_supported() {
    macro_rules! check {
        ($($type:ty),+) => { $(
            let plane = Plane::from_normal_point(
                Vector3::new(0 as $type, 1 as $type, 0 as $type),
                Vector3::new(0 as $type, 10 as $type, 0 as $type),
            ).unwrap();
            close(plane.signed_distance(Vector3::new(0 as $type, 13 as $type, 0 as $type)), 3.0);
            vector_close(plane.closest_point(Vector3::new(0 as $type, 13 as $type, 0 as $type)), Vector3::new(0.0, 10.0, 0.0));
        )+ };
    }
    check!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}
