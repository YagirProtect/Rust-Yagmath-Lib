use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn division() {
    let result = (Vector2::new(10_i32, -9) / Vector2::new(2, 3)).unwrap();
    assert_eq!((result.x, result.y), (5, -3));

    let result = (Vector3::new(12_u8, 15, 20) / Vector3::new(3, 5, 2)).unwrap();
    assert_eq!((result.x, result.y, result.z), (4, 3, 10));

    let result =
        (Vector4::new(5.0_f64, 9.0, -8.0, 0.0) / Vector4::new(2.0, 3.0, 4.0, 2.0)).unwrap();
    assert_eq!(
        (result.x, result.y, result.z, result.w),
        (2.5, 3.0, -2.0, 0.0)
    );
}

#[test]
fn division_returns_none_if_any_component_fails() {
    assert!((Vector2::new(10_i32, 20) / Vector2::new(0, 2)).is_none());
    assert!((Vector3::new(10_i32, i32::MIN, 30) / Vector3::new(2, -1, 3)).is_none());
    assert!((Vector4::new(10_u32, 20, 30, 40) / Vector4::new(2, 2, 3, 0)).is_none());
    assert!((Vector2::new(1.0_f32, 2.0) / Vector2::new(1.0, -0.0)).is_none());
    assert!((Vector2::new(f64::MAX, 2.0) / Vector2::new(0.5, 1.0)).is_none());
}
