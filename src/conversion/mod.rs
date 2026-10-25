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

use crate::float_math::FloatMath;
use crate::vector::VectorComponent;

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
            Self::Truncate => FloatMath::trunc(value),
            Self::Round => FloatMath::round(value),
            Self::Floor => FloatMath::floor(value),
            Self::Ceil => FloatMath::ceil(value),
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
/// Implements Display and core::error::Error.
/// With `serde`, deserialization accepts only component names x, y, z and w.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConversionError {
    /// Failed coordinate. Components are checked in field order.
    pub component: VectorComponent,
    /// Reason the component could not be converted.
    pub kind: ConversionErrorKind,
}

impl core::fmt::Display for ConversionErrorKind {
    /// Writes a human-readable conversion failure; vector errors include the field name.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::OutOfRange => "value is outside the target range",
            Self::PrecisionLoss => "value cannot be represented exactly",
            Self::NonFinite => "non-finite value cannot be converted in this mode",
            Self::UnsupportedMode => "Wrap only supports integer-to-integer conversions",
        })
    }
}

impl core::fmt::Display for ConversionError {
    /// Writes a human-readable conversion failure; vector errors include the field name.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "component {}: {}", self.component, self.kind)
    }
}

impl core::error::Error for ConversionError {}

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
