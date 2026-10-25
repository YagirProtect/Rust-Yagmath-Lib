use crate::vector::{Vector2, Vector3};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-14, "{actual} != {expected}");
}

#[test]
fn two_dimensional_angle_signs_and_half_turn() {
    let right = Vector2::<i32>::right();
    close(right.signed_angle_rad(Vector2::<i32>::up()), FRAC_PI_2);
    close(right.signed_angle_rad(Vector2::<i32>::down()), -FRAC_PI_2);
    close(Vector2::<i32>::up().signed_angle_rad(right), -FRAC_PI_2);
    close(right.signed_angle_rad(Vector2::new(4, 4)), FRAC_PI_4);
    close(right.signed_angle_rad(Vector2::new(4, -4)), -FRAC_PI_4);
    close(right.signed_angle_rad(Vector2::new(20, 0)), 0.0);
    close(right.signed_angle_rad(Vector2::<i32>::left()), PI);
    close(Vector2::<i32>::left().signed_angle_rad(right), PI);
}

#[test]
fn three_dimensional_sign_follows_axis_and_right_hand_rule() {
    let x = Vector3::<i32>::right();
    let y = Vector3::<i32>::up();
    let z = Vector3::<i32>::forward();
    close(x.signed_angle_rad(y, z), FRAC_PI_2);
    close(x.signed_angle_rad(y, Vector3::new(0, 0, -10)), -FRAC_PI_2);
    close(y.signed_angle_rad(x, z), -FRAC_PI_2);
    close(y.signed_angle_rad(z, x), FRAC_PI_2);
    close(z.signed_angle_rad(x, y), FRAC_PI_2);
    close(x.signed_angle_rad(x, z), 0.0);
    close(x.signed_angle_rad(Vector3::<i32>::left(), z), PI);
    close(x.signed_angle_rad(y, x), FRAC_PI_2); // Zero orientation sign chooses positive.
}

#[test]
fn three_dimensional_axis_controls_sign_without_projecting_inputs() {
    let a = Vector3::new(1, 0, 1);
    let b = Vector3::new(0, 1, 1);
    // Actual angle is 60 degrees; projected XY directions would have 90 degrees.
    close(a.signed_angle_rad(b, Vector3::<i32>::forward()), PI / 3.0);
    close(a.signed_angle_rad(b, Vector3::<i32>::backward()), -PI / 3.0);
    close(a.signed_angle_rad(b, a), PI / 3.0);
}

#[test]
fn zero_and_non_finite_arguments_return_nan() {
    let x = Vector2::<i32>::right();
    assert!(x.signed_angle_rad(Vector2::new(0, 0)).is_nan());
    assert!(Vector2::<i32>::zero().signed_angle_rad(x).is_nan());
    let x = Vector3::<i32>::right();
    let y = Vector3::<i32>::up();
    let z = Vector3::<i32>::forward();
    assert!(x.signed_angle_rad(y, Vector3::<i32>::zero()).is_nan());
    assert!(x.signed_angle_rad(Vector3::<i32>::zero(), z).is_nan());
    assert!(Vector3::<i32>::zero().signed_angle_rad(y, z).is_nan());
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            Vector2::new(bad, 1.0)
                .signed_angle_rad(Vector2::new(1, 0))
                .is_nan()
        );
        assert!(
            Vector2::new(1, 0)
                .signed_angle_rad(Vector2::new(1.0, bad))
                .is_nan()
        );
        assert!(Vector3::new(bad, 0.0, 1.0).signed_angle_rad(y, z).is_nan());
        assert!(x.signed_angle_rad(Vector3::new(0.0, bad, 1.0), z).is_nan());
        assert!(x.signed_angle_rad(y, Vector3::new(0.0, 0.0, bad)).is_nan());
    }
}

#[test]
fn huge_subnormal_and_mixed_numeric_directions() {
    for scale in [f64::MAX, f64::from_bits(1)] {
        close(
            Vector2::new(scale, 0.0).signed_angle_rad(Vector2::new(0.0, -scale)),
            -FRAC_PI_2,
        );
        close(
            Vector3::new(scale, 0.0, 0.0).signed_angle_rad(
                Vector3::new(0.0, scale, 0.0),
                Vector3::new(0.0, 0.0, -scale),
            ),
            -FRAC_PI_2,
        );
    }
    close(
        Vector2::new(5_i32, 0).signed_angle_rad(Vector2::new(0.0_f32, 7.0)),
        FRAC_PI_2,
    );
    close(
        Vector3::new(5_i32, 0, 0).signed_angle_rad(
            Vector3::new(0.0_f32, 7.0, 0.0),
            Vector3::new(0.0_f64, 0.0, 10.0),
        ),
        FRAC_PI_2,
    );
}

#[test]
fn tiny_positive_and_negative_angles_keep_their_sign() {
    let a = Vector2::new(1.0, 0.0);
    for sign in [-1.0, 1.0] {
        let angle = a.signed_angle_rad(Vector2::new(1.0, sign * 1e-12));
        assert!((angle / (sign * 1e-12) - 1.0).abs() < 1e-12);
        let angle = a.signed_angle_rad(Vector2::new(-1.0, sign * 1e-12));
        close(angle, sign * (PI - 1e-12));
        let angle = Vector3::new(1.0, 0.0, 0.0).signed_angle_rad(
            Vector3::new(1.0, sign * 1e-12, 0.0),
            Vector3::<i32>::forward(),
        );
        assert!((angle / (sign * 1e-12) - 1.0).abs() < 1e-12);
    }
}

#[test]
fn all_numeric_component_types_are_supported() {
    macro_rules! check {
        ($($type:ty),+) => { $(
            close(Vector2::<$type>::right().signed_angle_rad(Vector2::<$type>::up()), FRAC_PI_2);
            close(Vector3::<$type>::right().signed_angle_rad(Vector3::<$type>::up(), Vector3::<$type>::forward()), FRAC_PI_2);
        )+ };
    }
    check!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}
