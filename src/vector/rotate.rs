use super::{Vector2, Vector3, Vector4};
use crate::extensions::to_f64::t_to_f64::ToF64;

impl<T: ToF64> Vector2<T> {
    /// Returns a counterclockwise rotated copy using degrees.
    /// Complete turns are reduced before conversion to radians.
    /// Invalid input/result rules match rotated_rad.
    pub fn rotated_deg(&self, angle: f64) -> Option<Vector2<f64>> {
        self.rotated_rad(degrees_to_radians(angle))
    }

    /// Rotates counterclockwise in an x-right, y-up plane, in radians.
    /// Returns None for non-finite inputs or results.
    pub fn rotated_rad(&self, angle: f64) -> Option<Vector2<f64>> {
        let x = self.x.to_f64();
        let y = self.y.to_f64();
        if !angle.is_finite() || !x.is_finite() || !y.is_finite() {
            return None;
        }
        let (s, c) = angle.sin_cos();
        let result = Vector2::new(x * c - y * s, x * s + y * c);
        (result.x.is_finite() && result.y.is_finite()).then_some(result)
    }
}

impl<T: ToF64> Vector3<T> {
    /// Returns a copy rotated around axis using degrees and the right-hand rule.
    /// Axis is normalized internally. Invalid input/result rules match rotated_rad.
    pub fn rotated_deg<A: ToF64>(&self, angle: f64, axis: Vector3<A>) -> Option<Vector3<f64>> {
        self.rotated_rad(degrees_to_radians(angle), axis)
    }

    /// Rotates around an axis through the origin using the right-hand rule.
    /// The angle is in radians; the axis need not be normalized.
    /// Returns None for a zero axis, non-finite inputs, or non-finite results.
    pub fn rotated_rad<A: ToF64>(&self, angle: f64, axis: Vector3<A>) -> Option<Vector3<f64>> {
        let v = Vector3::new(self.x.to_f64(), self.y.to_f64(), self.z.to_f64());
        let axis = Vector3::new(axis.x.to_f64(), axis.y.to_f64(), axis.z.to_f64());
        if !angle.is_finite()
            || ![v.x, v.y, v.z, axis.x, axis.y, axis.z]
                .iter()
                .all(|v| v.is_finite())
        {
            return None;
        }

        // Scale first so very large or subnormal axes can be normalized.
        let axis_scale = axis.x.abs().max(axis.y.abs()).max(axis.z.abs());
        if axis_scale == 0.0 {
            return None;
        }
        let k = Vector3::new(
            axis.x / axis_scale,
            axis.y / axis_scale,
            axis.z / axis_scale,
        )
        .normalized()?;

        let scale = v.x.abs().max(v.y.abs()).max(v.z.abs());
        if scale == 0.0 {
            return Some(v);
        }
        let v = Vector3::new(v.x / scale, v.y / scale, v.z / scale);
        let (s, c) = angle.sin_cos();
        let cross = k.cross(v);
        let projection = k.dot(v) * (1.0 - c);
        let result = Vector3::new(
            (v.x * c + cross.x * s + k.x * projection) * scale,
            (v.y * c + cross.y * s + k.y * projection) * scale,
            (v.z * c + cross.z * s + k.z * projection) * scale,
        );
        [result.x, result.y, result.z]
            .iter()
            .all(|v| v.is_finite())
            .then_some(result)
    }
}

impl<T: ToF64> Vector4<T> {
    /// Returns a copy rotated in the ordered coordinate plane using degrees.
    /// The plane and invalid input/result rules match rotated_rad.
    pub fn rotated_deg(&self, angle: f64, plane: (usize, usize)) -> Option<Vector4<f64>> {
        self.rotated_rad(degrees_to_radians(angle), plane)
    }

