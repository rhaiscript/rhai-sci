use rhai::{packages::Package, Dynamic, Engine};
use rhai_sci::SciPackage;

fn main() {
    #[cfg(all(feature = "matrix", feature = "io"))]
    {
        // Create a new Rhai engine
        let mut engine = Engine::new();

        // Add the rhai-sci package to the new engine
        engine.register_global_module(SciPackage::new().as_shared_module());

        // Now run your code
        engine
            .eval_file::<Dynamic>("examples/download_and_regress.rhai".into())
            .unwrap();
    }
}
