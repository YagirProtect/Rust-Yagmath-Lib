use crate::vector::{Vector2, Vector3, Vector4};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

#[test]
fn rotated_supports_every_numeric_input_type() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let v: Vector2<f64> = Vector2::new(1 as $type, 0 as $type).rotated_rad(FRAC_PI_2).unwrap();
            close(v.x, 0.0); close(v.y, 1.0);
            let v: Vector3<f64> = Vector3::new(1 as $type, 0 as $type, 0 as $type)
                .rotated_rad(FRAC_PI_2, Vector3::new(0 as $type, 0 as $type, 1 as $type)).unwrap();
            close(v.x, 0.0); close(v.y, 1.0); close(v.z, 0.0);
            let v: Vector4<f64> = Vector4::new(1 as $type, 2 as $type, 3 as $type, 0 as $type)
                .rotated_rad(FRAC_PI_2, (0, 3)).unwrap();
            close(v.x, 0.0); close(v.y, 2.0); close(v.z, 3.0); close(v.w, 1.0);
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
}

#[test]
fn rotation_2d_direction_and_fractional_integer_result() {
    let v = Vector2::new(1, 0);
    let r: Vector2<f64> = v.rotated_rad(FRAC_PI_2).unwrap();
    close(r.x, 0.0);
    close(r.y, 1.0);
    let r = v.rotated_rad(-FRAC_PI_2).unwrap();
    close(r.x, 0.0);
    close(r.y, -1.0);
    let r = v.rotated_rad(FRAC_PI_4).unwrap();
    close(r.x, std::f64::consts::FRAC_1_SQRT_2);
    close(r.y, std::f64::consts::FRAC_1_SQRT_2);
    assert_eq!(v, Vector2::new(1, 0));
}

#[test]
fn rotation_3d_uses_right_hand_rule_for_each_axis() {
    let r = Vector3::new(1, 0, 0)
        .rotated_rad(FRAC_PI_2, Vector3::new(0, 0, 1))
        .unwrap();
    close(r.x, 0.0);
    close(r.y, 1.0);
    close(r.z, 0.0);
    let r = Vector3::new(0, 1, 0)
        .rotated_rad(FRAC_PI_2, Vector3::new(1, 0, 0))
        .unwrap();
    close(r.x, 0.0);
    close(r.y, 0.0);
    close(r.z, 1.0);
    let r = Vector3::new(0, 0, 1)
        .rotated_rad(FRAC_PI_2, Vector3::new(0, 1, 0))
        .unwrap();
    close(r.x, 1.0);
    close(r.y, 0.0);
    close(r.z, 0.0);
}

#[test]
fn rotation_3d_normalizes_axis_and_handles_extreme_axis_lengths() {
    let v = Vector3::new(1.0, 0.0, 0.0);
    for z in [1.0, 10.0, f64::MAX, f64::MIN_POSITIVE, f64::from_bits(1)] {
        let r = v.rotated_rad(FRAC_PI_2, Vector3::new(0.0, 0.0, z)).unwrap();
        close(r.x, 0.0);
        close(r.y, 1.0);
        close(r.z, 0.0);
    }
    let r = v.rotated_rad(FRAC_PI_2, Vector3::new(0, 0, -1)).unwrap();
    close(r.x, 0.0);
    close(r.y, -1.0);
    let r = Vector3::new(1, 0, 0)
        .rotated_rad(2.0 * PI / 3.0, Vector3::new(1, 1, 1))
        .unwrap();
    close(r.x, 0.0);
    close(r.y, 1.0);
    close(r.z, 0.0);
}

#[test]
fn rotation_4d_supports_all_six_coordinate_planes() {
    for i in 0..4 {
        for j in i + 1..4 {
            let mut values = [2.0, 3.0, 4.0, 5.0];
            values[i] = 1.0;
            values[j] = 0.0;
            let v = Vector4::new(values[0], values[1], values[2], values[3]);
            let r = v.rotated_rad(FRAC_PI_2, (i, j)).unwrap();
            let result = [r.x, r.y, r.z, r.w];
            for k in 0..4 {
                close(
                    result[k],
                    if k == i {
                        0.0
                    } else if k == j {
                        1.0
                    } else {
                        values[k]
                    },
                );
            }
            let r = v.rotated_rad(FRAC_PI_2, (j, i)).unwrap();
            let result = [r.x, r.y, r.z, r.w];
            close(result[i], 0.0);
            close(result[j], -1.0);
        }
    }
}

#[test]
fn rotation_preserves_length_and_can_be_reversed() {
    let a = Vector2::new(3.0, -4.0);
    let r = a.rotated_rad(0.73).unwrap();
    close(r.length(), a.length());
    let back = r.rotated_rad(-0.73).unwrap();
    close(back.x, a.x);
    close(back.y, a.y);

    let a = Vector3::new(2.0, -3.0, 4.0);
    let axis = Vector3::new(1, 2, -3);
    let r = a.rotated_rad(0.73, axis).unwrap();
    close(r.length(), a.length());
    close(
        r.dot(Vector3::new(1.0, 2.0, -3.0)),
        a.dot(Vector3::new(1.0, 2.0, -3.0)),
    );
    let back = r.rotated_rad(-0.73, axis).unwrap();
    close(back.x, a.x);
    close(back.y, a.y);
    close(back.z, a.z);

    let a = Vector4::new(2.0, -3.0, 4.0, 5.0);
    let r = a.rotated_rad(0.73, (1, 3)).unwrap();
    close(r.length(), a.length());
    let back = r.rotated_rad(-0.73, (1, 3)).unwrap();
    close(back.x, a.x);
    close(back.y, a.y);
    close(back.z, a.z);
    close(back.w, a.w);
}

