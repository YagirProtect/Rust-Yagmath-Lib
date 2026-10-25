use crate::vector::{Vector2, Vector3, Vector4};
use std::collections::{HashMap, HashSet};

#[test]
fn equality_checks_every_component() {
    let v = Vector2::new(1, 2);
    assert_eq!(v, Vector2::new(1, 2));
    assert_ne!(v, Vector2::new(9, 2));
    assert_ne!(v, Vector2::new(1, 9));

    let v = Vector3::new(1, 2, 3);
    assert_eq!(v, Vector3::new(1, 2, 3));
    assert_ne!(v, Vector3::new(9, 2, 3));
    assert_ne!(v, Vector3::new(1, 9, 3));
    assert_ne!(v, Vector3::new(1, 2, 9));

    let v = Vector4::new(1, 2, 3, 4);
    assert_eq!(v, Vector4::new(1, 2, 3, 4));
    assert_ne!(v, Vector4::new(9, 2, 3, 4));
    assert_ne!(v, Vector4::new(1, 9, 3, 4));
    assert_ne!(v, Vector4::new(1, 2, 9, 4));
    assert_ne!(v, Vector4::new(1, 2, 3, 9));
}

#[test]
fn float_equality_preserves_float_semantics() {
    assert_eq!(Vector2::new(0.0_f32, 1.0), Vector2::new(-0.0, 1.0));
    assert_ne!(Vector2::new(f32::NAN, 1.0), Vector2::new(f32::NAN, 1.0));
    assert_eq!(Vector3::new(1.0_f64, 2.0, 3.0), Vector3::new(1.0, 2.0, 3.0));
    assert_ne!(
        Vector4::new(1.0_f64, 2.0, 3.0, f64::NAN),
        Vector4::new(1.0, 2.0, 3.0, f64::NAN)
    );
}

#[test]
fn numeric_vectors_are_copy() {
    fn require_copy<T: Copy>(_: T) {}

    let v2 = Vector2::new(1_i32, 2);
    let v3 = Vector3::new(1.0_f32, 2.0, 3.0);
    let v4 = Vector4::new(1.0_f64, 2.0, 3.0, 4.0);
    require_copy(v2);
    require_copy(v3);
    require_copy(v4);
    assert_eq!(v2.x, 1);
    assert_eq!(v3.z, 3.0);
    assert_eq!(v4.w, 4.0);
}

#[test]
fn clone_works_for_non_copy_components() {
    let original = Vector2::new(String::from("x"), String::from("y"));
    let mut cloned = original.clone();
    assert_eq!(original, cloned);
    cloned.x.push('!');
    assert_eq!(original.x, "x");
    assert_ne!(original, cloned);
}

#[test]
fn integer_vectors_work_as_hash_keys() {
    let mut positions = HashMap::new();
    positions.insert(Vector2::new(1, 2), "tile");
    assert_eq!(positions.get(&Vector2::new(1, 2)), Some(&"tile"));
    assert_eq!(positions.get(&Vector2::new(2, 1)), None);

    let mut positions = HashSet::new();
    assert!(positions.insert(Vector3::new(1, 2, 3)));
    assert!(!positions.insert(Vector3::new(1, 2, 3)));
    assert!(positions.insert(Vector3::new(1, 2, 4)));

    let mut positions = HashSet::new();
    assert!(positions.insert(Vector4::new(1, 2, 3, 4)));
    assert!(!positions.insert(Vector4::new(1, 2, 3, 4)));
    assert!(positions.insert(Vector4::new(1, 2, 3, 5)));
}
