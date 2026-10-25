use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn dot_uses_all_components() {
    assert_eq!(Vector2::new(2, -3).dot(Vector2::new(5, 7)), -11.0);
    assert_eq!(Vector3::new(2, -3, 4).dot(Vector3::new(5, 7, 9)), 25.0);
    assert_eq!(Vector4::new(2, -3, 4, -5).dot(Vector4::new(5, 7, 9, 11)), -30.0);
}

#[test]
fn dot_sign_identifies_front_back_and_side() {
    let forward = Vector3::new(0.0, 0.0, 1.0);
    assert!(forward.dot(Vector3::new(3.0, 0.0, 10.0)) > 0.0);
    assert!(forward.dot(Vector3::new(3.0, 0.0, -10.0)) < 0.0);
    assert_eq!(forward.dot(Vector3::new(3.0, 0.0, 0.0)), 0.0);
    assert_eq!(forward.dot(Vector3::new(0.0, 0.0, 0.0)), 0.0);
}

#[test]
fn dot_is_not_limited_to_one_without_normalization() {
    let a = Vector2::new(3.0_f32, 4.0);
    assert_eq!(a.dot(a), 25.0);
    let unit = a.normalized().unwrap();
    assert!((unit.dot(unit) - 1.0).abs() < 1e-12);
}

#[test]
fn dot_is_symmetric_and_converts_before_multiplying() {
    let a = Vector4::new(100_i8, -100, 100, -100);
    let b = Vector4::new(100_i8, 100, 100, 100);
    assert_eq!(a.dot(b), b.dot(a));
    assert_eq!(a.dot(b), 0.0);
    assert_eq!(a.dot(a), 40000.0);
    assert_eq!(Vector2::new(0.5_f64, 1.5).dot(Vector2::new(2.0, 4.0)), 7.0);
}
