use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn follows_line_with_total_distance_limit_in_all_dimensions() {
    let v = Vector2::new(0, 0).move_towards(Vector2::new(3, 4), 2.5);
    assert_eq!(v, Vector2::new(1.5, 2.0));
    let v = Vector3::new(0, 0, 0).move_towards(Vector3::new(2, 3, 6), 3.5);
    assert_eq!(v, Vector3::new(1.0, 1.5, 3.0));
    let v = Vector4::new(0, 0, 0, 0).move_towards(Vector4::new(1, 1, 1, 1), 1.0);
    for component in [v.x, v.y, v.z, v.w] {
        assert!((component - 0.5).abs() < 1e-12);
    }
    assert!((v.length() - 1.0).abs() < 1e-12);
}

#[test]
fn reaches_target_and_handles_stationary_steps() {
    let start = Vector2::new(1, 2);
    let target = Vector2::new(4.0_f32, 6.0);
    assert_eq!(start.move_towards(target, 10.0), Vector2::new(4.0, 6.0));
    assert_eq!(start.move_towards(target, 0.0), Vector2::new(1.0, 2.0));
    assert_eq!(start.move_towards(target, -1.0), Vector2::new(1.0, 2.0));
    assert_eq!(
        start.move_towards(target, f64::INFINITY),
        Vector2::new(4.0, 6.0)
    );
    assert_eq!(start.move_towards(start, 1.0), Vector2::new(1.0, 2.0));
}

#[test]
fn extreme_distances_and_invalid_inputs() {
    let start = Vector2::new(-f64::MAX, 0.0);
    assert_eq!(
        start.move_towards(Vector2::new(f64::MAX, 0.0), f64::MAX),
        Vector2::new(0.0, 0.0)
    );
    let v = Vector2::new(0.0, 0.0).move_towards(Vector2::new(f64::MAX, f64::MAX), 1.0);
    assert!((v.length() - 1.0).abs() < 1e-15);
    let v = Vector2::new(0, 0).move_towards(Vector2::new(1, 1), f64::NAN);
    assert!(v.x.is_nan() && v.y.is_nan());
    let v = start.move_towards(Vector2::new(f64::INFINITY, 0.0), 1.0);
    assert!(v.x.is_nan() && v.y.is_nan());
}
