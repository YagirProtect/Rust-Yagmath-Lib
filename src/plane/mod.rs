//! Oriented 3D planes with equation normal.dot(point) + distance = 0.
//! The stored normal is unit length. Coordinates and results use f64.
//!
//! ```
//! use yagmath::{Plane, vector::Vector3};
//!
//! let ground = Plane::from_normal_point(
//!     Vector3::new(0, 1, 0), Vector3::new(0, 10, 0),
//! ).unwrap();
//! assert_eq!(ground.signed_distance(Vector3::new(0, 13, 0)), 3.0);
//! assert_eq!(ground.closest_point(Vector3::new(2, 13, 4)), Vector3::new(2.0, 10.0, 4.0));
//! ```

mod plane;

pub use plane::Plane;
