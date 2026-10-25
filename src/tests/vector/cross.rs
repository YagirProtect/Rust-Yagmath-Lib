use crate::vector::{Vector2, Vector3};

#[test]
fn cross_supports_every_numeric_type_and_returns_f64() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let result: f64 = Vector2::new(2 as $type, 3 as $type)
                .cross(Vector2::new(5 as $type, 7 as $type));
            assert_eq!(result, -1.0);

            let result: Vector3<f64> = Vector3::new(2 as $type, 3 as $type, 4 as $type)
                .cross(Vector3::new(5 as $type, 7 as $type, 9 as $type));
            assert_eq!(result, Vector3::new(-1.0, 2.0, -1.0));
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn cross_2d_sign_and_order() {
    let x = Vector2::new(1, 0);
    let y = Vector2::new(0, 1);
    assert_eq!(x.cross(y), 1.0);
    assert_eq!(y.cross(x), -1.0);
    assert_eq!(x.cross(Vector2::new(0, -1)), -1.0);

    let a = Vector2::new(3, -4);
    let b = Vector2::new(-2, 5);
    assert_eq!(a.cross(b), 7.0);
    assert_eq!(a.cross(b), -b.cross(a));
}

#[test]
fn cross_3d_follows_right_hand_rule() {
    let x = Vector3::new(1, 0, 0);
    let y = Vector3::new(0, 1, 0);
    let z = Vector3::new(0, 0, 1);
    assert_eq!(x.cross(y), Vector3::new(0.0, 0.0, 1.0));
    assert_eq!(y.cross(z), Vector3::new(1.0, 0.0, 0.0));
    assert_eq!(z.cross(x), Vector3::new(0.0, 1.0, 0.0));
    assert_eq!(y.cross(x), Vector3::new(0.0, 0.0, -1.0));
    assert_eq!(z.cross(y), Vector3::new(-1.0, 0.0, 0.0));
    assert_eq!(x.cross(z), Vector3::new(0.0, -1.0, 0.0));
}

#[test]
fn cross_is_zero_for_zero_and_parallel_vectors() {
    let a = Vector2::new(3, -4);
    assert_eq!(a.cross(a), 0.0);
    assert_eq!(a.cross(Vector2::new(6, -8)), 0.0);
    assert_eq!(a.cross(Vector2::new(-6, 8)), 0.0);
    assert_eq!(a.cross(Vector2::new(0, 0)), 0.0);
    assert_eq!(Vector2::new(0, 0).cross(a), 0.0);

    let a = Vector3::new(2, -3, 4);
    let zero = Vector3::new(0.0, 0.0, 0.0);
    assert_eq!(a.cross(a), zero);
    assert_eq!(a.cross(Vector3::new(4, -6, 8)), zero);
    assert_eq!(a.cross(Vector3::new(-4, 6, -8)), zero);
    assert_eq!(a.cross(Vector3::new(0, 0, 0)), zero);
    assert_eq!(Vector3::new(0, 0, 0).cross(a), zero);
}

#[test]
fn cross_3d_is_perpendicular_and_antisymmetric() {
    let a = Vector3::new(3.0_f64, -3.0, 1.0);
    let b = Vector3::new(4.0_f64, 9.0, 2.0);
    let result = a.cross(b);
    assert_eq!(result, Vector3::new(-15.0, -2.0, 39.0));
    assert!(result.dot(a).abs() < 1e-12);
    assert!(result.dot(b).abs() < 1e-12);
    let reverse = b.cross(a);
    assert_eq!(result, Vector3::new(-reverse.x, -reverse.y, -reverse.z));
}

#[test]
fn cross_converts_integers_before_multiplication_and_subtraction() {
    assert_eq!(
        Vector2::new(0_u8, 255).cross(Vector2::new(255, 0)),
        -65025.0
    );
    assert_eq!(
        Vector2::new(i8::MIN, i8::MAX).cross(Vector2::new(i8::MAX, i8::MIN)),
        255.0
    );
    assert_eq!(
        Vector3::new(255_u8, 0, 0).cross(Vector3::new(0, 255, 0)),
        Vector3::new(0.0, 0.0, 65025.0)
    );
}

#[test]
fn cross_preserves_fractional_values_and_inputs() {
    let a = Vector2::new(0.5_f32, 1.5);
    let b = Vector2::new(2.0_f32, 0.5);
    assert_eq!(a.cross(b), -2.75);
    assert_eq!(a, Vector2::new(0.5, 1.5));
    assert_eq!(b, Vector2::new(2.0, 0.5));

    let a = Vector3::new(0.5_f64, 1.5, -0.5);
    let b = Vector3::new(2.0_f64, 0.5, 1.0);
    assert_eq!(a.cross(b), Vector3::new(1.75, -1.5, -2.75));
    assert_eq!(a, Vector3::new(0.5, 1.5, -0.5));
    assert_eq!(b, Vector3::new(2.0, 0.5, 1.0));
}
