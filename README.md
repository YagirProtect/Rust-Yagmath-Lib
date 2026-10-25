# yagmath

Generic Rust vector math and geometry for games: **2D/3D/4D vectors, axis-aligned bounds, 3D planes, interpolation, smoothing and explicit numeric conversions**. No external dependencies.

Use yagmath for player movement, steering angles, surface projection, rectangular
world boundaries, interpolated animation values and 2D edge intersection queries.
The API supports integer, f32 and f64 vector components, with documented overflow
and conversion policies. It uses the Rust standard library and has no Python
requirement at build time or runtime.

[Documentation index](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/README.md) · [API map](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/api.md) · [Numeric behavior](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/numerics.md) · [Tests and architecture](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/development.md)

![Vectors, bounds and planes](https://raw.githubusercontent.com/YagirProtect/Rust-Yagmath-Lib/master/docs/assets/geometry.svg)

## Start here

| You need… | Read |
|---|---|
| Coordinates, directions, dot/cross, angles, lerp/slerp, rotation, reflection | [Vectors](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/vectors.md) |
| Inverse lerp, remap, smoothing, angles and 2D segment intersections | [Math](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/math.md) |
| Rectangular map regions, AABBs, containment, overlap, closest point | [Bounds](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/bounds.md) |
| Surface normals, signed distances, plane projection | [Plane](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/plane.md) |
| Integer/float conversion with Checked, Clamp, Wrap and rounding | [Conversion](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/conversion.md) |
| Arithmetic policies for numbers and custom scalar types | [Extensions](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/extensions.md) |
| Exact method names, modules and return types | [API map](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/api.md) |

## Installation

The checked-in package is named `yagmath`, version `0.1.0`, edition 2024. To use a local checkout:

```toml
[dependencies]
yagmath = { path = "../math_utils" }
```

Adjust the path to your checkout. Use a Rust toolchain supporting edition 2024. Source and the linked handbook are hosted in the [GitHub repository](https://github.com/YagirProtect/Rust-Yagmath-Lib).

## Quick start

```rust
use yagmath::{Math, Plane};
use yagmath::bounds::Bounds2;
use yagmath::vector::{Vector2, Vector3};

let position = Vector2::new(0, 0);
let next = position.move_towards(Vector2::new(3, 4), 2.5);
assert_eq!(next, Vector2::new(1.5, 2.0));

let map = Bounds2::new(Vector2::new(0, 0), Vector2::new(100, 100)).unwrap();
assert_eq!(map.closest_point(Vector2::new(120, 50)), Some(Vector2::new(100, 50)));

let ground = Plane::from_normal_point(
    Vector3::new(0, 1, 0), Vector3::new(0, 10, 0),
).unwrap();
assert_eq!(ground.signed_distance(Vector3::new(0, 13, 0)), 3.0);
assert_eq!(Math::remap(50, 0, 100, 0.0, 1.0), 0.5);
```

## Structures and conventions

| Type | Storage | Main use |
|---|---|---|
| `Vector2<T>` | public `x, y` | 2D coordinates/directions |
| `Vector3<T>` | public `x, y, z` | 3D coordinates/directions |
| `Vector4<T>` | public `x, y, z, w` | Four-dimensional coordinates; not a quaternion |
| `Bounds2<T>` / `Bounds3<T>` | public `min, max` | Closed axis-aligned regions |
| `Plane` | private unit `Vector3<f64>` normal and `f64` offset | Oriented 3D plane |
| `Math` | no state | Associated scalar functions and 2D segment intersection |
| `SegmentIntersection2` | Point or overlap endpoints, f64 | Nonempty intersection result |

Numeric methods support `i8/i16/i32/i64/i128/isize`, `u8/u16/u32/u64/u128/usize`, `f32/f64` through the relevant traits. Vectors themselves can store nonnumeric values. Bounds keep their component type for containment and closest-point queries; many other geometric results use `f64`.

Angle-valued APIs end in **`_rad` for radians** or **`_deg` for degrees**. Both variants exist on Math and the angular vector methods. They accept f32/f64 components through the relevant traits and return f64 geometric values. Directions use **+X right, +Y up, +Z forward**; 4D direction constructors set W to zero. 3D cross products and axis rotations follow the right-hand rule. Interpolation parameters such as slerp t are unitless.

## Read this before replacing another math crate

* Integer vector `+`, `-`, `*` **saturate**, including `+=`, `-=`, `*=`. Float arithmetic can produce NaN/infinity.
* Vector/scalar `/` returns **`Option<Vector>`**, including for floats. There is no `/=`.
* `normalized`, `normalize`, rotation and reflection use `Option`; `angle_between_rad` and slerp use NaN for undefined directions. Conversion uses a component-specific `Result`. See the [behavior table](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/numerics.md#failure-and-overflow-policies).
* Geometric conversion to `f64` can round wide integers. Integer-to-integer `convert` preserves exact integer handling and never passes through floats.
* Bounds include both ends. Bounds `0..9` have geometric size `9`, not `10` cells. Touching bounds intersect.
* `Plane` constructors validate geometry. Its normal stays normalized because fields are private.
* This is an early `0.1.0` crate. It currently has no matrices, quaternions, transforms, grids, raycasting or SIMD-specific API. It is not a drop-in replacement for glam or Unity.

## Documentation and validation

```sh
cargo doc --no-deps --open
cargo test
cargo test --release
cargo test --doc
```

Rustdoc includes the public methods generated by macros. The [cookbook pages](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/README.md) contain executable examples; their Rust blocks are included in doctests. [Development notes](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/development.md) explain test coverage and source organization.

## For code search and coding assistants

Import vectors from `yagmath::vector`, bounds from `yagmath::bounds`, and `Math`, `Plane`, `SegmentIntersection2` from the crate root. There is no prelude. Use [docs/api.md](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/api.md) to discover existing methods and [docs/numerics.md](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/numerics.md) before assuming standard operator semantics. Public macro-generated APIs are defined in `src/vector/vector.rs` and feature files; tests are organized by feature in `src/tests`.

The compact [API map](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/api.md) lists real method signatures, result types and
trait requirements. It is the entry point for code search, automated tooling and
coding assistants. Angular methods explicitly end in `_rad` or `_deg`; integer
arithmetic saturates, division returns Option, and conversions return Result.

## Project metadata

The manifest includes a descriptive summary, README, five search keywords
(`math`, `gamedev`, `vector`, `geometry`, `interpolation`) and the `mathematics`
and `game-engines` categories. The distributable package retains Rust source,
README, handbook and diagrams; repository maintenance scripts are excluded using
`exclude = ["scripts/"]`.

Licensed under the [MIT License](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/LICENSE). The manifest declares `license = "MIT"`,
and the full license text is included in the repository and Cargo package.
The manifest links to the GitHub source repository and the handbook. README links and images use absolute URLs so they also work when rendered on crates.io.
