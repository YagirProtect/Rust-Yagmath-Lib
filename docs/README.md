# Documentation

[Project home](../README.md)

This is a linked Markdown handbook. Relative links work in GitHub and ordinary Markdown readers; headings have explicit anchors for stable deep links. The Rust API reference is built locally with `cargo doc --no-deps --open`.

| Page | Contents |
|---|---|
| [Vectors](vectors.md) | Every vector method, operators and dimensions |
| [Math](math.md) | Associated scalar functions and 2D segment intersection |
| [Bounds](bounds.md) | Closed rectangles and AABBs |
| [Plane](plane.md) | Oriented 3D planes and projection |
| [Conversion](conversion.md) | Numeric conversion policies, rounding and errors |
| [Extensions](extensions.md) | Scalar traits and custom types |
| [Serde](serde.md) | Optional serialization, formats and validation on loading |
| [Numeric behavior](numerics.md) | Precision, invalid inputs, overflow and return conventions |
| [API map](api.md) | Compact inventory for humans and coding assistants |
| [no_std](no_std.md) | Embedded targets, feature selection, libm and builds without an allocator |
| [Development](development.md) | Source layout, macro expansion, tests and documentation checks |

Start with [numeric conventions](numerics.md#conventions) before porting arithmetic from another crate. Illustrations are editable, self-contained SVGs in [assets](assets/geometry.svg); they show schematic geometry rather than measured plots.

## Reading the examples

Examples separate setup/calculation from the assertions that show expected output.
Checked operations use `?` or explicit matching rather than unchecked extraction.
Standalone examples that mix Option and Result use a `main` returning Result;
`ok_or("...")?` gives an invalid Option result a readable example error. An
application can instead keep Option or use its own error type. These wrappers do not change the library API.
