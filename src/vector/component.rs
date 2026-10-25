use core::fmt;

/// Identifies a vector coordinate without storing a string or allocating memory.
/// Shared by Vector2 (X/Y), Vector3 (X/Y/Z) and Vector4 (X/Y/Z/W).
/// The in-memory representation is one byte. With Serde, the wire representation
/// remains a lowercase string, including for non-human-readable formats.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorComponent {
    /// First coordinate.
    X,
    /// Second coordinate.
    Y,
    /// Third coordinate.
    Z,
    /// Fourth coordinate.
    W,
}

impl VectorComponent {
    /// Returns the lowercase field name using a static literal, without allocation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
            Self::W => "w",
        }
    }
}

impl fmt::Display for VectorComponent {
    /// Writes the lowercase coordinate name, preserving conversion error messages.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for VectorComponent {
    /// Serializes a string so existing conversion-error payloads remain compatible.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for VectorComponent {
    /// Accepts borrowed or transient component strings without allocation.
    /// Reading a string rather than an enum tag also preserves binary wire formats.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Validates coordinate names from any string-based Serde deserializer.
        struct ComponentVisitor;
        impl<'de> serde::de::Visitor<'de> for ComponentVisitor {
            type Value = VectorComponent;
            /// Describes accepted names in decoding errors.
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("component x, y, z or w")
            }
            /// Maps a coordinate name to its compact discriminant.
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                match value {
                    "x" => Ok(VectorComponent::X),
                    "y" => Ok(VectorComponent::Y),
                    "z" => Ok(VectorComponent::Z),
                    "w" => Ok(VectorComponent::W),
                    _ => Err(E::unknown_variant(value, &["x", "y", "z", "w"])),
                }
            }
        }
        deserializer.deserialize_str(ComponentVisitor)
    }
}

/// Maps a generated lowercase field identifier to its enum variant at expansion
/// time. Unknown fields fail to compile instead of introducing a runtime fallback.
macro_rules! component_for_field {
    (x) => {
        crate::vector::VectorComponent::X
    };
    (y) => {
        crate::vector::VectorComponent::Y
    };
    (z) => {
        crate::vector::VectorComponent::Z
    };
    (w) => {
        crate::vector::VectorComponent::W
    };
}

pub(super) use component_for_field;
