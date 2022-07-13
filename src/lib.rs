#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]
#![doc = include_str!("./DOCS.md")]

use rhai::{def_package, packages::Package, plugin::*, Array, EvalAltResult, Position, FLOAT, INT};
use rhai_rand::RandomPackage;
use std::ops::{Range, RangeInclusive};

def_package! {
    /// Package for scientific computing
    pub SciPackage(lib) {

        // Load random package
        RandomPackage::init(lib);

        // Load scripts
        let engine = Engine::new();
        let ast = engine.compile(aggregate_functions()).unwrap();
        let my_module = Module::eval_ast_as_new(rhai::Scope::new(), &ast, &engine).unwrap();
        lib.fill_with(&my_module);
    }
}

/// This function aggregates all of the rhai scripts that will make up the library.
fn aggregate_functions() -> String {
    String::new()
        + include_str!("../scripts/max.rhai")
        + include_str!("../scripts/maxk.rhai")
        + include_str!("../scripts/argmax.rhai")
        + include_str!("../scripts/min.rhai")
        + include_str!("../scripts/argmin.rhai")
        + include_str!("../scripts/eye.rhai")
        + include_str!("../scripts/mink.rhai")
        + include_str!("../scripts/diag.rhai")
        + include_str!("../scripts/bounds.rhai")
        + include_str!("../scripts/size.rhai")
        + include_str!("../scripts/mean.rhai")
        + include_str!("../scripts/numel.rhai")
        + include_str!("../scripts/ndims.rhai")
        + include_str!("../scripts/variance.rhai")
        + include_str!("../scripts/std.rhai")
        + include_str!("../scripts/mode.rhai")
        + include_str!("../scripts/median.rhai")
        + include_str!("../scripts/iqr.rhai")
        + include_str!("../scripts/rms.rhai")
        + include_str!("../scripts/prctile.rhai")
        + include_str!("../scripts/sum.rhai")
        + include_str!("../scripts/cumsum.rhai")
        + include_str!("../scripts/interp1.rhai")
        + include_str!("../scripts/linspace.rhai")
        + include_str!("../scripts/logspace.rhai")
        + include_str!("../scripts/zeros.rhai")
        + include_str!("../scripts/ones.rhai")
        + include_str!("../scripts/rand.rhai")
        + include_str!("../scripts/constants.rhai")
        + include_str!("../scripts/transpose.rhai")
        + include_str!("../scripts/meshgrid.rhai")
        + include_str!("../scripts/diff.rhai")
        + include_str!("../scripts/prod.rhai")
        + include_str!("../scripts/cumprod.rhai")
        + include_str!("../scripts/cummax.rhai")
        + include_str!("../scripts/cummin.rhai")
        + include_str!("../scripts/cumtrapz.rhai")
        + include_str!("../scripts/trapz.rhai")
        + include_str!("../scripts/fliplr.rhai")
        + include_str!("../scripts/flipud.rhai")
        + include_str!("../scripts/rot90.rhai")
        + include_str!("../scripts/movmean.rhai")
        + include_str!("../scripts/movmedian.rhai")
        + include_str!("../scripts/movstd.rhai")
        + include_str!("../scripts/movvar.rhai")
        + include_str!("../scripts/mad.rhai")
        + include_str!("../scripts/movmad.rhai")
        + include_str!("../scripts/mov.rhai")
        + include_str!("../scripts/movsum.rhai")
        + include_str!("../scripts/movmax.rhai")
        + include_str!("../scripts/movmin.rhai")
        + include_str!("../scripts/movprod.rhai")
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
