#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]
#![doc = include_str!(concat!(env!("OUT_DIR"), "/rhai-sci-docs.md"))]
#![doc = include_str!("../docs/highlight.html")]

mod patterns;
use patterns::*;
use rhai::{def_package, packages::Package, plugin::*, Engine, EvalAltResult};
mod matrices_and_arrays;
use matrices_and_arrays::matrix_functions;
mod statistics;
use statistics::stats;
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
mod moving;
use moving::moving_functions;
mod sets;
use sets::set_functions;
mod validate;
use validate::validation_functions;

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
        combine_with_exported_module!(lib, "rhai_sci_moving", moving_functions);
        combine_with_exported_module!(lib, "rhai_sci_validation", validation_functions);
    }
}

/// This provides the ability to easily evaluate a line (or lines) of code without explicitly
/// setting up a script engine
/// ```
/// use rhai_sci::eval;
/// use rhai::FLOAT;
/// print!("{:?}", eval::<FLOAT>("let x = max(5, 2); x + min(3, 72)"));
/// ```
pub fn eval<T: Clone + std::marker::Send + std::marker::Sync + 'static>(
    script: &str,
) -> Result<T, Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine.eval::<T>(script)
}