#[test]
fn rotation_handles_zero_vectors_zero_angles_and_full_turns() {
    for angle in [0.0, FRAC_PI_2, TAU] {
        assert_eq!(
            Vector2::new(0, 0).rotated_rad(angle),
            Some(Vector2::new(0.0, 0.0))
        );
        assert_eq!(
            Vector3::new(0, 0, 0).rotated_rad(angle, Vector3::new(0, 0, 1)),
            Some(Vector3::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Vector4::new(0, 0, 0, 0).rotated_rad(angle, (0, 3)),
            Some(Vector4::new(0.0, 0.0, 0.0, 0.0))
        );
    }
    for angle in [0.0, TAU] {
        let r = Vector2::new(2, -3).rotated_rad(angle).unwrap();
        close(r.x, 2.0);
        close(r.y, -3.0);
        let r = Vector3::new(2, -3, 4)
            .rotated_rad(angle, Vector3::new(0, 0, 1))
            .unwrap();
        close(r.x, 2.0);
        close(r.y, -3.0);
        close(r.z, 4.0);
        let r = Vector4::new(2, -3, 4, 5).rotated_rad(angle, (0, 3)).unwrap();
        close(r.x, 2.0);
        close(r.y, -3.0);
        close(r.z, 4.0);
        close(r.w, 5.0);
    }
}

#[test]
fn rotate_changes_float_vectors_in_place() {
    macro_rules! check {
        ($type:ty) => {{
            let mut v = Vector2::new(1 as $type, 0 as $type);
            assert_eq!(v.rotate_rad(FRAC_PI_2), Some(()));
            close(f64::from(v.x), 0.0);
            close(f64::from(v.y), 1.0);
            let mut v = Vector3::new(1 as $type, 0 as $type, 0 as $type);
            assert_eq!(v.rotate_rad(FRAC_PI_2, Vector3::new(0, 0, 2)), Some(()));
            close(f64::from(v.x), 0.0);
            close(f64::from(v.y), 1.0);
            close(f64::from(v.z), 0.0);
            let mut v = Vector4::new(1 as $type, 2 as $type, 3 as $type, 0 as $type);
            assert_eq!(v.rotate_rad(FRAC_PI_2, (0, 3)), Some(()));
            close(f64::from(v.x), 0.0);
            close(f64::from(v.y), 2.0);
            close(f64::from(v.z), 3.0);
            close(f64::from(v.w), 1.0);
        }};
    }
    check!(f32);
    check!(f64);
}

#[test]
fn rotation_rejects_invalid_axis_plane_angle_and_components() {
    let v = Vector3::new(1, 2, 3);
    assert!(v.rotated_rad(0.0, Vector3::new(0, 0, 0)).is_none());
    for plane in [(0, 0), (1, 1), (0, 4), (4, 0), (usize::MAX, 1)] {
        assert!(Vector4::new(1, 2, 3, 4).rotated_rad(0.5, plane).is_none());
    }
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(Vector2::new(1, 2).rotated_rad(invalid).is_none());
        assert!(v.rotated_rad(invalid, Vector3::new(0, 0, 1)).is_none());
        assert!(Vector4::new(1, 2, 3, 4).rotated_rad(invalid, (0, 1)).is_none());
        assert!(Vector2::new(invalid, 2.0).rotated_rad(0.5).is_none());
        assert!(
            Vector3::new(1.0, invalid, 3.0)
                .rotated_rad(0.5, Vector3::new(0, 0, 1))
                .is_none()
        );
        assert!(v.rotated_rad(0.5, Vector3::new(0.0, 0.0, invalid)).is_none());
        assert!(
            Vector4::new(1.0, 2.0, 3.0, invalid)
                .rotated_rad(0.5, (0, 1))
                .is_none()
        );
    }
}

#[test]
fn failed_rotate_preserves_original_including_f32_overflow() {
    macro_rules! check {
        ($type:ty) => {{
            let mut v = Vector2::new(1 as $type, 2 as $type);
            let original = v;
            assert_eq!(v.rotate_rad(f64::NAN), None);
            assert_eq!(v, original);
            let mut v = Vector3::new(1 as $type, 2 as $type, 3 as $type);
            let original = v;
            assert_eq!(v.rotate_rad(0.5, Vector3::new(0, 0, 0)), None);
            assert_eq!(v, original);
            let mut v = Vector4::new(1 as $type, 2 as $type, 3 as $type, 4 as $type);
            let original = v;
            assert_eq!(v.rotate_rad(0.5, (0, 4)), None);
            assert_eq!(v, original);
        }};
    }
    check!(f32);
    check!(f64);
    let mut v = Vector2::new(f32::MAX, f32::MAX);
    let original = v;
    assert_eq!(v.rotate_rad(FRAC_PI_4), None);
    assert_eq!(v, original);
    let mut v = Vector3::new(f32::MAX, f32::MAX, 0.0);
    let original = v;
    assert_eq!(v.rotate_rad(FRAC_PI_4, Vector3::new(0, 0, 1)), None);
    assert_eq!(v, original);
    let mut v = Vector4::new(f32::MAX, f32::MAX, 0.0, 0.0);
    let original = v;
    assert_eq!(v.rotate_rad(FRAC_PI_4, (0, 1)), None);
    assert_eq!(v, original);
}
