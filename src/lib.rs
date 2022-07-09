use rhai::{def_package, packages::Package, plugin::*, Array, EvalAltResult, Position, FLOAT, INT};
use rhai_rand::RandomPackage;
use std::ops::{Range, RangeInclusive};

def_package! {
    pub LabPackage(lib) {

        RandomPackage::init(lib);

        //
        let engine = Engine::new();
        let ast = engine.compile(aggregate_functions()).unwrap();
        let my_module = Module::eval_ast_as_new(rhai::Scope::new(), &ast, &engine).unwrap();
        lib.fill_with(&my_module);

        // combine_with_exported_module!(lib, "rand", rand_functions);
    }
}

// #[export_module]
// mod rand_functions {
//     /// Generate a random boolean value with a probability of being `true`.
//     ///
//     /// `probability` must be between `0.0` and `1.0` (inclusive).
//     ///
//     /// # Example
//     ///
//     /// ```rhai
//     /// let decision = rand(0.01);  // 1% probability
//     ///
//     /// if decision {
//     ///     print("You hit the Jackpot!")
//     /// }
//     /// ```
//     #[rhai_fn(return_raw)]
//     pub fn rand_bool_with_probability(probability: FLOAT) -> Result<bool, Box<EvalAltResult>> {
//         if probability < 0.0 || probability > 1.0 {
//             Err(EvalAltResult::ErrorArithmetic(
//                 format!(
//                     "Invalid probability (must be between 0.0 and 1.0): {}",
//                     probability
//                 ),
//                 Position::NONE,
//             )
//             .into())
//         } else {
//             Ok(rand::thread_rng().gen_bool(probability as f64))
//         }
//     }
// }

fn aggregate_functions() -> String {
    String::new()
        + include_str!("../scripts/max.rhai")
        + include_str!("../scripts/maxk.rhai")
        + include_str!("../scripts/argmax.rhai")
        + include_str!("../scripts/min.rhai")
        + include_str!("../scripts/argmin.rhai")
        + include_str!("../scripts/mink.rhai")
        + include_str!("../scripts/bounds.rhai")
        + include_str!("../scripts/mean.rhai")
        + include_str!("../scripts/variance.rhai")
        + include_str!("../scripts/std.rhai")
        + include_str!("../scripts/mode.rhai")
        + include_str!("../scripts/median.rhai")
        + include_str!("../scripts/iqr.rhai")
        + include_str!("../scripts/prctile.rhai")
        + include_str!("../scripts/interp1.rhai")
        + include_str!("../scripts/linspace.rhai")
        + include_str!("../scripts/logspace.rhai")
        + zeros()
        + zeros_square()
        + ones()
        + ones_square()
        + rand()
        + rand_square()
        + rand_matrix()
        + include_str!("../scripts/constants.rhai")
}

