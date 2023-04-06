fn main() {
    const SCRIPT: &str = include_str!("download_and_regress.rhai");

    #[cfg(all(feature = "nalgebra", feature = "io"))]
    {
        use rhai::{packages::Package, Dynamic, Engine};
        use rhai_sci::SciPackage;
        // Create a new Rhai engine
        let mut engine = Engine::new();

        // Add the rhai-sci package to the new engine
        engine.register_global_module(SciPackage::new().as_shared_module());

        // Now run your code
        let fitting_results = engine.run(SCRIPT).unwrap();
        println!("{:?}", fitting_results);
    }
}
