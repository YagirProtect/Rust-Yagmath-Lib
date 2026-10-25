# API map

[Home](../README.md) · [Index](README.md) · [Numeric policies](numerics.md)

This inventory lists the implemented API, not proposed features. `V<T>`/`BoundsN<T>` are notation, not exported aliases. `ToF64` and other scalar bounds determine method availability. For exact generic signatures and trait implementations, build `cargo doc --no-deps --open`.

<a id="imports"></a>
## Imports

```text
yagmath::vector::{Vector2, Vector3, Vector4, DirectionScalar}
yagmath::bounds::{Bounds2, Bounds3}
yagmath::{Plane, Math, SegmentIntersection2}
yagmath::{ConversionMode, RoundingMode, ConversionError, ConversionErrorKind}
yagmath::conversion::ConvertFrom
yagmath::extensions::to_f64::t_to_f64::ToF64
yagmath::extensions::{add::t_safe_add::SafeAdd, sub::t_safe_sub::SafeSub,
                        mul::t_safe_mul::SafeMul, div::t_safe_div::SafeDiv}
```

Plane also exists at `yagmath::plane::Plane`; Math and SegmentIntersection2 also exist in `yagmath::math`. Vectors/bounds are not re-exported at the crate root. No prelude or public vector-generation macro is exported.

<a id="vectors"></a>
## Vector2 / Vector3 / Vector4

