use crate::vector::{Vector2, Vector3, Vector4};

// Run the same boundary cases against every dimension and integer type.
macro_rules! check_integer_vector {
    ($type:ty, $vector:ident, $($field:ident),+) => {{
        let result = $vector { $($field: <$type>::MAX),+ }
            - $vector { $($field: <$type>::MAX),+ };
        $(assert_eq!(result.$field, 0);)+

        let result = $vector { $($field: <$type>::MIN),+ }
            - $vector { $($field: 1),+ };
        $(assert_eq!(result.$field, <$type>::MIN);)+

        let result = $vector { $($field: <$type>::MAX),+ } * 2;
        $(assert_eq!(result.$field, <$type>::MAX);)+

        let result = $vector { $($field: <$type>::MAX),+ } * 0;
        $(assert_eq!(result.$field, 0);)+

        let result = $vector { $($field: <$type>::MAX),+ }
            * $vector { $($field: 0),+ };
        $(assert_eq!(result.$field, 0);)+

        assert!(($vector { $($field: 10 as $type),+ } / 0).is_none());
        assert!(($vector { $($field: 0 as $type),+ } / 0).is_none());
        assert!(($vector { $($field: 0 as $type),+ }
            / $vector { $($field: 0),+ }).is_none());

        let result = ($vector { $($field: 0 as $type),+ } / 2).unwrap();
        $(assert_eq!(result.$field, 0);)+

        let result = ($vector { $($field: 10 as $type),+ } / 3).unwrap();
        $(assert_eq!(result.$field, 3);)+
    }};
}

#[test]
fn integer_boundaries_in_every_dimension() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            check_integer_vector!($type, Vector2, x, y);
            check_integer_vector!($type, Vector3, x, y, z);
            check_integer_vector!($type, Vector4, x, y, z, w);
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );
}

macro_rules! check_signed_vector {
    ($type:ty, $vector:ident, $($field:ident),+) => {{
        let result = $vector { $($field: <$type>::MAX),+ }
            - $vector { $($field: -1),+ };
        $(assert_eq!(result.$field, <$type>::MAX);)+

        let result = $vector { $($field: <$type>::MIN),+ } * -1;
        $(assert_eq!(result.$field, <$type>::MAX);)+

        let result = $vector { $($field: <$type>::MIN),+ } * 2;
        $(assert_eq!(result.$field, <$type>::MIN);)+

        assert!(($vector { $($field: <$type>::MIN),+ } / -1).is_none());
        assert!(($vector { $($field: <$type>::MIN),+ }
            / $vector { $($field: -1),+ }).is_none());

        let result = ($vector { $($field: -10 as $type),+ } / 3).unwrap();
        $(assert_eq!(result.$field, -3);)+
    }};
}

#[test]
fn signed_overflow_and_negative_division_in_every_dimension() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            check_signed_vector!($type, Vector2, x, y);
            check_signed_vector!($type, Vector3, x, y, z);
            check_signed_vector!($type, Vector4, x, y, z, w);
        )+};
    }
    check_types!(i8, i16, i32, i64, i128, isize);
}

#[test]
fn division_rejects_zero_in_each_individual_component() {
    for axis in 0..2 {
        let mut divisor = [1_i32; 2];
        divisor[axis] = 0;
        assert!((Vector2::new(10, 20) / Vector2::new(divisor[0], divisor[1])).is_none());
    }
    for axis in 0..3 {
        let mut divisor = [1_i32; 3];
        divisor[axis] = 0;
        assert!(
            (Vector3::new(10, 20, 30) / Vector3::new(divisor[0], divisor[1], divisor[2])).is_none()
        );
    }
    for axis in 0..4 {
        let mut divisor = [1_i32; 4];
        divisor[axis] = 0;
        assert!(
            (Vector4::new(10, 20, 30, 40)
                / Vector4::new(divisor[0], divisor[1], divisor[2], divisor[3]))
            .is_none()
        );
    }
}

macro_rules! check_float_vector {
    ($type:ty, $vector:ident, $($field:ident),+) => {{
        let result = $vector { $($field: 1.5 as $type),+ } * -2.0;
        $(assert_eq!(result.$field, -3.0);)+

        let result = ($vector { $($field: 5.0 as $type),+ } / 2.0).unwrap();
        $(assert_eq!(result.$field, 2.5);)+

        let result = $vector { $($field: <$type>::MAX),+ }
            - $vector { $($field: <$type>::MIN),+ };
        $(assert_eq!(result.$field, <$type>::INFINITY);)+

        let result = $vector { $($field: <$type>::INFINITY),+ }
            - $vector { $($field: <$type>::INFINITY),+ };
        $(assert!(result.$field.is_nan());)+

        let result = $vector { $($field: <$type>::MAX),+ } * 2.0;
        $(assert_eq!(result.$field, <$type>::INFINITY);)+

        let result = $vector { $($field: <$type>::INFINITY),+ } * 0.0;
        $(assert!(result.$field.is_nan());)+

        for invalid in [0.0, -0.0, <$type>::NAN, <$type>::INFINITY, <$type>::NEG_INFINITY] {
            assert!(($vector { $($field: 1.0 as $type),+ } / invalid).is_none());
            assert!(($vector { $($field: 1.0 as $type),+ }
                / $vector { $($field: invalid),+ }).is_none());
        }

        assert!(($vector { $($field: 0.0 as $type),+ } / 0.0).is_none());
        assert!(($vector { $($field: <$type>::MAX),+ } / 0.5).is_none());

        for invalid in [<$type>::NAN, <$type>::INFINITY, <$type>::NEG_INFINITY] {
            assert!(($vector { $($field: invalid),+ } / 1.0).is_none());
        }
    }};
}

#[test]
fn float_arithmetic_and_invalid_division_in_every_dimension() {
    check_float_vector!(f32, Vector2, x, y);
    check_float_vector!(f32, Vector3, x, y, z);
    check_float_vector!(f32, Vector4, x, y, z, w);
    check_float_vector!(f64, Vector2, x, y);
    check_float_vector!(f64, Vector3, x, y, z);
    check_float_vector!(f64, Vector4, x, y, z, w);
}
