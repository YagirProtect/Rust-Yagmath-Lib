//! Generic 2D/3D/4D vectors, component operators and f64 geometry.
//!
//! See [`Vector2`], [`Vector3`] and [`Vector4`] for their generated methods.
//! Most methods only require the scalar trait they use. Construction itself
//! does not require a numeric type. Mutating normalization/rotation/reflection
//! are available only for `f32` and `f64`. Angular methods have explicit `_rad`
//! and `_deg` variants; interpolation parameters such as slerp t are unitless.
//!
//! ```
//! use yagmath::vector::Vector2;
//! let a = Vector2::new(3, 4);
//! assert_eq!(a.length(), 5.0);
//! assert_eq!(a.normalized(), Some(Vector2::new(0.6, 0.8)));
//! assert_eq!(Vector2::<i32>::right().angle_between_rad(Vector2::<i32>::up()).to_degrees(), 90.0);
//! ```

mod basic_operations;
mod cross;
mod convert;
mod directions;
mod rotate;
mod display;
mod length;
mod normalize;
mod normalized;
mod vector;
mod distance;
mod dot;
mod lerp;
mod min_max_clamp;
mod clamp_length;
mod rounds;
mod reflect;
mod move_towards;
mod angle;
mod signed_angle;
mod slerp;

pub use vector::{Vector2, Vector3, Vector4};
pub use directions::DirectionScalar;