| Method / expression | Return type | Availability | Cookbook |
|---|---|---|---|
| `new(fields...)`, `default()` | `V<T>` | All T / T: Default | [Creation](vectors.md#creation) |
| `+`, `-`, vector `*`, scalar `*` | `V<T>` | SafeAdd/Sub/Mul; scalar * also Copy | [Operators](vectors.md#operators) |
| vector `/`, scalar `/` | `Option<V<T>>` | SafeDiv; scalar / also Copy | [Operators](vectors.md#operators) |
| `+=`, `-=`, vector/scalar `*=` | `()` | Same scalar policies | [Operators](vectors.md#operators) |
| `zero/up/right` | `V<T>` | DirectionScalar; all dimensions | [Directions](vectors.md#directions) |
| `down/left` | `V<T>` | DirectionScalar + Neg | [Directions](vectors.md#directions) |
| `forward/backward` | `V<T>` | 3D/4D; backward also Neg | [Directions](vectors.md#directions) |
| `length`, `length_squared` | `f64` | ToF64 | [Lengths](vectors.md#length-distance) |
| `distance`, `distance_squared` | `f64` | ToF64; other is same V<T> | [Distances](vectors.md#length-distance) |
| `normalized` | `Option<V<f64>>` | ToF64 | [Normalization](vectors.md#normalization) |
| `normalize` | `Option<()>` | f32/f64; mutable receiver | [Normalization](vectors.md#normalization) |
| `dot` | `f64` | ToF64; other is same V<T> | [Products](vectors.md#products-angles) |
| 2D `cross` | `f64` | ToF64; same scalar type | [Products](vectors.md#products-angles) |
| 3D `cross` | `Vector3<f64>` | ToF64; same scalar type | [Products](vectors.md#products-angles) |
| `angle_between_rad` | `f64` | ToF64; mixed scalar types | [Angle](vectors.md#products-angles) |
| `angle_between_deg` | `f64` | Same query, in degrees | [Angle](vectors.md#products-angles) |
| 2D `signed_angle_rad(other)` | `f64` | ToF64; mixed scalar types; CCW positive | [Signed angle](vectors.md#products-angles) |
| 3D `signed_angle_rad(other, axis)` | `f64` | ToF64; mixed types; axis controls sign, not projection | [Signed angle](vectors.md#products-angles) |
| 2D `signed_angle_deg(other)` / 3D `signed_angle_deg(other, axis)` | `f64` | Same signed-angle rules, in degrees | [Signed angle](vectors.md#products-angles) |
| `lerp`, `lerp_unclamped` | `V<f64>` | ToF64; same scalar type | [Interpolation](vectors.md#interpolation) |
| `slerp`, `slerp_unclamped` | `V<f64>` | ToF64; mixed scalar types | [Interpolation](vectors.md#interpolation) |
| `move_towards` | `V<f64>` | ToF64; mixed scalar types | [Movement](vectors.md#movement) |
| `rotated_rad` | `Option<V<f64>>` | ToF64; dimension-specific args | [Rotation](vectors.md#rotation) |
| `rotate_rad` | `Option<()>` | f32/f64; mutable receiver | [Rotation](vectors.md#rotation) |
| `rotated_deg` / `rotate_deg` | `Option<V<f64>>` / `Option<()>` | Same rotation arguments, in degrees | [Rotation](vectors.md#rotation) |
| `reflected` | `Option<V<f64>>` | ToF64; mixed scalar normals | [Reflection](vectors.md#reflection) |
| `reflect` | `Option<()>` | f32/f64; mutable receiver | [Reflection](vectors.md#reflection) |
| `min`, `max` | `V<T>` | PartialOrd; consumes vectors | [Limits](vectors.md#limits) |
| `clamp` | `Option<V<T>>` | PartialOrd; consumes vectors | [Limits](vectors.md#limits) |
| `clamp_length_max` | `Option<V<f64>>` | ToF64 | [Limits](vectors.md#limits) |
| `floor`, `ceil`, `round` | `V<T>` | Concrete built-in numeric types | [Rounding](vectors.md#rounding) |
| `convert::<U>` | `Result<V<U>, ConversionError>` | U: ConvertFrom<T> | [Conversion](conversion.md#methods) |
| `convert_with_rounding::<U>` | `Result<V<U>, ConversionError>` | U: ConvertFrom<T> | [Conversion](conversion.md#rounding) |

<a id="bounds"></a>
## Bounds2 / Bounds3

All geometric methods need `T: PartialOrd + ToF64`; default only needs Default.

| Method | Result | Details |
|---|---|---|
| `new(min, max)` | `Option<BoundsN<T>>` | [Constructors](bounds.md#construction) |
| `from_start_end(start, end)` | `Option<BoundsN<T>>` | Sorts each axis |
| `from_center_size(center, size)` | `Option<BoundsN<f64>>` | Full size |
| `is_valid` | `bool` | [Queries](bounds.md#queries) |
| `center`, `size` | `Option<VectorN<f64>>` | Size may contain infinity |
| `contains(point)` | `bool` | Inclusive |
| `intersects(&other)` | `bool` | Touching counts |
| `closest_point(point)` | `Option<VectorN<T>>` | Also T: Copy |
| `encapsulate(point)` | `Option<()>` | Also T: Copy; [mutates](bounds.md#expansion) |

<a id="plane"></a>
## Plane

| Method | Result | Details |
|---|---|---|
| `from_normal_point(normal, point)` | `Option<Plane>` | [Construction](plane.md#construction) |
| `from_points(a,b,c)` | `Option<Plane>` | Right-hand rule |
| `normal()` | `Vector3<f64>` | [Queries](plane.md#queries) |
| `distance()` | `f64` | Equation coefficient |
| `signed_distance(point)` | `f64` | Positive on normal side |
| `closest_point(point)` | `Vector3<f64>` | Point projection |
| `project_vector(vector)` | `Vector3<f64>` | Ignores offset |
| `flipped()` | `Plane` | Reverses orientation |

<a id="math"></a>
## Math

| Associated function | Result | Reference |
|---|---|---|
| `inverse_lerp(start,end,value)` | `f64` | [Inverse lerp](math.md#inverse-lerp) |
| `inverse_lerp_unclamped(start,end,value)` | `f64` | Allows extrapolation |
| `checked_inverse_lerp(start,end,value)` | `Option<f64>` | Rejects invalid/equal bounds |
| `checked_inverse_lerp_unclamped(start,end,value)` | `Option<f64>` | Also rejects result overflow |
| `remap(value,in_min,in_max,out_min,out_max)` | `f64` | [Remap](math.md#remap) |
| `remap_clamped(value,in_min,in_max,out_min,out_max)` | `f64` | Clamps input fraction |
| `move_towards(current,target,max_delta)` | `f64` | [Movement](math.md#move-towards) |
| `smooth_step(from,to,t)` | `f64` | [Cubic interpolation](math.md#smooth-step) |
| `ping_pong(value,length)` | `f64` | [Triangle wave](math.md#ping-pong) |
| `smooth_damp(current,target,&mut velocity,smooth_time,max_speed,delta_time)` | `f64` | [Damping](math.md#smooth-damp) |
| `delta_angle_rad(current,target)` | `f64` | [Angular functions](math.md#angles) |
| `lerp_angle_rad(from,to,t)` | `f64` | Clamped t |
| `move_towards_angle_rad(current,target,max_delta)` | `f64` | Shortest arc |
| `smooth_damp_angle_rad(current,target,&mut velocity,smooth_time,max_speed,delta_time)` | `f64` | Shortest-arc damping |
| `delta_angle_deg(current,target)` | `f64` | Shortest signed degree delta, half-turn +180 |
| `lerp_angle_deg(from,to,t)` | `f64` | Degree interpolation, clamped t |
| `move_towards_angle_deg(current,target,max_delta)` | `f64` | Degree movement along shortest arc |
| `smooth_damp_angle_deg(current,target,&mut velocity,smooth_time,max_speed,delta_time)` | `f64` | Degree damping, speed/velocity in degrees per second |
| `segment_intersection(a_start,a_end,b_start,b_end)` | `Option<SegmentIntersection2>` | [2D closed segments](math.md#segment-intersection) |

`SegmentIntersection2` is either `Point(Vector2<f64>)` or `Overlap { start: Vector2<f64>, end: Vector2<f64> }`. The overlap follows the first segment's direction. None also represents non-finite input or loss of a nonzero coordinate during scaling. There is no 3D segment intersection method.

Angular APIs now require explicit unit suffixes. Earlier unqualified names
(`delta_angle`, `lerp_angle`, `move_towards_angle`, `smooth_damp_angle`,
`angle_between`, `signed_angle`, `rotated`, `rotate`) have been renamed to their
`_rad` versions, with `_deg` counterparts added. No ambiguous aliases remain.
Slerp keeps its name because its public interpolation parameter is unitless.

<a id="policies"></a>
## Numeric policies and traits

The optional [Serde feature](serde.md) adds Serialize/Deserialize to vectors,
bounds, Plane, SegmentIntersection2 and the conversion policies/errors.

* [ConversionMode](conversion.md#modes): Checked, Clamp, Wrap.
* [RoundingMode](conversion.md#rounding): Truncate, Round, Floor, Ceil.
* [ConversionError / ConversionErrorKind](conversion.md#errors): first component, OutOfRange / PrecisionLoss / NonFinite / UnsupportedMode.
* [Scalar traits](extensions.md#traits): SafeAdd, SafeSub, SafeMul, SafeDiv, ToF64, DirectionScalar and ConvertFrom.

No public `Math::lerp`, `approx_eq`, 4D signed vector angle, vector project_onto/project_on_plane, rotate_towards, Bounds union/intersection/expanded, matrices or quaternions are implemented at this stage. Plane projection exists on Plane; Bounds `intersects` is a boolean query, not an intersection-region constructor.
