# Numeric behavior and failure policies

[Home](../README.md) · [Index](README.md) · [API map](api.md)

[Conventions](#conventions) · [Failure policies](#failure-and-overflow-policies) · [Precision](#precision) · [Choosing a method](#choosing-a-method)

<a id="conventions"></a>
## Coordinate and type conventions

* Angles: `_rad` names use radians; `_deg` names use degrees. Results are f64. Unit conversion is also built into float types: `.to_radians()`, `.to_degrees()`.
* Time: seconds supplied by the caller; no engine clock dependency.
* Directions: +X right, +Y up, +Z forward. Vector4 axis constructors use W=0.
* Cross/3D rotation: right-hand rule. 2D positive rotation: +X toward +Y.
* Bounds: inclusive min and max; touching intersects; geometric size has no cell-count adjustment.
* Plane: `n·p+d=0`, unit n, positive distance on the normal's side.
* Stored scalar types: generic vectors/bounds; Plane uses f64.
* Geometric output: usually f64. Type-preserving component methods are documented separately.

These choices are API conventions, not a promise that any renderer or engine uses the same coordinate system.

<a id="failure-and-overflow-policies"></a>
## Failure and overflow policies

| API family | Result policy |
|---|---|
| Integer vector `+ - *` and assignments | Saturate at scalar MIN/MAX in both debug/release |
| Float vector `+ - *` and assignments | Ordinary IEEE results; NaN/infinity possible |
| Vector `/` | Option; any failed component makes the entire result None |
| length/distance/dot/cross/squared variants | Numeric f64 results; overflow/underflow and NaN possible |
| normalized/normalize | Option; zero/non-finite length fails, including finite components with overflowing magnitude |
| rotated_rad/rotate_rad/reflected/reflect | Option; reject invalid geometry/non-finite inputs and results |
| clamp | Option; invalid bounds/NaN components fail |
| clamp_length_max | Option; invalid components or negative/non-finite maximum fails |
| angle_between_rad / angle_between_deg | NaN for zero or non-finite directions |
| 2D/3D signed_angle_rad / signed_angle_deg | NaN for invalid directions/3D axis; exact half-turn chooses +π/+180; zero orientation sign chooses positive |
| vector move_towards | All-NaN vector for invalid components/NaN step; nonpositive step does nothing |
| slerp / slerp_unclamped | All-NaN vector for undefined directions/invalid t/result overflow; clamped ±infinite t becomes 0/1 |
| vector lerp | Ordinary component arithmetic; may overflow even at endpoint t |
| vector min/max | Prefer the other component if exactly one float value is NaN |
| floor/ceil/round | Preserve scalar type; float NaN/infinity preserved; integers unchanged |
| inverse_lerp ordinary variants | Equal bounds → 0; otherwise numeric IEEE result/clamping |
| checked inverse lerp | Option rejects degenerate/non-finite inputs; unchecked-result overflow rejected in unclamped variant |
| Math move_towards | Invalid endpoints/NaN step → NaN; nonpositive step does nothing |
| Math smooth_step/remap | Numeric outputs; see their endpoint and clamping rules |
| Math ping_pong | Negative/non-finite length or non-finite value → NaN; finite value and zero length → 0 |
| Math smooth_damp | Invalid inputs or non-finite output/velocity → NaN; velocity preserved on failure |
| Math segment_intersection | Option of Point/Overlap; None for disjoint/non-finite input or loss of a nonzero coordinate during scaling; exact float comparisons, no epsilon |
| Bounds construction/center/size/closest_point/encapsulate | Option; valid bounds size may still contain infinity |
| Bounds contains/intersects | False for invalid bounds; boundaries included |
| Plane constructors | Option rejects undefined/non-finite geometry or unrepresentable offset |
| Plane signed_distance | Numeric result; non-finite inputs/overflow can propagate |
| Plane closest_point/project_vector | All-NaN vector for invalid inputs or result overflow |
| Numeric conversion | Result with component name and error kind |

The API currently has multiple failure styles. Do not replace one with another merely because the formulas look similar. A named checked inverse lerp is available, but there is not a checked/unchecked pair for every operation.

In-place normalize/rotate_rad/reflect and bounds encapsulate validate before committing a change. Their failures leave the receiver unchanged. Ordinary scalar safe_add/sub/mul return values and do not modify the original scalar.

<a id="precision"></a>
## Precision limits

Geometry converts components to f64 before computing. f64 represents all integers through ±2^53 exactly, but not every integer beyond that. f32 has 24 significant binary digits; f64 has 53. Two distinct wide integer coordinates may convert to the same float. Converting back cannot recover lost information.

Integer-to-integer `convert` handles integers directly. Checked conversion into floats verifies exact representation; Clamp allows rounding. Ordinary geometry's ToF64 does not perform that checked conversion.

Hypot avoids premature square overflow in length/distance, but cannot represent a true magnitude beyond f64::MAX. Squared variants overflow sooner and may underflow to zero. Some newer methods scale components internally (angle, slerp, reflection, plane construction) and accept inputs whose plain normalized() fails. Do not assume that all methods share one normalization implementation or identical extreme-value limits.

All results remain floating point. Prefer a tolerance for calculated values rather than exact equality. `PartialEq` itself is exact; NaN does not compare equal to itself. There is no public approximate-equality method yet. Degenerate geometry such as almost-collinear plane points can be sensitive to rounding; the crate does not impose an application-specific epsilon.

Some methods have separate concrete implementations for several scalar types, such as round or in-place rotation. Unsuffixed literals can make method lookup ambiguous before Rust chooses a default numeric type. Write `Vector2::new(1.5_f64, -1.5)` or annotate `Vector2<f64>` when the compiler reports multiple applicable methods.

<a id="choosing-a-method"></a>
## Choosing the right operation

| Task | Use |
|---|---|
| Compare a distance to a radius | distance_squared and radius² when ranges are moderate |
| Obtain a direction | normalized and handle None |
| Determine front/behind | dot with the forward direction; sign matters |
| Obtain an actual angle | angle_between_rad; check NaN for zero directions |
| Choose a turn direction | signed_angle_rad; 3D axis only selects sign of the full angle |
| Interpolate positions | lerp |
| Interpolate directions along an arc | slerp |
| Travel at speed per second | move_towards(target, speed*delta_time) |
| Smoothly follow a scalar target | Math::smooth_damp; retain velocity |
| Bound coordinates | vector clamp or Bounds closest_point |
| Bound speed magnitude | clamp_length_max |
| Convert numeric component types | convert with explicit policy |
| Map a value between ranges | Math::remap; not numeric conversion |
| Project a point to a surface | Plane::closest_point |
| Remove motion through a surface | Plane::project_vector |
| Intersect two closed 2D edges | Math::segment_intersection; handle Point and Overlap |

Built-in numeric implementations avoid integer-overflow/division panics through these policies. A custom trait implementation can still panic or choose a different policy. This crate has not been benchmarked here against glam, and no performance/bitwise portability guarantees are asserted.
