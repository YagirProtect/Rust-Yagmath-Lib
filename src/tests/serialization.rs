use crate::VectorComponent;

#[test]
fn component_enum_keeps_the_existing_string_wire_format() {
    for (component, name) in [
        (VectorComponent::X, "x"),
        (VectorComponent::Y, "y"),
        (VectorComponent::Z, "z"),
        (VectorComponent::W, "w"),
    ] {
        assert_eq!(serde_json::to_string(&component).unwrap(), format!("\"{name}\""));
        round_trip(component);
    }
    let error = ConversionError {
        component: VectorComponent::Y,
        kind: ConversionErrorKind::OutOfRange,
    };
    assert_eq!(serde_json::to_string(&error).unwrap(), r#"{"component":"y","kind":"OutOfRange"}"#);
    for invalid in [r#""X""#, r#""unknown""#, "0", "null"] {
        assert!(serde_json::from_str::<VectorComponent>(invalid).is_err());
    }
}
use std::string::String;
use crate::bounds::{Bounds2, Bounds3};
use crate::vector::{Vector2, Vector3, Vector4};
use crate::{
    ConversionError, ConversionErrorKind, ConversionMode, Plane, RoundingMode, SegmentIntersection2,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fmt::Debug;

/// Checks serialization equality and decoding from both strings and owned readers.
fn round_trip<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: T) {
    let json = serde_json::to_string(&value).unwrap();
    assert_eq!(serde_json::from_str::<T>(&json).unwrap(), value);
    assert_eq!(
        serde_json::from_reader::<_, T>(json.as_bytes()).unwrap(),
        value
    );
}

#[test]
fn vectors_use_named_fields_in_component_order() {
    assert_eq!(
        serde_json::to_string(&Vector2::new(1, -2)).unwrap(),
        r#"{"x":1,"y":-2}"#
    );
    assert_eq!(
        serde_json::to_string(&Vector3::new(1, 2, 3)).unwrap(),
        r#"{"x":1,"y":2,"z":3}"#
    );
    assert_eq!(
        serde_json::to_string(&Vector4::new(1, 2, 3, 4)).unwrap(),
        r#"{"x":1,"y":2,"z":3,"w":4}"#
    );
    round_trip(Vector2::new(i128::MIN, i128::MAX));
    round_trip(Vector3::new(0_u128, u128::MAX, 42));
    round_trip(Vector4::new(f64::MIN_POSITIVE, f64::MAX, -0.0, 0.1));
    round_trip(Vector2::new(0.1_f32, f32::MAX));
}

#[test]
fn generic_components_need_no_numeric_default_or_copy_traits() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Label(String);
    round_trip(Vector2::new(Label("first".into()), Label("second".into())));
    round_trip(Bounds2 {
        min: Vector2::new(Label("a".into()), Label("b".into())),
        max: Vector2::new(Label("c".into()), Label("d".into())),
    });
    let borrowed: Vector2<&str> = serde_json::from_str(r#"{"x":"left","y":"up"}"#).unwrap();
    assert_eq!(borrowed, Vector2::new("left", "up"));
}

#[test]
fn invalid_vector_components_are_reported_by_the_scalar_deserializer() {
    for json in [r#"{"x":1}"#, r#"{"x":128,"y":0}"#, r#"{"x":null,"y":0}"#] {
        assert!(serde_json::from_str::<Vector2<i8>>(json).is_err());
    }
}

#[test]
fn bounds_restore_corners_and_preserve_validation_behavior() {
    let bounds = Bounds2::new(Vector2::new(-10, 20), Vector2::new(30, 40)).unwrap();
    assert_eq!(
        serde_json::to_string(&bounds).unwrap(),
        r#"{"min":{"x":-10,"y":20},"max":{"x":30,"y":40}}"#
    );
    round_trip(bounds);
    round_trip(Bounds3::new(Vector3::new(0.0, 1.0, 2.0), Vector3::new(3.0, 4.0, 5.0)).unwrap());
    let invalid: Bounds2<i32> =
        serde_json::from_str(r#"{"min":{"x":10,"y":0},"max":{"x":0,"y":10}}"#).unwrap();
    assert!(!invalid.is_valid());
    assert_eq!(invalid.closest_point(Vector2::new(5, 5)), None);
}

#[test]
fn plane_round_trips_preserve_coefficients_and_projection() {
    for normal in [
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(1.0, 2.0, 3.0),
        Vector3::new(f64::MAX, -f64::MAX, f64::MAX),
        Vector3::new(f64::from_bits(1), f64::from_bits(2), 0.0),
    ] {
        let plane = Plane::from_normal_point(normal, Vector3::new(2, 3, 4)).unwrap();
        round_trip(plane);
        round_trip(plane.flipped());
    }
    let plane: Plane =
        serde_json::from_str(r#"{"normal":{"x":0.0,"y":1.0,"z":0.0},"distance":-10.0}"#).unwrap();
    assert_eq!(plane.signed_distance(Vector3::new(0, 13, 0)), 3.0);
    assert_eq!(
        plane.closest_point(Vector3::new(5, 13, 6)),
        Vector3::new(5.0, 10.0, 6.0)
    );
}

#[test]
fn plane_deserialization_rejects_invalid_geometry_and_missing_coefficients() {
    for json in [
        r#"{"normal":{"x":0,"y":0,"z":0},"distance":0}"#,
        r#"{"normal":{"x":0,"y":2,"z":0},"distance":0}"#,
        r#"{"normal":{"x":1e308,"y":1e308,"z":0},"distance":0}"#,
        r#"{"normal":{"x":null,"y":1,"z":0},"distance":0}"#,
        r#"{"normal":{"x":0,"y":1,"z":0},"distance":null}"#,
        r#"{"normal":{"x":0,"y":1,"z":0}}"#,
    ] {
        assert!(
            serde_json::from_str::<Plane>(json).is_err(),
            "accepted {json}"
        );
    }
}

#[test]
fn plane_normal_tolerance_is_bounded_and_accepted_values_are_not_changed() {
    let near_unit = 1.0 + 4.0 * f64::EPSILON;
    let json = format!(r#"{{"normal":{{"x":0,"y":{near_unit},"z":0}},"distance":2}}"#);
    let plane: Plane = serde_json::from_str(&json).unwrap();
    assert_eq!(plane.normal().y, near_unit);
    assert_eq!(plane.distance(), 2.0);

    let outside = 1.0 + 32.0 * f64::EPSILON;
    let json = format!(r#"{{"normal":{{"x":0,"y":{outside},"z":0}},"distance":2}}"#);
    assert!(serde_json::from_str::<Plane>(&json).is_err());
}

#[test]
fn segment_intersection_variants_round_trip() {
    round_trip(SegmentIntersection2::Point(Vector2::new(1.0, 2.0)));
    round_trip(SegmentIntersection2::Overlap {
        start: Vector2::new(3.0, 4.0),
        end: Vector2::new(1.0, 2.0),
    });
}

#[test]
fn conversion_policies_and_error_kinds_round_trip() {
    for mode in [
        ConversionMode::Checked,
        ConversionMode::Clamp,
        ConversionMode::Wrap,
    ] {
        round_trip(mode);
    }
    for rounding in [
        RoundingMode::Truncate,
        RoundingMode::Round,
        RoundingMode::Floor,
        RoundingMode::Ceil,
    ] {
        round_trip(rounding);
    }
    for kind in [
        ConversionErrorKind::OutOfRange,
        ConversionErrorKind::PrecisionLoss,
        ConversionErrorKind::NonFinite,
        ConversionErrorKind::UnsupportedMode,
    ] {
        round_trip(kind);
        for component in [VectorComponent::X, VectorComponent::Y, VectorComponent::Z, VectorComponent::W] {
            round_trip(ConversionError { component, kind });
        }
    }
}

#[test]
fn conversion_errors_decode_owned_input_without_a_static_input_lifetime() {
    let json = String::from(r#"{"component":"z","kind":"OutOfRange"}"#);
    let error = serde_json::from_str::<ConversionError>(&json).unwrap();
    drop(json);
    assert_eq!(error.component, VectorComponent::Z);
    let escaped: ConversionError = serde_json::from_str(
        r#"{"component":"\u0078","kind":"OutOfRange"}"#,
    ).unwrap();
    assert_eq!(escaped.component, VectorComponent::X);
    assert!(
        serde_json::from_str::<ConversionError>(r#"{"component":"unknown","kind":"OutOfRange"}"#)
            .is_err()
    );
}
