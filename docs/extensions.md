# Scalar extensions and custom types

[Home](../README.md) · [Index](README.md) · [Operators](vectors.md#operators) · [Conversion](conversion.md)

[Traits](#traits) · [Direct calls](#direct-calls) · [Custom scalar](#custom-scalar)

These traits specify the operations available to generic vector implementations. They are ordinary Rust traits, not runtime extensions. Import the trait to call its method on a scalar.

<a id="traits"></a>
## Public traits

| Trait | Import path | Method / constants | Built-in policy |
|---|---|---|---|
| `SafeAdd` | `extensions::add::t_safe_add::SafeAdd` | `safe_add(&self, rhs: Self) -> Self` | Saturating integers, ordinary floats |
| `SafeSub` | `extensions::sub::t_safe_sub::SafeSub` | `safe_sub(&self, rhs: Self) -> Self` | Saturating integers, ordinary floats |
| `SafeMul` | `extensions::mul::t_safe_mul::SafeMul` | `safe_mul(&self, rhs: Self) -> Self` | Saturating integers, ordinary floats |
| `SafeDiv` | `extensions::div::t_safe_div::SafeDiv` | `safe_div(&self, rhs: Self) -> Option<Self>` | Checked integers; floats must be finite with nonzero divisor and finite result |
| `ToF64` | `extensions::to_f64::t_to_f64::ToF64` | `to_f64(&self) -> f64` | Rust casts; possibly lossy |
| `DirectionScalar` | `vector::DirectionScalar` | `ZERO`, `ONE` | Typed zero and one |
| `ConvertFrom<T>` | `conversion::ConvertFrom` | `convert_from(&T, mode, rounding)` | Explicit conversion policy |

All six scalar/direction traits are implemented for the 14 numeric types; ConvertFrom covers every ordered numeric pair. Traits do not imply each other: ToF64 does not promise Add/Default/SafeDiv. `Safe` does not mean float addition/multiplication always remains finite. A custom implementation can choose other behavior and may panic; built-in guarantees do not automatically extend to it.

The source has a `number_lerp` implementation behind `pub(crate)` visibility. It is not part of the public API; do not import it from downstream projects. There is currently no public scalar `Math::lerp` method.

<a id="direct-calls"></a>
## Direct scalar usage

```rust
use yagmath::extensions::add::t_safe_add::SafeAdd;
use yagmath::extensions::sub::t_safe_sub::SafeSub;
use yagmath::extensions::mul::t_safe_mul::SafeMul;
use yagmath::extensions::div::t_safe_div::SafeDiv;
use yagmath::extensions::to_f64::t_to_f64::ToF64;

assert_eq!(120_i8.safe_add(20), 127);
assert_eq!(0_u8.safe_sub(1), 0);
assert_eq!(100_u8.safe_mul(3), 255);
assert_eq!(10_i32.safe_div(0), None);
assert_eq!((-9_i32).safe_div(2), Some(-4));
assert_eq!(42_i32.to_f64(), 42.0);
assert!((f32::MAX).safe_mul(2.0).is_infinite());
```

<a id="custom-scalar"></a>
## Implement only the operations you need

You can store a custom scalar without implementing any math traits. Implement ToF64 to enable methods such as length and angle; implement SafeAdd separately to enable vector addition. In-place float-specific operations are still only implemented for f32/f64.

```rust
use yagmath::vector::Vector2;
use yagmath::extensions::to_f64::t_to_f64::ToF64;

struct Units(i32);
impl ToF64 for Units {
    fn to_f64(&self) -> f64 { self.0 as f64 }
}
let v = Vector2::new(Units(3), Units(4));
assert_eq!(v.length(), 5.0);
```

Direction constructors additionally use DirectionScalar. Negative directions need `std::ops::Neg<Output=Self>`, which is why unsigned scalars have up/right/forward but no down/left/backward methods.
