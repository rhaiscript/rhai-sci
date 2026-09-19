//! Demonstrates computing the inverse of a matrix using rhai-sci.

fn main() {
    #[cfg(feature = "nalgebra")]
    {
        use rhai::{packages::Package, Engine};
        use rhai_sci::SciPackage;

        // Create a new Rhai engine
        let mut engine = Engine::new();

        // Add the rhai-sci package to the engine
        engine.register_global_module(SciPackage::new().as_shared_module());

        // Run the script that inverts a matrix
        let result = engine
            .run_file("examples/matrix_inversion.rhai".into())
            .expect("script should run");
        println!("{:?}", result);
    }
}
