#![cfg(feature = "nalgebra")]

use rhai::{packages::Package, Array, Engine, Map};
use rhai_sci::SciPackage;

#[test]
fn neural_network_backprop_example_learns_xor() {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());

    let result: Map = engine
        .eval_file("examples/neural_network_backprop.rhai".into())
        .expect("script evaluation should succeed");

    let initial_loss = result["initial_loss"].clone().cast::<f64>();
    let final_loss = result["final_loss"].clone().cast::<f64>();
    let predictions = result["predictions"].clone().cast::<Array>();
    let predictions = predictions
        .into_iter()
        .map(|value| value.cast::<f64>())
        .collect::<Vec<_>>();

    assert!(initial_loss > 0.45);
    assert!(final_loss < 0.02);
    assert!(final_loss < initial_loss);
    assert!(predictions[0] < 0.15);
    assert!(predictions[1] > 0.85);
    assert!(predictions[2] > 0.85);
    assert!(predictions[3] < 0.15);
}
