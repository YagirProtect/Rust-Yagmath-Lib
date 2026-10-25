use crate::Math;
use std::f64::consts::{PI, TAU};

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
}

#[test]
fn degrees_wrap_in_both_directions_and_choose_positive_half_turn() {
    for (current, target, expected) in [
        (350.0, 10.0, 20.0),
        (10.0, 350.0, -20.0),
        (-10.0, 10.0, 20.0),
        (10.0, -10.0, -20.0),
        (720.0, 10.0, 10.0),
        (0.0, -720.0, 0.0),
        (0.0, 180.0, 180.0),
        (180.0, 0.0, 180.0),
        (0.0, -540.0, 180.0),
        (0.0, 181.0, -179.0),
        (0.0, -181.0, 179.0),
        (1e15, 1e15 + 90.0, 90.0),
    ] {
        assert_eq!(Math::delta_angle_deg(current, target), expected);
    }
}

#[test]
fn f32_and_f64_scalar_inputs_use_explicit_units() {
    let degrees: f64 = Math::delta_angle_deg(350.0_f32, 10.0_f32);
    assert_eq!(degrees, 20.0);
    let radians: f64 = Math::delta_angle_rad(0.0_f32, std::f32::consts::FRAC_PI_2);
    assert_eq!(radians, f64::from(std::f32::consts::FRAC_PI_2));
    assert_eq!(Math::delta_angle_deg(350.0_f64, 10.0), 20.0);
    close(Math::delta_angle_rad(PI * 1.5, PI * 0.5), PI);
    assert_eq!(Math::delta_angle_rad(PI, -PI), 0.0);
}

#[test]
fn tiny_negative_deltas_survive_wraparound_calculation() {
    for tiny in [1e-12, 1e-30, f64::MIN_POSITIVE, f64::from_bits(1)] {
        assert_eq!(Math::delta_angle_rad(0.0, -tiny), -tiny);
        assert_eq!(Math::delta_angle_deg(0.0, -tiny), -tiny);
        assert_eq!(Math::delta_angle_rad(0.0, tiny), tiny);
        assert_eq!(Math::delta_angle_deg(0.0, tiny), tiny);
    }
}

#[test]
fn invalid_angles_and_overflowing_differences_are_handled() {
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(Math::delta_angle_deg(0.0, bad).is_nan());
        assert!(Math::delta_angle_deg(bad, 0.0).is_nan());
        assert!(Math::delta_angle_rad(0.0, bad).is_nan());
        assert!(Math::delta_angle_rad(bad, 0.0).is_nan());
    }
    let deg = Math::delta_angle_deg(-f64::MAX, f64::MAX);
    let rad = Math::delta_angle_rad(-f64::MAX, f64::MAX);
    assert!(deg > -180.0 && deg <= 180.0);
    assert!(rad > -PI && rad <= PI);
}

#[test]
fn degree_interpolation_and_movement_follow_shortest_arc() {
    assert_eq!(Math::lerp_angle_deg(350.0_f32, 10.0, 0.5), 360.0);
    assert_eq!(Math::lerp_angle_deg(350, 10, -1.0), 350.0);
    assert_eq!(Math::lerp_angle_deg(350, 10, 2.0), 370.0);
    assert_eq!(Math::move_towards_angle_deg(350, 10, 5.0), 355.0);
    assert_eq!(Math::move_towards_angle_deg(350, 10, 40.0), 370.0);
    assert_eq!(Math::move_towards_angle_deg(10, 350, 5.0), 5.0);
    assert_eq!(Math::move_towards_angle_deg(350, 10, -1.0), 350.0);
    assert!(Math::lerp_angle_deg(0, 90, f64::NAN).is_nan());
    assert!(Math::move_towards_angle_deg(0, 90, f64::NAN).is_nan());
}

#[test]
fn damping_degree_and_radian_units_match_including_velocity() {
    let mut degrees = 350.0_f64;
    let mut radians = degrees.to_radians();
    let mut velocity_deg = 2.0;
    let mut velocity_rad = 2.0_f64.to_radians();
    for _ in 0..120 {
        degrees =
            Math::smooth_damp_angle_deg(degrees, 10.0, &mut velocity_deg, 0.3, 60.0, 1.0 / 60.0);
        radians = Math::smooth_damp_angle_rad(
            radians,
            10.0_f64.to_radians(),
            &mut velocity_rad,
            0.3,
            60.0_f64.to_radians(),
            1.0 / 60.0,
        );
        close(degrees.to_radians(), radians);
        close(velocity_deg.to_radians(), velocity_rad);
    }
    assert!(degrees > 350.0 && degrees <= 370.0);
    let mut velocity = 3.0;
    assert_eq!(
        Math::smooth_damp_angle_deg(350.0_f32, 10.0, &mut velocity, 0.3, 60.0, 0.0),
        350.0
    );
    assert_eq!(velocity, 3.0);
    assert!(Math::smooth_damp_angle_deg(350, 10, &mut velocity, 0.3, -1.0, 0.1).is_nan());
    assert_eq!(velocity, 3.0);
}

#[test]
fn equivalent_integer_turns_preserve_delta() {
    for current in (-720..=720).step_by(45) {
        for target in (-720..=720).step_by(45) {
            let delta = Math::delta_angle_deg(current, target);
            assert!(delta > -180.0 && delta <= 180.0);
            assert_eq!((f64::from(target - current) - delta) % 360.0, 0.0);
            close(
                Math::delta_angle_rad(
                    f64::from(current).to_radians(),
                    f64::from(target).to_radians(),
                )
                .abs(),
                delta.abs().to_radians(),
            );
        }
    }
    close(Math::delta_angle_rad(0.0, TAU), 0.0);
}
