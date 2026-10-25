//! Explicit numeric conversions without panics.
//!
//! Integer conversions never pass through f64. `Checked` rejects integer overflow
//! and precision loss when converting to a float. Float-to-integer conversions
//! apply the selected rounding first, then check the resulting integer's range.
//! `Clamp` allows rounding and limits overflow to finite target bounds.
//! NaN is always an error. Infinity is an error in Checked and saturates in Clamp.
//! `Wrap` supports integer-to-integer conversions only, using Rust's `as` rules.
//!
//! ```
//! use yagmath::{ConversionError, ConversionMode, RoundingMode};
//! use yagmath::vector::Vector2;
//!
//! fn main() -> Result<(), ConversionError> {
//!     let values = Vector2::new(300_i32, -130);
//!     let clamped = values.convert::<i8>(ConversionMode::Clamp)?;
//!     let wrapped = values.convert::<i8>(ConversionMode::Wrap)?;
//!
//!     assert_eq!(clamped, Vector2::new(127, -128));
//!     assert_eq!(wrapped, Vector2::new(44, 126));
//!
//!     let checked = values.convert::<i8>(ConversionMode::Checked);
//!
//!     assert!(checked.is_err());
//!
//!     let values = Vector2::new(1.9_f64, -1.9);
//!     let rounded = values.convert_with_rounding::<i32>(
//!         ConversionMode::Checked, RoundingMode::Round,
//!     )?;
//!
//!     assert_eq!(rounded, Vector2::new(2, -2));
//!
//!     Ok(())
//! }
//! ```

mod floats;
mod integers;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Policy for numeric range overflow and float representation loss.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConversionMode {
    /// Rejects range overflow and conversion-to-float precision loss.
    Checked,
    /// Allows float rounding and saturates overflow to finite target bounds.
    Clamp,
    /// Uses Rust integer cast rules; unsupported for any float source/target.
    Wrap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Rounding applied before float-to-integer range validation; ignored otherwise.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RoundingMode {
    /// Removes the fractional part toward zero. The default for vector convert().
    Truncate,
    /// Rounds halfway values away from zero.
    Round,
    /// Rounds toward negative infinity.
    Floor,
    /// Rounds toward positive infinity.
    Ceil,
}

impl RoundingMode {
    /// Applies float-to-integer rounding before conversion/range validation.
    /// Other conversion families ignore RoundingMode. Uses standard f64 rounding;
    /// NaN/infinity stay non-finite so the caller can apply its selected mode.
    fn apply(self, value: f64) -> f64 {
        match self {
            Self::Truncate => value.trunc(),
            Self::Round => value.round(),
            Self::Floor => value.floor(),
            Self::Ceil => value.ceil(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Why a scalar conversion failed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConversionErrorKind {
    /// The rounded/cast value is outside the destination's representable range.
    OutOfRange,
    /// A finite input cannot be represented exactly in the destination float.
    PrecisionLoss,
    /// NaN, or infinity in a mode that rejects it.
    NonFinite,
    /// Wrap was requested with a floating-point source or destination.
    UnsupportedMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The first failing vector component and its conversion error.
/// Implements Display and std::error::Error.
/// With `serde`, deserialization accepts only component names x, y, z and w.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConversionError {
    /// Component name: x, y, z or w. Components are checked in field order.
    pub component: &'static str,
    /// Reason the component could not be converted.
    pub kind: ConversionErrorKind,
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ConversionError {
    /// Maps an owned component name to a static field name without leaking
    /// allocations or requiring a static input buffer during decoding.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Wire representation with owned text so readers need no borrowed buffer.
        #[derive(serde::Deserialize)]
        #[serde(rename = "ConversionError")]
        struct StoredError {
            /// Field name to validate and map to a static identifier.
            component: String,
            /// Scalar conversion failure.
            kind: ConversionErrorKind,
        }
        let stored = StoredError::deserialize(deserializer)?;
        let component = match stored.component.as_str() {
            "x" => "x",
            "y" => "y",
            "z" => "z",
            "w" => "w",
            _ => return Err(serde::de::Error::custom("expected component x, y, z or w")),
        };
        Ok(Self { component, kind: stored.kind })
    }
}

impl std::fmt::Display for ConversionErrorKind {
    /// Writes a human-readable conversion failure; vector errors include the field name.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::OutOfRange => "value is outside the target range",
            Self::PrecisionLoss => "value cannot be represented exactly",
            Self::NonFinite => "non-finite value cannot be converted in this mode",
            Self::UnsupportedMode => "Wrap only supports integer-to-integer conversions",
        })
    }
}

impl std::fmt::Display for ConversionError {
    /// Writes a human-readable conversion failure; vector errors include the field name.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "component {}: {}", self.component, self.kind)
    }
}

impl std::error::Error for ConversionError {}

/// Converts a scalar; vector conversions attach the failing component's name.
pub trait ConvertFrom<T>: Sized {
    /// Converts a borrowed scalar using an explicit overflow and rounding policy.
    /// Implemented for all pairs of the 14 built-in numeric types. Custom
    /// implementations should document any additional validation or panics.
    fn convert_from(
        value: &T,
        mode: ConversionMode,
        rounding: RoundingMode,
    ) -> Result<Self, ConversionErrorKind>;
}
