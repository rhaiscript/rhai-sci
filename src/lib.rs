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
        let ast = engine.compile(aggregate_functions()).unwrap();
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

fn aggregate_functions() -> String {
    String::new()
        + max()
        + min()
        + max_array()
        + min_array()
        + mink()
        + maxk()
        + bounds()
        + mean()
        + linspace()
}

fn max() -> &'static str {
    "fn max(a, b) { if (a > b) { return a;} else {return b;}};"
}

fn min() -> &'static str {
    "fn min(a, b) { if (a < b) { return a;} else {return b;}};"
}

fn max_array() -> &'static str {
    "fn max(arr) { arr.sort(); arr[-1]; };"
}

fn min_array() -> &'static str {
    "fn min(arr) { arr.sort(); arr[0]; };"
}

fn mink() -> &'static str {
    "fn mink(arr, k) { arr.sort(); arr.extract(0..k); };"
}

fn maxk() -> &'static str {
    "fn maxk(arr, k) { arr.sort(); let L = arr.len(); arr.extract((L-k)..L); };"
}

fn bounds() -> &'static str {
    "fn bounds(arr) { [min(arr), max(arr)] };"
}

fn mean() -> &'static str {
    "fn mean(arr) { arr.reduce(|sum, v| sum + v*1.0, 0)/arr.len() };"
}

fn median() -> &'static str {
    "fn median(arr) { prctile(arr, 50); };"
}

fn iqr() -> &'static str {
    "fn iqr(arr) { prctile(arr, 75) - prctile(arr, 25) };"
}

fn prctile() -> &'static str {
    "fn prctile(arr, p) {
        let x = linspace(0, 100, arr.len());
        interp1(x, arr, p)
    };"
}

fn linspace() -> &'static str {
    "fn linspace(x1, x2, n) {
        let arr = [x1];
        let int = (1.0*x2 - x1)/(n-1); 
        for i in 0..(n-2) {
            arr.push(arr[-1] + int)
        }
        arr.push(x2);
        arr
    };"
}

fn interp1() -> &'static str {
    "fn interp1(x, v, xq) { };"
}
