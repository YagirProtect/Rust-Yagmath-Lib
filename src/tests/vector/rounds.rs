use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn rounds_handle_positive_negative_and_halfway_components() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let a = Vector2::new(1.25 as $type, -1.25 as $type);
            assert_eq!(a.floor(), Vector2::new(1.0, -2.0));
            assert_eq!(a.ceil(), Vector2::new(2.0, -1.0));
            assert_eq!(a.round(), Vector2::new(1.0, -1.0));

            let b = Vector3::new(1.25 as $type, -1.25 as $type, 2.5 as $type);
            assert_eq!(b.floor(), Vector3::new(1.0, -2.0, 2.0));
            assert_eq!(b.ceil(), Vector3::new(2.0, -1.0, 3.0));
            assert_eq!(b.round(), Vector3::new(1.0, -1.0, 3.0));

            let c = Vector4::new(1.25 as $type, -1.25 as $type, 2.5 as $type, -2.5 as $type);
            assert_eq!(c.floor(), Vector4::new(1.0, -2.0, 2.0, -3.0));
            assert_eq!(c.ceil(), Vector4::new(2.0, -1.0, 3.0, -2.0));
            assert_eq!(c.round(), Vector4::new(1.0, -1.0, 3.0, -3.0));
            assert_eq!(c, Vector4::new(1.25, -1.25, 2.5, -2.5));
        )+};
    }
    check_types!(f32, f64);
}

#[test]
fn integer_rounds_preserve_all_values_without_float_conversion() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let a = Vector2::new(<$type>::MIN, <$type>::MAX);
            assert_eq!(a.floor(), a);
            assert_eq!(a.ceil(), a);
            assert_eq!(a.round(), a);
            let b = Vector3::new(<$type>::MIN, 0, <$type>::MAX);
            assert_eq!(b.floor(), b);
            assert_eq!(b.ceil(), b);
            assert_eq!(b.round(), b);
            let c = Vector4::new(<$type>::MIN, 0, 1, <$type>::MAX);
            assert_eq!(c.floor(), c);
            assert_eq!(c.ceil(), c);
            assert_eq!(c.round(), c);
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );
}

#[test]
fn float_rounds_preserve_nan_infinities_and_signed_zero() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let v = Vector4::new(<$type>::NAN, <$type>::INFINITY, <$type>::NEG_INFINITY, -0.0 as $type);
            for result in [v.floor(), v.ceil(), v.round()] {
                assert!(result.x.is_nan());
                assert_eq!(result.y, <$type>::INFINITY);
                assert_eq!(result.z, <$type>::NEG_INFINITY);
                assert_eq!(result.w.to_bits(), (-0.0 as $type).to_bits());
            }
        )+};
    }
    check_types!(f32, f64);
}

#[test]
fn rounds_are_idempotent_and_order_the_original_value() {
    for value in [-10.75_f64, -2.5, -0.25, 0.0, 0.25, 2.5, 10.75] {
        let v = Vector2::new(value, -value);
        assert_eq!(v.floor().floor(), v.floor());
        assert_eq!(v.ceil().ceil(), v.ceil());
        assert_eq!(v.round().round(), v.round());
        assert!(v.floor().x <= v.x && v.x <= v.ceil().x);
        assert!(v.floor().y <= v.y && v.y <= v.ceil().y);
        assert!((v.round().x - v.x).abs() <= 0.5);
        assert!((v.round().y - v.y).abs() <= 0.5);
    }
}

#[test]
fn rounds_handle_extreme_and_subnormal_float_components() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let tiny = <$type>::from_bits(1);
            let v = Vector4::new(<$type>::MAX, <$type>::MIN, tiny, -tiny);
            assert_eq!(v.floor(), Vector4::new(<$type>::MAX, <$type>::MIN, 0.0, -1.0));
            assert_eq!(v.ceil(), Vector4::new(<$type>::MAX, <$type>::MIN, 1.0, -0.0));
            assert_eq!(v.round(), Vector4::new(<$type>::MAX, <$type>::MIN, 0.0, -0.0));
            assert!(v.round().w.is_sign_negative());
            assert!(v.ceil().w.is_sign_negative());
        )+};
    }
    check_types!(f32, f64);
}
