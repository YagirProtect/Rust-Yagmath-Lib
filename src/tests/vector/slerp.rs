use crate::vector::{Vector2, Vector3, Vector4};
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-13, "{actual} != {expected}");
}

#[test]
fn quarter_circle_in_all_dimensions() {
    let v = Vector2::new(1, 0).slerp(Vector2::new(0, 1), 0.5);
    close(v.x, FRAC_1_SQRT_2);
    close(v.y, FRAC_1_SQRT_2);
    close(v.length(), 1.0);
    let v = Vector3::new(1, 0, 0).slerp(Vector3::new(0, 0, 1), 0.5);
    close(v.x, FRAC_1_SQRT_2);
    close(v.y, 0.0);
    close(v.z, FRAC_1_SQRT_2);
    let v = Vector4::new(1, 0, 0, 0).slerp(Vector4::new(0, 0, 0, 1), 0.5);
    close(v.x, FRAC_1_SQRT_2);
    close(v.y, 0.0);
    close(v.z, 0.0);
    close(v.w, FRAC_1_SQRT_2);
}

#[test]
fn direction_changes_at_constant_angular_speed() {
    let start = Vector3::new(1.0, 2.0, 3.0);
    let end = Vector3::new(-2.0, 4.0, 1.0);
    let angle = start.angle_between_rad(end);
    for t in [0.1, 0.25, 0.5, 0.75, 0.9] {
        let v = start.slerp(end, t);
        close(start.angle_between_rad(v), angle * t);
        close(v.angle_between_rad(end), angle * (1.0 - t));
        close(v.length(), start.length() * (1.0 - t) + end.length() * t);
        let reverse = end.slerp(start, 1.0 - t);
        close(v.x, reverse.x);
        close(v.y, reverse.y);
        close(v.z, reverse.z);
    }
}

#[test]
fn unequal_lengths_and_mixed_component_types() {
    let v = Vector2::new(2, 0).slerp(Vector2::new(0.0_f32, 4.0), 0.5);
    close(v.length(), 3.0);
    close(v.x, 3.0 * FRAC_1_SQRT_2);
    close(v.y, 3.0 * FRAC_1_SQRT_2);
    assert_eq!(
        Vector2::new(2, 0).slerp(Vector2::new(4, 0), 0.5),
        Vector2::new(3.0, 0.0)
    );
}

#[test]
fn endpoints_clamping_and_extrapolation() {
    let start = Vector2::new(1.0, 0.0);
    let end = Vector2::new(0.0, 1.0);
    assert_eq!(start.slerp(end, 0.0), start);
    assert_eq!(start.slerp(end, 1.0), end);
    assert_eq!(start.slerp(end, -1.0), start);
    assert_eq!(start.slerp(end, f64::NEG_INFINITY), start);
    assert_eq!(start.slerp(end, 2.0), end);
    assert_eq!(start.slerp(end, f64::INFINITY), end);
    let v = start.slerp_unclamped(end, 2.0);
    close(v.x, -1.0);
    close(v.y, 0.0);
    let v = start.slerp_unclamped(end, -1.0);
    close(v.x, 0.0);
    close(v.y, -1.0);
    // Extrapolated length is signed, not clamped to zero.
    let v = Vector2::new(2, 0).slerp_unclamped(Vector2::new(1, 0), 3.0);
    close(v.x, -1.0);
}

#[test]
fn opposite_directions_choose_a_repeatable_arc() {
    let v = Vector2::new(1, 0).slerp(Vector2::new(-1, 0), 0.5);
    close(v.x, 0.0);
    close(v.y, 1.0);
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(-1.0, -2.0, -3.0);
    let v = a.slerp(b, 0.5);
    close(v.dot(a), 0.0);
    close(v.length(), a.length());
    assert_eq!(v, a.slerp(b, 0.5));
    let v = Vector4::new(1, 0, 0, 0).slerp(Vector4::new(-1, 0, 0, 0), 0.5);
    close(v.x, 0.0);
    close(v.y, 1.0);
    close(v.z, 0.0);
    close(v.w, 0.0);
}

