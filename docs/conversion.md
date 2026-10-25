# Explicit numeric conversion

[Home](../README.md) · [Index](README.md) · [Vectors](vectors.md#conversion) · [Numeric policies](numerics.md)

[Methods](#methods) · [Modes](#modes) · [Rounding](#rounding) · [Errors](#errors) · [Scalar conversion](#scalar-conversion)

Conversion preserves the vector dimension and changes its scalar type. The implementations cover all 196 ordered source/target pairs of the 14 built-in numeric types. Integer-to-integer conversion never passes through f64, including i128/u128.

<a id="methods"></a>
## Methods

```text
v.convert::<U>(mode) -> Result<VectorN<U>, ConversionError>
v.convert_with_rounding::<U>(mode, rounding) -> Result<VectorN<U>, ConversionError>
```

The type argument is the **component type**, not `VectorN<U>`. Source is borrowed. Conversions stop at the first failing component in x/y/z/w order; no partial vector is returned. Default rounding is Truncate; it only applies to float-to-integer conversion.

```rust
use yagmath::{ConversionError, ConversionMode, vector::Vector3};

fn main() -> Result<(), ConversionError> {
    let input = Vector3::new(1_i32, 2, 3);
    let output = input.convert::<f64>(ConversionMode::Checked)?;

    assert_eq!(output, Vector3::new(1.0, 2.0, 3.0));

    Ok(())
}
```

<a id="modes"></a>
## ConversionMode

| Mode | Integer → integer | Integer → float | Float → integer | Float → float |
|---|---|---|---|---|
| `Checked` | Reject range overflow | Reject overflow and precision loss | Round first, then check range | Reject overflow and finite representation loss |
| `Clamp` | Saturate to target bounds | Allow rounding, saturate finite range overflow | Round first, then saturate | Allow rounding, saturate to finite bounds |
| `Wrap` | Rust integer `as` cast semantics | Unsupported | Unsupported | Unsupported |

Checked float-to-integer conversion **does not reject a fractional source** merely for having fractions: the selected rounding defines the desired integer. NaN is always an error; infinity is rejected in Checked and saturates in Clamp. Clamp can round very small floats to zero during narrowing; it is not an exactness policy.

```rust
use yagmath::{ConversionError, ConversionMode, vector::Vector2};

fn main() -> Result<(), ConversionError> {
    let values = Vector2::new(300_i32, -130);
    let checked = values.convert::<i8>(ConversionMode::Checked);

    assert!(checked.is_err());

    let clamped = values.convert::<i8>(ConversionMode::Clamp)?;
    let wrapped = values.convert::<i8>(ConversionMode::Wrap)?;

    assert_eq!(clamped, Vector2::new(127, -128));
    assert_eq!(wrapped, Vector2::new(44, 126));

    let wide = Vector2::new(16_777_217_i32, 1);
    let exact = wide.convert::<f32>(ConversionMode::Checked);

    assert!(exact.is_err());

    let rounded = wide.convert::<f32>(ConversionMode::Clamp)?;

    assert_eq!(rounded.x, 16_777_216.0);

    Ok(())
}
```

Wrap is modular integer conversion, not geometric wrapping and not remapping one interval proportionally onto another. `300_i32 -> i8` wrapping produces `44`; clamping produces `127`.

<a id="rounding"></a>
## RoundingMode

| Mode | `1.5` | `-1.5` | Meaning |
|---|---|---|---|
| `Truncate` | 1 | -1 | Toward zero, default |
| `Round` | 2 | -2 | Nearest, ties away from zero |
| `Floor` | 1 | -2 | Toward negative infinity |
| `Ceil` | 2 | -1 | Toward positive infinity |

Rounding occurs **before** the target range check. For example, flooring `-0.1` produces -1, which cannot fit a checked unsigned target, while truncation produces zero.

```rust
use yagmath::{ConversionError, ConversionMode, RoundingMode, vector::Vector2};

fn main() -> Result<(), ConversionError> {
    let values = Vector2::new(1.5_f64, -1.5);
    let truncated = values.convert::<i32>(ConversionMode::Checked)?;

    assert_eq!(truncated, Vector2::new(1, -1));

    let rounded = values.convert_with_rounding::<i32>(
        ConversionMode::Checked, RoundingMode::Round,
    )?;

    assert_eq!(rounded, Vector2::new(2, -2));

    let floored = values.convert_with_rounding::<i32>(
        ConversionMode::Checked, RoundingMode::Floor,
    )?;

    assert_eq!(floored, Vector2::new(1, -2));

    let ceiled = values.convert_with_rounding::<i32>(
        ConversionMode::Checked, RoundingMode::Ceil,
    )?;

    assert_eq!(ceiled, Vector2::new(2, -1));

    Ok(())
}
```

<a id="errors"></a>
## ConversionError and ConversionErrorKind

`ConversionError` has public fields `component: VectorComponent` and `kind: ConversionErrorKind`. It implements Display and `core::error::Error`; all policy/error types derive Debug, Clone, Copy, PartialEq and Eq.

`VectorComponent` is re-exported from `yagmath` and `yagmath::vector`. Its variants
are `X`, `Y`, `Z`, `W`, shared across dimensions. It occupies one byte using
`#[repr(u8)]` and prevents invalid component names. Display and `as_str()` return
the lowercase field name. Serde still reads/writes lowercase strings to preserve
existing payloads; the memory reduction does not change the wire representation.
Changing `component` from a string to this enum is a source-level API change.

| Kind | Cause |
|---|---|
| `OutOfRange` | Rounded/cast result cannot fit the target range |
| `PrecisionLoss` | Checked conversion into a float loses finite representation precision |
| `NonFinite` | NaN, or infinity in a mode that rejects it |
| `UnsupportedMode` | Wrap requested with any float source/target |

```rust
use yagmath::{ConversionErrorKind, ConversionMode, VectorComponent, vector::Vector2};

let input = Vector2::new(10_i32, 300);
let result = input.convert::<i8>(ConversionMode::Checked);

match result {
    Ok(converted) => println!("Converted: {converted}"),
    Err(error) => {
        eprintln!("Cannot convert component {}: {}", error.component, error.kind);

        assert_eq!(error.component, VectorComponent::Y);
        assert_eq!(error.kind, ConversionErrorKind::OutOfRange);
    }
}
```

<a id="scalar-conversion"></a>
## ConvertFrom for a scalar or custom component

`yagmath::conversion::ConvertFrom<T>` is implemented on the destination type. Its method borrows the source and returns `Result<Self, ConversionErrorKind>`. It can be used directly; vector conversion adds the field name. Implement this trait on a local destination type to extend vector conversion.

```rust
use yagmath::{ConversionErrorKind, ConversionMode, RoundingMode};
use yagmath::conversion::ConvertFrom;

fn main() -> Result<(), ConversionErrorKind> {
    let value = i8::convert_from(
        &300_i32, ConversionMode::Clamp, RoundingMode::Truncate,
    )?;

    assert_eq!(value, 127);

    Ok(())
}
```
