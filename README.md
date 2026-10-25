# yagmath

[![Rust CI](https://github.com/YagirProtect/Rust-Yagmath-Lib/actions/workflows/rust.yml/badge.svg)](https://github.com/YagirProtect/Rust-Yagmath-Lib/actions/workflows/rust.yml)

Generic Rust vector math and geometry for games: **2D/3D/4D vectors, axis-aligned bounds, 3D planes, interpolation, smoothing and explicit numeric conversions**. No external dependencies by default; optional Serde and no_std support with libm.

[Documentation index](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/README.md) · [API map](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/api.md) · [Numeric behavior](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/numerics.md) · [Tests and architecture](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/development.md)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
yagmath = "0.1.3"
```

Then use it in your Rust code:

```rust
use yagmath::{Math, vector::Vector2};

fn main() {
    let position = Vector2::new(0, 0);
    let target = Vector2::new(3, 4);
    let next = position.move_towards(target, 2.5);

    println!("Next position: {next}"); // (1.5, 2)

    let turn = Math::delta_angle_deg(350.0, 10.0);

    println!("Turn: {turn} degrees"); // 20 degrees
}
```

Use a Rust toolchain supporting edition 2024. See the linked guides below for
bounds, planes, conversions and error handling.

## Documentation

### Cargo features

| Use case | Default features | Features to request |
|---|---|---|
| Desktop/server math | Enabled | None |
| Desktop/server math with serialization | Enabled | `["serde"]` |
| Embedded math without std | Disabled | `["no_std"]` |
| Embedded math with serialization | Disabled | `["no_std", "serde"]` |

`no_std` selects the libm math backend. `libm` remains an equivalent feature
name. Neither feature disables std: set `default-features = false` for an
embedded build. If another dependency enables std, std takes precedence.

For embedded Rust, disable the default `std` feature and enable `no_std`:

```toml
[dependencies]
yagmath = { version = "0.1.3", default-features = false, features = ["no_std"] }
```

no_std support was introduced in version 0.1.3.
It also supports `features = ["no_std", "serde"]` without a heap allocator.
See the [no_std guide](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/no_std.md)
for feature combinations, target checks and microcontroller considerations.

For saving/loading vectors, bounds and other data, enable optional Serde support:

```toml
[dependencies]
yagmath = { version = "0.1.3", features = ["serde"] }
```

See the [serialization guide](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/serde.md)
for JSON examples, supported types and validation behavior. Serde support was
introduced in version 0.1.2.

![Vectors, bounds and planes](https://raw.githubusercontent.com/YagirProtect/Rust-Yagmath-Lib/master/docs/assets/geometry.svg)

| You need… | Read |
|---|---|
| Coordinates, directions, dot/cross, angles, lerp/slerp, rotation, reflection | [Vectors](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/vectors.md) |
| Inverse lerp, remap, smoothing, angles and 2D segment intersections | [Math](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/math.md) |
| Rectangular map regions, AABBs, containment, overlap, closest point | [Bounds](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/bounds.md) |
| Surface normals, signed distances, plane projection | [Plane](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/plane.md) |
| Integer/float conversion with Checked, Clamp, Wrap and rounding | [Conversion](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/conversion.md) |
| Arithmetic policies for numbers and custom scalar types | [Extensions](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/extensions.md) |
| Optional serialization, JSON formats and loading validation | [Serde](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/serde.md) |
| Embedded targets, libm and builds without an allocator | [no_std](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/no_std.md) |
| Exact method names, modules and return types | [API map](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/api.md) |

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
| `VectorComponent` | One-byte enum: `X`, `Y`, `Z`, `W` | Identifies a coordinate without storing a string |
| `ConversionError` | `component: VectorComponent`, `kind: ConversionErrorKind` | Explains which coordinate failed and why |

Numeric methods support `i8/i16/i32/i64/i128/isize`, `u8/u16/u32/u64/u128/usize`, `f32/f64` through the relevant traits. Vectors themselves can store nonnumeric values. Bounds keep their component type for containment and closest-point queries; many other geometric results use `f64`.

Angle-valued APIs end in **`_rad` for radians** or **`_deg` for degrees**. Both variants exist on Math and the angular vector methods. They accept f32/f64 components through the relevant traits and return f64 geometric values. Directions use **+X right, +Y up, +Z forward**; 4D direction constructors set W to zero. 3D cross products and axis rotations follow the right-hand rule. Interpolation parameters such as slerp t are unitless.

## Conversion errors

Version 0.1.3 returns `VectorComponent` instead of a string in
`ConversionError.component`. Update comparisons such as `error.component == "x"`
to `error.component == VectorComponent::X` when upgrading from 0.1.2.

```rust
use yagmath::{ConversionMode, VectorComponent, vector::Vector2};

let position = Vector2::new(300_i32, 20);
let result = position.convert::<i8>(ConversionMode::Checked);

match result {
    Ok(converted) => println!("Converted position: {converted}"),
    Err(error) if error.component == VectorComponent::X => {
        println!("Cannot convert X: {}", error.kind);
    }
    Err(error) => println!("Cannot convert position: {error}"),
}
```

The enum occupies one byte with `#[repr(u8)]`; `ConversionError` occupies two
bytes in the tested layout. Display still prints lowercase names. Serde keeps
the existing `{"component":"x","kind":"OutOfRange"}` representation, so serialized
errors remain compatible. See the [conversion guide](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/conversion.md#errors)
for all error kinds and scalar conversion policies.

## Read this before replacing another math crate

* Integer vector `+`, `-`, `*` **saturate**, including `+=`, `-=`, `*=`. Float arithmetic can produce NaN/infinity.
* Vector/scalar `/` returns **`Option<Vector>`**, including for floats. There is no `/=`.
* `normalized`, `normalize`, rotation and reflection use `Option`; `angle_between_rad` and slerp use NaN for undefined directions. Conversion uses a component-specific `Result`. See the [behavior table](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/numerics.md#failure-and-overflow-policies).
* Geometric conversion to `f64` can round wide integers. Integer-to-integer `convert` preserves exact integer handling and never passes through floats.
* Bounds include both ends. Bounds `0..9` have geometric size `9`, not `10` cells. Touching bounds intersect.
* `Plane` constructors validate geometry. Its normal stays normalized because fields are private.
* This is an early `0.1.x` crate. It currently has no matrices, quaternions, transforms, grids, raycasting or SIMD-specific API. It is not a drop-in replacement for glam or Unity.

## Documentation and validation

```sh
cargo doc --no-deps --open
cargo test
cargo test --release
cargo test --features serde
cargo test --no-default-features --features no_std,serde
cargo test --doc
```

Rustdoc includes the public methods generated by macros. The [cookbook pages](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/README.md) contain executable examples; their Rust blocks are included in doctests. [Development notes](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/development.md) explain test coverage and source organization.

## For code search and coding assistants

Import vectors from `yagmath::vector`, bounds from `yagmath::bounds`, and `Math`, `Plane`, `SegmentIntersection2` from the crate root. `VectorComponent` is available from both the crate root and `yagmath::vector`. There is no prelude. Use [docs/api.md](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/api.md) to discover existing methods and [docs/numerics.md](https://github.com/YagirProtect/Rust-Yagmath-Lib/blob/master/docs/numerics.md) before assuming standard operator semantics. Public macro-generated APIs are defined in `src/vector/vector.rs` and feature files; tests are organized by feature in `src/tests`.

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
