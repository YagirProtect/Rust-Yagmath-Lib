use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn addition_supports_all_types_and_dimensions() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            assert_eq!(Vector2::new(1 as $type, 2 as $type) + Vector2::new(3, 5), Vector2::new(4, 7));
            assert_eq!(Vector3::new(1 as $type, 2 as $type, 3 as $type) + Vector3::new(3, 5, 7), Vector3::new(4, 7, 10));
            assert_eq!(Vector4::new(1 as $type, 2 as $type, 3 as $type, 4 as $type) + Vector4::new(3, 5, 7, 9), Vector4::new(4, 7, 10, 13));
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );
    assert_eq!(
        Vector2::new(1.5_f32, -2.5) + Vector2::new(2.0, 1.0),
        Vector2::new(3.5, -1.5)
    );
    assert_eq!(
        Vector3::new(1.5_f64, -2.5, 0.0) + Vector3::new(2.0, 1.0, 0.5),
        Vector3::new(3.5, -1.5, 0.5)
    );
    assert_eq!(
        Vector4::new(1.5_f64, -2.5, 0.0, 2.0) + Vector4::new(2.0, 1.0, 0.5, -1.0),
        Vector4::new(3.5, -1.5, 0.5, 1.0)
    );
}

#[test]
fn addition_saturates_each_component_independently() {
    assert_eq!(
        Vector2::new(i8::MAX, i8::MIN) + Vector2::new(1, -1),
        Vector2::new(i8::MAX, i8::MIN)
    );
    assert_eq!(
        Vector3::new(u8::MAX, 1, 0) + Vector3::new(1, 2, 3),
        Vector3::new(u8::MAX, 3, 3)
    );
    assert_eq!(
        Vector4::new(i32::MAX, i32::MIN, -3, 0) + Vector4::new(1, -1, 5, 7),
        Vector4::new(i32::MAX, i32::MIN, 2, 7)
    );
}

#[test]
fn addition_of_zero_preserves_values() {
    let a = Vector4::new(-1_i32, 2, -3, 4);
    assert_eq!(a + Vector4::zero(), a);
    assert_eq!(Vector4::zero() + a, a);
}

#[test]
fn creation_add() {
    let result = Vector2 { x: 1, y: 2 } + Vector2 { x: 10, y: 20 };

    assert_eq!((result.x, result.y), (11, 22));

    let result = Vector3 { x: 1, y: 2, z: 3 }
        + Vector3 {
            x: 10,
            y: 20,
            z: 30,
        };

    assert_eq!((result.x, result.y, result.z), (11, 22, 33));

    let result = Vector4 {
        x: 1,
        y: 2,
        z: 3,
        w: 4,
    } + Vector4 {
        x: 10,
        y: 20,
        z: 30,
        w: 40,
    };

    assert_eq!((result.x, result.y, result.z, result.w), (11, 22, 33, 44),);
}
