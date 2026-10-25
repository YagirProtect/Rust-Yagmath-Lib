//! Game-oriented vector math and geometry with explicit numeric conversion policies.
//!
//! Start with [`vector::Vector2`], [`vector::Vector3`], [`vector::Vector4`],
//! [`bounds::Bounds2`], [`bounds::Bounds3`], [`Plane`] and [`Math`].
//! Vectors store generic components; most geometric calculations return `f64`.
//!
//! # Numeric conventions
//!
//! * Angle-valued APIs use `_rad` for radians and `_deg` for degrees.
//!   Directions use +X right, +Y up and +Z forward.
//! * Integer vector `+`, `-` and `*` saturate; `/` returns `Option<Vector>`.
//! * Float addition/subtraction/multiplication use ordinary IEEE arithmetic.
//! * Normalization/rotation/reflection return `Option`; angle/slerp use NaN for
//!   undefined directions. Conversion returns a component-specific `Result`.
//! * Wide integers can lose precision in geometry after conversion to `f64`.
//!   Integer-to-integer [`conversion`] does not pass through `f64`.
//!
//! # Example
//!
//! ```
//! use yagmath::{Math, Plane, vector::Vector3};
//!
//! let position = Vector3::new(0, 0, 0);
//! let next = position.move_towards(Vector3::new(3, 4, 0), 2.5);
//! assert_eq!(next, Vector3::new(1.5, 2.0, 0.0));
//! assert_eq!(Math::remap(50, 0, 100, 0.0, 1.0), 0.5);
//! let ground = Plane::from_normal_point(Vector3::<i32>::up(), Vector3::new(0_i32, 10, 0));
//! assert!(ground.is_some());
//! ```
//!
//! The repository's `README.md` and `docs/` contain the full method cookbook,
//! illustrated geometry, design notes and validation instructions.

pub mod vector;
pub mod bounds;
pub mod plane;
pub use plane::Plane;
pub mod math;
pub use math::{Math, SegmentIntersection2};
pub mod conversion;
pub use conversion::{ConversionError, ConversionErrorKind, ConversionMode, RoundingMode};
pub mod extensions;

// Compile the Markdown cookbook examples with `cargo test --doc` as well.
#[cfg(doctest)]
mod guide_examples {
    #[doc = include_str!("../README.md")]
    mod readme {}
    #[doc = include_str!("../docs/vectors.md")]
    mod vectors {}
    #[doc = include_str!("../docs/math.md")]
    mod math {}
    #[doc = include_str!("../docs/bounds.md")]
    mod bounds {}
    #[doc = include_str!("../docs/plane.md")]
    mod plane {}
    #[doc = include_str!("../docs/conversion.md")]
    mod conversion {}
    #[doc = include_str!("../docs/extensions.md")]
    mod extensions {}
}
#[cfg(test)]
mod tests;
