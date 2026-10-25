use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn creation_sub() {
    let result = Vector2::new(10_i32, 2) - Vector2::new(3, 5);
    assert_eq!((result.x, result.y), (7, -3));

    let result = Vector3::new(10_i32, 2, 8) - Vector3::new(3, 5, 8);
    assert_eq!((result.x, result.y, result.z), (7, -3, 0));

    let result = Vector4::new(10_i32, 2, 8, -4) - Vector4::new(3, 5, 8, 6);
    assert_eq!((result.x, result.y, result.z, result.w), (7, -3, 0, -10));
}

#[test]
fn subtraction_saturates() {
    let result = Vector2::new(i8::MIN, i8::MAX) - Vector2::new(1, -1);
    assert_eq!((result.x, result.y), (i8::MIN, i8::MAX));

    let result = Vector3::new(0_u8, 3, 10) - Vector3::new(1, 5, 4);
    assert_eq!((result.x, result.y, result.z), (0, 0, 6));

    let result = Vector4::new(0_usize, 3, 10, usize::MAX) - Vector4::new(1, 5, 4, usize::MAX);
    assert_eq!((result.x, result.y, result.z, result.w), (0, 0, 6, 0));
}

#[test]
fn subtraction_floats() {
    let result = Vector2::new(1.5_f32, -2.0) - Vector2::new(0.5, 3.0);
    assert_eq!((result.x, result.y), (1.0, -5.0));

    let result = Vector3::new(1.5_f64, -2.0, 4.0) - Vector3::new(0.5, 3.0, 4.0);
    assert_eq!((result.x, result.y, result.z), (1.0, -5.0, 0.0));
}
