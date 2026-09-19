#[cfg(all(feature = "io", feature = "nalgebra"))]
fn main() -> Result<(), Box<rhai::EvalAltResult>> {
    use rhai::{packages::Package, Array, Engine, Scope};
    use rhai_sci::SciPackage;

    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    let observations = engine.eval::<Array>(r#"read_matrix("examples/data/calibration.csv")"#)?;
    let mut scope = Scope::new();
    scope.push("observations", observations);
    engine.run_with_scope(&mut scope, include_str!("regression_workflow.rhai"))
}

#[cfg(not(all(feature = "io", feature = "nalgebra")))]
fn main() {
    eprintln!("This CSV example requires the io and nalgebra features.");
}
