use rhai::{
    def_package,
    packages::Package,
    plugin::*,
    EvalAltResult, Position, INT, {Array, FLOAT},
};
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
        + max()
        + max_array()
        + maxk()
        + argmax()
        + min()
        + min_array()
        + mink()
        + argmin()
        + bounds()
        + mode()
        + mean()
        + median()
        + iqr()
        + prctile()
        + interp1()
        + linspace()
        + logspace()
        + zeros()
        + zeros_square()
        + ones()
        + ones_square()
        + rand()
        + rand_square()
        + rand_matrix()
        + pi()
        + e()
}

/// ```
/// # use rhai::INT;
/// # use rhai_lab::one_line_eval;
/// let result: INT = one_line_eval("max(41, 42)").unwrap();
/// assert_eq!(result, 42);
/// ```
fn max() -> &'static str {
    "fn max(a, b) {
        if (a > b) { 
            return a;
        } else {
            return b;
        }
    };"
}

/// ```
/// # use rhai::INT;
/// # use rhai_lab::one_line_eval;
/// let result: INT = one_line_eval("min(43, 42)").unwrap();
/// assert_eq!(result, 42);
/// ```
fn min() -> &'static str {
    "fn min(a, b) {
        if (a < b) { 
            return a;
        } else {
            return b;
        }
    };"
}

fn max_array() -> &'static str {
    "fn max(arr) { 
        arr.sort(); 
        arr[-1]
    };"
}

/// ```
/// # use rhai::INT;
/// # use rhai_lab::one_line_eval;
/// let result: INT = one_line_eval("argmax([43, 42, 500])").unwrap();
/// assert_eq!(result, 2);
/// ```
fn argmax() -> &'static str {
    "fn argmax(arr) { 
        let original = arr;
        arr.sort(); 
        original.index_of(arr[-1])
    };"
}

fn min_array() -> &'static str {
    "fn min(arr) {
        arr.sort(); 
        arr[0]; 
    };"
}

/// ```
/// # use rhai::INT;
/// # use rhai_lab::one_line_eval;
/// let result: INT = one_line_eval("argmin([43, 42, -500])").unwrap();
/// assert_eq!(result, 2);
/// ```
fn argmin() -> &'static str {
    "fn argmin(arr) { 
        let original = arr;
        arr.sort(); 
        original.index_of(arr[0])
    };"
}

fn mink() -> &'static str {
    "fn mink(arr, k) {
        arr.sort(); 
        arr.extract(0..k); 
    };"
}

fn maxk() -> &'static str {
    "fn maxk(arr, k) {
        arr.sort();
        let L = arr.len();
        arr.extract((L-k)..L);
    };"
}

fn bounds() -> &'static str {
    "fn bounds(arr) {
        [min(arr), max(arr)]
    };"
}

fn mean() -> &'static str {
    "fn mean(arr) {
        arr.reduce(|sum, v| sum + v*1.0, 0)/arr.len()
    };"
}

fn mode() -> &'static str {
    "fn mode(arr) {
        arr.sort();
        let unique = arr;
        unique.dedup();
        let count = []; count.pad(unique.len(), 0);
        for el in arr {
            let idx = unique.index_of(el);
            count[idx] = count[idx] + 1;
        }
        let mode_idx = count.index_of(max(count));
        unique[mode_idx]
    };"
}

fn median() -> &'static str {
    "fn median(arr) {
        prctile(arr, 50);
    };"
}

fn iqr() -> &'static str {
    "fn iqr(arr) {
        prctile(arr, 75) - prctile(arr, 25)
    };"
}

/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("prctile([0, 1, 2, 3, 4], 50)").unwrap();
/// assert_eq!(result, 2.0);
/// ```
fn prctile() -> &'static str {
    "fn prctile(arr, p) {
        let x = linspace(0, 100, arr.len());
        interp1(x, arr, p)
    };"
}

fn linspace() -> &'static str {
    "fn linspace(x1, x2, n) {
        let arr = [1.0*x1];
        let int = (1.0*x2 - x1)/(n-1); 
        for i in 0..(n-2) {
            arr.push(arr[-1] + int)
        }
        arr.push(1.0*x2);
        arr
    };"
}

fn logspace() -> &'static str {
    "fn logspace(a, b, n) {
        let exponents = linspace(a, b, n);
        exponents.map(|e| 10**e)
    };"
}

/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("interp1([0, 1], [1, 2], 0.5)").unwrap();
/// assert_eq!(result, 1.5);
/// ```
fn interp1() -> &'static str {
    "fn interp1(x, y, xq) { 
        x.sort();
        let b = 0;
        for idx in 0..x.len() {
            if x[idx] < xq {
                b = idx;
                break;
            }
        }
        let a = b - 1;
        y[a] + (xq - x[a])*(y[b] - y[a])/(x[b] - x[a])
    };"
}

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
fn pi() -> &'static str {
    "const pi = 3.14159265358979323846264338327950288; export pi;"
}

/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::one_line_eval;
/// let result: FLOAT = one_line_eval("e").unwrap();
/// assert_eq!(result, std::f64::consts::E);
fn e() -> &'static str {
    "const e = 2.71828182845904523536028747135266250; export e;"
}

pub fn one_line_eval<T: Clone + 'static>(script: &str) -> Result<T, Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_global_module(LabPackage::new().as_shared_module());
    engine.eval::<T>(script)
}

pub trait Eval<T> {
    fn eval(&self) -> Result<T, Box<EvalAltResult>>;
}

/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::Eval;
/// let result: FLOAT = "e".to_string().eval().unwrap();
/// assert_eq!(result, std::f64::consts::E);
impl<T: Clone + 'static> Eval<T> for String {
    fn eval(&self) -> Result<T, Box<EvalAltResult>> {
        let mut engine = Engine::new();
        engine.register_global_module(LabPackage::new().as_shared_module());
        engine.eval::<T>(self)
    }
}

/// ```
/// # use rhai::FLOAT;
/// # use rhai_lab::Eval;
/// let result: FLOAT = "e".eval().unwrap();
/// assert_eq!(result, std::f64::consts::E);
impl<T: Clone + 'static> Eval<T> for str {
    fn eval(&self) -> Result<T, Box<EvalAltResult>> {
        self.to_owned().eval()
    }
}
