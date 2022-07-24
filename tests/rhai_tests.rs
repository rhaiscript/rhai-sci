//! asdf

#[cfg(test)]
mod rhai_tests {
    use rhai::{packages::Package, Dynamic, Engine, INT};
    use rhai_sci::SciPackage;

    #[test]
    fn test_matrices() {
        // Create a new Rhai engine
        let mut engine = Engine::new();

        // Add the rhai-sci package to the new engine
        engine.register_global_module(SciPackage::new().as_shared_module());

        // Now run your code
        engine.run_file("tests/matrix_tests.rhai".into()).unwrap();
    }
}
