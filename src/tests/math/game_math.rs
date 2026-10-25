use crate::Math;
use std::f64::consts::{PI, TAU};

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
}

#[test]
fn remap_ranges() {
    close(Math::remap(50, 0, 100, -1.0, 1.0), 0.0);
    close(Math::remap(150, 0, 100, 0, 10), 15.0);
    close(Math::remap_clamped(150, 0, 100, 0, 10), 10.0);
    close(Math::remap(25, 100, 0, 0, 20), 15.0);
    close(Math::remap(25, 0, 100, 20, 0), 15.0);
    close(Math::remap(5, 1, 1, 20, 30), 20.0);
    close(Math::remap(0.5, 0.0, 1.0, -f64::MAX, f64::MAX), 0.0);
    assert!(Math::remap(f64::NAN, 0.0, 1.0, 0, 10).is_nan());
}

#[test]
fn scalar_move_towards() {
    assert_eq!(Math::move_towards(0, 10, 3.0), 3.0);
    assert_eq!(Math::move_towards(8, 10, 3.0), 10.0);
    assert_eq!(Math::move_towards(10, 0, 3.0), 7.0);
    assert_eq!(Math::move_towards(8, 10, -3.0), 8.0);
    assert_eq!(Math::move_towards(8, 10, 0.0), 8.0);
    assert_eq!(Math::move_towards(-f64::MAX, f64::MAX, f64::MAX), 0.0);
    assert_eq!(Math::move_towards(0, 10, f64::INFINITY), 10.0);
    assert!(Math::move_towards(0, 10, f64::NAN).is_nan());
    assert!(Math::move_towards(f64::INFINITY, 1.0, 1.0).is_nan());
}

#[test]
fn smooth_step_curve_and_limits() {
    close(Math::smooth_step(0, 1, 0.25), 0.15625);
    close(Math::smooth_step(0, 10, 0.5), 5.0);
    close(Math::smooth_step(10, 0, 0.25), 8.4375);
    assert_eq!(Math::smooth_step(2, 4, -1.0), 2.0);
    assert_eq!(Math::smooth_step(2, 4, 2.0), 4.0);
    assert_eq!(Math::smooth_step(-f64::MAX, f64::MAX, 0.5), 0.0);
    assert!(Math::smooth_step(0, 1, f64::NAN).is_nan());
    let h = 1e-6;
    assert!(Math::smooth_step(0, 1, h) / h < 1e-5);
    assert!((1.0 - Math::smooth_step(0, 1, 1.0 - h)) / h < 1e-5);
}

#[test]
fn ping_pong_period_and_negative_values() {
    let expected = [0.0, 1.0, 2.0, 3.0, 2.0, 1.0, 0.0];
    for (value, expected) in expected.into_iter().enumerate() {
        assert_eq!(Math::ping_pong(value as i32, 3), expected);
        assert_eq!(Math::ping_pong(-(value as i32), 3), expected);
    }
    assert_eq!(Math::ping_pong(1, 0), 0.0);
    assert!(Math::ping_pong(1, -1).is_nan());
    assert!(Math::ping_pong(f64::INFINITY, 3.0).is_nan());
    assert!(Math::ping_pong(1.0, f64::NAN).is_nan());
    assert_eq!(Math::ping_pong(f64::MAX, f64::MAX), f64::MAX);
    assert_eq!(Math::ping_pong(-f64::MAX, f64::MAX), f64::MAX);
}

#[test]
fn shortest_angles_and_interpolation() {
    let from = 350.0_f64.to_radians();
    let to = 10.0_f64.to_radians();
    close(Math::delta_angle_rad(from, to), 20.0_f64.to_radians());
    close(Math::delta_angle_rad(to, from), -20.0_f64.to_radians());
    close(Math::delta_angle_rad(0.0, PI), PI);
    close(Math::delta_angle_rad(PI, 0.0), PI);
    close(Math::delta_angle_rad(0.0, TAU * 5.0), 0.0);
    close(Math::lerp_angle_rad(from, to, 0.5), TAU);
    close(Math::lerp_angle_rad(from, to, -1.0), from);
    close(Math::lerp_angle_rad(from, to, 2.0), TAU + to);
    close(
        Math::move_towards_angle_rad(from, to, 5.0_f64.to_radians()),
        355.0_f64.to_radians(),
    );
    close(Math::move_towards_angle_rad(from, to, 1.0), TAU + to);
    close(Math::move_towards_angle_rad(from, to, -1.0), from);
    assert!(Math::delta_angle_rad(f64::INFINITY, 0.0).is_nan());
    assert!(Math::lerp_angle_rad(0.0, PI, f64::NAN).is_nan());
    assert!(Math::delta_angle_rad(-f64::MAX, f64::MAX).is_finite());
}

