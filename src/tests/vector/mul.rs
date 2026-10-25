use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn multiplication() {
    let result = Vector2::new(2_i32, -3) * Vector2::new(5, 7);
    assert_eq!((result.x, result.y), (10, -21));

    let result = Vector3::new(2_u32, 3, 4) * Vector3::new(5, 7, 9);
    assert_eq!((result.x, result.y, result.z), (10, 21, 36));

    let result = Vector4::new(2_i32, -3, 4, 0) * Vector4::new(5, 7, -9, 11);
    assert_eq!((result.x, result.y, result.z, result.w), (10, -21, -36, 0));
}

#[test]
fn multiplication_saturates() {
    let result = Vector2::new(i8::MIN, i8::MAX) * Vector2::new(-1, 2);
    assert_eq!((result.x, result.y), (i8::MAX, i8::MAX));

    let result = Vector3::new(u8::MAX, 0_u8, 3) * Vector3::new(2, 10, 4);
    assert_eq!((result.x, result.y, result.z), (u8::MAX, 0, 12));

    let result = Vector4::new(i32::MIN, i32::MAX, -3, 4) * Vector4::new(2, 2, 5, -6);
    assert_eq!(
        (result.x, result.y, result.z, result.w),
        (i32::MIN, i32::MAX, -15, -24)
    );
}

#[test]
fn multiplication_floats() {
    let result = Vector2::new(1.5_f32, -2.0) * Vector2::new(2.0, 3.0);
    assert_eq!((result.x, result.y), (3.0, -6.0));

    let result = Vector3::new(1.5_f64, -2.0, 4.0) * Vector3::new(2.0, 3.0, 0.5);
    assert_eq!((result.x, result.y, result.z), (3.0, -6.0, 2.0));
}
