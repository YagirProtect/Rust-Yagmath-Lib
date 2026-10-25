#![cfg_attr(not(feature = "std"), no_std)]

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
//! # Optional serialization
//!
//! Enable the `serde` Cargo feature for Serialize/Deserialize on vectors, bounds,
//! planes, segment intersections and conversion policies/errors. Generic
//! components need only the corresponding Serde trait, not numeric traits.
//! Bounds restore public corners without validation; Plane validates its private
//! unit-normal and finite-offset invariants. Default std builds have no external dependencies.
//!
//! # Embedded targets
//!
//! The default `std` feature uses standard float operations. Disable defaults and
//! enable `no_std` for libm math without a heap allocator. Add `serde` for optional
//! allocation-free serialization implementations. If both backends are enabled,
//! std takes precedence. At least one backend must be enabled. Geometry still
//! returns f64; float results can differ in their last bits between backends.
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

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("enable the default `std` feature or use --no-default-features --features no_std");

// Unit tests use the host test runner while the numeric backend still follows features.
#[cfg(test)]
#[macro_use]
extern crate std;

mod float_math;

pub mod vector;
pub use vector::VectorComponent;
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
    #[cfg(feature = "serde")]
    #[doc = include_str!("../docs/serde.md")]
    mod serialization {}
    #[doc = include_str!("../docs/no_std.md")]
    mod no_std {}
}
#[cfg(test)]
mod tests;
