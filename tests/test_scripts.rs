use rhai_sci::SciPackage;
use rhai::{Engine, packages::Package};

#[test]
fn script_tests() {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    let result: bool = engine.eval_file("tests/rhai-lab-test.txt".into()).unwrap();
    assert!(result);
}