/// ```
/// # use rhai::INT;
/// # use rhai_lab::one_line_eval;
/// let result: INT = one_line_eval("max(41, 42)").unwrap();
/// assert_eq!(result, 42);
/// ```
/// ```
/// # use rhai::INT;
/// # use rhai_lab::one_line_eval;
/// let result: INT = one_line_eval("min(43, 42)").unwrap();
/// assert_eq!(result, 42);
/// ```
/// ```
/// # use rhai::INT;
/// # use rhai_lab::one_line_eval;
/// let result: INT = one_line_eval("max([41, 42, -1, 7, 2])").unwrap();
/// assert_eq!(result, 42);
/// ```
/// ```
/// # use rhai::INT;
/// # use rhai_lab::one_line_eval;
/// let result: INT = one_line_eval("argmax([43, 42, 500])").unwrap();
/// assert_eq!(result, 2);
/// ```
/// ```
/// # use rhai::INT;
/// # use rhai_lab::one_line_eval;
/// let result: INT = one_line_eval("min([43, 42, 500])").unwrap();
/// assert_eq!(result, 42);
/// ```
/// ```
/// # use rhai::INT;
/// # use rhai_lab::one_line_eval;
/// let result: INT = one_line_eval("argmin([43, 42, -500])").unwrap();
/// assert_eq!(result, 2);
/// ```
/// ```
/// # use rhai::Array;
/// # use rhai_lab::one_line_eval;
/// let result: Array = one_line_eval("mink([32, 15, -7, 10, 1000, 41, 42], 3)").unwrap();
/// println!("{result:?}");
/// assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![-7, 10, 15]);
/// ```
/// ```
/// # use rhai::Array;
/// # use rhai_lab::one_line_eval;
/// let result: Array = one_line_eval("maxk([32, 15, -7, 10, 1000, 41, 42], 3)").unwrap();
/// println!("{result:?}");
/// assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![41, 42, 1000]);
/// ```
/// ```
/// # use rhai::Array;
/// # use rhai_lab::one_line_eval;
/// let result: Array = one_line_eval("bounds([32, 15, -7, 10, 1000, 41, 42])").unwrap();
/// println!("{result:?}");
/// assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![-7, 1000]);
/// ```
/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("mean([1, 2, 3])").unwrap();
/// assert_eq!(result, 2.0);
/// ```
/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("variance([1, 2, 3])").unwrap();
/// assert_eq!(result, 1.0);
/// ```
/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("std([1, 2, 3])").unwrap();
/// assert_eq!(result, 1.0);
/// ```
/// ```
/// # use rhai::INT;
/// # use rhai_lab::one_line_eval;
/// let result: INT = one_line_eval("mode([1, 2, 2, 2, 2, 3])").unwrap();
/// assert_eq!(result, 2);
/// ```
///
/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("median([1, 1, 1, 1, 2, 5, 6, 7, 8])").unwrap();
/// assert_eq!(result, 2.0);
/// ```
///
/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("iqr([1, 1, 1, 1, 1, 1, 1, 5, 6, 9, 9, 9, 9, 9, 9, 9, 9])").unwrap();
/// assert_eq!(result, 8.0);
/// ```
///
/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("prctile([1, 2, 0, 3, 4], 50)").unwrap();
/// assert_eq!(result, 2.0);
/// ```
///
/// ```
/// # use rhai::Array;
/// # use rhai_lab::one_line_eval;
/// let result: Array = one_line_eval("linspace(1, 2, 3)").unwrap();
/// assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![1.0, 1.5, 2.0]);
/// ```
///
/// ```
/// # use rhai::Array;
/// # use rhai_lab::one_line_eval;
/// # let result: Array = one_line_eval("logspace(1, 3, 3)").unwrap();
/// # assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![10.0, 100.0, 1000.0]);
/// ```
///
/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("interp1([0, 1], [1, 2], 0.5)").unwrap();
/// assert_eq!(result, 1.5);
/// ```
///

fn zeros_square() -> &'static str {
    "fn zeros(n) {
        zeros(n, n)
    };"
}

fn zeros() -> &'static str {
    "fn zeros(nx, ny) {
        let row = [];
        row.pad(ny, 0.0);

        let matrix = [];
        matrix.pad(nx, row);

        matrix
    };"
}

fn ones_square() -> &'static str {
    "fn ones(n) {
        ones(n, n)
    };"
}

fn ones() -> &'static str {
    "fn ones(nx, ny) {
        let row = [];
        row.pad(ny, 1.0);

        let matrix = [];
        matrix.pad(nx, row);

        matrix
    };"
}

/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("rand()").unwrap();
/// assert!(result < 1.0 && result > 0.0);
fn rand() -> &'static str {
    "fn rand() {
        rand_float()
    };"
}

fn rand_square() -> &'static str {
    "fn rand(n) {
        rand(n, n)
    };"
}

fn rand_matrix() -> &'static str {
    "fn rand(nx, ny) {
        let matrix = zeros(nx, ny);
        for i in 0..nx {
            for j in 0..ny {
                m[i][j] = rand();
            }
        }
    };"
}

/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("pi").unwrap();
/// assert_eq!(result, std::f64::consts::PI);
/// ```
/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("e").unwrap();
/// assert_eq!(result, std::f64::consts::E);
pub fn one_line_eval<T: Clone + 'static>(script: &str) -> Result<T, Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_global_module(LabPackage::new().as_shared_module());
    engine.eval::<T>(script)
}
