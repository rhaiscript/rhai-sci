use rhai::{packages::Package, Engine, EvalAltResult, INT};
use rhai_sci::LabPackage;

use rhai::FLOAT;

// #[test]
// fn test_rand() -> Result<(), Box<EvalAltResult>> {
//     let mut engine = Engine::new();
//
//     engine.register_global_module(LabPackage::new().as_shared_module());
//
//     let first = engine.eval::<bool>("rand_bool_with_probability(1.0)")?;
//     let second = engine.eval::<bool>("rand_bool_with_probability(0.0)")?;
//     print!("{first}");
//
//     assert!(first != second);
//     Ok(())
// }

#[test]
fn test_matlab() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(LabPackage::new().as_shared_module());
    let first = engine.eval::<INT>("max(42, 41)")?;

    assert!(first == 42);

    Ok(())
}
