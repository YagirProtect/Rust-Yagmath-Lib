use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn distance_and_squared_distance_use_all_components() {
    let a = Vector2::new(1, 2);
    let b = Vector2::new(4, 6);
    assert_eq!(a.distance(b), 5.0);
    assert_eq!(a.distance_squared(b), 25.0);

    let a = Vector3::new(1, 2, 3);
    let b = Vector3::new(3, 5, 9);
    assert_eq!(a.distance(b), 7.0);
    assert_eq!(a.distance_squared(b), 49.0);

    let a = Vector4::new(1, 2, 3, 4);
    let b = Vector4::new(2, 4, 5, 8);
    assert_eq!(a.distance(b), 5.0);
    assert_eq!(a.distance_squared(b), 25.0);
}

#[test]
fn distance_is_symmetric_and_zero_for_identical_points() {
    let a = Vector3::new(-1.5_f64, 2.0, -3.0);
    let b = Vector3::new(2.5_f64, -1.0, 9.0);
    assert_eq!(a.distance(b), b.distance(a));
    assert_eq!(a.distance_squared(b), b.distance_squared(a));
    assert_eq!(a.distance(a), 0.0);
    assert_eq!(a.distance_squared(a), 0.0);
    assert_eq!(Vector2::new(0.0_f32, 0.0).distance(Vector2::new(0.5, 0.0)), 0.5);
}

#[test]
fn integer_coordinates_are_converted_before_subtraction_and_squaring() {
    let a = Vector2::new(i8::MIN, 0);
    let b = Vector2::new(i8::MAX, 0);
    assert_eq!(a.distance(b), 255.0);
    assert_eq!(a.distance_squared(b), 65025.0);

    let a = Vector2::new(0_u8, 0);
    let b = Vector2::new(255_u8, 0);
    assert_eq!(a.distance(b), 255.0);
    assert_eq!(b.distance(a), 255.0);
    assert_eq!(a.distance_squared(b), 65025.0);
}

#[test]
fn distance_avoids_intermediate_square_overflow() {
    let a = Vector2::new(0.0_f64, 0.0);
    let b = Vector2::new(3e200_f64, 4e200);
    assert!((a.distance(b) / 5e200 - 1.0).abs() < 1e-12);
    assert_eq!(a.distance_squared(b), f64::INFINITY);
}
