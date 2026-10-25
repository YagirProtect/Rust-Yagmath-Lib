//! Axis-aligned bounds with inclusive min/max coordinates.
//!
//! Size is geometric: bounds from 0 to 9 have size 9, not 10 cells.
//! Center and size use f64; very wide integer coordinates may lose precision.
//!
//! ```
//! fn main() -> Result<(), Box<dyn core::error::Error>> {
//!     use yagmath::bounds::Bounds2;
//!     use yagmath::vector::Vector2;
//!
//!     let mut bounds = Bounds2::new(Vector2::new(0, 0), Vector2::new(10, 20)).ok_or("invalid bounds or point")?;
//!
//!     assert_eq!(bounds.center(), Some(Vector2::new(5.0, 10.0)));
//!     assert!(bounds.contains(Vector2::new(10, 20)));
//!
//!     bounds.encapsulate(Vector2::new(-5, 30)).ok_or("invalid bounds or point")?;
//!
//!     assert_eq!(bounds.min, Vector2::new(-5, 0));
//!     assert_eq!(bounds.max, Vector2::new(10, 30));
//!
//!     Ok(())
//! }
//! ```
//!
//! ```
//! fn main() -> Result<(), Box<dyn core::error::Error>> {
//!     use yagmath::bounds::Bounds2;
//!     use yagmath::vector::Vector2;
//!
//!     let bounds = Bounds2::from_start_end(Vector2::new(10, 0), Vector2::new(0, 20)).ok_or("invalid bounds or point")?;
//!
//!     assert_eq!(bounds.min, Vector2::new(0, 0));
//!     assert_eq!(bounds.max, Vector2::new(10, 20));
//!
//!     let bounds = Bounds2::from_center_size(Vector2::new(5, 10), Vector2::new(3, 6)).ok_or("invalid bounds or point")?;
//!
//!     assert_eq!(bounds.min, Vector2::new(3.5, 7.0));
//!     assert_eq!(bounds.max, Vector2::new(6.5, 13.0));
//!
//!     Ok(())
//! }
//! ```

mod bounds;

pub use bounds::{Bounds2, Bounds3};