    /// Rotates in an ordered coordinate plane, in radians.
    /// Indices 0, 1, 2, 3 mean x, y, z, w. For example (0, 1) selects XY.
    /// Positive rotation takes the first basis axis toward the second.
    /// Returns None for equal/out-of-range indices or non-finite inputs/results.
    pub fn rotated_rad(&self, angle: f64, plane: (usize, usize)) -> Option<Vector4<f64>> {
        let (i, j) = plane;
        let mut values = [
            self.x.to_f64(),
            self.y.to_f64(),
            self.z.to_f64(),
            self.w.to_f64(),
        ];
        if i >= 4 || j >= 4 || i == j || !angle.is_finite() || !values.iter().all(|v| v.is_finite())
        {
            return None;
        }
        let (s, c) = angle.sin_cos();
        let a = values[i];
        let b = values[j];
        values[i] = a * c - b * s;
        values[j] = a * s + b * c;
        if !values.iter().all(|v| v.is_finite()) {
            return None;
        }
        Some(Vector4::new(values[0], values[1], values[2], values[3]))
    }
}

// Validate after conversion before changing any component of the original.
/// Commits an already calculated rotation after validating destination casts.
/// $target and $result are local variable identifiers; $type is the float scalar,
/// and $field lists the coordinates. The extra block keeps temporary names local.
/// Returns None from the enclosing method before any assignment if a cast overflows.
macro_rules! assign_rotated {
    ($target:ident, $result:ident, $type:ty, $($field:ident),+) => {{
        $(let $field = $result.$field as $type;)+
        if !($($field.is_finite())&&+) {
            return None;
        }
        $($target.$field = $field;)+
        Some(())
    }};
}

/// Adds in-place rotation methods for each listed concrete float $type.
/// Dimension-specific methods delegate to rotated_rad and use assign_rotated to
/// validate the entire converted result before changing the receiver.
macro_rules! impl_rotate {
    ($($type:ty),+) => {$(
        impl Vector2<$type> {
            /// Rotates in place using degrees; failure leaves self unchanged.
            pub fn rotate_deg(&mut self, angle: f64) -> Option<()> {
                self.rotate_rad(degrees_to_radians(angle))
            }

            /// Rotates in place in radians, leaving self unchanged on failure.
            pub fn rotate_rad(&mut self, angle: f64) -> Option<()> {
                let result = self.rotated_rad(angle)?;
                assign_rotated!(self, result, $type, x, y)
            }
        }

        impl Vector3<$type> {
            /// Rotates in place around axis using degrees; failure preserves self.
            pub fn rotate_deg<A: ToF64>(&mut self, angle: f64, axis: Vector3<A>) -> Option<()> {
                self.rotate_rad(degrees_to_radians(angle), axis)
            }

            /// Rotates in place around an axis in radians. Failure leaves self unchanged.
            pub fn rotate_rad<A: ToF64>(&mut self, angle: f64, axis: Vector3<A>) -> Option<()> {
                let result = self.rotated_rad(angle, axis)?;
                assign_rotated!(self, result, $type, x, y, z)
            }
        }

        impl Vector4<$type> {
            /// Rotates in place in an ordered coordinate plane using degrees.
            /// Invalid plane/input or cast overflow leaves self unchanged.
            pub fn rotate_deg(&mut self, angle: f64, plane: (usize, usize)) -> Option<()> {
                self.rotate_rad(degrees_to_radians(angle), plane)
            }

            /// Rotates in place in an ordered coordinate plane, in radians.
            /// Failure leaves self unchanged, including on component overflow.
            pub fn rotate_rad(&mut self, angle: f64, plane: (usize, usize)) -> Option<()> {
                let result = self.rotated_rad(angle, plane)?;
                assign_rotated!(self, result, $type, x, y, z, w)
            }
        }
    )+};
}

impl_rotate!(f32, f64);

/// Reduces finite degree angles before conversion, preserving exact full turns
/// and avoiding unnecessary argument-reduction error for large degree inputs.
/// Non-finite input becomes NaN and is rejected by the radian implementation.
fn degrees_to_radians(angle: f64) -> f64 {
    (angle % 360.0).to_radians()
}