#[test]
fn smooth_damp_converges_without_overshoot() {
    for target in [-10.0, 10.0] {
        let mut current = 0.0;
        let mut velocity = 0.0;
        for _ in 0..300 {
            let next = Math::smooth_damp(
                current,
                target,
                &mut velocity,
                0.3,
                f64::INFINITY,
                1.0 / 60.0,
            );
            assert!(next.is_finite() && velocity.is_finite());
            assert!((target - next).abs() <= (target - current).abs());
            assert!(next.abs() <= target.abs());
            current = next;
        }
        close(current, target);
    }
}

#[test]
fn smooth_damp_velocity_speed_limit_and_time() {
    let mut velocity = 0.0;
    let first = Math::smooth_damp(0, 10, &mut velocity, 0.5, f64::INFINITY, 0.1);
    assert!(first > 0.0 && first < 10.0 && velocity > 0.0);
    close(first, 0.6156156156156136);
    close(velocity, 10.725010725010726);
    let continued = Math::smooth_damp(first, 10.0, &mut velocity, 0.5, f64::INFINITY, 0.1);
    let reset = Math::smooth_damp(first, 10.0, &mut 0.0, 0.5, f64::INFINITY, 0.1);
    assert!(continued > reset);
    let mut velocity = 0.0;
    let limited = Math::smooth_damp(0, 10, &mut velocity, 0.5, 1.0, 0.1);
    assert!(limited > 0.0 && limited < first && limited <= 0.1);
    let mut velocity = 3.0;
    assert_eq!(Math::smooth_damp(2, 10, &mut velocity, 0.5, 1.0, 0.0), 2.0);
    assert_eq!(velocity, 3.0);
    let mut velocity = 100.0;
    assert_eq!(
        Math::smooth_damp(0.0, 0.1, &mut velocity, 0.5, f64::INFINITY, 0.1),
        0.1
    );
    assert_eq!(velocity, 0.0);
    assert!(Math::smooth_damp(0, 10, &mut 0.0, 0.0, f64::INFINITY, 0.1).is_finite());
    assert_eq!(Math::smooth_damp(2, 10, &mut 0.0, 0.5, 0.0, 0.1), 2.0);
}

#[test]
fn smooth_damp_invalid_arguments_preserve_velocity() {
    for (time, speed, dt) in [
        (-1.0, 1.0, 0.1),
        (0.5, -1.0, 0.1),
        (0.5, 1.0, -0.1),
        (f64::NAN, 1.0, 0.1),
        (0.5, f64::NAN, 0.1),
        (0.5, 1.0, f64::INFINITY),
    ] {
        let mut velocity = 3.0;
        assert!(Math::smooth_damp(0, 10, &mut velocity, time, speed, dt).is_nan());
        assert_eq!(velocity, 3.0);
    }
    let mut velocity = 3.0;
    assert!(Math::smooth_damp(f64::NAN, 1.0, &mut velocity, 0.5, 1.0, 0.1).is_nan());
    assert_eq!(velocity, 3.0);
    assert!(
        Math::smooth_damp(-f64::MAX, f64::MAX, &mut velocity, 0.5, f64::INFINITY, 0.1).is_nan()
    );
    assert_eq!(velocity, 3.0);
}

#[test]
fn scalar_methods_accept_all_numeric_types() {
    macro_rules! check {
        ($($type:ty),+) => { $(
            assert_eq!(Math::move_towards(0 as $type, 10 as $type, 2.0), 2.0);
            assert_eq!(Math::remap(5 as $type, 0 as $type, 10 as $type, 0, 100), 50.0);
            assert_eq!(Math::smooth_step(0 as $type, 10 as $type, 0.5), 5.0);
            assert_eq!(Math::ping_pong(4 as $type, 3 as $type), 2.0);
            assert_eq!(Math::delta_angle_rad(0 as $type, 1 as $type), 1.0);
            assert_eq!(Math::lerp_angle_rad(0 as $type, 1 as $type, 0.5), 0.5);
            assert!(Math::smooth_damp(0 as $type, 10 as $type, &mut 0.0, 0.5, 1.0, 0.1).is_finite());
        )+ };
    }
    check!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn smooth_damp_angle_uses_shortest_arc() {
    let mut velocity = 0.0;
    let mut angle = 350.0_f64.to_radians();
    let target = 10.0_f64.to_radians();
    for _ in 0..300 {
        let next =
            Math::smooth_damp_angle_rad(angle, target, &mut velocity, 0.3, f64::INFINITY, 1.0 / 60.0);
        assert!(next >= angle - 1e-12);
        angle = next;
    }
    close(angle, TAU + target);
}