#[test]
fn almost_parallel_and_almost_opposite() {
    let start = Vector2::new(1.0, 0.0);
    for end in [Vector2::new(1.0, 1e-12), Vector2::new(-1.0, 1e-12)] {
        let v = start.slerp(end, 0.5);
        close(v.length(), 1.0);
        close(start.angle_between_rad(v), start.angle_between_rad(end) * 0.5);
        assert!(v.y > 0.0);
    }
    close(
        start.angle_between_rad(start.slerp(Vector2::new(0.0, 1.0), 0.5)),
        FRAC_PI_4,
    );
    close(start.angle_between_rad(Vector2::new(0.0, 1.0)), FRAC_PI_2);
}

#[test]
fn huge_and_subnormal_inputs() {
    let huge = f64::MAX;
    let a = Vector2::new(huge, 0.0);
    let v = a.slerp(Vector2::new(0.0, huge), 0.5);
    close(v.x / huge, FRAC_1_SQRT_2);
    close(v.y / huge, FRAC_1_SQRT_2);
    let a = Vector4::new(huge, huge, 0.0, 0.0);
    let v = a.slerp(Vector4::new(0.0, 0.0, huge, huge), 0.5);
    close(v.x / huge, FRAC_1_SQRT_2);
    close(v.w / huge, FRAC_1_SQRT_2);
    assert_eq!(a.slerp(a, 0.5), a);
    let tiny = f64::from_bits(1);
    let v = Vector2::new(tiny, 0.0).slerp(Vector2::new(0.0, tiny), 0.5);
    assert_eq!(v, Vector2::new(tiny, tiny)); // Rounded to representable subnormals.
}

#[test]
fn invalid_inputs_return_nan_components() {
    for t in [0.0, 0.5, 1.0] {
        let v = Vector2::new(0, 0).slerp(Vector2::new(1, 0), t);
        assert!(v.x.is_nan() && v.y.is_nan());
        let v = Vector3::new(1, 0, 0).slerp(Vector3::new(0, 0, 0), t);
        assert!(v.x.is_nan() && v.y.is_nan() && v.z.is_nan());
    }
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let v = Vector2::new(bad, 0.0).slerp(Vector2::new(0, 1), 0.5);
        assert!(v.x.is_nan() && v.y.is_nan());
        let v = Vector4::new(1, 0, 0, 0).slerp_unclamped(Vector4::new(0, 0, 0, 1), bad);
        assert!(v.x.is_nan() && v.y.is_nan() && v.z.is_nan() && v.w.is_nan());
    }
    let v = Vector2::new(1, 0).slerp(Vector2::new(0, 1), f64::NAN);
    assert!(v.x.is_nan() && v.y.is_nan());
    let v = Vector2::new(1.0, 0.0).slerp_unclamped(Vector2::new(f64::MAX, 0.0), 2.0);
    assert!(v.x.is_nan() && v.y.is_nan());
}

#[test]
fn every_numeric_component_type_is_supported() {
    macro_rules! check {
        ($($type:ty),+) => { $(
            let v = Vector2::new(1 as $type, 0 as $type)
                .slerp(Vector2::new(0 as $type, 1 as $type), 0.5);
            close(v.x, FRAC_1_SQRT_2);
            close(v.y, FRAC_1_SQRT_2);
            let v = Vector3::new(1 as $type, 0 as $type, 0 as $type)
                .slerp(Vector3::new(0 as $type, 1 as $type, 0 as $type), 0.5);
            close(v.length(), 1.0);
            let v = Vector4::new(1 as $type, 0 as $type, 0 as $type, 0 as $type)
                .slerp(Vector4::new(0 as $type, 0 as $type, 0 as $type, 1 as $type), 0.5);
            close(v.length(), 1.0);
        )+ };
    }
    check!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}
