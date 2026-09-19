[![Github CI](https://github.com/rhaiscript/rhai-sci/actions/workflows/tests.yml/badge.svg)](https://github.com/rhaiscript/rhai-sci/actions)
[![Crates.io](https://img.shields.io/crates/v/rhai-sci.svg)](https://crates.io/crates/rhai-sci)
[![docs.rs](https://img.shields.io/docsrs/rhai-sci/latest?logo=rust)](https://docs.rs/rhai-sci)

# rhai-sci

Scientific computing for the [Rhai](https://rhai.rs/) scripting language, inspired
by MATLAB, Octave, and R. Includes statistics, linear algebra, interpolation,
integration, and regression.

## Quickstart

Add the crate to your `Cargo.toml`:

```toml
rhai-sci = "0.4.0"
```

Evaluate a Rhai expression:

```rust
use rhai::INT;
use rhai_sci::eval;

let result = eval::<INT>("argmin([43, 42, -500])").unwrap();
assert_eq!(result, 2);
```

For a persistent engine, register `SciPackage` as shown in the
[Rust host example](examples/regression_workflow.rs).

## Numerical workflows

Use `row`, `col`, and `mat` to construct vectors and matrices from ordinary Rhai
arrays. Statistics accept lists, rows, or columns; sequence results are flat lists.
Use `mtimes` for matrix multiplication and `dot` for a scalar vector inner product.

`regress(X, y)` fits an intercept automatically and returns it separately from the
predictor coefficients. See the [workflow guide](docs/numerical-workflows.md) for
shape conventions, predictions, and input validation.

Run the bundled CSV example to fit a model and summarize its residuals:

```bash
cargo run --example regression_workflow
```

More examples: [matrix inversion](examples/matrix_inversion.rhai),
[projectile motion](examples/projectile_motion.rhai), and
[XOR backpropagation](examples/neural_network_backprop.rhai).

## Features

| Feature | Default | Enables |
| --- | --- | --- |
| `io` | On | CSV loading with `read_matrix` |
| `nalgebra` | On | Matrix operations and regression |
| `rand` | On | Random values and matrices |
| `metadata` | Off | Function metadata and Rhai documentation tests |

Disable default features and select only what you need for smaller builds;
CSV support brings in Polars.

## Reference

[API documentation](https://docs.rs/rhai-sci) ·
[Changelog](CHANGELOG.md) ·
[Development checks](docs/numerical-workflows.md#development)

## License

Licensed under [MIT](LICENSE-MIT.txt) or [Apache-2.0](LICENSE-APACHE.txt), at your option.
