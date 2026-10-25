use std::string::String;
use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn constructors_preserve_every_component_for_all_numeric_types() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let v = Vector2::new(1 as $type, 2 as $type);
            assert_eq!((v.x, v.y), (1 as $type, 2 as $type));
            let v = Vector3::new(1 as $type, 2 as $type, 3 as $type);
            assert_eq!((v.x, v.y, v.z), (1 as $type, 2 as $type, 3 as $type));
            let v = Vector4::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
            assert_eq!((v.x, v.y, v.z, v.w), (1 as $type, 2 as $type, 3 as $type, 4 as $type));
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn default_is_zero_for_all_numeric_types_and_dimensions() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            assert_eq!(Vector2::<$type>::default(), Vector2::zero());
            assert_eq!(Vector3::<$type>::default(), Vector3::zero());
            assert_eq!(Vector4::<$type>::default(), Vector4::zero());
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn constructor_and_default_do_not_require_numeric_components() {
    let v = Vector2::new(String::from("x"), String::from("y"));
    assert_eq!((v.x.as_str(), v.y.as_str()), ("x", "y"));
    assert_eq!(
        Vector3::<String>::default(),
        Vector3::new(String::new(), String::new(), String::new())
    );
    let v = Vector4::new(false, true, false, true);
    assert_eq!((v.x, v.y, v.z, v.w), (false, true, false, true));
}

#[test]
fn public_fields_can_be_updated_independently() {
    let mut v = Vector4::new(1, 2, 3, 4);
    v.x = 5;
    v.y = 6;
    v.z = 7;
    v.w = 8;
    assert_eq!(v, Vector4::new(5, 6, 7, 8));
}
