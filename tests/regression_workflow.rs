#![cfg(feature = "nalgebra")]

use rhai::{packages::Package, Array, Dynamic, Engine, Map, Scope};
use rhai_sci::SciPackage;

fn engine() -> Engine {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine
}

fn check_calibration(engine: &Engine, observations: Array) {
    let mut scope = Scope::new();
    scope.push("observations", observations);
    let result: Map = engine
        .eval_with_scope(
            &mut scope,
            include_str!("../examples/regression_workflow.rhai"),
        )
        .unwrap();
    let parameters = result["parameters"].clone().cast::<Array>();
    // The chosen residuals are orthogonal to the intercept and both predictors.
    assert!((result["intercept"].as_float().unwrap() - 1.0).abs() < 1e-10);
    assert!((parameters[0].as_float().unwrap() - 2.0).abs() < 1e-10);
    assert!((parameters[1].as_float().unwrap() - 0.5).abs() < 1e-10);
    assert!(result["residual_mean"].as_float().unwrap().abs() < 1e-10);
    assert!((result["rmse"].as_float().unwrap() - 0.02_f64.sqrt()).abs() < 1e-10);
    assert!((result["residual_std"].as_float().unwrap() - 0.024_f64.sqrt()).abs() < 1e-10);
    let predictions = result["predictions"].clone().cast::<Array>();
    for (prediction, expected) in predictions.iter().zip([1.0, 3.5, 5.0, 7.5, 9.0, 11.5]) {
        assert!((prediction.as_float().unwrap() - expected).abs() < 1e-10);
    }
}

#[test]
fn embedded_regression_predicts_and_summarizes_residuals() {
    let engine = engine();
    let observations: Array = engine
        .eval("[[0, 0, 1.1], [1, 1, 3.3], [2, 0, 5.1], [3, 1, 7.6], [4, 0, 8.8], [5, 1, 11.6]]")
        .unwrap();
    check_calibration(&engine, observations);
}

#[cfg(feature = "io")]
#[test]
fn csv_regression_workflow_loads_the_bundled_data() {
    let engine = engine();
    let observations = engine
        .eval::<Array>(r#"read_matrix("examples/data/calibration.csv")"#)
        .unwrap();
    check_calibration(&engine, observations);
}

#[test]
fn regression_accepts_vector_responses_and_reports_invalid_data() {
    let engine = engine();
    for response in [
        "[1.1, 2.8, 5.1]",
        "row([1.1, 2.8, 5.1])",
        "col([1.1, 2.8, 5.1])",
    ] {
        let fit: Map = engine
            .eval(&format!("regress(col([0, 1, 2]), {response})"))
            .unwrap();
        assert!((fit["intercept"].as_float().unwrap() - 1.0).abs() < 1e-10);
        let parameters = fit["parameters"].clone().cast::<Array>();
        assert!((parameters[0].as_float().unwrap() - 2.0).abs() < 1e-10);
    }
    for script in [
        "regress([], [])",
        "regress([[1], [2, 3]], [1, 2])",
        "regress(col([1, 2, 3]), [1, 2])",
        "regress(col([1, 2, 3]), [[1, 2], [3, 4]])",
        "regress(col([1, 2, 3]), [1, \"x\", 3])",
        "regress(col([1, 2, 3]), [inf, inf, inf])",
    ] {
        assert!(engine.eval::<Dynamic>(script).is_err(), "{script}");
    }
}
