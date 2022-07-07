`rhai-rand` - Package to Generate Random Numbers
===============================================

[![License](https://img.shields.io/crates/l/rhai)](https://github.com/license/rhaiscript/rhai-rand)
[![crates.io](https://img.shields.io/crates/v/rhai-rand?logo=rust)](https://crates.io/crates/rhai-rand/)
[![crates.io](https://img.shields.io/crates/d/rhai-rand?logo=rust)](https://crates.io/crates/rhai-rand/)

[![Rhai logo](https://rhai.rs/book/images/logo/rhai-banner-transparent-colour.svg)](https://rhai.rs)

`rhai-rand` is a [Rhai] package to provide random number generation using the [`rand`] crate.

[Rhai] is an embedded scripting language and evaluation engine for Rust that gives a safe and easy way
to add scripting to any application.


Usage
-----

### `Cargo.toml`

```toml
[dependencies]
rhai-rand = "0.1"
```

### [Rhai] script

```js
// Create random boolean
let decision = rand_bool();

if decision {
    // Create random number
    let random_value = rand();
    print(`Random number = ${random_value}`);
} else {
    print("Fixed number = 42");
}

// Create array
let a = [1, 2, 3, 4, 5];

// Shuffle it!
a.shuffle();

// Now the array is shuffled randomly!
print(a);

// Sample a random value from the array
print(a.sample());

// Or sample multiple values
print(a.sample(3));
```

### Rust source

```rust
// packages::Package implements `as_shared_module`
// which we need to register the RandomPackage
use rhai::{Engine, packages::Package};
use rhai_rand::RandomPackage;

// Create Rhai scripting engine
let mut engine = Engine::new();

// Create random number package and add the package into the engine
engine.register_global_module(RandomPackage::new().as_shared_module());

// Print 10 random numbers, each of which between 0-99!
for _ in 0..10 {
    let value = engine.eval::<INT>("(rand() % 100).abs()")?;

    println!("Random number = {}", value);
}
```


Features
--------

|  Feature   | Default  | Description                                                                        |
| :--------: | :------: | ---------------------------------------------------------------------------------- |
| `metadata` | disabled | includes functions metadata: parameter names/types, return type, doc-comments      |
| `decimal`  | Disabled | provides random [decimal](https://crates.io/crates/rust_decimal) number generation |
|  `float`   | enabled  | provides random floating-point number generation                                   |
|  `array`   | enabled  | provides methods for [Rhai] arrays                                                 |


API
---

The following functions are defined in this package:

|          Function          | Return value |  Feature  | Description                                                            |
| :------------------------: | :----------: | :-------: | ---------------------------------------------------------------------- |
|          `rand()`          |    `INT`     |           | generates a random integer number                                      |
|     `rand(start..end)`     |    `INT`     |           | generates a random integer number within an exclusive range            |
|    `rand(start..=end)`     |    `INT`     |           | generates a random integer number within an inclusive range            |
|     `rand(start, end)`     |    `INT`     |           | generates a random integer number within a range                       |
|       `rand_float()`       |   `FLOAT`    |  `float`  | generates a random floating-point number between `0.0` and `1.0`       |
|  `rand_float(start, end)`  |   `FLOAT`    |  `float`  | generates a random floating-point number an exclusive range            |
|      `rand_decimal()`      | [`Decimal`]  | `decimal` | generates a random [decimal][`Decimal`] number                         |
| `rand_decimal(start, end)` | [`Decimal`]  | `decimal` | generates a random [decimal][`Decimal`] number within a range          |
|       `rand_bool()`        |    `bool`    |           | generates a random boolean                                             |
|       `rand_bool(p)`       |    `bool`    |           | generates a random boolean with the probability `p` of being `true`    |
|     `Array.shuffle()`      |              |  `array`  | shuffles the items in the [Rhai] array                                 |
|      `Array.sample()`      |  `Dynamic`   |  `array`  | copies a random element from the [Rhai] array                          |
|     `Array.sample(n)`      |   `Array`    |  `array`  | copies a non-repeating random sample of elements from the [Rhai] array |


[Rhai]: https://rhai.rs
[`rand`]: https://crates.io/crates/rand
[`Decimal`]: https://crates.io/crates/rust_decimal
