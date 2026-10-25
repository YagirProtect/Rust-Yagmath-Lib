use crate::extensions::to_f64::t_to_f64::ToF64;
use crate::vector::Vector3;

/// An oriented plane: normal.dot(point) + distance = 0.
/// Constructors normalize the normal; private fields preserve that invariant.
/// Equality compares stored coefficients exactly, including orientation.
/// Generic numeric inputs convert to f64; wide integers may lose precision.
/// With the `serde` feature, deserialization rejects non-finite coefficients and
/// normals whose length differs from one by more than `16 * f64::EPSILON`.
/// Accepted coefficients are preserved exactly, without renormalization.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Plane {
    normal: Vector3<f64>,
    distance: f64,
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Plane {
    /// Restores stored coefficients after validating the private plane invariants.
    /// A small tolerance accommodates floating-point normalization error.
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Wire representation with the same name and field order as Plane.
        #[derive(serde::Deserialize)]
        #[serde(rename = "Plane")]
        struct Coefficients {
            /// Expected finite unit normal.
            normal: Vector3<f64>,
            /// Finite constant in the plane equation.
            distance: f64,
        }

        let coefficients = Coefficients::deserialize(deserializer)?;
        if !finite(coefficients.normal)
            || !coefficients.distance.is_finite()
            || (coefficients.normal.length() - 1.0).abs() > 16.0 * f64::EPSILON
        {
            return Err(serde::de::Error::custom(
                "expected a finite unit normal and a finite plane distance",
            ));
        }
        Ok(Self {
            normal: coefficients.normal,
            distance: coefficients.distance,
        })
    }
}

impl Plane {
    /// Constructs a plane through point with the given normal direction.
    /// Returns None for zero/non-finite normals, non-finite points or an offset
    /// outside f64's finite range. The normal need not already be normalized.
    pub fn from_normal_point<N: ToF64, P: ToF64>(
        normal: Vector3<N>,
        point: Vector3<P>,
    ) -> Option<Self> {
        let normal = unit(convert(normal))?;
        let point = convert(point);
        if !finite(point) {
            return None;
        }
        let distance = -scaled_dot(normal, point, 0.0);
        distance.is_finite().then_some(Self { normal, distance })
    }

    /// Constructs a plane through three points. The normal follows
    /// (b - a).cross(c - a), using the right-hand rule.
    /// Returns None for non-finite, coincident or collinear points, an overflowing
    /// offset, or degeneracy caused by floating-point rounding/underflow.
    pub fn from_points<T: ToF64>(a: Vector3<T>, b: Vector3<T>, c: Vector3<T>) -> Option<Self> {
        let a = convert(a);
        let b = convert(b);
        let c = convert(c);
        if !finite(a) || !finite(b) || !finite(c) {
            return None;
        }
        let ab = edge(a, b)?;
        let ac = edge(a, c)?;
        Self::from_normal_point(ab.cross(ac), a)
    }

    /// Returns the unit normal, pointing toward positive signed distances.
    pub fn normal(&self) -> Vector3<f64> {
        self.normal
    }

    /// Returns the constant term in normal.dot(point) + distance = 0.
    /// This equals signed_distance of the origin.
    pub fn distance(&self) -> f64 {
        self.distance
    }

    /// Signed perpendicular distance: positive on the normal's side, negative
    /// on the opposite side. NaN/infinity can propagate; result overflow is infinite.
    pub fn signed_distance<T: ToF64>(&self, point: Vector3<T>) -> f64 {
        scaled_dot(self.normal, convert(point), self.distance)
    }

    /// Projects a point onto the plane. Non-finite input or result overflow
    /// returns a vector of NaNs. The original point is unchanged.
    pub fn closest_point<T: ToF64>(&self, point: Vector3<T>) -> Vector3<f64> {
        project(self.normal, self.distance, convert(point))
    }

    /// Removes the vector component along the normal, preserving components
    /// parallel to the plane. Unlike closest_point, this ignores the plane offset.
    /// Non-finite input or result overflow returns a vector of NaNs.
    pub fn project_vector<T: ToF64>(&self, vector: Vector3<T>) -> Vector3<f64> {
        project(self.normal, 0.0, convert(vector))
    }

