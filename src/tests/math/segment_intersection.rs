use std::vec::Vec;
use crate::{Math, SegmentIntersection2 as Hit, vector::Vector2};

#[test]
fn crossings_and_endpoint_touches() {
    for (a, b, c, d, point) in [
        ((0, 0), (4, 4), (0, 4), (4, 0), (2.0, 2.0)),
        ((0, 0), (4, 0), (2, -2), (2, 2), (2.0, 0.0)),
        ((0, 0), (2, 2), (2, 2), (4, 0), (2.0, 2.0)),
        ((0, 0), (4, 0), (2, 0), (2, 2), (2.0, 0.0)),
    ] {
        let v = |p: (i32, i32)| Vector2::new(p.0, p.1);
        let expected = Some(Hit::Point(Vector2::new(point.0, point.1)));
        assert_eq!(Math::segment_intersection(v(a), v(b), v(c), v(d)), expected);
        assert_eq!(Math::segment_intersection(v(b), v(a), v(c), v(d)), expected);
        assert_eq!(Math::segment_intersection(v(c), v(d), v(a), v(b)), expected);
    }
}

#[test]
fn disjoint_parallel_and_infinite_lines_only() {
    let v = |x, y| Vector2::new(x, y);
    assert_eq!(
        Math::segment_intersection(v(0, 0), v(1, 0), v(2, -1), v(2, 1)),
        None
    );
    assert_eq!(
        Math::segment_intersection(v(0, 0), v(2, 0), v(0, 1), v(2, 1)),
        None
    );
    assert_eq!(
        Math::segment_intersection(v(0, 0), v(1, 0), v(2, 0), v(3, 0)),
        None
    );
    assert_eq!(
        Math::segment_intersection(v(0, 0), v(1, 1), v(0, 2), v(1, 2)),
        None
    );
}

#[test]
fn overlap_endpoints_follow_first_segment_direction() {
    let a = Vector2::new(0, 0);
    let b = Vector2::new(6, 0);
    let c = Vector2::new(2, 0);
    let d = Vector2::new(8, 0);
    assert_eq!(
        Math::segment_intersection(a, b, c, d),
        Some(Hit::Overlap {
            start: Vector2::new(2.0, 0.0),
            end: Vector2::new(6.0, 0.0),
        })
    );
    assert_eq!(
        Math::segment_intersection(b, a, c, d),
        Some(Hit::Overlap {
            start: Vector2::new(6.0, 0.0),
            end: Vector2::new(2.0, 0.0),
        })
    );
    assert_eq!(
        Math::segment_intersection(a, b, b, a),
        Some(Hit::Overlap {
            start: Vector2::new(0.0, 0.0),
            end: Vector2::new(6.0, 0.0),
        })
    );
    assert_eq!(
        Math::segment_intersection(
            Vector2::new(0, 0),
            Vector2::new(4, 4),
            Vector2::new(3, 3),
            Vector2::new(1, 1)
        ),
        Some(Hit::Overlap {
            start: Vector2::new(1.0, 1.0),
            end: Vector2::new(3.0, 3.0),
        })
    );
    assert_eq!(
        Math::segment_intersection(
            Vector2::new(2, 6),
            Vector2::new(2, 0),
            Vector2::new(2, 1),
            Vector2::new(2, 8)
        ),
        Some(Hit::Overlap {
            start: Vector2::new(2.0, 6.0),
            end: Vector2::new(2.0, 1.0),
        })
    );
}

#[test]
fn collinear_touch_and_zero_length_segments() {
    let v = |x, y| Vector2::new(x, y);
    let point = Some(Hit::Point(v(2.0, 0.0)));
    assert_eq!(
        Math::segment_intersection(v(0.0, 0.0), v(2.0, 0.0), v(2.0, 0.0), v(4.0, 0.0)),
        point
    );
    assert_eq!(
        Math::segment_intersection(v(2.0, 0.0), v(2.0, 0.0), v(0.0, 0.0), v(4.0, 0.0)),
        point
    );
    assert_eq!(
        Math::segment_intersection(v(0.0, 0.0), v(4.0, 0.0), v(2.0, 0.0), v(2.0, 0.0)),
        point
    );
    assert_eq!(
        Math::segment_intersection(v(2.0, 0.0), v(2.0, 0.0), v(2.0, 0.0), v(2.0, 0.0)),
        point
    );
    assert_eq!(
        Math::segment_intersection(v(1.0, 2.0), v(1.0, 2.0), v(0.0, 0.0), v(4.0, 4.0)),
        None
    );
    assert_eq!(
        Math::segment_intersection(v(1.0, 0.0), v(1.0, 0.0), v(2.0, 0.0), v(2.0, 0.0)),
        None
    );
}

#[test]
fn fractional_intersection_and_mixed_scalars() {
    let hit = Math::segment_intersection(
        Vector2::new(0, 0),
        Vector2::new(3, 1),
        Vector2::new(0.0_f32, 1.0),
        Vector2::new(3.0, 0.0),
    );
    assert_eq!(hit, Some(Hit::Point(Vector2::new(1.5, 0.5))));
    let hit = Math::segment_intersection(
        Vector2::new(0, 0),
        Vector2::new(3, 1),
        Vector2::new(1, -1),
        Vector2::new(1, 2),
    );
    let Some(Hit::Point(p)) = hit else {
        panic!("expected point")
    };
    assert!((p.x - 1.0).abs() < 1e-14 && (p.y - 1.0 / 3.0).abs() < 1e-14);
}

