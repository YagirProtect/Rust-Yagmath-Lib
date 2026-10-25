# Embedded and no_std support

[Home](../README.md) · [Index](README.md) · [Serde](serde.md)

The default `std` feature uses Rust's standard float operations and introduces
no external dependencies. For firmware without an operating system, disable
default features and enable `no_std`, which selects the optional `libm` math backend:

```toml
[dependencies]
yagmath = { version = "0.1.3", default-features = false, features = ["no_std"] }
```

no_std support was introduced in version 0.1.3. For a local checkout, replace
`version` with `path = "../math_utils"`, adjusting the path to your checkout.

| Features | Behavior |
|---|---|
| Default (`std`) | Standard float operations; no external dependencies |
| `std, serde` | Standard operations and Serde with std support |
| `no_std` with defaults disabled | no_std, no allocator required, libm float operations |
| `no_std, serde` with defaults disabled | no_std and allocation-free serialization implementations |
| `std, no_std` | std takes precedence; useful when dependency features are unified |
| Neither `std` nor `no_std` (or `libm`) | Unsupported; a compile error explains which backend to enable |

Cargo features are additive. Disable defaults on every dependency edge that
should avoid std; another crate can enable it again through feature unification.
The `no_std` feature enables the libm backend; the older `libm` feature remains
an equivalent alias for callers that already use it. Neither name disables std:
`default-features = false` is still required. If std is enabled elsewhere, std
takes precedence.

The library itself uses neither `alloc` nor a global allocator. Generic vector
components can still be allocating types when the application supports them.
Serde's default features are disabled; its derive macros run on the build host.
The data format you choose must also support your target and memory constraints.

## The same geometry API

```rust
use yagmath::vector::Vector3;

let direction = Vector3::new(3.0_f32, 4.0, 0.0);
let distance = direction.length();
let unit = direction.normalized();

assert!((distance - 5.0).abs() < 1e-12);
assert!(unit.is_some());
```

Angles, normalization, bounds, planes, conversion policies and other public
methods remain available. Display uses `core::fmt`; ConversionError implements
`core::error::Error`. Integer arithmetic retains the same saturation policies.
The private FloatMath adapter explicitly selects the backend even in unit tests,
where the host test runner necessarily links std.

libm supplies hypot, trigonometry and rounding. Float Euclidean remainder uses
the same signed-remainder correction as std; libm's IEEE remainder is a different
operation. Finite results from different backends may differ in their last bits.
Use tolerances for geometric comparisons rather than assuming bitwise equality.

Most geometric results still use f64, even with f32 inputs. On small processors
without suitable floating-point hardware, these operations can be expensive.
no_std does not guarantee support for every Arduino board: the Rust toolchain
and firmware ecosystem must support that board's processor.

## Validation

```sh
cargo test --locked
cargo test --locked --all-features
cargo test --locked --no-default-features --features no_std
cargo test --locked --no-default-features --features no_std,serde
rustup target add thumbv6m-none-eabi
cargo build --locked --lib --target thumbv6m-none-eabi --no-default-features --features no_std
cargo build --locked --lib --target thumbv6m-none-eabi --no-default-features --features no_std,serde
```

The Cortex-M0 target has no operating system or standard library, so cross builds
verify that normal library dependencies do not silently require std or alloc.
Tests run on the host; cross builds do not prove runtime correctness on a board.
Panic handlers, startup code and hardware initialization belong to the firmware,
not this library. JSON and other host test dependencies are dev-dependencies.

## AVR validation

AVR requires a nightly compiler with rust-src, AVR GCC, an explicit target CPU
and rebuilding core. See the [Rust AVR target guide](https://doc.rust-lang.org/rustc/platform-support/avr-none.html).
The following commands were verified locally with the already installed
`nightly-2025-04-27` toolchain and AVR GCC 14.1.0:

```powershell
$env:CARGO_TARGET_AVR_NONE_RUSTFLAGS = '-C target-cpu=atmega328p'
cargo +nightly-2025-04-27 build --locked --release --lib --target avr-none --no-default-features --features no_std -Z build-std=core
cargo +nightly-2025-04-27 build --locked --release --lib --target avr-none --no-default-features --features no_std,serde -Z build-std=core
```

Both library variants compiled. Separate temporary no_std firmware examples
also linked successfully with size optimization and LTO:

| Processor | Example operations | text + data bytes reported by avr-size |
|---|---|---|
| ATmega328P | Vector2 length, normalized copy and saturating i8 vector addition | 19,188 |
| ATmega2560 | Length, normalization, rotation, angle, rounding, ping-pong, bounds and plane construction | 61,298 |

The wider example exceeded ATmega328P's 32 KiB flash. These are sizes of specific
example programs with compiler optimization, not a fixed cost of using yagmath.
The ELF file size on disk is different from flash usage. The table does not
measure runtime stack consumption or performance. f32 inputs still lead to f64
geometry calculations, which can be costly on 8-bit AVR.

The smoke programs were generated under ignored target directories and were not
flashed or run on hardware or in a simulator. AVR checks are local validation;
the current CI cross-build job targets Cortex-M0.
