use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn display_formats_components_in_order() {
    assert_eq!(Vector2::new(1, -2).to_string(), "(1, -2)");
    assert_eq!(Vector3::new(1, 0, -3).to_string(), "(1, 0, -3)");
    assert_eq!(Vector4::new(1, 2, 3, 4).to_string(), "(1, 2, 3, 4)");
}

#[test]
fn display_supports_floats_and_precision() {
    assert_eq!(Vector2::new(1.5_f32, -2.5).to_string(), "(1.5, -2.5)");
    assert_eq!(
        format!("{:.2}", Vector3::new(1.25_f64, -2.5, 3.0)),
        "(1.25, -2.50, 3.00)"
    );
}

#[test]
fn display_does_not_require_numeric_or_copy_components() {
    let vector = Vector2::new(String::from("left"), String::from("right"));
    assert_eq!(vector.to_string(), "(left, right)");
    assert_eq!(vector.x, "left");
}
