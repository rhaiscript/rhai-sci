[![Github CI](https://github.com/rhaiscript/rhai-sci/actions/workflows/tests.yml/badge.svg)](https://github.com/rhaiscript/rhai-sci/actions)
[![Crates.io](https://img.shields.io/crates/v/rhai-sci.svg)](https://crates.io/crates/rhai-sci)
[![docs.rs](https://img.shields.io/docsrs/rhai-sci/latest?logo=rust)](https://docs.rs/rhai-sci)

# rhai-sci

## What & Why

`rhai-sci` adds basic scientific computing utilities to the [Rhai](https://rhai.rs/) scripting language. It is inspired by tools such as MATLAB, Octave, and R.

## Quickstart

Add the crate to your `Cargo.toml`:

```toml
rhai-sci = "0.4.0"
```

Evaluate a single line of Rhai code:

```rust
use rhai::INT;
use rhai_sci::eval;

let result = eval::<INT>("argmin([43, 42, -500])").unwrap();
```

## Examples

Integrate the package with a persistent Rhai engine:

```rust
use rhai::{packages::Package, Engine, INT};
use rhai_sci::SciPackage;

let mut engine = Engine::new();
engine.register_global_module(SciPackage::new().as_shared_module());

let value = engine.eval::<INT>("argmin([43, 42, -500])").unwrap();
```

For a complete data workflow, run:

```bash
cargo run --example regression_workflow
```

This loads the bundled calibration CSV, constructs two predictor columns, fits a
linear model, predicts responses, and summarizes residuals. The sample has an
intercept of 1, slopes of 2 and 0.5, and an RMSE of approximately 0.1414.
It uses local data and requires the `io` and `nalgebra` features (both on by default).
Rust hosts that already have data can pass an `observations` array to the same
[script](examples/regression_workflow.rhai) without enabling `io`.

Other examples cover [CSV regression](examples/download_and_regress.rhai),
[matrix inversion](examples/matrix_inversion.rhai), and
[projectile motion](examples/projectile_motion.rhai).
The [XOR backpropagation example](examples/neural_network_backprop.rhai) is an
advanced demonstration of composing matrix operations.

## Matrix and Vector Conventions

Matrices use ordinary Rhai arrays of rows. Constructors make orientation explicit
and validate numeric values while preserving their INT/FLOAT types:

```typescript
let values = [1, 2, 3];        // plain Rhai list
let c = col(values);           // N by 1: [[1], [2], [3]]
let r = row(values);           // 1 by N: [[1, 2, 3]]
let A = mat([[1, 2], [3, 4]]); // rectangular numeric matrix
```

`row` and `col` also convert between vector orientations. Constructors reject empty,
ragged, or nonnumeric inputs. The returned arrays remain editable; matrix operations
check their inputs again. General matrix arithmetic does not implicitly broadcast
or turn a flat list into a row or column.

Use `mtimes` for matrix multiplication and `dot` for a real scalar inner product:

```typescript
let A = mat([[1, 2], [3, 4]]);
let x = col([5, 6]);
let prediction = mtimes(A, x); // [[17.0], [39.0]]
let energy = dot(x, x);        // 61.0; also accepts lists or row vectors
let At = transpose(A);
let augmented = horzcat(A, x);
let extended = vertcat(A, row([7, 8]));
```

`dot` accepts equal-length, nonempty vectors in any combination of orientations and
returns FLOAT. It does not implement MATLAB's matrix/axis overloads or complex
conjugation. `mtimes` requires matching inner dimensions and returns a matrix,
including a 1 by 1 matrix for a row times a column.

Statistics, moving/cumulative operations, differences, interpolation, and trapezoidal
integration accept numeric lists, rows, and columns. Mixed INT/FLOAT values are
supported. Scalar statistics return scalars; sequence operations return flat lists,
so use `col` or `row` when feeding those results back into matrix operations.
Inputs retain their original shape:

```typescript
let samples = col([1, 2.0, 3]);
let average = mean(samples);         // 2.0
let smoothed = movmean(samples, 3);  // [1.5, 2.0, 2.5]
let area = trapz(row([0, 1, 2]), samples); // 4.0
```

Empty numeric samples produce a script error where a value is required. The
flat-list identities `sum([]) == 0`, `prod([]) == 1`, `diff([]) == []`, and
`unique([]) == []` remain available.

### Regression and predictions

`regress(X, y)` treats rows as observations and fits an intercept automatically.
Do not add a column of ones. The returned `parameters`, `pvalues`, and
`standard_errors` correspond to predictor columns in order. The fitted `intercept`
is returned separately:

```typescript
let X = col([0, 1, 2]);
let fit = regress(X, col([1.1, 2.8, 5.1]));
let linear_part = mtimes(X, col(fit.parameters));
let first_prediction = fit.intercept + linear_part[0][0];
```

Earlier releases omitted the fitted intercept from the result. Existing result
fields are retained; use the new `intercept` field when calculating predictions.

### Features

- **metadata** *(disabled)*: export function metadata; required for running doc-tests on Rhai examples.
- **io** *(enabled)*: provides `read_matrix` but pulls in `polars`, `url`, `temp-file`, `csv-sniffer`, and `minreq`.
- **nalgebra** *(enabled)*: enables matrix functions such as `regress`, `inv`, `mtimes`, `horzcat`, `vertcat`, `repmat`, `svd`, `hessenberg`, and `qr` via the `nalgebra` and `linregress` crates.
- **rand** *(enabled)*: adds the `rand` function for generating random values and matrices using the `rand` crate.

## CLI/API reference

The full API is documented on [docs.rs](https://docs.rs/rhai-sci).

## Development

```bash
cargo fmt --all -- --check
cargo test --no-default-features
cargo test --no-default-features --features rand,nalgebra
cargo test --all-features
```

## Troubleshooting

- Building with the `io` feature enabled pulls in heavy dependencies. Disable default features and enable only what you need if builds are slow.

## License

Licensed under either of

- [MIT license](LICENSE-MIT.txt)
- [Apache License, Version 2.0](LICENSE-APACHE.txt)

at your option.
