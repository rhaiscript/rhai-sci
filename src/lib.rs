use rand::prelude::*;
use rhai::def_package;
use rhai::plugin::*;
use rhai::{EvalAltResult, Position, INT};
use std::ops::{Range, RangeInclusive};

use rhai::Array;
use rhai::FLOAT;

def_package! {
    /// Package for random number generation, sampling and shuffling.
    pub RandomPackage(lib) {

        //
        let engine = Engine::new();
        let ast = engine.compile(rhai_max().to_owned() + rhai_min() + rhai_max_array() + rhai_min_array()).unwrap();
        let my_module = Module::eval_ast_as_new(rhai::Scope::new(), &ast, &engine).unwrap();
        lib.fill_with(&my_module);

        combine_with_exported_module!(lib, "rand", rand_functions);

    }
}

#[export_module]
mod rand_functions {
    /// Generate a random boolean value with a probability of being `true`.
    ///
    /// `probability` must be between `0.0` and `1.0` (inclusive).
    ///
    /// # Example
    ///
    /// ```rhai
    /// let decision = rand(0.01);  // 1% probability
    ///
    /// if decision {
    ///     print("You hit the Jackpot!")
    /// }
    /// ```
    #[rhai_fn(return_raw)]
    pub fn rand_bool_with_probability(probability: FLOAT) -> Result<bool, Box<EvalAltResult>> {
        if probability < 0.0 || probability > 1.0 {
            Err(EvalAltResult::ErrorArithmetic(
                format!(
                    "Invalid probability (must be between 0.0 and 1.0): {}",
                    probability
                ),
                Position::NONE,
            )
            .into())
        } else {
            Ok(rand::thread_rng().gen_bool(probability as f64))
        }
    }
}

fn rhai_max() -> &'static str {
    "fn max(a, b) { if (a > b) { return a;} else {return b;}};"
}

fn rhai_min() -> &'static str {
    "fn min(a, b) { if (a < b) { return a;} else {return b;}};"
}

fn rhai_max_array() -> &'static str {
    "fn max(arr) { arr.sort(); arr[-1]; };"
}

fn rhai_min_array() -> &'static str {
    "fn min(arr) { arr.sort(); arr[0]; };"
}
