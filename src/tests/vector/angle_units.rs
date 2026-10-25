use crate::vector::{Vector2, Vector3, Vector4};

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
}

#[test]
fn vector_angles_have_degree_and_radian_variants() {
    let x = Vector2::<i32>::right();
    let y = Vector2::<i32>::up();
    assert_eq!(x.angle_between_deg(y), 90.0);
    assert_eq!(x.signed_angle_deg(y), 90.0);
    assert_eq!(y.signed_angle_deg(x), -90.0);
    assert_eq!(x.signed_angle_deg(Vector2::<i32>::left()), 180.0);
    assert_eq!(
        Vector3::<f32>::right().angle_between_deg(Vector3::<i32>::up()),
        90.0
    );
    assert_eq!(
        Vector3::<f64>::right().signed_angle_deg(Vector3::<i32>::up(), Vector3::<i32>::backward()),
        -90.0
    );
    assert_eq!(
        Vector4::<i32>::right().angle_between_deg(Vector4::<i32>::up()),
        90.0
    );
    assert!(x.angle_between_deg(Vector2::<i32>::zero()).is_nan());
    assert!(x.signed_angle_deg(Vector2::<i32>::zero()).is_nan());
}

#[test]
fn copy_rotation_in_degrees_works_in_every_dimension() {
    let v = Vector2::new(1, 0).rotated_deg(90.0).unwrap();
    close(v.x, 0.0);
    close(v.y, 1.0);
    let v = Vector3::new(1, 0, 0)
        .rotated_deg(90.0, Vector3::<i32>::forward())
        .unwrap();
    close(v.x, 0.0);
    close(v.y, 1.0);
    close(v.z, 0.0);
    let v = Vector4::new(1, 0, 7, 8).rotated_deg(90.0, (0, 1)).unwrap();
    close(v.x, 0.0);
    close(v.y, 1.0);
    assert_eq!((v.z, v.w), (7.0, 8.0));
    let v = Vector2::new(1, 0).rotated_deg(360.0 * 1e12 + 90.0).unwrap();
    close(v.y, 1.0);
    assert_eq!(
        Vector2::new(3, 4).rotated_deg(360.0),
        Some(Vector2::new(3.0, 4.0))
    );
}

#[test]
fn mutating_degree_rotation_preserves_float_type_and_validates_inputs() {
    macro_rules! check {
        ($type:ty) => {{
            let mut v = Vector2::new(1 as $type, 0 as $type);
            v.rotate_deg(90.0).unwrap();
            assert!((v.y - 1.0).abs() < 1e-6);
            let before = v;
            assert_eq!(v.rotate_deg(f64::NAN), None);
            assert_eq!(v, before);
            let mut v = Vector3::new(1 as $type, 0 as $type, 0 as $type);
            v.rotate_deg(90.0, Vector3::<i32>::forward()).unwrap();
            assert!((v.y - 1.0).abs() < 1e-6);
            let before = v;
            assert_eq!(v.rotate_deg(90.0, Vector3::<i32>::zero()), None);
            assert_eq!(v, before);
            let mut v = Vector4::new(1 as $type, 0 as $type, 0 as $type, 0 as $type);
            v.rotate_deg(90.0, (0, 3)).unwrap();
            assert!((v.w - 1.0).abs() < 1e-6);
            let before = v;
            assert_eq!(v.rotate_deg(90.0, (0, 4)), None);
            assert_eq!(v, before);
        }};
    }
    check!(f32);
    check!(f64);
    assert!(Vector2::new(1, 0).rotated_deg(f64::INFINITY).is_none());
    assert!(
        Vector3::new(1, 0, 0)
            .rotated_deg(90.0, Vector3::<i32>::zero())
            .is_none()
    );
    assert!(Vector4::new(1, 0, 0, 0).rotated_deg(90.0, (0, 4)).is_none());
}
