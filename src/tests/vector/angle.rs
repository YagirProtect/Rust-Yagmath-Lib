use crate::vector::{Vector2, Vector3, Vector4};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-14, "{actual} != {expected}");
}

#[test]
fn known_angles_in_every_dimension() {
    close(
        Vector2::new(1, 0).angle_between_rad(Vector2::new(0, 1)),
        FRAC_PI_2,
    );
    close(
        Vector2::new(1, 0).angle_between_rad(Vector2::new(1, 1)),
        FRAC_PI_4,
    );
    close(
        Vector3::new(1, 0, 0).angle_between_rad(Vector3::new(0, 1, 0)),
        FRAC_PI_2,
    );
    close(
        Vector3::new(1, 0, 0).angle_between_rad(Vector3::new(-1, 0, 0)),
        PI,
    );
    close(
        Vector4::new(1, 0, 0, 0).angle_between_rad(Vector4::new(0, 0, 0, 1)),
        FRAC_PI_2,
    );
    close(
        Vector4::new(1, 2, 3, 4).angle_between_rad(Vector4::new(1, 2, 3, 4)),
        0.0,
    );
    close(
        Vector4::new(1, 2, 3, 4).angle_between_rad(Vector4::new(-1, -2, -3, -4)),
        PI,
    );
}

#[test]
fn lengths_do_not_affect_angle_and_mixed_types_work() {
    let a = Vector3::new(3, 4, 0);
    let b = Vector3::new(-8.0_f32, 6.0, 0.0);
    close(a.angle_between_rad(b), FRAC_PI_2);
    close(b.angle_between_rad(a), a.angle_between_rad(b));
    close(
        Vector2::new(2, 0).angle_between_rad(Vector2::new(50, 50)),
        FRAC_PI_4,
    );
    close(Vector2::new(2, 0).angle_between_rad(Vector2::new(50, 0)), 0.0);
}

#[test]
fn zero_and_non_finite_directions_return_nan() {
    assert!(
        Vector2::new(0, 0)
            .angle_between_rad(Vector2::new(1, 0))
            .is_nan()
    );
    assert!(
        Vector2::new(1, 0)
            .angle_between_rad(Vector2::new(0, 0))
            .is_nan()
    );
    assert!(
        Vector3::new(0, 0, 0)
            .angle_between_rad(Vector3::new(0, 0, 0))
            .is_nan()
    );
    assert!(
        Vector4::new(0, 0, 0, 0)
            .angle_between_rad(Vector4::new(1, 0, 0, 0))
            .is_nan()
    );
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            Vector2::new(invalid, 0.0)
                .angle_between_rad(Vector2::new(1, 0))
                .is_nan()
        );
        assert!(
            Vector3::new(1, 0, 0)
                .angle_between_rad(Vector3::new(0.0, invalid, 0.0))
                .is_nan()
        );
        assert!(
            Vector4::new(1, 0, 0, 0)
                .angle_between_rad(Vector4::new(0.0, 0.0, 0.0, invalid))
                .is_nan()
        );
    }
}

#[test]
fn huge_and_subnormal_components_keep_their_direction() {
    let huge = f64::MAX;
    let tiny = f64::from_bits(1);
    close(
        Vector2::new(huge, huge).angle_between_rad(Vector2::new(huge, -huge)),
        FRAC_PI_2,
    );
    close(
        Vector2::new(tiny, tiny).angle_between_rad(Vector2::new(tiny, -tiny)),
        FRAC_PI_2,
    );
    close(
        Vector3::new(huge, huge, huge).angle_between_rad(Vector3::new(tiny, tiny, tiny)),
        0.0,
    );
    close(
        Vector4::new(huge, huge, huge, huge)
            .angle_between_rad(Vector4::new(-tiny, -tiny, -tiny, -tiny)),
        PI,
    );
}

#[test]
fn nearly_parallel_and_opposite_vectors_keep_small_angles() {
    let tiny_angle = 1e-12;
    let a = Vector2::new(1.0, 0.0);
    let angle = a.angle_between_rad(Vector2::new(1.0, tiny_angle));
    assert!((angle / tiny_angle - 1.0).abs() < 1e-12);
    let angle = a.angle_between_rad(Vector2::new(-1.0, tiny_angle));
    assert!((angle - (PI - tiny_angle)).abs() < 1e-15);
}

#[test]
fn all_numeric_component_types_are_supported() {
    macro_rules! check {
        ($($type:ty),+) => { $(
            close(Vector2::new(1 as $type, 0 as $type)
                .angle_between_rad(Vector2::new(0 as $type, 1 as $type)), FRAC_PI_2);
            close(Vector3::new(1 as $type, 0 as $type, 0 as $type)
                .angle_between_rad(Vector3::new(0 as $type, 1 as $type, 0 as $type)), FRAC_PI_2);
            close(Vector4::new(1 as $type, 0 as $type, 0 as $type, 0 as $type)
                .angle_between_rad(Vector4::new(0 as $type, 0 as $type, 0 as $type, 1 as $type)), FRAC_PI_2);
        )+ };
    }
    check!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}
