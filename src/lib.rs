#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]
#![doc = include_str!(concat!(env!("OUT_DIR"), "/rhai-sci-docs.md"))]
#![doc = include_str!("../docs/highlight.html")]

use rhai::{def_package, packages::Package, plugin::*, Engine, EvalAltResult};
use rhai_rand::RandomPackage;

def_package! {
    /// Package for scientific computing
    pub SciPackage(lib) {

        // Load random package
        RandomPackage::init(lib);

        combine_with_exported_module!(lib, "linalg", linalg_functions);

        // Load scripts
        let engine = Engine::new();
        let ast = engine.compile(include_str!(concat!(env!("OUT_DIR"), "/rhai-sci-compiled.txt"))).unwrap();
        let my_module = Module::eval_ast_as_new(rhai::Scope::new(), &ast, &engine).unwrap();
        lib.fill_with(&my_module);
    }
}

/// This provides the ability to easily evaluate a line (or lines) of code without explicitly
/// setting up a script engine
/// ```
/// use rhai_sci::eval;
/// use rhai::FLOAT;
/// print!("{:?}", eval::<FLOAT>("let x = max(5, 2); x + min(3, 72)"));
/// ```
pub fn eval<T: Clone + 'static>(script: &str) -> Result<T, Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine.eval::<T>(script)
}

#[export_module]
mod linalg_functions {
    use nalgebra::DMatrix;
    use rhai::serde::{from_dynamic, to_dynamic};
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT};

    #[rhai_fn(return_raw)]
    // TODO FIX INVERSION - output is currently 1D
    pub fn invert_matrix(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        // Turn into Vec<Vec<f64>>
        let matrix_as_vec = matrix
            .into_iter()
            .map(|x| from_dynamic(&x).unwrap())
            .collect::<Vec<Vec<f64>>>();

        // Turn into DMatrixs
        let dm = DMatrix::from_fn(matrix_as_vec.len(), matrix_as_vec[0].len(), |i, j| {
            matrix_as_vec[i][j]
        });

        let dm = dm.try_inverse().expect("Cannot invert");

        // Turn into Vec<Dynamic>
        let matrix_as_array = dm
            .into_iter()
            .map(|x| to_dynamic(&x).unwrap())
            .collect::<Vec<Dynamic>>();
        Ok(matrix_as_array)
    }
}
