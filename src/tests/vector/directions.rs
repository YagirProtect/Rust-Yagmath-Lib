use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn direction_type_can_be_inferred_from_context() {
    let a: Vector2<i16> = Vector2::left();
    let b: Vector3<f32> = Vector3::up();
    let c: Vector4<u32> = Vector4::forward();
    assert_eq!(a, Vector2::new(-1, 0));
    assert_eq!(b, Vector3::new(0.0, 1.0, 0.0));
    assert_eq!(c, Vector4::new(0, 0, 1, 0));
}

#[test]
fn zero_and_positive_directions_support_every_numeric_type() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            assert_eq!(Vector2::<$type>::zero(), Vector2::new(0 as $type, 0 as $type));
            assert_eq!(Vector2::<$type>::right(), Vector2::new(1 as $type, 0 as $type));
            assert_eq!(Vector2::<$type>::up(), Vector2::new(0 as $type, 1 as $type));

            assert_eq!(Vector3::<$type>::zero(), Vector3::new(0 as $type, 0 as $type, 0 as $type));
            assert_eq!(Vector3::<$type>::right(), Vector3::new(1 as $type, 0 as $type, 0 as $type));
            assert_eq!(Vector3::<$type>::up(), Vector3::new(0 as $type, 1 as $type, 0 as $type));
            assert_eq!(Vector3::<$type>::forward(), Vector3::new(0 as $type, 0 as $type, 1 as $type));

            assert_eq!(Vector4::<$type>::zero(), Vector4::new(0 as $type, 0 as $type, 0 as $type, 0 as $type));
            assert_eq!(Vector4::<$type>::right(), Vector4::new(1 as $type, 0 as $type, 0 as $type, 0 as $type));
            assert_eq!(Vector4::<$type>::up(), Vector4::new(0 as $type, 1 as $type, 0 as $type, 0 as $type));
            assert_eq!(Vector4::<$type>::forward(), Vector4::new(0 as $type, 0 as $type, 1 as $type, 0 as $type));
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn negative_directions_support_signed_integers_and_floats() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            assert_eq!(Vector2::<$type>::left(), Vector2::new(-1 as $type, 0 as $type));
            assert_eq!(Vector2::<$type>::down(), Vector2::new(0 as $type, -1 as $type));

            assert_eq!(Vector3::<$type>::left(), Vector3::new(-1 as $type, 0 as $type, 0 as $type));
            assert_eq!(Vector3::<$type>::down(), Vector3::new(0 as $type, -1 as $type, 0 as $type));
            assert_eq!(Vector3::<$type>::backward(), Vector3::new(0 as $type, 0 as $type, -1 as $type));

            assert_eq!(Vector4::<$type>::left(), Vector4::new(-1 as $type, 0 as $type, 0 as $type, 0 as $type));
            assert_eq!(Vector4::<$type>::down(), Vector4::new(0 as $type, -1 as $type, 0 as $type, 0 as $type));
            assert_eq!(Vector4::<$type>::backward(), Vector4::new(0 as $type, 0 as $type, -1 as $type, 0 as $type));
        )+};
    }
    check_types!(i8, i16, i32, i64, i128, isize, f32, f64);
}

#[test]
fn opposite_directions_cancel_and_axes_are_orthogonal() {
    assert_eq!(Vector2::<i32>::up() + Vector2::down(), Vector2::zero());
    assert_eq!(Vector3::<f32>::left() + Vector3::right(), Vector3::zero());
    assert_eq!(
        Vector4::<f64>::forward() + Vector4::backward(),
        Vector4::zero()
    );
    assert_eq!(Vector3::<f64>::up().dot(Vector3::right()), 0.0);
    assert_eq!(Vector3::<f64>::up().dot(Vector3::forward()), 0.0);
    assert_eq!(Vector3::<f64>::right().dot(Vector3::forward()), 0.0);
    assert_eq!(
        Vector3::<f64>::right().cross(Vector3::up()),
        Vector3::forward()
    );
}

#[test]
fn float_directions_have_unit_length() {
    assert_eq!(Vector2::<f32>::up().length(), 1.0);
    assert_eq!(Vector2::<f64>::left().length(), 1.0);
    assert_eq!(Vector3::<f32>::backward().length(), 1.0);
    assert_eq!(Vector3::<f64>::right().length(), 1.0);
    assert_eq!(Vector4::<f32>::down().length(), 1.0);
    assert_eq!(Vector4::<f64>::forward().length(), 1.0);
}