    /// Returns the same geometric plane with its orientation reversed.
    pub fn flipped(&self) -> Self {
        Self {
            normal: Vector3::new(-self.normal.x, -self.normal.y, -self.normal.z),
            distance: -self.distance,
        }
    }
}

/// Converts the three coordinates through ToF64 without validating finiteness.
/// Callers choose whether invalid inputs should return None, NaN or an invalid vector.
fn convert<T: ToF64>(v: Vector3<T>) -> Vector3<f64> {
    Vector3::new(v.x.to_f64(), v.y.to_f64(), v.z.to_f64())
}

/// Checks every coordinate explicitly; NaN and both infinities are rejected.
fn finite(v: Vector3<f64>) -> bool {
    v.x.is_finite() && v.y.is_finite() && v.z.is_finite()
}

/// Returns the largest absolute coordinate for scaling.
/// Callers validate finiteness separately; float max alone can hide a NaN coordinate.
fn max_component(v: Vector3<f64>) -> f64 {
    v.x.abs().max(v.y.abs()).max(v.z.abs())
}

/// Produces a unit direction or None for a zero/non-finite vector.
/// Divide by the largest absolute coordinate before taking length to avoid
/// overflow and the poor relative precision of subnormal magnitudes.
fn unit(v: Vector3<f64>) -> Option<Vector3<f64>> {
    let scale = max_component(v);
    if !finite(v) || scale == 0.0 {
        return None;
    }
    // Scaling first keeps huge and subnormal normals usable.
    Vector3::new(v.x / scale, v.y / scale, v.z / scale).normalized()
}

/// Returns the unit direction from a to b for plane construction.
/// Both points must already be finite. Prefer direct subtraction for precision;
/// if it overflows, scale endpoints first. Coincident/rounded-away edges fail.
fn edge(a: Vector3<f64>, b: Vector3<f64>) -> Option<Vector3<f64>> {
    let delta = Vector3::new(b.x - a.x, b.y - a.y, b.z - a.z);
    if finite(delta) {
        return unit(delta);
    }
    // Subtract scaled endpoints when their difference overflows.
    let scale = max_component(a).max(max_component(b));
    unit(Vector3::new(
        b.x / scale - a.x / scale,
        b.y / scale - a.y / scale,
        b.z / scale - a.z / scale,
    ))
}

/// Evaluates normal.dot(point) + offset, preferring direct arithmetic.
/// The normal and offset are finite for a constructed Plane. Fall back to a
/// common scale only after a non-finite result from finite point coordinates;
/// this can recover finite answers hidden by intermediate dot-product overflow.
fn scaled_dot(normal: Vector3<f64>, point: Vector3<f64>, offset: f64) -> f64 {
    let result = normal.dot(point) + offset;
    if result.is_finite() || !finite(point) {
        return result;
    }
    let scale = max_component(point).max(offset.abs());
    if scale == 0.0 {
        return 0.0;
    }
    let scaled = Vector3::new(point.x / scale, point.y / scale, point.z / scale);
    (normal.dot(scaled) + offset / scale) * scale
}

/// Projects a point using a unit normal and finite equation offset.
/// An offset of zero projects a displacement vector instead. Try direct arithmetic
/// first, then project scaled coordinates if necessary. Invalid inputs or an
/// unrepresentable result return all NaNs, never a partially invalid vector.
fn project(normal: Vector3<f64>, offset: f64, point: Vector3<f64>) -> Vector3<f64> {
    let invalid = Vector3::new(f64::NAN, f64::NAN, f64::NAN);
    if !finite(point) {
        return invalid;
    }
    let distance = normal.dot(point) + offset;
    let result = Vector3::new(
        point.x - normal.x * distance,
        point.y - normal.y * distance,
        point.z - normal.z * distance,
    );
    if finite(result) {
        return result;
    }
    let scale = max_component(point).max(offset.abs());
    if scale == 0.0 {
        return point;
    }
    // Project before restoring scale so an overflowing signed distance does not
    // invalidate a projection whose individual components are representable.
    let point = Vector3::new(point.x / scale, point.y / scale, point.z / scale);
    let distance = normal.dot(point) + offset / scale;
    let result = Vector3::new(
        (point.x - normal.x * distance) * scale,
        (point.y - normal.y * distance) * scale,
        (point.z - normal.z * distance) * scale,
    );
    if finite(result) { result } else { invalid }
}
