# Structure, tests and documentation maintenance

[Home](../README.md) · [Index](README.md) · [API map](api.md)

[Source layout](#source-layout) · [Macros](#macros) · [Tests](#tests) · [Documentation checks](#documentation-checks) · [Release metadata](#release-metadata)

<a id="source-layout"></a>
## Source layout

```text
Cargo.toml                 Package yagmath, edition 2024, no dependencies
README.md                 Project overview and documentation entry point
docs/                     Linked cookbook, API inventory, numeric policies, SVGs
scripts/check_docs.py      Local Markdown link/anchor and SVG validation (Python stdlib)
src/
  lib.rs                  Public modules, root re-exports, doctest cookbook imports
  vector/
    vector.rs             create_vector! and Vector2/3/4 declarations
    basic_operations.rs   Add/Sub/Mul/Div and assignment traits
    length.rs             length and length_squared
    normalize.rs          In-place float normalization
    normalized.rs         New-copy f64 normalization
    distance.rs           distance and distance_squared
    dot.rs / cross.rs     Scalar/cross products
    angle.rs              Unsigned angle between vectors
    signed_angle.rs       Signed 2D/3D angles and internal scaling helpers
    lerp.rs / slerp.rs    Linear / spherical interpolation
    move_towards.rs       Euclidean movement limit
    rotate.rs             Dimension-specific rotations
    reflect.rs            New-copy and in-place reflection
    directions.rs         Axis constructors and DirectionScalar
    min_max_clamp.rs      Component limits
    clamp_length.rs       Maximum magnitude
    rounds.rs             floor/ceil/round
    convert.rs            Component conversion entry points
    display.rs            Tuple-like Display
  bounds/bounds.rs        create_bounds! and Bounds2/3
  plane/plane.rs          Plane and private numeric helpers
  math/                   Math scalar functions and 2D segment intersection
  conversion/             Policies/errors and integer/float conversion matrices
  extensions/             Numeric traits and built-in scalar implementations
  tests/
    vector/               Tests grouped by vector feature
    bounds/               Constructors, queries, closest point, validation
    math/                 Inverse lerp and scalar game math
    extensions/           Scalar arithmetic and ToF64
    plane.rs              Plane constructors, projection, invariants
```

Private implementation modules are intentionally not downstream import paths. Use the public re-exports listed in the [API map](api.md#imports).

<a id="macros"></a>
## How generation works

`create_vector!(Vector3, x, y, z)` defines the public structure, constructor and Default implementation, then calls feature macros with the same structure/field identifiers. A feature macro repeats a component expression with `$( ... )+`. The feature files usually declare `pub(super) use impl_feature;` so sibling generation code can resolve the macro.

There is no runtime macro dispatch or inherited base Vector type. Rust sees ordinary concrete structures and implementations after expansion. Trait bounds belong to individual implementations so constructing a vector does not unnecessarily require arithmetic traits.

Cross products and rotations have dimension-specific formulas/signatures, so those files also use explicit implementations. Bounds use their own generation macro for 2D/3D. Plane is a single 3D structure and does not need generation.

Internal macros and helpers have documentation comments beside their definitions,
including the macro arguments, generated implementations, validation rules and
numerical fallbacks. For example, the two private `impl_safe_div` macros separately
describe integer checked division and finite-only float division. Generated trait
methods also explain their local policy rather than relying only on public trait docs.

To include private implementation items in the local HTML reference, run:

```sh
cargo doc --no-deps --document-private-items --open
```

The public handbook remains the downstream API guide; private macros/helpers are
implementation details and are not stable downstream import paths.

<a id="tests"></a>
## Test commands and coverage

```sh
cargo test
cargo test --release
cargo test --doc
cargo test tests::vector::slerp
cargo test tests::plane
```

The suite contains **232 unit tests** plus executable documentation examples. Totals grow as examples/tests are added; command output is authoritative. This is a test count, not a coverage percentage or a claim that every possible numeric input has been proved correct.

Tests include:

| Area | Representative checks |
|---|---|
| Construction/traits | Dimensions, scalar defaults, nonnumeric fields, equality/hash where available, Display |
| Arithmetic | Component and scalar operators, saturation boundaries, signed MIN/-1, zero division, float finite checks, assignments |
| Geometry | Known lengths, squared distances, dot/cross, normalization, zero/non-finite inputs |
| Movement/interpolation | Clamping/extrapolation, preserved magnitude where expected, angle progression, opposite directions, mixed scalar types |
| Rotation/reflection | Orientation, axes/4D planes, invalid inputs, mutation on failure, large/subnormal values |
| Conversion | All 196 numeric pairs, integer wrapping, 128-bit limits, float precision, rounding and an exhaustive i16→i8 test |
| Bounds | Corner order, center/full size, inclusive contact, public-field invalidation, closest point |
| Plane | Unit normal, constructor winding, signed distances, point vs vector projection, flipped orientation, extreme/invalid geometry |
| Math | Reversed/equal ranges, remap, waves, damping convergence/velocity/time validation, angular wraparound |
| Segment intersection | Crossings, contacts, collinear overlap/direction, zero-length segments, mixed scalars, extreme coordinates, all 6,561 configurations on a 3x3 integer grid against integer orientation predicates |

Unit tests live under `src/tests` and are enabled by `#[cfg(test)] mod tests` in lib.rs. Cookbook Rust examples are included under a separate `#[cfg(doctest)]` module, so `cargo test --doc` checks that their public API usage actually compiles and runs.

<a id="documentation-checks"></a>
## Build and verify documentation

```sh
cargo doc --no-deps --open
cargo test --doc
```

For strict rustdoc validation in PowerShell:

```powershell
$env:RUSTDOCFLAGS = '-D missing_docs -D rustdoc::broken_intra_doc_links'
cargo doc --no-deps
Remove-Item Env:RUSTDOCFLAGS
```

In a POSIX shell:

```sh
RUSTDOCFLAGS='-D missing_docs -D rustdoc::broken_intra_doc_links' cargo doc --no-deps
```

Rustdoc is generated under `target/doc/yagmath/index.html`. It exposes macro-generated methods as ordinary API members. Markdown pages remain readable without a site generator or an Obsidian plugin. They use relative links and explicit anchors; assets are standalone SVG with no external font or image dependency.

In a repository checkout, run `python scripts/check_docs.py` to validate local Markdown link targets/anchors and SVG XML using Python's standard library. Python is only needed for this optional maintenance check, not for building or using the Rust crate. External URLs are not fetched by the checker. The `scripts/` directory remains in Git but is excluded from the published Cargo archive; registry downloads do not contain this maintenance script.

When adding a method:

1. Document its signature, scalar bounds, result type, units, invalid inputs and mutation behavior in Rust.
2. Add a cookbook example and update [api.md](api.md).
3. Add meaningful unit tests for the new behavior, including chosen failure policy.
4. Update numeric policy tables if the method introduces different behavior.
5. Run doctests, debug/release unit tests and strict rustdoc.

No benchmarks, automated CI workflow, or coverage measurement are set up by this documentation. Existing unused-import/dead-code compiler warnings are separate from rustdoc checks.

<a id="release-metadata"></a>
## Release metadata

The manifest includes a description, README, five search keywords and the
`mathematics`/`game-engines` categories. It excludes repository scripts from the
Cargo archive while retaining the source and handbook required by doctests.
Inspect the actual archive contents with `cargo package --list --allow-dirty`;
this lists files locally and does not publish anything.

The crate uses the [MIT License](../LICENSE), with `license = "MIT"` in the
manifest and the complete text in the Cargo package. The manifest's repository
field points to GitHub and its documentation field points to the linked handbook.
README links use absolute GitHub URLs and its overview image uses a raw file URL,
so rendering the README outside GitHub does not depend on relative-path rewriting.
The handbook's internal links remain relative for local browsing. Build the API
reference locally with cargo doc; the linked handbook is not generated rustdoc.
