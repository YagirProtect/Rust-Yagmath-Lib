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
use yagmath::{ConversionMode, vector::Vector3};
let input = Vector3::new(1_i32, 2, 3);
let output = input.convert::<f64>(ConversionMode::Checked).unwrap();
assert_eq!(output, Vector3::new(1.0, 2.0, 3.0));
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
use yagmath::{ConversionMode, vector::Vector2};
let values = Vector2::new(300_i32, -130);
assert!(values.convert::<i8>(ConversionMode::Checked).is_err());
assert_eq!(values.convert::<i8>(ConversionMode::Clamp).unwrap(), Vector2::new(127, -128));
assert_eq!(values.convert::<i8>(ConversionMode::Wrap).unwrap(), Vector2::new(44, 126));

let values = Vector2::new(16_777_217_i32, 1);
assert!(values.convert::<f32>(ConversionMode::Checked).is_err());
assert_eq!(values.convert::<f32>(ConversionMode::Clamp).unwrap().x, 16_777_216.0);
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
use yagmath::{ConversionMode as C, RoundingMode as R, vector::Vector2};
let v = Vector2::new(1.5_f64, -1.5);
assert_eq!(v.convert::<i32>(C::Checked).unwrap(), Vector2::new(1, -1));
assert_eq!(v.convert_with_rounding::<i32>(C::Checked, R::Round).unwrap(), Vector2::new(2, -2));
assert_eq!(v.convert_with_rounding::<i32>(C::Checked, R::Floor).unwrap(), Vector2::new(1, -2));
assert_eq!(v.convert_with_rounding::<i32>(C::Checked, R::Ceil).unwrap(), Vector2::new(2, -1));
```

<a id="errors"></a>
## ConversionError and ConversionErrorKind

`ConversionError` has public fields `component: &'static str` and `kind: ConversionErrorKind`. It implements Display and `std::error::Error`; all policy/error types derive Debug, Clone, Copy, PartialEq and Eq.

| Kind | Cause |
|---|---|
| `OutOfRange` | Rounded/cast result cannot fit the target range |
| `PrecisionLoss` | Checked conversion into a float loses finite representation precision |
| `NonFinite` | NaN, or infinity in a mode that rejects it |
| `UnsupportedMode` | Wrap requested with any float source/target |

```rust
use yagmath::{ConversionErrorKind, ConversionMode, vector::Vector2};
let error = Vector2::new(10_i32, 300).convert::<i8>(ConversionMode::Checked).unwrap_err();
assert_eq!(error.component, "y");
assert_eq!(error.kind, ConversionErrorKind::OutOfRange);
assert!(error.to_string().contains("component y"));
```

<a id="scalar-conversion"></a>
## ConvertFrom for a scalar or custom component

`yagmath::conversion::ConvertFrom<T>` is implemented on the destination type. Its method borrows the source and returns `Result<Self, ConversionErrorKind>`. It can be used directly; vector conversion adds the field name. Implement this trait on a local destination type to extend vector conversion.

```rust
use yagmath::{ConversionMode, RoundingMode};
use yagmath::conversion::ConvertFrom;
let value = i8::convert_from(&300_i32, ConversionMode::Clamp, RoundingMode::Truncate).unwrap();
assert_eq!(value, 127);
```
