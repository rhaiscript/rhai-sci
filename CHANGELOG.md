# Changelog

## 0.4.0 — 2026-09-19

### Numerical workflows

- Add `row`, `col`, and `mat` constructors over ordinary Rhai arrays with explicit orientation and numeric shape validation.
- Add `dot` as a real scalar inner product of equal-length lists, rows, or columns. Use `mtimes` for matrix multiplication.
- Accept numeric row and column vectors consistently in statistics, moving and cumulative operations, differences, interpolation, and trapezoidal integration. Mixed INT/FLOAT samples are supported; sequence results remain flat lists.
- Validate complete matrix shapes and preserve row/column orientation through transpose, concatenation, and related operations.
- Preserve integer precision in integer statistics, compare extrema numerically, and report malformed numerical input as script errors in the revised paths.
- Return the fitted `intercept` from `regress`, alongside the existing predictor coefficients, p-values, and standard errors. Validate response shape and length before fitting.
- Add examples for local CSV calibration and residual diagnostics, matrix inversion, projectile motion, and explicit XOR backpropagation.

### Compatibility and migration

- `regress` fits an intercept automatically. Do not add a column of ones; calculate predictions as `intercept + X * parameters`. Existing result fields retain their meanings, with `intercept` added as a separate field.
- The Rhai names `transpose`, `horzcat`, `vertcat`, and `mtimes` remain available. Rust callers of matrix helpers should use the new `RhaiMatrix` wrappers or the corresponding `*_from_array` functions for array inputs.
- New constructors reject empty or ragged shapes. The flat-list identities for `sum`, `prod`, `diff`, and `unique` remain available on empty arrays.
- `dot` supports real vectors only; it does not implement matrix-axis overloads or complex conjugation. No implicit broadcasting is introduced.

### Build and validation

- Restore minimal-feature builds without the optional linear-algebra backend.
- Limit Polars to the supported 0.45 API and enable only its CSV feature.
- Test minimal, numerical, and full/documentation feature configurations in CI.
