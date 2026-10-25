use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn lerp_endpoints_and_midpoints_in_every_dimension() {
    let a = Vector2::new(0, 10);
    let b = Vector2::new(10, -10);
    assert_eq!(a.lerp(b, 0.0), Vector2::new(0.0, 10.0));
    assert_eq!(a.lerp(b, 1.0), Vector2::new(10.0, -10.0));
    assert_eq!(a.lerp(b, 0.5), Vector2::new(5.0, 0.0));

    let a = Vector3::new(0, 10, -4);
    let b = Vector3::new(10, -10, 8);
    assert_eq!(a.lerp(b, 0.0), Vector3::new(0.0, 10.0, -4.0));
    assert_eq!(a.lerp(b, 1.0), Vector3::new(10.0, -10.0, 8.0));
    assert_eq!(a.lerp(b, 0.5), Vector3::new(5.0, 0.0, 2.0));

    let a = Vector4::new(0, 10, -4, 6);
    let b = Vector4::new(10, -10, 8, 2);
    assert_eq!(a.lerp(b, 0.0), Vector4::new(0.0, 10.0, -4.0, 6.0));
    assert_eq!(a.lerp(b, 1.0), Vector4::new(10.0, -10.0, 8.0, 2.0));
    assert_eq!(a.lerp(b, 0.5), Vector4::new(5.0, 0.0, 2.0, 4.0));
}

#[test]
fn lerp_clamps_while_unclamped_extrapolates() {
    macro_rules! check {
        ($vector:ident, $($field:ident),+) => {{
            let a = $vector { $($field: 10_i32),+ };
            let b = $vector { $($field: 20_i32),+ };
            assert_eq!(a.lerp(b, -0.5), $vector { $($field: 10.0),+ });
            assert_eq!(a.lerp(b, 1.5), $vector { $($field: 20.0),+ });
            assert_eq!(a.lerp_unclamped(b, -0.5), $vector { $($field: 5.0),+ });
            assert_eq!(a.lerp_unclamped(b, 1.5), $vector { $($field: 25.0),+ });
            for t in [0.0, 0.25, 0.5, 1.0] {
                assert_eq!(a.lerp(b, t), a.lerp_unclamped(b, t));
            }
        }};
    }
    check!(Vector2, x, y);
    check!(Vector3, x, y, z);
    check!(Vector4, x, y, z, w);
}

#[test]
fn lerp_preserves_fractions_and_converts_before_integer_subtraction() {
    assert_eq!(Vector2::new(0_u8, 255).lerp(Vector2::new(255, 0), 0.5),
        Vector2::new(127.5, 127.5));
    assert_eq!(Vector2::new(i8::MIN, i8::MAX).lerp(Vector2::new(i8::MAX, i8::MIN), 0.5),
        Vector2::new(-0.5, -0.5));
    assert_eq!(Vector2::new(1.0_f32, 3.0).lerp(Vector2::new(3.0, 7.0), 0.25),
        Vector2::new(1.5, 4.0));
    assert_eq!(Vector2::new(1.0_f64, 3.0).lerp(Vector2::new(3.0, 7.0), 0.25),
        Vector2::new(1.5, 4.0));
}

#[test]
fn lerp_leaves_source_unchanged_and_equal_endpoints_stay_equal() {
    let a = Vector3::new(1, 2, 3);
    let b = Vector3::new(5, 6, 7);
    let _ = a.lerp(b, 0.5);
    assert_eq!(a, Vector3::new(1, 2, 3));
    assert_eq!(b, Vector3::new(5, 6, 7));
    assert_eq!(a.lerp_unclamped(a, 2.0), Vector3::new(1.0, 2.0, 3.0));
}
