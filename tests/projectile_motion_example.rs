use rhai::{packages::Package, Engine, Map};
use rhai_sci::SciPackage;

#[test]
fn projectile_motion_example_produces_expected_result() {
    // Arrange: set up engine with rhai-sci package
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());

    // Act: evaluate projectile motion script
    let result: Map = engine
        .eval_file("examples/projectile_motion.rhai".into())
        .expect("script evaluation should succeed");

    // Assert: compare against analytical solution
    let max_height = result["max_height"].clone().cast::<f64>();
    let time_of_flight = result["time_of_flight"].clone().cast::<f64>();
    let range = result["range"].clone().cast::<f64>();

    let expected_max_height =
        (25.0_f64.powi(2) * (45_f64.to_radians().sin().powi(2))) / (2.0 * 9.81);
    let expected_time_of_flight = 2.0 * 25.0 * 45_f64.to_radians().sin() / 9.81;
    let expected_range = (25.0_f64.powi(2) * (2.0 * 45_f64.to_radians()).sin()) / 9.81;

    assert!((max_height - expected_max_height).abs() < 1e-2);
    assert!((time_of_flight - expected_time_of_flight).abs() < 1e-6);
    assert!((range - expected_range).abs() < 1e-6);
}
