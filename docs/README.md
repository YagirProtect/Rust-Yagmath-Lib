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
| [Numeric behavior](numerics.md) | Precision, invalid inputs, overflow and return conventions |
| [API map](api.md) | Compact inventory for humans and coding assistants |
| [Development](development.md) | Source layout, macro expansion, tests and documentation checks |

Start with [numeric conventions](numerics.md#conventions) before porting arithmetic from another crate. Illustrations are editable, self-contained SVGs in [assets](assets/geometry.svg); they show schematic geometry rather than measured plots.
