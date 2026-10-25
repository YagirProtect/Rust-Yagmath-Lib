//! Scalar traits for arithmetic and f64 geometry. Integer add/sub/mul saturate; float add/sub/mul use ordinary IEEE arithmetic. Division separately rejects non-finite floats.

pub mod add;
pub mod sub;
pub mod div;
pub mod mul;
pub mod to_f64;
pub(crate) mod number_lerp;