#[test]
fn very_large_tiny_and_anisotropic_coordinates() {
    for size in [f64::MAX, f64::MIN_POSITIVE, f64::from_bits(1)] {
        let hit = Math::segment_intersection(
            Vector2::new(-size, 0.0),
            Vector2::new(size, 0.0),
            Vector2::new(0.0, -size),
            Vector2::new(0.0, size),
        );
        assert_eq!(hit, Some(Hit::Point(Vector2::new(0.0, 0.0))));
    }
    let hit = Math::segment_intersection(
        Vector2::new(-1e300, 0.0),
        Vector2::new(1e300, 0.0),
        Vector2::new(0.0, -1e-300),
        Vector2::new(0.0, 1e-300),
    );
    assert_eq!(hit, Some(Hit::Point(Vector2::new(0.0, 0.0))));
    let hit = Math::segment_intersection(
        Vector2::new(-f64::MAX, -f64::MAX),
        Vector2::new(f64::MAX, f64::MAX),
        Vector2::new(-f64::MAX, f64::MAX),
        Vector2::new(f64::MAX, -f64::MAX),
    );
    assert_eq!(hit, Some(Hit::Point(Vector2::new(0.0, 0.0))));
}

#[test]
fn invalid_coordinates_are_rejected_in_every_argument() {
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        for index in 0..4 {
            for use_x in [true, false] {
                let mut points = [
                    Vector2::new(0.0, 0.0),
                    Vector2::new(4.0, 4.0),
                    Vector2::new(0.0, 4.0),
                    Vector2::new(4.0, 0.0),
                ];
                if use_x {
                    points[index].x = bad;
                } else {
                    points[index].y = bad;
                }
                assert_eq!(
                    Math::segment_intersection(points[0], points[1], points[2], points[3]),
                    None
                );
            }
        }
    }
}

#[test]
fn every_numeric_component_type_is_supported() {
    macro_rules! check {
        ($($type:ty),+) => { $(
            assert_eq!(Math::segment_intersection(
                Vector2::new(0 as $type, 0 as $type), Vector2::new(4 as $type, 4 as $type),
                Vector2::new(0 as $type, 4 as $type), Vector2::new(4 as $type, 0 as $type)),
                Some(Hit::Point(Vector2::new(2.0, 2.0))));
        )+ };
    }
    check!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn near_parallel_crossing_does_not_use_an_absolute_epsilon() {
    let result = Math::segment_intersection(
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 1e-14),
        Vector2::new(0.0, 1e-14),
        Vector2::new(1.0, 0.0),
    );
    let Some(Hit::Point(p)) = result else {
        panic!("expected a shallow crossing")
    };
    assert!((p.x - 0.5).abs() < 1e-14);
    assert!((p.y / 5e-15 - 1.0).abs() < 1e-14);
}

#[test]
fn coordinate_lost_during_scaling_is_rejected() {
    let tiny = f64::from_bits(1);
    // The actual crossing has x=tiny. Scaling the enormous x range would
    // erase this coordinate, so the documented policy is None, not a false x=0.
    assert_eq!(
        Math::segment_intersection(
            Vector2::new(-f64::MAX, 0.0),
            Vector2::new(f64::MAX, 0.0),
            Vector2::new(tiny, -1.0),
            Vector2::new(tiny, 1.0),
        ),
        None
    );
}

#[test]
fn exhaustive_small_integer_grid_matches_integer_orientation_reference() {
    // Independent integer orientation predicates classify all 6,561 endpoint
    // configurations in a 3x3 grid, including degenerate segments.
    fn orient(a: Vector2<i32>, b: Vector2<i32>, c: Vector2<i32>) -> i32 {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }
    let points: Vec<_> = (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| Vector2::new(x, y)))
        .collect();
    for &a in &points {
        for &b in &points {
            for &c in &points {
                for &d in &points {
                    let o = [
                        orient(a, b, c),
                        orient(a, b, d),
                        orient(c, d, a),
                        orient(c, d, b),
                    ];
                    let x_low = a.x.min(b.x).max(c.x.min(d.x));
                    let x_high = a.x.max(b.x).min(c.x.max(d.x));
                    let y_low = a.y.min(b.y).max(c.y.min(d.y));
                    let y_high = a.y.max(b.y).min(c.y.max(d.y));
                    let intersects =
                        x_low <= x_high && y_low <= y_high && o[0] * o[1] <= 0 && o[2] * o[3] <= 0;
                    let overlap = intersects && o == [0; 4] && (x_low < x_high || y_low < y_high);
                    let result = Math::segment_intersection(a, b, c, d);
                    assert_eq!(result.is_some(), intersects, "{a:?} {b:?} {c:?} {d:?}");
                    assert_eq!(
                        matches!(result, Some(Hit::Overlap { .. })),
                        overlap,
                        "{a:?} {b:?} {c:?} {d:?}"
                    );
                    if let Some(Hit::Point(p)) = result {
                        let af = Vector2::new(f64::from(a.x), f64::from(a.y));
                        let bf = Vector2::new(f64::from(b.x), f64::from(b.y));
                        let cf = Vector2::new(f64::from(c.x), f64::from(c.y));
                        let df = Vector2::new(f64::from(d.x), f64::from(d.y));
                        assert!(
                            ((bf.x - af.x) * (p.y - af.y) - (bf.y - af.y) * (p.x - af.x)).abs()
                                < 1e-12
                        );
                        assert!(
                            ((df.x - cf.x) * (p.y - cf.y) - (df.y - cf.y) * (p.x - cf.x)).abs()
                                < 1e-12
                        );
                    }
                }
            }
        }
    }
}
