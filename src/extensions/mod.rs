//! Scalar traits for arithmetic and f64 geometry. Integer add/sub/mul saturate; float add/sub/mul use ordinary IEEE arithmetic. Division separately rejects non-finite floats.

pub mod add;
pub mod sub;
pub mod div;
pub mod mul;
pub mod to_f64;
mod number_lerp;

pub use number_lerp::Lerp;

pub(crate) use self::{
    add::t_safe_add::SafeAdd, div::t_safe_div::SafeDiv, mul::t_safe_mul::SafeMul,
    sub::t_safe_sub::SafeSub, to_f64::t_to_f64::ToF64,
};
