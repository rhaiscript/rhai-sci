#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]
#![doc = include_str!(concat!(env!("OUT_DIR"), "/rhai-sci-docs.md"))]
#![doc = include_str!("../docs/highlight.html")]

use rhai::{def_package, packages::Package, plugin::*, Engine, EvalAltResult};
mod matrix;
use matrix::matrix_functions;
mod basic_statistics;
use basic_statistics::stats;
mod misc;
use misc::misc_functions;
mod cumulative;
use cumulative::cum_functions;
mod integration_and_differentiation;
use integration_and_differentiation::int_and_diff;
mod assertions;
use assertions::assert_functions;
mod constants;
use constants::constant_definitions;
mod sets;
use sets::set_functions;

def_package! {
    /// Package for scientific computing
    pub SciPackage(lib) {

        combine_with_exported_module!(lib, "rhai_sci_matrix_function", matrix_functions);
        combine_with_exported_module!(lib, "rhai_sci_miscellaneous_functions", misc_functions);
        combine_with_exported_module!(lib, "rhai_sci_basic_stats", stats);
        combine_with_exported_module!(lib, "rhai_sci_cumulative", cum_functions);
        combine_with_exported_module!(lib, "rhai_sci_int_and_diff", int_and_diff);
        combine_with_exported_module!(lib, "rhai_sci_assertions", assert_functions);
        combine_with_exported_module!(lib, "rhai_sci_constants", constant_definitions);
        combine_with_exported_module!(lib, "rhai_sci_sets", set_functions);

        // Load scripts - TODO: Remove this block once rust->Rhai updates are done
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
