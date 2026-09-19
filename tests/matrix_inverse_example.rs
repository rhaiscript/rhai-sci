#![cfg(feature = "nalgebra")]
use rhai::{packages::Package, Array, Engine};
use rhai_sci::SciPackage;

#[test]
fn matrix_inverse_example_produces_expected_result() {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());

    let result: Array = engine
        .eval("inv([[1, 2], [3, 4]])")
        .expect("script evaluation should succeed");

    let first_row: Array = result[0].clone().cast::<Array>();
    let second_row: Array = result[1].clone().cast::<Array>();

    let r0: Vec<f64> = first_row.into_iter().map(|v| v.cast::<f64>()).collect();
    let r1: Vec<f64> = second_row.into_iter().map(|v| v.cast::<f64>()).collect();

    assert!((r0[0] + 2.0).abs() < f64::EPSILON);
    assert!((r0[1] - 1.0).abs() < f64::EPSILON);
    assert!((r1[0] - 1.5).abs() < f64::EPSILON);
    assert!((r1[1] + 0.5).abs() < f64::EPSILON);
}
