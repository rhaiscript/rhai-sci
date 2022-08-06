[![Github CI](https://github.com/rhaiscript/rhai-sci/actions/workflows/tests.yml/badge.svg)](https://github.com/rhaiscript/rhai-sci/actions)
[![Crates.io](https://img.shields.io/crates/v/rhai-sci.svg)](https://crates.io/crates/rhai-sci)
[![docs.rs](https://img.shields.io/docsrs/rhai-sci/latest?logo=rust)](https://docs.rs/rhai-sci)

# About `rhai-sci`
This crate provides some basic scientific computing utilities for the [`Rhai`](https://rhai.rs/) scripting language, inspired by languages 
like MATLAB, Octave, and R. For a complete API reference, check [the docs](https://docs.rs/rhai-sci).

# Install
To use the latest released version of `rhai-sci`, add this to your `Cargo.toml`:
```toml
rhai-sci = "0.1.7"
```
To use the bleeding edge instead, add this:
```toml
rhai-sci = { git = "https://github.com/cmccomb/rhai-sci" }
```

# Usage
Using this crate is pretty simple! If you just want to evaluate a single line of [`Rhai`](https://rhai.rs/), then you only need:
```rust
use rhai::INT;
use rhai_sci::eval;
let result = eval::<INT>("argmin([43, 42, -500])").unwrap();
```
If you need to use `rhai-sci` as part of a persistent [`Rhai`](https://rhai.rs/) scripting engine, then do this instead:
```rust
use rhai::{Engine, packages::Package, INT};
use rhai_sci::SciPackage;

// Create a new Rhai engine
let mut engine = Engine::new();

// Add the rhai-sci package to the new engine
engine.register_global_module(SciPackage::new().as_shared_module());

// Now run your code
let value = engine.eval::<INT>("argmin([43, 42, -500])").unwrap();
```

# Features
| Feature Flag | Default | Description                                                                                                                                                                   |
|--------------|---------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `metadata`   | No      | Enables exporting function metadata and is ___necessary for running doc-tests on Rhai examples___.                                                                            |
| `io`         | Yes     | Enables the `read_matrix` function but pulls in several additional dependencies (`polars`, `url`, `temp-file`, `csv-sniffer`, `minreq`). Also actives the `nalgebra` feature. | 
| `nalgebra`   | Yes     | Enables several functions (`regress`, `inv`, `mtimes`, `horzcat`, `vertcat`, and `repmat`) but brings in the `nalgebra` and `linregress` libraries.                           | 
| `rand`       | Yes     | Enables the `rand` function for generating random FLOATs values and random matrices, but brings in the `rand` library.                                                        |