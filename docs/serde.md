# Optional Serde serialization

[Home](../README.md) · [Index](README.md) · [Numeric behavior](numerics.md)

Enable the `serde` feature in the application that uses yagmath:

```toml
[dependencies]
yagmath = { version = "0.1.3", features = ["serde"] }
serde_json = "1"
```

Serde support was introduced in version 0.1.2. To use a local checkout, replace
`version` with `path = "../math_utils"`, adjusting the path to your checkout.

Serde support is disabled by default. Without this feature, yagmath has no
external runtime dependencies in the default std build. `serde_json` is one possible format and is not
required by yagmath; you can use other Serde serializers instead.

For embedded targets, use `default-features = false, features = ["no_std", "serde"]`.
The library's Serde implementations require neither std nor an allocator. Your
chosen format must also support those constraints; see the [no_std guide](no_std.md).

## Save and load a vector

```rust
use yagmath::vector::Vector3;

fn main() -> Result<(), serde_json::Error> {
    let position = Vector3::new(10.0_f32, 20.0, 30.0);
    let json = serde_json::to_string(&position)?;

    println!("{json}"); // {"x":10.0,"y":20.0,"z":30.0}

    let restored: Vector3<f32> = serde_json::from_str(&json)?;

    println!("Loaded position: {restored}");
    Ok(())
}
```

## Include vectors in your own data

Add `serde = { version = "1", features = ["derive"] }` to your application's
dependencies to derive Serde traits for your own structures:

```rust
use serde::{Deserialize, Serialize};
use yagmath::{bounds::Bounds2, vector::Vector2};

#[derive(Serialize, Deserialize)]
struct MapRegion {
    name: String,
    bounds: Bounds2<i32>,
    spawn: Vector2<f32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let region = MapRegion {
        name: "Start area".into(),
        bounds: Bounds2::new(Vector2::new(0, 0), Vector2::new(100, 100))
            .ok_or("invalid map bounds")?,
        spawn: Vector2::new(10.0, 20.0),
    };
    let json = serde_json::to_string_pretty(&region)?;
    let restored: MapRegion = serde_json::from_str(&json)?;
    if !restored.bounds.is_valid() {
        return Err("loaded map bounds are invalid".into());
    }

    println!("Loaded region: {}", restored.name);
    Ok(())
}
```

## Supported types and representation

All types below implement both `serde::Serialize` and `serde::Deserialize`
when the feature is enabled. No arithmetic, Copy or Default bounds are added.
Generic components need Serialize for encoding or Deserialize for decoding.

| Type | JSON representation / validation |
|---|---|
| `Vector2<T>`, `Vector3<T>`, `Vector4<T>` | Named fields: `{"x":1,"y":2}`, with `z` and `w` as appropriate |
| `Bounds2<T>`, `Bounds3<T>` | `{"min":{...},"max":{...}}`; restores public corners without geometric validation. Use `is_valid()` for untrusted numeric data |
| `Plane` | `{"normal":{"x":0.0,"y":1.0,"z":0.0},"distance":-10.0}`; rejects non-finite coefficients and nonunit normals |
| `SegmentIntersection2` | `{"Point":{"x":1.0,"y":2.0}}` or `{"Overlap":{"start":{...},"end":{...}}}` |
| `ConversionMode`, `RoundingMode`, `ConversionErrorKind` | Variant names, for example `"Clamp"`, `"Floor"`, `"OutOfRange"` |
| `VectorComponent` | Lowercase strings `"x"`, `"y"`, `"z"`, `"w"`; one byte in memory, with the same string representation in binary formats |
| `ConversionError` | `{"component":"x","kind":"OutOfRange"}`; component must be `x`, `y`, `z` or `w` when decoding |

Plane decoding accepts a normal length within `16 * f64::EPSILON` of one to
allow floating-point normalization error. Accepted coefficients are preserved;
decoding never silently rescales the plane. Invalid geometry produces the
serializer's error type, such as `serde_json::Error`.

Vectors and bounds preserve their scalar type and its deserialization rules.
Floating-point NaN/infinity support depends on the chosen format; JSON cannot
represent them as numbers. Select a suitable format or validate values before
writing them. For exact f64 round trips with serde_json, enable its
`float_roundtrip` feature in your application.

`Math` has no stored data and does not need serialization. Scalar operation
traits describe behavior rather than serialized values.

## How the optional implementation works

Cargo's `optional = true` keeps Serde out of ordinary builds. The feature
`serde = ["dep:serde"]` enables that dependency when a consumer requests it.
The vector and bounds macros include this attribute on the generated structures:

```rust,ignore
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`cfg_attr` adds the `derive` only when the feature is enabled. The generated
implementations serialize each field and require only the matching Serde trait
on its scalar type. Plane has a manual Deserialize implementation to validate
its private invariants; VectorComponent uses an allocation-free string visitor
to decode field names into enum variants, including with owned input buffers.
