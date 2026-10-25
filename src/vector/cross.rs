use super::{Vector2, Vector3};
use crate::extensions::to_f64::t_to_f64::ToF64;

impl<T: ToF64> Vector2<T> {
    /// Returns the signed 2D determinant: x1 * y2 - y1 * x2.
    /// Positive means rhs is counterclockwise from self in an x-right, y-up plane.
    pub fn cross(&self, rhs: Self) -> f64 {
        self.x.to_f64() * rhs.y.to_f64() - self.y.to_f64() * rhs.x.to_f64()
    }
}

impl<T: ToF64> Vector3<T> {
    /// Returns the right-handed cross product, perpendicular to both vectors.
    pub fn cross(&self, rhs: Self) -> Vector3<f64> {
        let ax = self.x.to_f64();
        let ay = self.y.to_f64();
        let az = self.z.to_f64();
        let bx = rhs.x.to_f64();
        let by = rhs.y.to_f64();
        let bz = rhs.z.to_f64();

        Vector3::new(ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
    }
}
