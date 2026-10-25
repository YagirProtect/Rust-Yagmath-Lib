use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn normalize_subnormal_diagonals_preserves_unit_length() {
    let tiny = f64::from_bits(1);
    let mut a = Vector2::new(tiny, -tiny);
    let mut b = Vector3::new(tiny, tiny, -tiny);
    let mut c = Vector4::new(tiny, tiny, -tiny, -tiny);
    assert_eq!(a.normalize(), Some(()));
    assert_eq!(b.normalize(), Some(()));
    assert_eq!(c.normalize(), Some(()));
    assert!((a.length() - 1.0).abs() < 1e-12);
    assert!((b.length() - 1.0).abs() < 1e-12);
    assert!((c.length() - 1.0).abs() < 1e-12);
}

#[test]
fn normalize_changes_components_and_preserves_direction() {
    macro_rules! check {
        ($type:ty, $tolerance:expr) => {{
            let mut v = Vector2::new(3 as $type, -4 as $type);
            assert_eq!(v.normalize(), Some(()));
            assert!((f64::from(v.x) - 0.6).abs() < $tolerance);
            assert!((f64::from(v.y) + 0.8).abs() < $tolerance);
            assert!((v.length() - 1.0).abs() < $tolerance);

            let mut v = Vector3::new(-2 as $type, 3 as $type, -6 as $type);
            assert_eq!(v.normalize(), Some(()));
            assert!((f64::from(v.x) + 2.0 / 7.0).abs() < $tolerance);
            assert!((f64::from(v.y) - 3.0 / 7.0).abs() < $tolerance);
            assert!((f64::from(v.z) + 6.0 / 7.0).abs() < $tolerance);
            assert!((v.length() - 1.0).abs() < $tolerance);

            let mut v = Vector4::new(1 as $type, -2 as $type, 2 as $type, -4 as $type);
            assert_eq!(v.normalize(), Some(()));
            assert!((f64::from(v.x) - 0.2).abs() < $tolerance);
            assert!((f64::from(v.y) + 0.4).abs() < $tolerance);
            assert!((f64::from(v.z) - 0.4).abs() < $tolerance);
            assert!((f64::from(v.w) + 0.8).abs() < $tolerance);
            assert!((v.length() - 1.0).abs() < $tolerance);
        }};
    }
    check!(f32, 1e-6);
    check!(f64, 1e-12);
}

#[test]
fn failed_normalization_leaves_every_component_unchanged() {
    macro_rules! check {
        ($type:ty, $vector:ident, $($field:ident),+) => {{
            for invalid in [0.0 as $type, -0.0, <$type>::NAN, <$type>::INFINITY, <$type>::NEG_INFINITY] {
                let mut v = $vector { $($field: invalid),+ };
                let original = v;
                assert_eq!(v.normalize(), None);
                $(assert_eq!(v.$field.to_bits(), original.$field.to_bits());)+
            }
        }};
    }
    check!(f32, Vector2, x, y);
    check!(f32, Vector3, x, y, z);
    check!(f32, Vector4, x, y, z, w);
    check!(f64, Vector2, x, y);
    check!(f64, Vector3, x, y, z);
    check!(f64, Vector4, x, y, z, w);

    let mut v = Vector3::new(1.0_f64, f64::NAN, 3.0);
    assert_eq!(v.normalize(), None);
    assert_eq!((v.x, v.z), (1.0, 3.0));
    assert!(v.y.is_nan());

    let mut v = Vector2::new(f64::MAX, f64::MAX);
    assert_eq!(v.normalize(), None);
    assert_eq!(v, Vector2::new(f64::MAX, f64::MAX));
}

#[test]
fn normalize_handles_extreme_finite_scales_and_unit_vectors() {
    for scale in [1e200_f64, 1e-200] {
        let mut v = Vector2::new(3.0 * scale, -4.0 * scale);
        assert_eq!(v.normalize(), Some(()));
        assert!((v.x - 0.6).abs() < 1e-12);
        assert!((v.y + 0.8).abs() < 1e-12);
    }
    for value in [f64::MAX, f64::MIN_POSITIVE, f64::from_bits(1)] {
        let mut v = Vector2::new(value, 0.0);
        assert_eq!(v.normalize(), Some(()));
        assert_eq!(v, Vector2::new(1.0, 0.0));
        assert_eq!(v.normalize(), Some(()));
        assert_eq!(v, Vector2::new(1.0, 0.0));
    }
    for value in [f32::MAX, f32::MIN_POSITIVE, f32::from_bits(1)] {
        let mut v = Vector2::new(0.0_f32, -value);
        assert_eq!(v.normalize(), Some(()));
        assert_eq!(v, Vector2::new(0.0, -1.0));
    }
}
